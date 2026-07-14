use crate::blob::BlobTransactionData;
use alloy::primitives::FixedBytes;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::primitives::ByteStream;
use log_macros::log_info;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct BlobsQueueS3JsonFile {
    pub tx_data: BlobTransactionData,
    pub image_id: FixedBytes<32>,
}

pub struct ProcessedBlobStorage {
    pub owen_instance: String,
    pub blobs_temp_storage_bucket_name: String,
    pub s3_client: aws_sdk_s3::Client,
}

impl ProcessedBlobStorage {
    pub async fn build() -> anyhow::Result<Self> {
        let blobs_temp_storage_bucket_name = Self::get_env_var("BLOBS_TEMP_STORAGE_BUCKET_NAME");
        let owen_instance = Self::get_env_var("USERNAME");
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

        let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        let s3_client = aws_sdk_s3::Client::new(&aws_main_config);

        Ok(Self {
            owen_instance,
            s3_client,
            blobs_temp_storage_bucket_name,
        })
    }
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }

    pub async fn send_to_s3(
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
}
