use crate::constants::{self, REQWEST_CLIENT};
use alloy::{
    eips::eip4844::BYTES_PER_BLOB,
    primitives::{Bytes, FixedBytes},
};
use log_macros::{format_error, log_info, log_warn};
use serde::Deserialize;

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

pub struct BlobFinder {
    beacon_rpc_url: String,
}

impl BlobFinder {
    pub fn new(beacon_rpc_url: String) -> Self {
        Self { beacon_rpc_url }
    }

    async fn get_parent_beacon_block_slot(
        &self,
        parent_beacon_block_root: &FixedBytes<32>,
    ) -> anyhow::Result<u64> {
        let url = format!(
            "{}{}{}",
            self.beacon_rpc_url,
            constants::GET_BEACON_BLOCK_API_PATH,
            parent_beacon_block_root
        );

        log_info!("Fetching initial beacon block slot...");

        let response = REQWEST_CLIENT.get(url).send().await?;
        let slot;

        if response.status().is_success() {
            let beacon_block = response.json::<BeaconBlock>().await?;
            slot = beacon_block.data.message.slot.parse::<u64>()?;
        } else {
            let reason = response.text().await?;
            return Err(format_error!("Beacon RPC returned error: {}", reason));
        }

        log_info!("Initial beacon block slot: {}", slot);

        Ok(slot)
    }

    async fn find_commitment_in_sidecars(
        &self,
        beacon_slot: u64,
        commitment: &Bytes,
    ) -> anyhow::Result<Option<BlobSidecarData>> {
        log_info!("Fetching beacon block at slot {}...", beacon_slot);

        let url = format!(
            "{}{}{}",
            self.beacon_rpc_url,
            constants::GET_SIDECARS_API_PATH,
            beacon_slot
        );

        let response = REQWEST_CLIENT.get(url).send().await?;
        let status_code = response.status().as_u16();

        if status_code == 200 {
            log_info!("Beacon block found");
            let blob_sidecars = response.json::<BlobSidecars>().await?;
            return Ok(blob_sidecars
                .data
                .into_iter()
                .find(|sidecar| &sidecar.kzg_commitment == commitment));
        } else if status_code == 404 {
            log_warn!("Beacon block at slot {} not found", { beacon_slot });
            return Ok(None);
        } else {
            let reason = response.text().await?;
            return Err(format_error!("Beacon RPC returned error: {}", reason));
        }
    }

    fn blob_vec_from_string(prefixed_blob: String) -> anyhow::Result<Vec<u8>> {
        if prefixed_blob.len() != BYTES_PER_BLOB * 2 + 2 {
            return Err(format_error!(
                "Invalid blob length: {}",
                prefixed_blob.len()
            ));
        }

        let mut byte_vec = vec![0u8; BYTES_PER_BLOB];

        let blob = &prefixed_blob[2..];

        for (i, byte) in byte_vec.iter_mut().enumerate() {
            let hex_byte = &blob[i * 2..i * 2 + 2];
            *byte = u8::from_str_radix(hex_byte, 16)
                .map_err(|_| format_error!("Invalid hex string value: {}", hex_byte.to_string()))?;
        }
        Ok(byte_vec)
    }

    pub async fn find(
        &self,
        commitment: &Bytes,
        parent_beacon_block_root: &FixedBytes<32>,
    ) -> anyhow::Result<Vec<u8>> {
        let slot = self
            .get_parent_beacon_block_slot(parent_beacon_block_root)
            .await?;
        let mut blob_sidecar_data: Option<BlobSidecarData> =
            self.find_commitment_in_sidecars(slot, &commitment).await?;

        let mut next_slot = slot;

        while blob_sidecar_data.is_none() {
            if next_slot - slot >= 20 {
                return Err(format_error!(
                    "Looked for commitment in 20 blocks. Aborting."
                ));
            }

            log_info!(
                "Commitment not found in beacon block at slot {}. Looking at next one",
                slot
            );

            next_slot += 1;
            blob_sidecar_data = self
                .find_commitment_in_sidecars(next_slot, &commitment)
                .await?;
        }
        log_info!("Commitment found in beacon block at slot {}", slot);

        let blob = blob_sidecar_data
            .ok_or_else(|| {
                format_error!("Failed to find blob sidecar for commitment: {}", {
                    commitment.to_string()
                })
            })?
            .blob;

        let blob_vec = Self::blob_vec_from_string(blob)?;

        Ok(blob_vec)
    }
}
