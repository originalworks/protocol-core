mod message_queue;
mod secrets;

use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use blob_codec::{errors::OwCodecError, BlobEstimator};
use lambda_runtime::{service_fn, tracing, LambdaEvent};
use log_macros::log_warn;
use message_queue::MessageQueue;
use owen::{
    logger::{init_logging, init_sentry},
    s3_message_storage::MessageStorage,
};
use secrets::set_secret_envs;
use std::path::Path;

async fn build_input_folder(
    queue: &MessageQueue,
    storage: &mut MessageStorage,
) -> anyhow::Result<()> {
    storage.clear_input_folder()?;
    let blob_estimator = BlobEstimator::default();

    loop {
        match queue.reserve_message_folder().await? {
            Some(s3_message_folder) => {
                let local_message_folder = storage.sync_message_folder(&s3_message_folder).await?;
                match blob_estimator.estimate_and_check(Path::new(&storage.input_files_dir)) {
                    Ok(_) => {
                        storage
                            .local_to_s3_folder_mapping
                            .insert(local_message_folder.clone(), s3_message_folder.clone());

                        storage.s3_message_folders.push(s3_message_folder)
                    }
                    Err(err) => {
                        log_warn!(err);
                        std::fs::remove_dir_all(Path::new(&local_message_folder))?;
                        queue
                            .set_single_message_folder_status(
                                s3_message_folder,
                                queue.unprocessed_status_value.clone(),
                            )
                            .await?;
                        break;
                    }
                }
            }
            None => break,
        }
    }
    Ok(())
}

async fn function_handler(
    event: LambdaEvent<CloudWatchEvent>,
) -> Result<(), lambda_runtime::Error> {
    println!("Lambda execution enter");
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let owen_config = owen::Config::build();

    let queue = MessageQueue::build(&aws_main_config);
    let mut storage = MessageStorage::build(&aws_main_config);

    build_input_folder(&queue, &mut storage).await?;

    let local_to_s3_folder_mapping = storage.local_to_s3_folder_mapping.clone();
    let s3_message_folders = storage.s3_message_folders.clone();

    if s3_message_folders.is_empty() {
        tracing::info!("No message folders found, queue is empty. Terminating execution.");
        return Ok(());
    }

    println!("synced directories: {s3_message_folders:?}");

    match owen::run_with_sentry(&owen_config).await {
        Ok(message_processing_context) => {
            queue
                .sync_message_folder_statuses(
                    local_to_s3_folder_mapping,
                    message_processing_context,
                    s3_message_folders,
                )
                .await
                .map_err(|err| format!("Sync message folder statuses error: {err}"))?;
        }
        Err(e)
            if e.to_string()
                .to_lowercase()
                .contains(&"blob already submitted".to_string().to_lowercase()) =>
        {
            queue
                .set_message_folders_status(
                    &s3_message_folders,
                    queue.processed_status_value.clone(),
                )
                .await
                .map_err(|err| {
                    format!("Setting message folder statuses as processed failed: {err}")
                })?;
        }
        Err(e) if e.is::<OwCodecError>() => {
            if let Some(e) = e.downcast_ref::<OwCodecError>() {
                match e {
                    OwCodecError::BlobOverflow {
                        path: _path,
                        loc: _loc,
                    } => {
                        panic!("Not enough space in the blob to pack all messages. Lower the MESSAGES_PER_BLOB variable value")
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            tracing::info!("Unhandled error: {}", e);
            queue
                .set_message_folders_status(
                    &s3_message_folders,
                    queue.rejected_status_value.clone(),
                )
                .await
                .map_err(|err| {
                    format!("Setting message folder statuses as rejected failed: {err}")
                })?;
        }
    };
    println!("Lambda execution leave");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    println!("Lambda cold start");

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    set_secret_envs(&aws_main_config)
        .await
        .map_err(|err| format!("Setting secret envs failed: {err}"))?;
    let owen_config = owen::Config::build();
    let _guard = init_sentry(&owen_config);
    init_logging()?;

    tracing::init_default_subscriber();

    lambda_runtime::run(service_fn(function_handler)).await
}
