use crate::{
    aws::storage::egress::ProcessedBlobStorage,
    constants::{AA_BLOB_SENDER_MAX_TX_AGE_SEC, BLOBS_QUEUE_MESSAGE_GROUP_ID},
};
use aa_db_types::BlobStorageType;
use aa_tx_request::blob_tx::BlobTxRequestBody;
use alloy::primitives::FixedBytes;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};

use log_macros::log_info;
use serde::{Deserialize, Serialize};

use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Deserialize, Serialize)]
pub struct BlobsQueueMessageBody {
    pub blobhash: String,
    pub owen_instance: String,
}

pub struct ProcessedBlobQueue {
    queue_url: String,
    owen_instance: String,
    sqs_client: aws_sdk_sqs::Client,
    chain_id: i64,
}

impl ProcessedBlobQueue {
    pub async fn build() -> anyhow::Result<Self> {
        let queue_url = Self::get_env_var("PROCESSED_BLOB_QUEUE_URL");
        let owen_instance = Self::get_env_var("USERNAME");
        let chain_id = Self::get_env_var("CHAIN_ID").parse::<i64>()?;
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
            chain_id,
        })
    }
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }

    pub async fn send(&self, blobhash: &FixedBytes<32>) -> anyhow::Result<()> {
        log_info!("Enqueue: {}", blobhash.to_string());

        let deadline_timestamp = i64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + AA_BLOB_SENDER_MAX_TX_AGE_SEC,
        )?;

        let blobs_queue_message_body = BlobTxRequestBody {
            tx_id: format!("blobhash:{}", blobhash.to_string()),
            requester_id: self.owen_instance.clone(),
            chain_id: self.chain_id,
            deadline_timestamp,
            storage_type: BlobStorageType::S3,
            source_file_path: ProcessedBlobStorage::build_processed_blob_path(blobhash.to_string()),
            use_operator_wallet_id: None,
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

        log_info!("send_message_output: {:?}", send_message_output);
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
