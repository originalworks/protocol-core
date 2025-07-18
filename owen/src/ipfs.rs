use crate::{
    constants::{self, IPFS_API_ADD_FILE, REQWEST_CLIENT},
    Config,
};
use alloy::{
    hex,
    providers::{Provider, ProviderBuilder},
    signers::{local::PrivateKeySigner, Signer},
};
use alloy_signer_aws::AwsSigner;
use anyhow::Context;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use log_macros::{format_error, log_info};
use reqwest::{multipart, Body};
use serde::{Deserialize, Serialize};
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct IpfsKuboResponse {
    Hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StorachaBridgeResponse {
    cid: String,
    url: String,
}

pub struct IpfsManager {
    local_ipfs: bool,
    ipfs_api_base_url: String,
    storacha_bridge_url: String,
    private_key_signer: Option<PrivateKeySigner>,
    use_kms: bool,
    kms_signer: Option<AwsSigner>,
}

impl IpfsManager {
    pub async fn build(config: &Config) -> anyhow::Result<Self> {
        let rpc_provider = ProviderBuilder::new().on_http(config.rpc_url.parse()?);
        let chain_id = rpc_provider.get_chain_id().await?;
        let mut kms_signer: Option<AwsSigner> = None;
        let mut private_key_signer: Option<PrivateKeySigner> = None;

        if config.use_kms {
            let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
            let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
                .region(region_provider)
                .load()
                .await;

            let client = aws_sdk_kms::Client::new(&aws_main_config);

            let key_id = config
                .signer_kms_id
                .clone()
                .expect("'use_kms' is set to true but 'signer_kms_id' is missing");

            kms_signer = Some(
                AwsSigner::new(client, key_id, Some(chain_id))
                    .await
                    .unwrap(),
            );
        } else {
            let private_key_signer_parsed: PrivateKeySigner = config
                .private_key
                .parse()
                .with_context(|| "Failed to parse PRIVATE_KEY")?;
            private_key_signer = Some(private_key_signer_parsed);
        }

        Ok(Self {
            local_ipfs: config.local_ipfs.clone(),
            ipfs_api_base_url: config.ipfs_api_base_url.clone(),
            storacha_bridge_url: config.storacha_bridge_url.clone(),
            use_kms: config.use_kms,
            kms_signer,
            private_key_signer,
        })
    }

    pub async fn pin_file(&self, path: &String) -> anyhow::Result<String> {
        if self.local_ipfs {
            Ok(self.pin_file_ipfs_kubo(path).await?)
        } else {
            Ok(self.pin_file_storacha(path).await?)
        }
    }

    async fn pin_file_ipfs_kubo(&self, file_path: &String) -> anyhow::Result<String> {
        log_info!("Pinning {} to IPFS using KUBO...", file_path);
        let multipart_form = Self::file_to_multipart_form(file_path, None).await?;

        let response = REQWEST_CLIENT
            .post(format!("{}{}", self.ipfs_api_base_url, IPFS_API_ADD_FILE))
            .multipart(multipart_form)
            .send()
            .await
            .with_context(|| format_error!("Pinning to IPFS failed"))?;

        let result = response.json::<IpfsKuboResponse>().await?;

        Ok(result.Hash)
    }
    async fn file_to_multipart_form(
        file_path: &String,
        mime_type: Option<&str>,
    ) -> anyhow::Result<multipart::Form> {
        let file = tokio::fs::File::open(file_path).await?;
        let file_stream = FramedRead::new(file, BytesCodec::new());
        let mut multipart_stream =
            multipart::Part::stream(Body::wrap_stream(file_stream)).file_name("filename");

        if let Some(mime_str) = mime_type {
            multipart_stream = multipart_stream.mime_str(&mime_str)?;
        }

        let multipart_form = multipart::Form::new().part("file", multipart_stream);
        Ok(multipart_form)
    }

    async fn sign_authorization_header(&self) -> anyhow::Result<String> {
        let signature;
        if self.use_kms {
            let kms_signer = self
                .kms_signer
                .clone()
                .expect("IpfsManager: Failed to get kms_signer");
            signature = kms_signer.sign_message(constants::CLIENT).await?;
        } else {
            let private_key_signer = self
                .private_key_signer
                .clone()
                .expect("IpfsManager: Failed to get private_key_signer");
            signature = private_key_signer.sign_message(constants::CLIENT).await?;
        }
        let authorization = format!("{}::0x{}", "OWEN", hex::encode(signature.as_bytes()));
        Ok(authorization)
    }

    async fn pin_file_storacha(&self, file_path: &String) -> anyhow::Result<String> {
        log_info!("Pinning {} to IPFS using STORACHA...", file_path);

        let form = Self::file_to_multipart_form(&file_path, Some("image/avif")).await?;

        let authorization = self.sign_authorization_header().await?;

        let response = REQWEST_CLIENT
            .post(format!("{}w3up/file", self.storacha_bridge_url))
            .header("authorization", authorization)
            .multipart(form)
            .send()
            .await
            .with_context(|| format_error!("Pinning to Storacha failed"))?;

        let res: StorachaBridgeResponse;

        if response.status().is_success() {
            res = response.json().await?;
        } else {
            let reason = response.text().await?;
            return Err(format_error!("Storacha Bridge returned error: {}", reason));
        }

        log_info!("Pinned! CID: {}", res.cid);

        Ok(res.cid)
    }
}
