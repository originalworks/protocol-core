#[cfg(feature = "aws-integration")]
mod blobs_queue;

mod blob;
pub mod constants;
mod contracts;
mod image_processor;
mod ipfs;
pub mod logger;
pub mod output_generator;
use alloy::primitives::Address;
use blob::BlobTransactionData;
use contracts::ContractsManager;
use ddex_parser::ParserError;
pub use log;
use log_macros::log_error;
use output_generator::MessageDirProcessingContext;
use sentry::User;
use serde_json::json;
use std::env;
use std::str::FromStr;

#[cfg(any(feature = "aws-integration", feature = "local-s3"))]
pub mod s3_message_storage;

pub fn is_local() -> bool {
    matches!(
        std::env::var("LOCAL")
            .unwrap_or_else(|_| "false".to_string())
            .as_str(),
        "1" | "true"
    )
}

#[derive(Debug, PartialEq, serde::Serialize, Clone)]
pub enum IpfsInterface {
    KUBO,
    PINATA,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub private_key: String,
    pub folder_path: String,
    pub local_ipfs: bool,
    pub output_files_dir: String,
    pub username: String,
    pub environment: String,
    pub ddex_sequencer_address: Address,
    pub disable_telemetry: bool,
    pub storacha_bridge_url: String,
    pub ipfs_api_base_url: String,
    pub use_kms: bool,
    pub signer_kms_id: Option<String>,
    pub use_batch_sender: bool,
}

impl Config {
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }

    pub fn build() -> Config {
        if is_local() {
            println!("Running local setup");
            dotenvy::from_filename(".env.local").unwrap();
        } else {
            dotenvy::dotenv().ok();
        }

        let mut args = std::env::args();
        args.next();

        let folder_path = args
            .next()
            .unwrap_or_else(|| Config::get_env_var("INPUT_FILES_DIR").to_string());

        let rpc_url = Config::get_env_var("RPC_URL");
        let private_key = Config::get_env_var("PRIVATE_KEY");
        let local_ipfs = matches!(
            std::env::var("LOCAL_IPFS")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );
        let output_files_dir = Config::get_env_var("OUTPUT_FILES_DIR");
        let username = Config::get_env_var("USERNAME");
        let environment = Config::get_env_var("ENVIRONMENT");
        let ddex_sequencer_address = Address::from_str(
            std::env::var("DDEX_SEQUENCER_ADDRESS")
                .unwrap_or_else(|_| constants::DDEX_SEQUENCER_ADDRESS.to_string())
                .as_str(),
        )
        .expect("Could not parse ddex sequencer address");

        let disable_telemetry: bool = matches!(
            std::env::var("DISABLE_TELEMETRY")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );
        let mut storacha_bridge_url = env::var("STORACHA_BRIDGE_URL")
            .unwrap_or_else(|_| constants::DEFAULT_STORACHA_BRIDGE_URL.to_string());

        if !storacha_bridge_url.ends_with("/") {
            storacha_bridge_url = format!("{}/", storacha_bridge_url)
        }

        let ipfs_api_base_url = env::var("IPFS_API_BASE_URL")
            .unwrap_or_else(|_| constants::IPFS_API_BASE_URL.to_string());

        let use_kms = matches!(
            std::env::var("USE_KMS")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );

        let use_batch_sender = matches!(
            std::env::var("USE_BATCH_SENDER")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );

        let mut signer_kms_id: Option<String> = None;

        if use_kms {
            signer_kms_id = Some(Config::get_env_var("SIGNER_KMS_ID"));
        }

        let config = Config {
            rpc_url,
            private_key,
            folder_path,
            local_ipfs,
            ipfs_api_base_url,
            output_files_dir,
            environment,
            username,
            ddex_sequencer_address,
            disable_telemetry,
            storacha_bridge_url,
            use_kms,
            signer_kms_id,
            use_batch_sender,
        };

        config
    }
}

pub async fn run(
    config: &Config,
    contracts_manager: &ContractsManager,
) -> anyhow::Result<Vec<MessageDirProcessingContext>> {
    contracts_manager.check_image_compatibility().await?;

    let message_dir_processing_log = output_generator::create_output_files(&config).await?;

    let blob_transaction_data = BlobTransactionData::build(&config.output_files_dir)?;

    if config.use_batch_sender == true {
        if cfg!(feature = "aws-integration") {
            let blobs_queue_producer = blobs_queue::BlobsQueueProducer::build().await?;
            blobs_queue_producer
                .enqueue_blob(blob_transaction_data)
                .await?;
        } else {
            panic!(
                "'USE_BATCH_SENDER' .env flag works only with 'aws-integration' feature enabled"
            );
        }
    } else {
        contracts_manager.send_blob(blob_transaction_data).await?;
    }

    Ok(message_dir_processing_log)
}

pub async fn run_with_sentry(config: &Config) -> anyhow::Result<Vec<MessageDirProcessingContext>> {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(User {
            username: Some(config.username.to_owned()),
            ..Default::default()
        }));

        let mut cloned_config = config.clone();
        cloned_config.private_key = "***".to_string();
        scope.set_extra("config", json!(cloned_config));
    });

    let contracts_manager = ContractsManager::build(&config).await?;

    let message_dir_processing_context = run(&config, &contracts_manager).await.map_err(|e| {
        sentry::configure_scope(|scope| {
            scope.set_tag("error_type", {
                if e.is::<ParserError>() {
                    "parser"
                } else {
                    "other"
                }
            });
            scope.set_extra("error_object", json!(format!("{e:#?}")));
        });

        log_error!("{e}")
    })?;

    Ok(message_dir_processing_context)
}
