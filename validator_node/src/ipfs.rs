use std::error::Error;

use crate::{
    constants::{IPFS_API_BASE_URL, IPFS_API_CAT_FILE},
    errors::OwValidatorNodeError,
};

pub async fn check_file_accessibility(cids: Vec<String>) -> Result<(), Box<dyn Error>> {
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
            return Err(Box::new(OwValidatorNodeError::ImageFileNotFoundInIpfs(cid)));
        }
    }

    Ok(())
}
