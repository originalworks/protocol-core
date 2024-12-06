mod blob;
mod constants;
mod ddex_sequencer;
mod errors;
mod ipfs;
mod output_generator;

use alloy::network::EthereumWallet;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use blob::BlobTransactionData;
use ddex_sequencer::DdexSequencerContext;
use errors::OwenCliError;
use std::env;
use std::error::Error;

#[derive(PartialEq)]
pub enum IpfsInterface {
    KUBO,
    PINATA,
}

pub struct Config {
    pub rpc_url: String,
    pub private_key: String,
    pub folder_path: String,
    pub default_ipfs_interface: IpfsInterface,
    pub ipfs_kubo_url: String,
    pub pinata_jwt: String,
}

impl Config {
    fn get_env_var(key: &str) -> Result<String, Box<dyn Error>> {
        match env::var(key) {
            Ok(value) => Ok(value),
            Err(_) => return Err(Box::new(OwenCliError::MissingEnvVar(key.to_string()))),
        }
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();

        let folder_path = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(Box::new(OwenCliError::MissingCliArg(
                    "folder path".to_string(),
                )))
            }
        };

        let rpc_url = Config::get_env_var("RPC_URL")?;
        let private_key = Config::get_env_var("PRIVATE_KEY")?;
        let ipfs_kubo_url = Config::get_env_var("IPFS_KUBO_URL")?;
        let pinata_jwt = Config::get_env_var("PINATA_JWT")?;
        let default_ipfs_interface_string = Config::get_env_var("DEFAULT_IPFS_INTERFACE")?;
        let mut default_ipfs_interface = IpfsInterface::KUBO;
        if default_ipfs_interface_string == "PINATA".to_string() {
            default_ipfs_interface = IpfsInterface::PINATA;
        }

        Ok(Config {
            rpc_url,
            private_key,
            folder_path,
            default_ipfs_interface,
            pinata_jwt,
            ipfs_kubo_url,
        })
    }
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    output_generator::create_output_files(&config).await?;

    let private_key_signer: PrivateKeySigner = config
        .private_key
        .parse()
        .expect("Failed to parse PRIVATE_KEY:");
    let wallet = EthereumWallet::from(private_key_signer);

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(config.rpc_url.parse()?);

    let ddex_sequencer_context = DdexSequencerContext::build(&provider).await?;
    let blob_transaction_data = BlobTransactionData::build()?;
    println!("sending tx...");
    ddex_sequencer_context
        .send_blob(blob_transaction_data)
        .await?;
    Ok(())
}
