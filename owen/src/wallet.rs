use crate::Config;
use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use alloy_signer_aws::AwsSigner;
use anyhow::Context;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use log_macros::{format_error, log_info};

pub struct OwenWallet {
    pub use_kms: bool,
    aws_signer: Option<AwsSigner>,
    private_key_signer: Option<PrivateKeySigner>,
    pub wallet: EthereumWallet,
}

pub trait HasOwenWalletFields {
    fn use_kms(&self) -> bool;
    fn rpc_url(&self) -> String;
    fn private_key(&self) -> Option<String>;
    fn signer_kms_id(&self) -> Option<String>;
}

impl HasOwenWalletFields for Config {
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

pub struct OwenWalletConfig {
    pub use_kms: bool,
    pub rpc_url: String,
    pub private_key: Option<String>,
    pub signer_kms_id: Option<String>,
}

impl OwenWalletConfig {
    pub fn build() -> anyhow::Result<Self> {
        let rpc_url = Config::get_env_var("RPC_URL");
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

        Ok(Self {
            use_kms,
            rpc_url,
            private_key,
            signer_kms_id,
        })
    }

    pub fn from<C: HasOwenWalletFields>(config_source: &C) -> anyhow::Result<Self> {
        Ok(Self {
            use_kms: config_source.use_kms(),
            rpc_url: config_source.rpc_url(),
            private_key: config_source.private_key(),
            signer_kms_id: config_source.signer_kms_id(),
        })
    }
    fn try_private_key(&self) -> anyhow::Result<&String> {
        if self.use_kms == false {
            self.private_key
                .as_ref()
                .ok_or_else(|| format_error!("Missing private_key"))
        } else {
            return Err(format_error!(
                "private_key not available with USE_KMS=true flag"
            ));
        }
    }

    fn try_signer_kms_id(&self) -> anyhow::Result<&String> {
        if self.use_kms == true {
            self.signer_kms_id
                .as_ref()
                .ok_or_else(|| format_error!("Missing signer_kms_id"))
        } else {
            return Err(format_error!(
                "signer_kms_id not available without USE_KMS=true flag"
            ));
        }
    }
}

impl OwenWallet {
    pub async fn build(config: &OwenWalletConfig) -> anyhow::Result<Self> {
        let wallet: EthereumWallet;
        let mut aws_signer = None;
        let mut private_key_signer: Option<PrivateKeySigner> = None;
        if config.use_kms {
            let rpc_provider = ProviderBuilder::new().connect_http(config.rpc_url.parse()?);
            let chain_id = rpc_provider.get_chain_id().await?;

            let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
            let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
                .region(region_provider)
                .load()
                .await;

            let client = aws_sdk_kms::Client::new(&aws_main_config);

            let key_id = config.try_signer_kms_id()?;

            let chain_id = Some(chain_id);
            let signer = AwsSigner::new(client, key_id.to_string(), chain_id)
                .await
                .expect("Failed to initialize AwsSigner");

            let pubkey = signer.get_pubkey().await?;
            let address = Address::from_public_key(&pubkey);
            log_info!("Using KMS with address: {}", address);
            aws_signer = Some(signer.clone());
            wallet = EthereumWallet::from(signer);
        } else {
            let pk_signer: PrivateKeySigner = config
                .try_private_key()?
                .parse()
                .with_context(|| "Failed to parse PRIVATE_KEY")?;
            private_key_signer = Some(pk_signer.clone());
            wallet = EthereumWallet::from(pk_signer);
        }
        Ok(Self {
            use_kms: config.use_kms.clone(),
            aws_signer,
            private_key_signer,
            wallet,
        })
    }

    fn try_aws_signer(&self) -> anyhow::Result<&AwsSigner> {
        self.aws_signer
            .as_ref()
            .ok_or_else(|| format_error!("Missing aws_signer"))
    }

    fn try_private_key_signer(&self) -> anyhow::Result<&PrivateKeySigner> {
        self.private_key_signer
            .as_ref()
            .ok_or_else(|| format_error!("Missing private_key_signer"))
    }

    pub async fn sign_message(&self, message: &[u8]) -> anyhow::Result<alloy::signers::Signature> {
        let signature;
        if self.use_kms {
            let aws_signer = self.try_aws_signer()?;
            signature = aws_signer.sign_message(message).await?;
        } else {
            let private_key_signer = self.try_private_key_signer()?;
            signature = private_key_signer.sign_message(message).await?;
        }
        Ok(signature)
    }
}
