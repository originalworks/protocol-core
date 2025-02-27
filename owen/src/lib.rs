mod blob;
mod constants;
mod ddex_sequencer;
mod ipfs;
pub mod logger;
pub mod output_generator;

use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use anyhow::Context;
use blob::BlobTransactionData;
use ddex_parser::ParserError;
use ddex_sequencer::DdexSequencerContext;
pub use log;
use log_macros::log_error;
use output_generator::MessageDirProcessingContext;
use sentry::User;
use serde_json::json;
use std::env;
use std::str::FromStr;

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
    pub default_ipfs_interface: IpfsInterface,
    pub ipfs_kubo_url: String,
    pub pinata_jwt: String,
    pub output_files_dir: String,
    pub username: String,
    pub environment: String,
    pub ddex_sequencer_address: Address,
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
        let ipfs_kubo_url = Config::get_env_var("IPFS_KUBO_URL");
        let pinata_jwt = Config::get_env_var("PINATA_JWT");
        let output_files_dir = Config::get_env_var("OUTPUT_FILES_DIR");
        let username = Config::get_env_var("USERNAME");
        let environment = Config::get_env_var("ENVIRONMENT");
        let default_ipfs_interface = match Config::get_env_var("DEFAULT_IPFS_INTERFACE").as_str() {
            "PINATA" => IpfsInterface::PINATA,
            "KUBO" => IpfsInterface::KUBO,
            _ => {
                panic!("Invalid DEFAULT_IPFS_INTERFACE. Supported values: KUBO/PINATA")
            }
        };
        let ddex_sequencer_address = Address::from_str(
            std::env::var("DDEX_SEQUENCER_ADDRESS")
                .unwrap_or_else(|_| constants::DDEX_SEQUENCER_ADDRESS.to_string())
                .as_str(),
        )
        .expect("Could not parse ddex sequencer address");

        let config = Config {
            rpc_url,
            private_key,
            folder_path,
            default_ipfs_interface,
            pinata_jwt,
            ipfs_kubo_url,
            output_files_dir,
            environment,
            username,
            ddex_sequencer_address,
        };

        config
    }
}

pub async fn run(config: &Config) -> anyhow::Result<Vec<MessageDirProcessingContext>> {
    let message_dir_processing_log = output_generator::create_output_files(&config).await?;

    let private_key_signer: PrivateKeySigner = config
        .private_key
        .parse()
        .with_context(|| "Failed to parse PRIVATE_KEY")?;
    let wallet = EthereumWallet::from(private_key_signer);

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(config.rpc_url.parse()?);

    let ddex_sequencer_context =
        DdexSequencerContext::build(&provider, config.ddex_sequencer_address).await?;
    let blob_transaction_data = BlobTransactionData::build(&config.output_files_dir)?;
    ddex_sequencer_context
        .send_blob(blob_transaction_data)
        .await?;
    Ok(message_dir_processing_log)
}

pub async fn run_with_sentry(config: &Config) -> anyhow::Result<Vec<MessageDirProcessingContext>> {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(User {
            username: Some(config.username.to_owned()),
            ..Default::default()
        }));

        let mut cloned_config = config.clone();
        cloned_config.pinata_jwt = "***".to_string();
        cloned_config.private_key = "***".to_string();
        scope.set_extra("config", json!(cloned_config));
    });

    let message_dir_processing_context = run(&config).await.map_err(|e| {
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
