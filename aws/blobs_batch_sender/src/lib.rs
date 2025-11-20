use alloy::primitives::Address;
use lambda_runtime::Error;
use owen::constants;
use owen::wallet::HasOwenWalletFields;
use std::env;
use std::str::FromStr;

impl HasOwenWalletFields for BlobsBatchSenderConfig {
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

pub struct BlobsBatchSenderConfig {
    pub ddex_sequencer_address: Address,
    pub s_eoa_address: Address,
    pub rpc_url: String,
    pub blobs_temp_storage_bucket_name: String,
    pub use_kms: bool,
    pub private_key: Option<String>,
    pub signer_kms_id: Option<String>,
}

impl BlobsBatchSenderConfig {
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }
    pub fn build() -> Result<BlobsBatchSenderConfig, Error> {
        let rpc_url = Self::get_env_var("RPC_URL");
        let ddex_sequencer_address = Address::from_str(
            std::env::var("DDEX_SEQUENCER_ADDRESS")
                .unwrap_or_else(|_| constants::DDEX_SEQUENCER_ADDRESS.to_string())
                .as_str(),
        )
        .expect("Could not parse ddex sequencer address");

        let s_eoa_address = Address::from_str(Self::get_env_var("S_EOA_ADDRESS").as_str())?;

        let blobs_temp_storage_bucket_name = Self::get_env_var("BLOBS_TEMP_STORAGE_BUCKET_NAME");

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

        Ok(BlobsBatchSenderConfig {
            ddex_sequencer_address,
            rpc_url,
            s_eoa_address,
            blobs_temp_storage_bucket_name,
            private_key,
            use_kms,
            signer_kms_id,
        })
    }
}
