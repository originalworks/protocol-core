use log_macros::format_error;

use crate::constants::{IPFS_API_BASE_URL, IPFS_API_CAT_FILE};

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
