use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{Error, LambdaEvent};
use owen::blobs_queue::BlobsQueueMessageBody;

use crate::s3;

pub(crate) async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
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

    s3::read_blobs(blobhashes).await?;

    Ok(())
}
