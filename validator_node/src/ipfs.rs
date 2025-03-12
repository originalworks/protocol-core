use crate::constants::{self, IPFS_API_BASE_URL, IPFS_API_CAT_FILE};
use crate::contracts::QueueHeadData;
use anyhow::{anyhow, Context};
use blob_codec::BlobCodec;
use cid::Cid;
use ddex_parser::{DdexParser, NewReleaseMessage};
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

#[allow(dead_code)]
pub async fn check_file_accessibility(cids: Vec<String>) -> anyhow::Result<()> {
    println!("{cids:?}");
    let client = reqwest::Client::new();

    for cid in cids {
        let response = client
            .post(format!(
                "{}{}?arg={}",
                IPFS_API_BASE_URL, IPFS_API_CAT_FILE, cid
            ))
            .send()
            .await?;

        if response.status() != 200 {
            return Err(format_error!("Image file not found in IPFS: {}", cid));
        }
    }

    Ok(())
}

pub async fn prepare_blob_folder(
    blob: [u8; 131072],
    queue_head_data: &QueueHeadData,
) -> anyhow::Result<()> {
    let blob_folder_path = Path::new(constants::TEMP_FOLDER);

    if blob_folder_path.is_dir() {
        fs::remove_dir_all(blob_folder_path)?;
    }
    fs::create_dir_all(blob_folder_path.join("json"))?;
    fs::create_dir_all(blob_folder_path.join("blob"))?;
    fs::create_dir_all(blob_folder_path.join("images"))?;

    let blob_codec = BlobCodec::from_vec(blob.into())?;
    let message_vecs = blob_codec.decode()?;

    let mut reader: Cursor<&Vec<u8>>;
    let mut parsed_messages: Vec<NewReleaseMessage> = vec![];

    for (i, message_vec) in message_vecs.iter().enumerate() {
        reader = Cursor::new(&message_vec);
        let parsed = match DdexParser::from_json_reader(reader) {
            std::result::Result::Ok(p) => p,
            Err(e) => {
                log_warn!(
                    "Skipping message upload {} due to parsing error: {:?}",
                    i,
                    e
                );
                continue;
            }
        };
        let json_output = match parsed.to_json_string_pretty() {
            std::result::Result::Ok(json) => json,
            Err(e) => {
                log_warn!(
                    "Skipping message upload {} due to JSON serialization error: {:?}",
                    i,
                    e
                );
                continue;
            }
        };

        fs::write(
            &blob_folder_path.join(format!("json/{}.json", i)),
            json_output,
        )?;
        parsed_messages.push(parsed);
    }
    let mut blob_file = File::create(blob_folder_path.join(format!("blob/blob_data.bin")))?;
    blob_file.write_all(&blob)?;

    let blob_metadata = BlobMetadata {
        versioned_hash: queue_head_data.versioned_blobhash.to_string(),
        transaction_hash: queue_head_data.transaction_hash.to_string(),
        block_number: queue_head_data.block_number.to_string(),
        timestamp: queue_head_data.timestamp.to_string(),
        chain_id: "10200".to_string(),
        network_name: "Gnosis".to_string(),
        blob_ipfs_cid: calculate_blob_data_cid()?,
    };

    let blob_metadata_json_string = serde_json::to_string_pretty(&blob_metadata)?;
    let blob_metadata_json_path = blob_folder_path.join(format!("blob/metadata.json"));
    let mut file = File::create(blob_metadata_json_path)?;
    file.write_all(blob_metadata_json_string.as_bytes())?;

    Ok(())
}

fn calculate_blob_data_cid() -> anyhow::Result<String> {
    const RAW: u64 = 0x55;
    let blob_folder_path = Path::new(constants::TEMP_FOLDER).join("blob/blob_data.bin");
    let data = fs::read(blob_folder_path).expect("Failed to read file");
    let h = Code::Sha2_256.digest(&data);
    let cid = Cid::new_v1(RAW, h);
    Ok(cid.to_string())
}

///
/// Uploads the contents of the `TEMP_FOLDER` directory to IPFS via `w3 up <folder>`,
/// extracts and returns the CID, logs it, and then removes everything **inside** `TEMP_FOLDER`
/// (but not the folder itself).
///
/// If anything fails (command error, parse error, etc.), logs an error and returns `Err(...)`.
///
pub fn upload_blob_folder_and_cleanup() -> anyhow::Result<String> {
    // This is your top-level folder (e.g. "tmp" or "temp" or similar).
    let folder_path = Path::new(crate::constants::TEMP_FOLDER);

    // --- 1) Invoke `w3 up <folder>` synchronously ---
    let output = Command::new("w3")
        .args(["up", folder_path.to_str().unwrap()])
        .output()
        .with_context(|| format!("Failed to run `w3 up {}`", folder_path.display()))?;

    // If `w3` returned a non-zero exit code, treat that as failure
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

    // --- 2) Parse the CID from the lines like:
    //     "⁂ https://w3s.link/ipfs/bafy...something..."
    //
    //     We want to extract only the part after the last '/'
    //     e.g. "bafybeibtetuqfs3xwr7makqdh6yebiuwkuxrerkjx5f6swmauzv6ivpevi"
    //
    let mut cid: Option<String> = None;
    for line in stdout_str.lines() {
        if line.starts_with("⁂ https://") {
            // "⁂ https://w3s.link/ipfs/bafybeibtetuqfs3x..."
            let tokens: Vec<_> = line.split_whitespace().collect();
            if tokens.len() >= 2 {
                // tokens[0] = "⁂"
                // tokens[1] = "https://w3s.link/ipfs/bafy..."
                if let Some(url) = tokens.get(1) {
                    // isolate the string after the final slash
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
    log_info!(
        "URL: https://{}.ipfs.w3s.link/",
        cid
    );
    

    // --- 3) Remove the *contents* of `TEMP_FOLDER`, but leave the folder itself ---
    for entry in fs::read_dir(folder_path)? {
        let path = entry?.path();
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    // Return the parsed CID
    Ok(cid)
}

