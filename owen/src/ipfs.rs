use crate::{
    constants::{self, IPFS_API_ADD_FILE, REQWEST_CLIENT},
    Config,
};
use alloy::{
    hex,
    signers::{local::PrivateKeySigner, Signer},
};
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

pub async fn pin_file(path: &String, config: &Config) -> anyhow::Result<String> {
    if config.local_ipfs {
        Ok(pin_file_ipfs_kubo(path, &config.ipfs_api_base_url).await?)
    } else {
        Ok(pin_file_storacha(path, &config.storacha_bridge_url, &config.private_key).await?)
    }
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

async fn pin_file_ipfs_kubo(
    file_path: &String,
    ipfs_api_base_url: &String,
) -> anyhow::Result<String> {
    log_info!("Pinning {} to IPFS using KUBO...", file_path);
    let multipart_form = file_to_multipart_form(file_path, None).await?;

    let response = REQWEST_CLIENT
        .post(format!("{}{}", ipfs_api_base_url, IPFS_API_ADD_FILE))
        .multipart(multipart_form)
        .send()
        .await
        .with_context(|| format_error!("Pinning to IPFS failed"))?;

    let result = response.json::<IpfsKuboResponse>().await?;

    Ok(result.Hash)
}

async fn pin_file_storacha(
    file_path: &String,
    storacha_bridge_url: &String,
    private_key: &String,
) -> anyhow::Result<String> {
    log_info!("Pinning {} to IPFS using STORACHA...", file_path);

    let form = file_to_multipart_form(&file_path, Some("image/avif")).await?;

    let signer: PrivateKeySigner = private_key
        .parse()
        .with_context(|| "Failed to parse PRIVATE_KEY")?;

    let signed_message = signer.sign_message(constants::CLIENT).await?;

    let authorization = format!("{}::0x{}", "OWEN", hex::encode(signed_message.as_bytes()));

    let response = REQWEST_CLIENT
        .post(format!("{}w3up/file", storacha_bridge_url))
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::constants::{IPFS_API_BASE_URL, IPFS_API_CAT_FILE};

    async fn fetch_ipfs_file(cid: &String) -> anyhow::Result<tokio_util::bytes::Bytes> {
        let response = REQWEST_CLIENT
            .post(format!(
                "{}{}?arg={}",
                IPFS_API_BASE_URL, IPFS_API_CAT_FILE, cid
            ))
            .send()
            .await?;

        if response.status() != 200 {
            panic!("Image CID not found {cid}");
        }
        let bytes = response.bytes().await?;

        Ok(bytes)
    }

    #[tokio::test]
    async fn pin_and_read() -> anyhow::Result<()> {
        let path = &"./tests/msg_one.json".to_string();
        let expected_file = fs::read(path)?;

        let cid = pin_file_ipfs_kubo(path, &constants::IPFS_API_BASE_URL.to_string()).await?;

        let fetched_file = fetch_ipfs_file(&cid).await?;
        assert_eq!(expected_file, fetched_file.to_vec(), "should be equal");

        Ok(())
    }
}
