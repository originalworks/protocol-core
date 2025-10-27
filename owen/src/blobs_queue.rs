use alloy::primitives::{Bytes, FixedBytes};
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::primitives::ByteStream;
use log_macros::log_info;

use crate::{blob::BlobTransactionData, constants::BLOBS_QUEUE_MESSAGE_GROUP_ID};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct BlobsQueueMessageBody {
    pub blobhash: String,
    pub owen_instance: String,
}

#[derive(Deserialize, Serialize)]
pub struct BlobsQueueS3JsonFile {
    pub tx_data: BlobTransactionData,
    pub image_id: FixedBytes<32>,
}

pub struct BlobsQueueProducer {
    queue_url: String,
    owen_instance: String,
    blobs_temp_storage_bucket_name: String,
    s3_client: aws_sdk_s3::Client,
    sqs_client: aws_sdk_sqs::Client,
}

impl BlobsQueueProducer {
    pub async fn build() -> anyhow::Result<Self> {
        let queue_url = Self::get_env_var("OWEN_BLOBS_QUEUE_URL");
        let blobs_temp_storage_bucket_name = Self::get_env_var("BLOBS_TEMP_STORAGE_BUCKET_NAME");
        let owen_instance = Self::get_env_var("USERNAME");
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

        let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        let s3_client = aws_sdk_s3::Client::new(&aws_main_config);
        let sqs_client = aws_sdk_sqs::Client::new(&aws_main_config);

        Ok(Self {
            queue_url,
            owen_instance,
            s3_client,
            sqs_client,
            blobs_temp_storage_bucket_name,
        })
    }
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }
    pub async fn enqueue_blob(
        &self,
        transaction_data: BlobTransactionData,
        image_id: FixedBytes<32>,
    ) -> anyhow::Result<()> {
        let kzg_commitment = Bytes::from(transaction_data.kzg_commitment.to_vec());
        let blobhash: FixedBytes<32> = commitment_to_blobhash(&kzg_commitment);

        self.send_to_s3(&transaction_data, image_id, &blobhash)
            .await?;
        self.send_to_sqs(&blobhash).await?;
        Ok(())
    }

    async fn send_to_s3(
        &self,
        transaction_data: &BlobTransactionData,
        image_id: FixedBytes<32>,
        blobhash: &FixedBytes<32>,
    ) -> anyhow::Result<()> {
        log_info!(
            "Sending transaction data to S3 for: {}",
            blobhash.to_string()
        );
        let blobs_queue_s3_json_file = BlobsQueueS3JsonFile {
            tx_data: transaction_data.clone(),
            image_id,
        };
        let json_string = serde_json::to_string_pretty(&blobs_queue_s3_json_file)?;

        let put_object_output = self
            .s3_client
            .put_object()
            .bucket(&self.blobs_temp_storage_bucket_name)
            .key(format!("blobs/{}.json", blobhash.to_string()))
            .body(ByteStream::from(json_string.into_bytes()))
            .content_type("application/json")
            .send()
            .await?;

        println!("put_object_output: {put_object_output:?}");
        Ok(())
    }

    async fn send_to_sqs(&self, blobhash: &FixedBytes<32>) -> anyhow::Result<()> {
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

pub fn commitment_to_blobhash(commitment: &Bytes) -> FixedBytes<32> {
    let mut hasher = Sha256::new();
    hasher.update(commitment);
    let mut hashed_commitment = hasher.finalize();
    hashed_commitment[0] = 1;

    let mut fixed_bytes_input: [u8; 32] = [0u8; 32];
    fixed_bytes_input.copy_from_slice(&hashed_commitment);

    FixedBytes::<32>::from(fixed_bytes_input)
}
