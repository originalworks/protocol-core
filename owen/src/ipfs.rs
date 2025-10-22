use crate::{
    constants::{self, IPFS_API_ADD_FILE, REQWEST_CLIENT},
    wallet::OwenWallet,
    Config,
};
use alloy::hex;
use anyhow::Context;
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

pub struct IpfsManager<'a> {
    local_ipfs: bool,
    ipfs_api_base_url: String,
    storacha_bridge_url: String,
    owen_wallet: &'a OwenWallet,
}

impl<'a> IpfsManager<'a> {
    pub async fn build(config: &Config, owen_wallet: &'a OwenWallet) -> anyhow::Result<Self> {
        Ok(Self {
            local_ipfs: config.local_ipfs.clone(),
            ipfs_api_base_url: config.ipfs_api_base_url.clone(),
            storacha_bridge_url: config.storacha_bridge_url.clone(),
            owen_wallet,
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
        let signature = self.owen_wallet.sign_message(constants::CLIENT).await?;

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
