use alloy::{consensus::Blob, eips::eip7594::BlobTransactionSidecarEip7594};
use anyhow::Context;
use blob_codec::BlobCodec;
use log_macros::log_info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct BlobTransactionData {
    pub kzg_commitment: Vec<u8>,
    pub blob_sidecar: BlobTransactionSidecarEip7594,
    pub blob_sha2: [u8; 32],
}

impl BlobTransactionData {
    pub fn build(output_files_dir: &String) -> anyhow::Result<Self> {
        log_info!("Creating blob...");
        let blob_codec = BlobCodec::from_dir(output_files_dir, None)?;
        let blob_sha2: [u8; 32] = blob_codec.digest();
        let blob: [u8; 131072] = blob_codec.to_bytes();
        let blob: Blob = blob.into();

        let sidecar = BlobTransactionSidecarEip7594::try_from_blobs(vec![blob])?;
        let kzg_commitment = sidecar
            .commitments
            .first()
            .context("No commitment in sidecar")?
            .to_owned();

        Ok(BlobTransactionData {
            kzg_commitment: kzg_commitment.to_vec(),
            blob_sidecar: sidecar,
            blob_sha2,
        })
    }
}
