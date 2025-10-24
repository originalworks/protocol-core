#[cfg(feature = "aws-integration")]
pub mod blobs_queue;

pub mod blob;
pub mod constants;
mod contracts;
mod image_processor;
mod ipfs;
pub mod logger;
pub mod output_generator;
pub mod wallet;
use alloy::primitives::Address;
use blob::BlobTransactionData;
use contracts::ContractsManager;
use ddex_parser::ParserError;
pub use log;
use log_macros::log_error;
use sentry::User;
use serde_json::json;
use std::env;
use std::str::FromStr;

use crate::ipfs::IpfsManager;
use crate::output_generator::{DdexMessage, OutputFilesGenerator};
use crate::wallet::{OwenWallet, OwenWalletConfig};

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
    pub private_key: Option<String>,
    pub input_files_dir: String,
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

    pub fn build() -> anyhow::Result<Config> {
        if is_local() {
            println!("Running local setup");
            dotenvy::from_filename(".env.local").unwrap();
        } else {
            dotenvy::dotenv().ok();
        }

        let mut args = std::env::args();
        args.next();

        let input_files_dir = args
            .next()
            .unwrap_or_else(|| Config::get_env_var("INPUT_FILES_DIR").to_string());

        let rpc_url = Config::get_env_var("RPC_URL");
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

        let mut signer_kms_id = None;
        let mut private_key = None;
        let use_kms = matches!(
            std::env::var("USE_KMS")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );

        if use_kms {
            signer_kms_id = Some(Config::get_env_var("SIGNER_KMS_ID"));
        } else {
            private_key = Some(Config::get_env_var("PRIVATE_KEY"));
        }

        let use_batch_sender = matches!(
            std::env::var("USE_BATCH_SENDER")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );

        let config = Config {
            rpc_url,
            private_key,
            input_files_dir,
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

        Ok(config)
    }
}

pub async fn run(config: &Config) -> anyhow::Result<Vec<DdexMessage>> {
    let owen_wallet_config = OwenWalletConfig::from(config)?;
    let owen_wallet = OwenWallet::build(&owen_wallet_config).await?;
    let contracts_manager = ContractsManager::build(&config, &owen_wallet).await?;
    contracts_manager.check_image_compatibility().await?;

    let ipfs_manager = IpfsManager::build(&config, &owen_wallet).await?;
    let output_files_generator = OutputFilesGenerator::build(&config, &ipfs_manager)?;
    let ddex_messages = output_files_generator.generate_files().await?;

    let blob_transaction_data = BlobTransactionData::build(&config.output_files_dir)?;

    if config.use_batch_sender == true {
        if cfg!(feature = "aws-integration") {
            let image_id = contracts_manager.image_id;
            #[cfg(feature = "aws-integration")]
            let blobs_queue_producer = blobs_queue::BlobsQueueProducer::build().await?;
            #[cfg(feature = "aws-integration")]
            blobs_queue_producer
                .enqueue_blob(blob_transaction_data, image_id)
                .await?;
        } else {
            panic!(
                "'USE_BATCH_SENDER' .env flag works only with 'aws-integration' feature enabled"
            );
        }
    } else {
        contracts_manager.send_blob(blob_transaction_data).await?;
    }
    Ok(ddex_messages)
}

pub async fn run_with_sentry(config: &Config) -> anyhow::Result<Vec<DdexMessage>> {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(User {
            username: Some(config.username.to_owned()),
            ..Default::default()
        }));
    });

    let ddex_messages = run(&config).await.map_err(|e| {
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

    anyhow::Ok(ddex_messages)
}
