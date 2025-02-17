mod message_queue;
mod message_storage;
mod secrets;

// use anyhow::Ok;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use lambda_runtime::{service_fn, tracing, LambdaEvent};
use message_queue::MessageQueue;
use message_storage::MessageStorage;
use owen_cli::logger::{init_logging, init_sentry};
use secrets::set_secret_envs;
use std::fs;

async fn function_handler(
    event: LambdaEvent<CloudWatchEvent>,
) -> Result<(), lambda_runtime::Error> {
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

    // second owen_cli::Config declared here :(
    let owen_config = owen_cli::Config::build();

    owen_cli::run_with_sentry(&owen_config).await.unwrap();

    Ok(())
}

fn main() -> Result<(), lambda_runtime::Error> {
    tracing::init_default_subscriber();

    init_logging()?;
    // first owen_cli::Config declared here :(
    let config = owen_cli::Config::build();
    let _guard = init_sentry(&config);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(lambda_runtime::run(service_fn(function_handler)))
}
