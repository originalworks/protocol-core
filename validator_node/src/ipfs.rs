use crate::constants::{self, IPFS_API_BASE_URL, IPFS_API_CAT_FILE};
use crate::contracts::QueueHeadData;
use anyhow::Ok;
use blob_codec::BlobCodec;
use cid::Cid;
use ddex_parser::{DdexParser, NewReleaseMessage};
use log_macros::format_error;
use multihash_codetable::{Code, MultihashDigest};
use serde::{Deserialize, Serialize};
use serde_valid::json::ToJsonString;
use std::fs::File;
use std::io::Write;
use std::{fs, io::Cursor, path::Path};

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

    let blob_codec = BlobCodec::from_vec(blob.into())?;
    let message_vecs = blob_codec.decode()?;

    let mut reader: Cursor<&Vec<u8>>;
    let mut parsed_messages: Vec<NewReleaseMessage> = vec![];

    for (i, message_vec) in message_vecs.iter().enumerate() {
        reader = Cursor::new(&message_vec);
        let parsed = match DdexParser::from_json_reader(reader) {
            std::result::Result::Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "Skipping message upload {} due to parsing error: {:?}",
                    i, e
                );
                continue;
            }
        };
        let json_output = match parsed.to_json_string_pretty() {
            std::result::Result::Ok(json) => json,
            Err(e) => {
                eprintln!(
                    "Skipping message upload {} due to JSON serialization error: {:?}",
                    i, e
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
