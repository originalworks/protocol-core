use crate::{constants, errors::OwValidatorNodeError};
use alloy::{
    eips::eip4844::BYTES_PER_BLOB,
    primitives::{Bytes, FixedBytes},
};
use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct BeaconBlockDataMessage {
    slot: String,
}

#[derive(Deserialize, Debug)]
struct BeaconBlockData {
    message: BeaconBlockDataMessage,
}

#[derive(Deserialize, Debug)]
struct BeaconBlock {
    data: BeaconBlockData,
}

#[derive(Deserialize, Debug)]
pub struct BlobSidecarData {
    pub blob: String,
    pub kzg_commitment: Bytes,
}

#[derive(Deserialize, Debug)]
struct BlobSidecars {
    data: Vec<BlobSidecarData>,
}

async fn get_parent_beacon_block_slot(
    beacon_rpc_url: &String,
    parent_beacon_block_root: FixedBytes<32>,
) -> Result<u64, Box<dyn Error>> {
    let url = format!(
        "{}{}{}",
        beacon_rpc_url,
        constants::GET_BEACON_BLOCK_API_PATH,
        parent_beacon_block_root
    );

    let response = reqwest::get(url).await?.json::<BeaconBlock>().await?;
    println!("{response:?}");

    let slot = response.data.message.slot.parse::<u64>()?;

    Ok(slot)
}

async fn find_commitment_in_sidecars(
    beacon_rpc_url: &String,
    beacon_slot: u64,
    commitment: &Bytes,
) -> Option<BlobSidecarData> {
    let url = format!(
        "{}{}{}",
        beacon_rpc_url,
        constants::GET_SIDECARS_API_PATH,
        beacon_slot
    );

    let response = reqwest::get(url)
        .await
        .ok()?
        .json::<BlobSidecars>()
        .await
        .ok()?;

    let sidecars_filtered: Vec<BlobSidecarData> = response
        .data
        .into_iter()
        .filter(|sidecar| &sidecar.kzg_commitment == commitment)
        .collect();

    if sidecars_filtered.len() == 1 {
        Some(sidecars_filtered.into_iter().next()?)
    } else {
        None
    }
}

fn blob_vec_from_string(prefixed_blob: String) -> Result<[u8; BYTES_PER_BLOB], Box<dyn Error>> {
    if prefixed_blob.len() != BYTES_PER_BLOB * 2 + 2 {
        return Err(Box::new(OwValidatorNodeError::InvalidBlobLength(
            prefixed_blob.len(),
        )));
    }
    let mut byte_array = [0u8; BYTES_PER_BLOB];

    let blob = &prefixed_blob[2..];

    for (i, byte) in byte_array.iter_mut().enumerate() {
        let hex_byte = &blob[i * 2..i * 2 + 2];
        *byte = u8::from_str_radix(hex_byte, 16).map_err(|_| {
            Box::new(OwValidatorNodeError::InvalidHexStringValue(
                hex_byte.to_string(),
            ))
        })?;
    }
    Ok(byte_array)
}

pub async fn find_blob(
    beacon_rpc_url: &String,
    commitment: Bytes,
    parent_beacon_block_root: FixedBytes<32>,
) -> Result<[u8; BYTES_PER_BLOB], Box<dyn Error>> {
    let mut slot = get_parent_beacon_block_slot(beacon_rpc_url, parent_beacon_block_root).await?;
    let mut blob_sidecar_data: Option<BlobSidecarData> =
        find_commitment_in_sidecars(beacon_rpc_url, slot, &commitment).await;

    while blob_sidecar_data.is_none() {
        slot += 1;
        blob_sidecar_data = find_commitment_in_sidecars(beacon_rpc_url, slot, &commitment).await;
    }

    let blob = blob_sidecar_data
        .ok_or_else(|| {
            return Box::new(OwValidatorNodeError::FailedToFindBlobSidecar(
                commitment.to_string(),
            ));
        })?
        .blob;

    let blob_array = blob_vec_from_string(blob)?;

    Ok(blob_array)
}
