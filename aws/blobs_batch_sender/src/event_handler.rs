use aws_lambda_events::event::sqs::SqsEvent;
use blobs_batch_sender::BlobsBatchSenderConfig;
use lambda_runtime::{Error, LambdaEvent};
use owen::{
    blobs_queue::BlobsQueueMessageBody,
    wallet::{OwenWallet, OwenWalletConfig},
};

use crate::{contract::SmartEoaManager, s3::BlobsStorage};

pub(crate) async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    println!("Building...");
    let config = BlobsBatchSenderConfig::build()?;
    let blobs_storage = BlobsStorage::build(&config).await?;
    let owen_wallet_config = OwenWalletConfig::from(&config)?;
    let owen_wallet = OwenWallet::build(&owen_wallet_config).await?;
    let smart_eoa_manager = SmartEoaManager::build(&config, owen_wallet.wallet)?;

    println!("Build complate");
    let blobhashes = extract_blobhashes(event)?;
    println!("Extracted blobhashes: {blobhashes:?}");
    let blob_tx_data_vec = blobs_storage.read(blobhashes).await?;

    smart_eoa_manager.send_batch(blob_tx_data_vec).await?;

    Ok(())
}

fn extract_blobhashes(event: LambdaEvent<SqsEvent>) -> Result<Vec<String>, Error> {
    let messages: Vec<BlobsQueueMessageBody> = event
        .payload
        .records
        .iter()
        .map(|sqs_message| {
            let body = sqs_message
                .body
                .clone()
                .expect("body not found for sqs record");
            let blobs_queue_message_body: BlobsQueueMessageBody =
                serde_json::from_str(&body).expect("failed to parse sqs message body");
            blobs_queue_message_body
        })
        .collect();

    let blobhashes: Vec<String> = messages
        .iter()
        .map(|message| message.blobhash.clone())
        .collect();
    Ok(blobhashes)
}
