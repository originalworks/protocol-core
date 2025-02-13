mod message_queue;
mod message_storage;
mod secrets;

use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use message_queue::MessageQueue;
use message_storage::MessageStorage;
use secrets::set_secret_envs;
use std::fs;

async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    set_secret_envs(&aws_main_config).await.unwrap();

    let queue = MessageQueue::build(&aws_main_config);
    let storage = MessageStorage::build(&aws_main_config);

    let message_folders = queue.get_message_folders().await.unwrap();
    if message_folders.is_empty() {
        tracing::info!("No message folders found, queue is empty. Terminating execution.");
        return Ok(());
    }

    storage.sync_message_folders(message_folders).await.unwrap();

    let directories_tmp: Vec<String> = fs::read_dir("/tmp")
        .unwrap()
        .into_iter()
        .map(|dir| dir.unwrap().path().to_string_lossy().to_string())
        .collect();

    println!("synced directories: {directories_tmp:?}");

    let owen_config = owen_cli::Config::build();

    owen_cli::run(&owen_config).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
