use crate::constants::{DDEX_SEQUENCER_ADDRESS, DEFAULT_IPFS_BRIDGE_URL, IPFS_API_BASE_URL};
use alloy::primitives::Address;
pub use log;
use ow_wallet_adapter::HasOwWalletFields;
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
    pub private_key: Option<String>,
    pub input_files_dir: String,
    pub local_ipfs: bool,
    pub output_files_dir: String,
    pub username: String,
    pub environment: String,
    pub ddex_sequencer_address: Address,
    pub disable_telemetry: bool,
    pub ipfs_bridge_url: String,
    pub ipfs_api_base_url: String,
    pub use_kms: bool,
    pub signer_kms_id: Option<String>,
    pub chain_id: i64,
}

impl Config {
    pub fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }

    pub fn build() -> anyhow::Result<Self> {
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
            .unwrap_or_else(|| Self::get_env_var("INPUT_FILES_DIR").to_string());

        let rpc_url = Self::get_env_var("RPC_URL");
        let local_ipfs = matches!(
            std::env::var("LOCAL_IPFS")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );
        let output_files_dir = Self::get_env_var("OUTPUT_FILES_DIR");
        let username = Self::get_env_var("USERNAME");
        let environment = Self::get_env_var("ENVIRONMENT");
        let ddex_sequencer_address = Address::from_str(
            std::env::var("DDEX_SEQUENCER_ADDRESS")
                .unwrap_or_else(|_| DDEX_SEQUENCER_ADDRESS.to_string())
                .as_str(),
        )
        .expect("Could not parse ddex sequencer address");

        let disable_telemetry: bool = matches!(
            std::env::var("DISABLE_TELEMETRY")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );
        let mut ipfs_bridge_url =
            env::var("IPFS_BRIDGE_URL").unwrap_or_else(|_| DEFAULT_IPFS_BRIDGE_URL.to_string());

        if !ipfs_bridge_url.ends_with("/") {
            ipfs_bridge_url = format!("{}/", ipfs_bridge_url)
        }

        let ipfs_api_base_url =
            env::var("IPFS_API_BASE_URL").unwrap_or_else(|_| IPFS_API_BASE_URL.to_string());

        let mut signer_kms_id = None;
        let mut private_key = None;
        let use_kms = matches!(
            std::env::var("USE_KMS")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );

        if use_kms {
            signer_kms_id = Some(Self::get_env_var("SIGNER_KMS_ID"));
        } else {
            private_key = Some(Self::get_env_var("PRIVATE_KEY"));
        }

        let chain_id = Self::get_env_var("CHAIN_ID").parse::<i64>()?;

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
            ipfs_bridge_url,
            use_kms,
            signer_kms_id,
            chain_id,
        };

        Ok(config)
    }
}

impl HasOwWalletFields for Config {
    fn use_kms(&self) -> bool {
        self.use_kms
    }
    fn rpc_url(&self) -> String {
        self.rpc_url.clone()
    }
    fn private_key(&self) -> Option<String> {
        self.private_key.clone()
    }
    fn signer_kms_id(&self) -> Option<String> {
        self.signer_kms_id.clone()
    }
}
