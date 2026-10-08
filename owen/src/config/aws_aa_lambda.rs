use crate::aws::queue::ingress::DdexIngestionQueueConfig;
use crate::aws::storage::ingress::DdexIngestionStorageConfig;
use crate::config::core::Config;
pub use log;

#[derive(Debug, serde::Serialize, Clone)]
pub struct AwsAaLambdaConfig {
    pub core_config: Config,
    pub ddex_ingestion_queue_config: DdexIngestionQueueConfig,
    pub ddex_ingestion_storage_config: DdexIngestionStorageConfig,
}

impl AwsAaLambdaConfig {
    pub fn build(core_config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            core_config: core_config.clone(),
            ddex_ingestion_queue_config: DdexIngestionQueueConfig {
                table_name: Config::get_env_var("MESSAGE_STATUS_TABLE_NAME"),
                index_name: Config::get_env_var("PROCESSING_STATUS_INDEX_NAME"),
                pk_name: Config::get_env_var("MESSAGE_FOLDER_ATTRIBUTE_NAME"),
                processing_status_attribute_name: Config::get_env_var(
                    "PROCESSING_STATUS_ATTRIBUTE_NAME",
                ),
                owen_instance_attribute_name: Config::get_env_var("OWEN_INSTANCE_ATTRIBUTE_NAME"),
                unprocessed_status_value: Config::get_env_var("UNPROCESSED_STATUS_VALUE"),
                processed_status_value: Config::get_env_var("PROCESSED_STATUS_VALUE"),
                reserved_status_value: Config::get_env_var("RESERVED_STATUS_VALUE"),
                rejected_status_value: Config::get_env_var("REJECTED_STATUS_VALUE"),
                owen_instance_name: Config::get_env_var("USERNAME"),
            },
            ddex_ingestion_storage_config: DdexIngestionStorageConfig {
                bucket_name: Config::get_env_var("MESSAGES_BUCKET_NAME"),
                input_files_dir: Config::get_env_var("INPUT_FILES_DIR"),
                message_bucket_prefix: Config::get_env_var("MESSAGE_BUCKET_PREFIX"),
                fallback_bucket_name: Config::get_env_var("FALLBACK_BUCKET_NAME"),
            },
        })
    }
}
