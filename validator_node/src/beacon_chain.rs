use crate::constants;
use alloy::{
    eips::eip4844::BYTES_PER_BLOB,
    primitives::{Bytes, FixedBytes},
};
use log_macros::{format_error, log_info};
use reqwest;
use serde::Deserialize;
use tokio::time::{sleep, Duration};

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

/// A small helper to perform a GET request and deserialize into T,
/// retrying on failure up to `max_retries` times. If it fails all attempts,
/// returns Err with the last error.
async fn get_with_retry<T: serde::de::DeserializeOwned>(
    url: String,
    max_retries: u32,
    delay_seconds: u64,
) -> anyhow::Result<T> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        match reqwest::get(&url).await {
            Ok(resp) => {
                let result = resp.json::<T>().await;
                match result {
                    Ok(parsed) => {
                        return Ok(parsed);
                    }
                    Err(e) => {
                        if attempt >= max_retries {
                            log::error!(
                                "Beacon RPC call failed (attempt {} of {}). Last error: {}",
                                attempt,
                                max_retries,
                                e
                            );
                            return Err(anyhow::anyhow!(e));
                        } else {
                            log::warn!(
                                "Beacon RPC call failed (attempt {} of {}): {}. Retrying in {}s...",
                                attempt,
                                max_retries,
                                e,
                                delay_seconds
                            );
                            sleep(Duration::from_secs(delay_seconds)).await;
                        }
                    }
                }
            }
            Err(e) => {
                if attempt >= max_retries {
                    log::error!(
                        "Beacon RPC call failed (attempt {} of {}). Last error: {}",
                        attempt,
                        max_retries,
                        e
                    );
                    return Err(anyhow::anyhow!(e));
                } else {
                    log::warn!(
                        "Beacon RPC call failed (attempt {} of {}): {}. Retrying in {}s...",
                        attempt,
                        max_retries,
                        e,
                        delay_seconds
                    );
                    sleep(Duration::from_secs(delay_seconds)).await;
                }
            }
        }
    }
}

async fn get_parent_beacon_block_slot(
    beacon_rpc_url: &String,
    parent_beacon_block_root: &FixedBytes<32>,
    max_retries: u32,
    delay_seconds: u64,
) -> anyhow::Result<u64> {
    let url = format!(
        "{}{}{}",
        beacon_rpc_url,
        constants::GET_BEACON_BLOCK_API_PATH,
        parent_beacon_block_root
    );

    // Use the new retry helper
    let response: BeaconBlock = get_with_retry(url, max_retries, delay_seconds).await?;

    // Print for debug
    println!("{response:?}");

    let slot = response.data.message.slot.parse::<u64>()?;
    Ok(slot)
}

async fn find_commitment_in_sidecars(
    beacon_rpc_url: &String,
    beacon_slot: u64,
    commitment: &Bytes,
    max_retries: u32,
    delay_seconds: u64,
) -> anyhow::Result<Option<BlobSidecarData>> {
    let url = format!(
        "{}{}{}",
        beacon_rpc_url,
        constants::GET_SIDECARS_API_PATH,
        beacon_slot
    );

    let response: BlobSidecars = get_with_retry(url, max_retries, delay_seconds).await?;

    let sidecars_filtered: Vec<BlobSidecarData> = response
        .data
        .into_iter()
        .filter(|sidecar| &sidecar.kzg_commitment == commitment)
        .collect();

    if sidecars_filtered.len() == 1 {
        Ok(sidecars_filtered.into_iter().next())
    } else {
        Ok(None)
    }
}

/// Convert hex-encoded blob string (0x + 2*BYTES_PER_BLOB hex chars) into a [u8; BYTES_PER_BLOB].
fn blob_vec_from_string(prefixed_blob: String) -> anyhow::Result<[u8; BYTES_PER_BLOB]> {
    if prefixed_blob.len() != BYTES_PER_BLOB * 2 + 2 {
        return Err(format_error!(
            "Invalid blob length: {}",
            prefixed_blob.len()
        ));
    }

    let mut byte_array = [0u8; BYTES_PER_BLOB];

    let blob = &prefixed_blob[2..];

    for (i, byte) in byte_array.iter_mut().enumerate() {
        let hex_byte = &blob[i * 2..i * 2 + 2];
        *byte = u8::from_str_radix(hex_byte, 16)
            .map_err(|_| format_error!("Invalid hex string value: {}", hex_byte.to_string()))?;
    }
    Ok(byte_array)
}

/// Repeatedly increments slot until we find the needed blob sidecar. Each
/// individual RPC call is retried up to `max_retries`.
pub async fn find_blob(
    beacon_rpc_url: &String,
    commitment: &Bytes,
    parent_beacon_block_root: &FixedBytes<32>,
    max_retries: u32,
    delay_seconds: u64,
) -> anyhow::Result<[u8; BYTES_PER_BLOB]> {
    log_info!("Finding a blob...");

    let mut slot = get_parent_beacon_block_slot(
        beacon_rpc_url,
        parent_beacon_block_root,
        max_retries,
        delay_seconds,
    )
    .await?;

    // Check sidecars for the correct commitment; if not found, increment slot
    // and keep going. Each call is also retried on failure.
    let mut blob_sidecar_data = find_commitment_in_sidecars(
        beacon_rpc_url,
        slot,
        commitment,
        max_retries,
        delay_seconds,
    )
    .await?;

    while blob_sidecar_data.is_none() {
        slot += 1;
        blob_sidecar_data = find_commitment_in_sidecars(
            beacon_rpc_url,
            slot,
            commitment,
            max_retries,
            delay_seconds,
        )
        .await?;
    }

    let blob = blob_sidecar_data
        .ok_or_else(|| {
            format_error!("Failed to find blob sidecar for commitment: {}", {
                commitment.to_string()
            })
        })?
        .blob;

    let blob_array = blob_vec_from_string(blob)?;
    Ok(blob_array)
}
