mod message_queue;
mod message_storage;
mod secrets;

use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use blob_codec::errors::OwCodecError;
use lambda_runtime::{service_fn, tracing, LambdaEvent};
use message_queue::MessageQueue;
use message_storage::MessageStorage;
use owen::logger::{init_logging, init_sentry};
use secrets::set_secret_envs;
use std::fs;

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
    let storage = MessageStorage::build(&aws_main_config);

    let message_folders = queue.get_message_folders().await.unwrap();
    if message_folders.is_empty() {
        tracing::info!("No message folders found, queue is empty. Terminating execution.");
        return Ok(());
    }
    storage.clear_input_folder().unwrap();

    let local_to_s3_folder_mapping = storage
        .sync_message_folders(&message_folders)
        .await
        .unwrap();

    println!("synced directories: {message_folders:?}");

    match owen::run_with_sentry(&owen_config).await {
        Ok(message_processing_context) => {
            queue
                .sync_message_folder_statuses(
                    local_to_s3_folder_mapping,
                    message_processing_context,
                    message_folders,
                )
                .await
                .unwrap();
        }
        Err(e)
            if e.to_string()
                .to_lowercase()
                .contains(&"blob already submitted".to_string().to_lowercase()) =>
        {
            queue
                .set_message_folders_as_processed(message_folders)
                .await
                .unwrap();
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
        Err(_) => {
            queue
                .set_message_folders_as_rejected(message_folders)
                .await
                .unwrap();
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
    set_secret_envs(&aws_main_config).await.unwrap();
    let owen_config = owen::Config::build();
    let _guard = init_sentry(&owen_config);
    init_logging()?;

    tracing::init_default_subscriber();

    lambda_runtime::run(service_fn(function_handler)).await
}
