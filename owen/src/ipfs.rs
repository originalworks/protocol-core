use crate::constants::{IPFS_API_ADD_FILE, IPFS_API_BASE_URL};
use reqwest::{multipart, Body};
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct IpfsKuboResponse {
    Hash: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct IpfsPinataResponse {
    IpfsHash: String,
}

async fn file_to_multipart_form(file_path: &String) -> Result<multipart::Form, Box<dyn Error>> {
    let file = tokio::fs::File::open(file_path).await?;
    let file_stream = FramedRead::new(file, BytesCodec::new());
    let multipart_stream =
        multipart::Part::stream(Body::wrap_stream(file_stream)).file_name("filename");
    let multipart_form = multipart::Form::new().part("file", multipart_stream);
    Ok(multipart_form)
}

pub async fn pin_file_ipfs_kubo(file_path: &String) -> Result<String, Box<dyn Error>> {
    let multipart_form = file_to_multipart_form(file_path).await?;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}{}", IPFS_API_BASE_URL, IPFS_API_ADD_FILE))
        .multipart(multipart_form)
        .send()
        .await?;

    let result = response.json::<IpfsKuboResponse>().await?;

    Ok(result.Hash)
}

pub async fn pin_file_pinata(
    file_path: &String,
    pinata_jwt: &String,
) -> Result<String, Box<dyn Error>> {
    let multipart_form = file_to_multipart_form(file_path).await?;
    let client = reqwest::Client::new();

    let response = client
        .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
        .header("Authorization", format!("Bearer {}", pinata_jwt))
        .multipart(multipart_form)
        .send()
        .await?;

    let result = response.json::<IpfsPinataResponse>().await?;
    println!("{result:?}");

    Ok(result.IpfsHash)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::constants::IPFS_API_CAT_FILE;

    async fn fetch_ipfs_file(cid: &String) -> Result<tokio_util::bytes::Bytes, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client
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
    async fn pin_and_read() -> Result<(), Box<dyn Error>> {
        let path = &"./tests/msg_one.json".to_string();
        let expected_file = fs::read(path)?;

        let cid = pin_file_ipfs_kubo(path).await?;

        let fetched_file = fetch_ipfs_file(&cid).await?;
        assert_eq!(expected_file, fetched_file.to_vec(), "should be equal");

        Ok(())
    }
}
