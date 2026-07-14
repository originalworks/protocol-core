use crate::constants::BLOBS_QUEUE_MESSAGE_GROUP_ID;
use alloy::primitives::FixedBytes;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};

use log_macros::log_info;
use serde::{Deserialize, Serialize};

use std::env;

#[derive(Deserialize, Serialize)]
pub struct BlobsQueueMessageBody {
    pub blobhash: String,
    pub owen_instance: String,
}

pub struct ProcessedBlobQueue {
    queue_url: String,
    owen_instance: String,
    sqs_client: aws_sdk_sqs::Client,
}

impl ProcessedBlobQueue {
    pub async fn build() -> anyhow::Result<Self> {
        let queue_url = Self::get_env_var("OWEN_BLOBS_QUEUE_URL");
        let owen_instance = Self::get_env_var("USERNAME");
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

        let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        let sqs_client = aws_sdk_sqs::Client::new(&aws_main_config);

        Ok(Self {
            queue_url,
            owen_instance,
            sqs_client,
        })
    }
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }
    // pub async fn enqueue_blob(
    //     &self,
    //     transaction_data: BlobTransactionData,
    //     image_id: FixedBytes<32>,
    // ) -> anyhow::Result<()> {
    //     let kzg_commitment = Bytes::from(transaction_data.kzg_commitment.to_vec());
    //     let blobhash: FixedBytes<32> = commitment_to_blobhash(&kzg_commitment);

    //     self.send_to_s3(&transaction_data, image_id, &blobhash)
    //         .await?;
    //     self.send_to_sqs(&blobhash).await?;
    //     Ok(())
    // }

    // async fn send_to_s3(
    //     &self,
    //     transaction_data: &BlobTransactionData,
    //     image_id: FixedBytes<32>,
    //     blobhash: &FixedBytes<32>,
    // ) -> anyhow::Result<()> {
    //     log_info!(
    //         "Sending transaction data to S3 for: {}",
    //         blobhash.to_string()
    //     );
    //     let blobs_queue_s3_json_file = BlobsQueueS3JsonFile {
    //         tx_data: transaction_data.clone(),
    //         image_id,
    //     };
    //     let json_string = serde_json::to_string_pretty(&blobs_queue_s3_json_file)?;

    //     let put_object_output = self
    //         .s3_client
    //         .put_object()
    //         .bucket(&self.blobs_temp_storage_bucket_name)
    //         .key(format!("blobs/{}.json", blobhash.to_string()))
    //         .body(ByteStream::from(json_string.into_bytes()))
    //         .content_type("application/json")
    //         .send()
    //         .await?;

    //     println!("put_object_output: {put_object_output:?}");
    //     Ok(())
    // }

    pub async fn send(&self, blobhash: &FixedBytes<32>) -> anyhow::Result<()> {
        log_info!("Enqueue: {}", blobhash.to_string());
        let blobs_queue_message_body = BlobsQueueMessageBody {
            blobhash: blobhash.to_string(),
            owen_instance: self.owen_instance.clone(),
        };
        let json_string = serde_json::to_string_pretty(&blobs_queue_message_body)?;
        let send_message_output = self
            .sqs_client
            .send_message()
            .queue_url(&self.queue_url)
            .message_body(json_string)
            .message_group_id(BLOBS_QUEUE_MESSAGE_GROUP_ID)
            .send()
            .await?;
        println!("send_message_output: {send_message_output:?}");
        Ok(())
    }
}

// pub fn commitment_to_blobhash(commitment: &Bytes) -> FixedBytes<32> {
//     let mut hasher = Sha256::new();
//     hasher.update(commitment);
//     let mut hashed_commitment = hasher.finalize();
//     hashed_commitment[0] = 1;

//     let mut fixed_bytes_input: [u8; 32] = [0u8; 32];
//     fixed_bytes_input.copy_from_slice(&hashed_commitment);

//     FixedBytes::<32>::from(fixed_bytes_input)
// }
