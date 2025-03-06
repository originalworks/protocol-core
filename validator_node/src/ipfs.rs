use crate::constants::{self, IPFS_API_BASE_URL, IPFS_API_CAT_FILE};
use anyhow::Ok;
use blob_codec::BlobCodec;
use ddex_parser::{DdexParser, NewReleaseMessage};
use log_macros::format_error;
use serde_valid::json::ToJsonString;
use std::fs::File;
use std::io::Write;
use std::{fs, io::Cursor, path::Path};

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

pub async fn prepare_blob_folder(blob: [u8; 131072]) -> anyhow::Result<()> {
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

    Ok(())
}
