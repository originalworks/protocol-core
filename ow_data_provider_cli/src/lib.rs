mod blob;
mod constants;
mod ddex_sequencer;
mod errors;
mod ipfs;

use alloy::network::EthereumWallet;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use blob::BlobTransactionData;
use ddex_sequencer::DdexSequencerContext;
use errors::OwDataProviderCliError;
use std::env;
use std::error::Error;

pub struct Config {
    pub rpc_url: String,
    pub private_key: String,
    pub folder_path: String,
}

impl Config {
    fn get_env_var(key: &str) -> Result<String, Box<dyn Error>> {
        match env::var(key) {
            Ok(value) => Ok(value),
            Err(_) => {
                return Err(Box::new(OwDataProviderCliError::MissingEnvVar(
                    key.to_string(),
                )))
            }
        }
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();

        let folder_path = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(Box::new(OwDataProviderCliError::MissingCliArg(
                    "folder path".to_string(),
                )))
            }
        };

        let rpc_url = Config::get_env_var("RPC_URL")?;
        let private_key = Config::get_env_var("PRIVATE_KEY")?;

        Ok(Config {
            rpc_url,
            private_key,
            folder_path,
        })
    }
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    ipfs::create_output_files(&config.folder_path).await?;
    let private_key_signer: PrivateKeySigner = config
        .private_key
        .parse()
        .expect("Failed to parse PRIVATE_KEY:");
    let wallet = EthereumWallet::from(private_key_signer);

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(config.rpc_url.parse().unwrap());

    let ddex_sequencer_context = DdexSequencerContext::build(&provider).await?;
    let blob_transaction_data = BlobTransactionData::build().unwrap();
    println!("sending tx...");
    ddex_sequencer_context
        .send_blob(blob_transaction_data)
        .await?;
    Ok(())
}
