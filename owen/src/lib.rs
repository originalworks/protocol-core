mod blob;
mod constants;
mod ddex_sequencer;
mod ipfs;
mod output_generator;

use alloy::network::EthereumWallet;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use anyhow::Context;
use blob::BlobTransactionData;
use ddex_sequencer::DdexSequencerContext;
pub use log;
use log_macros::{log_debug, log_error, log_info};
use std::env;

#[derive(Debug, PartialEq)]
pub enum IpfsInterface {
    KUBO,
    PINATA,
}

#[derive(Debug)]
pub struct Config {
    pub rpc_url: String,
    pub private_key: String,
    pub folder_path: String,
    pub default_ipfs_interface: IpfsInterface,
    pub ipfs_kubo_url: String,
    pub pinata_jwt: String,
    pub output_files_dir: String,
}

impl Config {
    fn get_env_var(key: &str) -> anyhow::Result<String> {
        env::var(key).with_context(|| log_error!("Missing variable in .env file: {key}"))
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> anyhow::Result<Config> {
        args.next();

        let folder_path = args
            .next()
            .ok_or_else(|| log_error!("Missing command line argument: folder path"))?;

        let rpc_url = Config::get_env_var("RPC_URL")?;
        let private_key = Config::get_env_var("PRIVATE_KEY")?;
        let ipfs_kubo_url = Config::get_env_var("IPFS_KUBO_URL")?;
        let pinata_jwt = Config::get_env_var("PINATA_JWT")?;
        let default_ipfs_interface_string = Config::get_env_var("DEFAULT_IPFS_INTERFACE")?;
        let mut default_ipfs_interface = IpfsInterface::KUBO;
        if default_ipfs_interface_string == "PINATA".to_string() {
            default_ipfs_interface = IpfsInterface::PINATA;
        }
        let output_files_dir = Config::get_env_var("OUTPUT_FILES_DIR")?;

        let config = Config {
            rpc_url,
            private_key,
            folder_path,
            default_ipfs_interface,
            pinata_jwt,
            ipfs_kubo_url,
            output_files_dir,
        };

        log_debug!("{:?}", config);
        log_info!("Config created");
        Ok(config)
    }
}

pub async fn run(config: Config) -> anyhow::Result<()> {
    output_generator::create_output_files(&config).await?;

    let private_key_signer: PrivateKeySigner = config
        .private_key
        .parse()
        .with_context(|| "Failed to parse PRIVATE_KEY")?;
    let wallet = EthereumWallet::from(private_key_signer);

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(config.rpc_url.parse()?);

    let ddex_sequencer_context = DdexSequencerContext::build(&provider).await?;
    let blob_transaction_data = BlobTransactionData::build(&config.output_files_dir)?;
    ddex_sequencer_context
        .send_blob(blob_transaction_data)
        .await?;
    Ok(())
}
