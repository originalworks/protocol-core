mod message_queue;
mod secrets;

use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use blob_codec::errors::OwCodecError;
use lambda_runtime::{service_fn, tracing, LambdaEvent};
use message_queue::MessageQueue;
use owen::{
    logger::{init_logging, init_sentry},
    s3_message_storage::MessageStorage,
};
use secrets::set_secret_envs;

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

    storage
        .clear_input_folder()
        .map_err(|err| format!("Clearing input folder error: {err}"))?;
    storage.download_message_folders().await?;

    let local_to_s3_folder_mapping = storage.local_to_s3_folder_mapping.clone();
    let s3_message_folders = storage.s3_message_folders.clone();

    queue
        .set_message_folders_status(&s3_message_folders, queue.reserved_status_value.clone())
        .await?;

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
