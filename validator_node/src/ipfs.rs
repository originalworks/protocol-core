use crate::blob_assignment::manager::BlobAssignment;
use crate::constants::{
    self, network_name, IPFS_API_BASE_URL, IPFS_API_CAT_FILE, IPFS_TEMP_FILES_FOLDER_NAME,
    REQWEST_CLIENT, TEMP_FOLDER,
};
use crate::is_local;
use crate::zip::zip_directory;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use anyhow::Context;
use blob_codec::BlobCodec;
use cid::Cid;
use ddex_parser::DdexParser;
use log_macros::{format_error, log_info, log_warn};
use multihash_codetable::{Code, MultihashDigest};

use reqwest::multipart;
use serde::{Deserialize, Serialize};
use serde_valid::json::ToJsonString;
use std::{
    fs::{self, File},
    io::{Cursor, Write},
    path::Path,
};

#[derive(Serialize, Deserialize)]
struct BlobMetadata {
    versioned_hash: String,
    transaction_hash: String,
    block_number: String,
    timestamp: String,
    chain_id: String,
    network_name: String,
    blob_ipfs_cid: String,
    image_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct StorachaBridgeResponse {
    cid: String,
    url: String,
}

pub struct IpfsManager {
    blob_folder_path: String,
    storacha_bridge_url: String,
    signer: PrivateKeySigner,
}

impl IpfsManager {
    pub fn build(storacha_bridge_url: String, private_key: String) -> anyhow::Result<Self> {
        let blob_folder_path = Path::new(constants::TEMP_FOLDER)
            .join(IPFS_TEMP_FILES_FOLDER_NAME)
            .to_string_lossy()
            .to_string();

        let signer: PrivateKeySigner = private_key
            .parse()
            .with_context(|| "Failed to parse PRIVATE_KEY")?;

        Ok(Self {
            blob_folder_path,
            storacha_bridge_url,
            signer,
        })
    }

    fn prepare_folders(&self) -> anyhow::Result<()> {
        let blob_folder_path = Path::new(&self.blob_folder_path);
        if blob_folder_path.is_dir() {
            fs::remove_dir_all(blob_folder_path)?;
        }
        fs::create_dir_all(blob_folder_path.join("json"))?;
        fs::create_dir_all(blob_folder_path.join("blob"))?;
        fs::create_dir_all(blob_folder_path.join("images"))?;
        Ok(())
    }

    pub async fn build_blob_folder(
        &self,
        blob: &Vec<u8>,
        blob_assignment: &BlobAssignment,
    ) -> anyhow::Result<()> {
        self.prepare_folders()?;

        let blob_folder_path = Path::new(&self.blob_folder_path);
        // Decode the blob
        let blob_codec = BlobCodec::from_vec(blob.clone())?;
        let message_vecs = blob_codec.decode()?;

        // We'll store images with sequential filenames: 0.avif, 1.avif, etc.
        let mut image_counter = 0usize;

        for (msg_idx, message_bytes) in message_vecs.iter().enumerate() {
            // parse the JSON
            let parsed = match DdexParser::from_json_reader(Cursor::new(&message_bytes)) {
                Ok(p) => p,
                Err(e) => {
                    log_warn!(
                        "Skipping message upload {} due to parsing error: {:?}",
                        msg_idx,
                        e
                    );
                    continue;
                }
            };

            // Write out the pretty JSON
            match parsed.to_json_string_pretty() {
                Ok(json_output) => {
                    fs::write(
                        blob_folder_path.join(format!("json/{msg_idx}.json")),
                        json_output,
                    )?;
                }
                Err(e) => {
                    log_warn!(
                        "Skipping message upload {} due to JSON serialization error: {:?}",
                        msg_idx,
                        e
                    );
                    continue;
                }
            };

            let resource_list = &parsed.resource_list;

            for image in &resource_list.images {
                for tech_detail in &image.technical_details {
                    if let Some(file) = &tech_detail.file {
                        let cid = &file.uri;
                        log_info!("Found image CID: {}", cid);

                        let url = format!("{}{}{}", IPFS_API_BASE_URL, IPFS_API_CAT_FILE, cid);
                        log_info!("Downloading image CID: {} from {}", cid, url);

                        let response = REQWEST_CLIENT
                            .get(&url)
                            .send()
                            .await
                            .with_context(|| format!("Failed to download CID={}", cid))?;

                        if response.status() != 200 {
                            log_warn!(
                                "Could not download image CID={} => status={}",
                                cid,
                                response.status()
                            );
                            continue;
                        }

                        let bytes = response.bytes().await.with_context(|| {
                            format!("Failed to read bytes from IPFS for CID={}", cid)
                        })?;

                        let out_path = blob_folder_path
                            .join("images")
                            .join(format!("{image_counter}.avif"));

                        fs::write(&out_path, &bytes).with_context(|| {
                            format!("Could not write downloaded image to {:?}", &out_path)
                        })?;

                        image_counter += 1;
                    } else {
                        log_warn!("Skipping an image with no file entry in tech_detail");
                    }
                }
            }
        }

        let mut blob_file = File::create(blob_folder_path.join("blob/blob_data.bin"))?;
        blob_file.write_all(blob)?;

        let blob_metadata = BlobMetadata {
            versioned_hash: blob_assignment.blobhash.to_string(),
            transaction_hash: blob_assignment.blob_submission_tx_hash.to_string(),
            block_number: blob_assignment.blob_submission_block.to_string(),
            timestamp: blob_assignment.blob_submission_timestamp.to_string(),
            chain_id: blob_assignment.chain_id.to_string(),
            network_name: network_name(&blob_assignment.chain_id).to_string(),
            blob_ipfs_cid: self.calculate_blob_data_cid()?,
            image_id: format!("0x{}", hex::encode(blob_assignment.image_id)),
        };

        let blob_metadata_json_string = serde_json::to_string_pretty(&blob_metadata)?;
        let blob_metadata_json_path = blob_folder_path.join("blob/metadata.json");
        let mut file = File::create(blob_metadata_json_path)?;
        file.write_all(blob_metadata_json_string.as_bytes())?;

        Ok(())
    }

