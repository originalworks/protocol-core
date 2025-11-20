use reqwest::multipart;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;

use crate::constants::{LOCAL_IPFS_NODE_URL, UPLOAD_FOLDER_API_PATH};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct IpfsResponse {
    Hash: String,
    Name: String,
}

async fn dir_to_multipart_form(dir_path: &String) -> anyhow::Result<multipart::Form> {
    let mut multipart_form = multipart::Form::new();

    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let rel_path = path.strip_prefix(dir_path)?;
        let part = multipart::Part::bytes(buf).file_name(rel_path.to_string_lossy().into_owned());

        multipart_form = multipart_form.part("file", part);
    }

    Ok(multipart_form)
}

pub async fn calculate_dir_cid(
    reqwest_client: &reqwest::Client,
    dir_path: &String,
) -> anyhow::Result<String> {
    println!("Calculating CID for a folder...{dir_path}");
    let multipart_form = dir_to_multipart_form(dir_path).await?;

    let response = reqwest_client
        .post(format!("{}{}", LOCAL_IPFS_NODE_URL, UPLOAD_FOLDER_API_PATH))
        .multipart(multipart_form)
        .send()
        .await?;

    let response_string = response.text().await?;

    let response_vec: Vec<IpfsResponse> = response_string
        .lines()
        .filter_map(|line| serde_json::from_str::<IpfsResponse>(line).ok())
        .collect();

    let folder_pin_response = response_vec.last().expect("Folder pin not found");

    if folder_pin_response.Name != "".to_string() {
        panic!("Calculating folder CID failed");
    }
    Ok(folder_pin_response.Hash.clone())
}
