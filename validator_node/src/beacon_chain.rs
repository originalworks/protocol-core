use crate::constants;
use alloy::{
    eips::eip4844::BYTES_PER_BLOB,
    primitives::{Bytes, FixedBytes},
};
use log_macros::{format_error, log_info};
use reqwest;
use serde::Deserialize;
use tokio::time::{sleep, Duration};
use anyhow::anyhow;

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

/// Helper to GET from `url` and parse JSON into `T`, with up to `max_retries` attempts.
/// If any step fails (bad HTTP status, invalid JSON, etc.), it logs details, sleeps for
/// `delay_seconds`, and tries again. Logs the body text on error so you can see what's
/// actually being returned by the server.
async fn get_with_retry<T: serde::de::DeserializeOwned>(
    url: &str,
    max_retries: u32,
    delay_seconds: u64,
) -> anyhow::Result<T> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        let request_result = reqwest::get(url).await;

        let this_attempt_failed = match request_result {
            Ok(resp) => {
                let status = resp.status();

                // Read entire body as text so we can log it on error
                let body_text_result = resp.text().await;
                match body_text_result {
                    Ok(body_text) => {
                        // If status != 200..=299, treat that as an error
                        if !status.is_success() {
                            log::warn!(
                                "[Beacon RPC] Non-2xx status (attempt {} of {}): {}, body:\n{}",
                                attempt,
                                max_retries,
                                status,
                                body_text,
                            );
                            true // means "error, will retry"
                        } else {
                            // Attempt to parse JSON
                            match serde_json::from_str::<T>(&body_text) {
                                Ok(parsed) => {
                                    // Success, return
                                    return Ok(parsed);
                                }
                                Err(e) => {
                                    // Log the raw body so we can see what's invalid
                                    log::warn!(
                                        "[Beacon RPC] JSON parse error (attempt {} of {}): {}\nBody was:\n{}",
                                        attempt,
                                        max_retries,
                                        e,
                                        body_text
                                    );
                                    true
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!(
                            "[Beacon RPC] Could not read body text (attempt {} of {}): {}",
                            attempt,
                            max_retries,
                            e
                        );
                        true
                    }
                }
            }
            Err(e) => {
                log::warn!(
                    "[Beacon RPC] Request error (attempt {} of {}): {}",
                    attempt,
                    max_retries,
                    e
                );
                true
            }
        };

        // If we get here, it means something failed on this attempt
        if this_attempt_failed {
            if attempt >= max_retries {
                log::error!(
                    "[Beacon RPC] Failed after {} attempts. Giving up.",
                    max_retries
                );
                return Err(anyhow!("Beacon RPC call failed after {} attempts.", max_retries));
            }
            // Sleep and retry
            sleep(Duration::from_secs(delay_seconds)).await;
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

    let response: BeaconBlock = get_with_retry(&url, max_retries, delay_seconds).await?;

    // For debugging:
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

    let response: BlobSidecars = get_with_retry(&url, max_retries, delay_seconds).await?;

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
/// call is retried up to `max_retries`. If none succeed, returns an error.
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

    // Keep trying next slots until we find the sidecar that has our commitment
    // (and each call is internally retried, if the server is unresponsive).
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
