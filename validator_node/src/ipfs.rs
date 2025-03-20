use crate::constants::{self, IPFS_API_BASE_URL, IPFS_API_CAT_FILE};
use crate::contracts::QueueHeadData;
use anyhow::{anyhow, Context};
use blob_codec::BlobCodec;
use cid::Cid;
use ddex_parser::DdexParser; // only import what you need
use log_macros::{format_error, log_warn, log_error, log_info};
use multihash_codetable::{Code, MultihashDigest};
use serde::{Deserialize, Serialize};
use serde_valid::json::ToJsonString;
use std::{
    fs::{self, File},
    io::{Write, Cursor},
    path::Path,
    process::Command,
};
use reqwest::blocking::Client; // synchronous

#[derive(Serialize, Deserialize)]
struct BlobMetadata {
    versioned_hash: String,
    transaction_hash: String,
    block_number: String,
    timestamp: String,
    chain_id: String,
    network_name: String,
    blob_ipfs_cid: String,
}

/// Fully synchronous check for each CID (via POST to /api/v0/cat).
#[allow(dead_code)]
pub fn check_file_accessibility(cids: Vec<String>) -> anyhow::Result<()> {
    println!("Checking these CIDs for accessibility: {:?}", cids);
    let client = Client::new(); // blocking client

    for cid in cids {
        let response = client
            .post(format!("{}{}{}", IPFS_API_BASE_URL, IPFS_API_CAT_FILE, cid))
            .send()
            .with_context(|| format!("Failed to POST cat request for CID={}", cid))?;

        if response.status() != 200 {
            return Err(format_error!("Image file not found in IPFS: {}", cid));
        }
    }

    Ok(())
}

/// Decodes the blob, writes JSON out, downloads any images to `images/`,
/// stores the raw blob in `blob/`, and writes `metadata.json` (all synchronously).
pub fn prepare_blob_folder(
    blob: [u8; 131072],
    queue_head_data: &QueueHeadData,
) -> anyhow::Result<()> {
    let blob_folder_path = Path::new(constants::TEMP_FOLDER);

    // Clear out any old data
    if blob_folder_path.is_dir() {
        fs::remove_dir_all(blob_folder_path)?;
    }
    fs::create_dir_all(blob_folder_path.join("json"))?;
    fs::create_dir_all(blob_folder_path.join("blob"))?;
    fs::create_dir_all(blob_folder_path.join("images"))?;

    // Decode the blob
    let blob_codec = BlobCodec::from_vec(blob.into())?;
    let message_vecs = blob_codec.decode()?;

    let client = Client::new(); // We'll reuse this blocking client

    // We'll store images with sequential filenames: 0.avif, 1.avif, etc.
    let mut image_counter = 0usize;

    for (msg_idx, message_bytes) in message_vecs.iter().enumerate() {
        // parse the JSON
        let parsed = match DdexParser::from_json_reader(Cursor::new(&message_bytes)) {
            Ok(p) => p,
            Err(e) => {
                log_warn!("Skipping message upload {} due to parsing error: {:?}", msg_idx, e);
                continue;
            }
        };

        // Write out the pretty JSON
        match parsed.to_json_string_pretty() {
            Ok(json_output) => {
                fs::write(blob_folder_path.join(format!("json/{msg_idx}.json")), json_output)?;
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

        // -------------------------------------------------
        //   Download images from resource_list.images
        // -------------------------------------------------
        let resource_list = &parsed.resource_list;

        // If `resource_list.images` is a Vec, loop directly:
        for image in &resource_list.images {
            for tech_detail in &image.technical_details {
                // tech_detail.file is an Option<File>, so do:
                if let Some(file) = &tech_detail.file {
                    let cid = &file.uri;
                    log_info!("Found image CID: {}", cid);

                    let url = format!("{}{}{}", IPFS_API_BASE_URL, IPFS_API_CAT_FILE, cid);
                    log_info!("Downloading image CID: {} from {}", cid, url);

                    let response = client
                        .get(&url)
                        .send()
                        .with_context(|| format!("Failed to download CID={}", cid))?;

                    if response.status() != 200 {
                        log_warn!(
                            "Could not download image CID={} => status={}",
                            cid,
                            response.status()
                        );
                        continue;
                    }

                    let bytes = response.bytes().with_context(|| {
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

    // Write the raw blob to disk
    let mut blob_file = File::create(blob_folder_path.join("blob/blob_data.bin"))?;
    blob_file.write_all(&blob)?;

    // Write metadata
    let blob_metadata = BlobMetadata {
        versioned_hash: queue_head_data.versioned_blobhash.to_string(),
        transaction_hash: queue_head_data.transaction_hash.to_string(),
        block_number: queue_head_data.block_number.to_string(),
        timestamp: queue_head_data.timestamp.to_string(),
        chain_id: "17000".to_string(),
        network_name: "Holesky".to_string(),
        blob_ipfs_cid: calculate_blob_data_cid()?,
    };

    let blob_metadata_json_string = serde_json::to_string_pretty(&blob_metadata)?;
    let blob_metadata_json_path = blob_folder_path.join("blob/metadata.json");
    let mut file = File::create(blob_metadata_json_path)?;
    file.write_all(blob_metadata_json_string.as_bytes())?;

    Ok(())
}

/// Compute a CID for the file at `TEMP_FOLDER/blob/blob_data.bin`
fn calculate_blob_data_cid() -> anyhow::Result<String> {
    const RAW: u64 = 0x55;
    let blob_path = Path::new(constants::TEMP_FOLDER).join("blob/blob_data.bin");
    let data = fs::read(&blob_path).expect("Failed to read file");
    let h = Code::Sha2_256.digest(&data);
    let cid = Cid::new_v1(RAW, h);
    Ok(cid.to_string())
}

/// Upload the contents of `TEMP_FOLDER` to IPFS via `w3 up <folder>`, parse the CID,
/// log it, then remove all files in `TEMP_FOLDER`.
pub fn upload_blob_folder_and_cleanup() -> anyhow::Result<String> {
    let folder_path = Path::new(constants::TEMP_FOLDER);

    let output = Command::new("w3")
        .args(["up", folder_path.to_str().unwrap()])
        .output()
        .with_context(|| format!("Failed to run `w3 up {}`", folder_path.display()))?;

    if !output.status.success() {
        let msg = format!(
            "`w3 up` command failed with status: {:?}\nStdout: {}\nStderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        log_error!("{}", msg);
        return Err(anyhow!(msg));
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    let mut cid: Option<String> = None;
    for line in stdout_str.lines() {
        if line.starts_with("⁂ https://") {
            let tokens: Vec<_> = line.split_whitespace().collect();
            if tokens.len() >= 2 {
                if let Some(url) = tokens.get(1) {
                    if let Some(pos) = url.rfind('/') {
                        cid = Some(url[(pos + 1)..].to_string());
                        break;
                    }
                }
            }
        }
    }

    let cid = match cid {
        Some(c) => c,
        None => {
            let msg = format!(
                "Could not parse the CID from w3 output.\nFull output:\n{}",
                stdout_str
            );
            log_error!("{}", msg);
            return Err(anyhow!(msg));
        }
    };

    log_info!("Successfully uploaded folder to IPFS. CID: {}", cid);
    log_info!("URL: https://{}.ipfs.w3s.link/", cid);

    for entry in fs::read_dir(folder_path)? {
        let path = entry?.path();
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(cid)
}
