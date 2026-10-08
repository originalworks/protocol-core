use alloy::{
    consensus::Blob,
    eips::eip7594::BlobTransactionSidecarEip7594,
    primitives::{Bytes, FixedBytes},
};
use anyhow::Context;
use blob_codec::BlobCodec;
use log_macros::log_info;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

pub fn commitment_to_blobhash(commitment: &Bytes) -> FixedBytes<32> {
    let mut hasher = Sha256::new();
    hasher.update(commitment);
    let mut hashed_commitment = hasher.finalize();
    hashed_commitment[0] = 1;

    let mut fixed_bytes_input: [u8; 32] = [0u8; 32];
    fixed_bytes_input.copy_from_slice(&hashed_commitment);

    FixedBytes::<32>::from(fixed_bytes_input)
}