    fn calculate_blob_data_cid(&self) -> anyhow::Result<String> {
        const RAW: u64 = 0x55;
        let blob_path = Path::new(&self.blob_folder_path).join("blob/blob_data.bin");
        let data = fs::read(&blob_path).expect("Failed to read file");
        let h = Code::Sha2_256.digest(&data);
        let cid = Cid::new_v1(RAW, h);
        Ok(cid.to_string())
    }

    pub async fn upload_blob_folder_and_cleanup(&self) -> anyhow::Result<String> {
        log::info!("Zipping files...");
        let src_path = Path::new(&self.blob_folder_path);
        let zip_file_name = "temp.zip".to_string();
        let zip_path = Path::new(TEMP_FOLDER).join(&zip_file_name);

        if zip_path.exists() {
            fs::remove_file(&zip_path)?;
        }

        zip_directory(src_path, &zip_path, zip::CompressionMethod::Deflated)?;

        let signed_message = self.signer.sign_message(constants::CLIENT).await?;

        let authorization = format!(
            "{}::0x{}",
            "VALIDATOR",
            hex::encode(signed_message.as_bytes())
        );

        let file_part = multipart::Part::file(&zip_path)
            .await?
            .file_name(zip_file_name)
            .mime_str("application/zip")?;
        let form = multipart::Form::new().part("file", file_part);

        let res: StorachaBridgeResponse;

        if is_local() {
            log::info!("Skipping upload to Storacha Bridge in local mode");
            res = StorachaBridgeResponse {
                cid: "test_cid".to_string(),
                url: "test_url".to_string(),
            };
        } else {
            log::info!("Uploading zip to Storacha Bridge...");

            let response = REQWEST_CLIENT
                .post(format!("{}w3up/dir", self.storacha_bridge_url))
                .header("authorization", authorization)
                .multipart(form)
                .send()
                .await?;

            if response.status().is_success() {
                res = response.json().await?;
            } else {
                let reason = response.text().await?;
                return Err(format_error!("Storacha Bridge returned error: {}", reason));
            }

            log_info!("Successfully uploaded folder to IPFS. CID: {}", res.cid);
            log_info!("URL: {}", res.url);
        }

        for entry in fs::read_dir(src_path)? {
            let path = entry?.path();
            if path.is_dir() {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(&path)?;
            }
        }

        fs::remove_file(zip_path)?;

        Ok(res.cid)
    }
}
