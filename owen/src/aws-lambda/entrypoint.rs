mod message_queue;
mod message_storage;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use message_queue::MessageQueue;
use message_storage::MessageStorage;

async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<(), Error> {
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let queue = MessageQueue::build(&aws_main_config);
    let storage = MessageStorage::build(&aws_main_config);

    let message_folders = queue.get_message_folders().await.unwrap();

    println!("test 123 elo {message_folders:?}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
