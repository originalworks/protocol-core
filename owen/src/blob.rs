use alloy::consensus::BlobTransactionSidecar;
use blob_codec::{BlobCodec, CalldataLimitConfig};
use c_kzg::{ethereum_kzg_settings, Blob, KzgCommitment};
use log_macros::{format_error, log_info};

pub struct BlobTransactionData {
    pub kzg_commitment: KzgCommitment,
    pub blob_sidecar: BlobTransactionSidecar,
    pub blob_sha2: [u8; 32],
}

impl BlobTransactionData {
    pub fn build(output_files_dir: &String) -> anyhow::Result<Self> {
        log_info!("Creating blob...");
        let blob_codec =
            BlobCodec::from_dir(output_files_dir, Some(CalldataLimitConfig::default()))?;
        let blob_sha2: [u8; 32] = blob_codec.digest();
        let blob: [u8; 131072] = blob_codec.to_bytes();

        let kzg_blob = Blob::new(blob);

        let kzg_settings = ethereum_kzg_settings(0u64);

        let kzg_commitment = kzg_settings.blob_to_kzg_commitment(&kzg_blob)?;
        let kzg_proof =
            kzg_settings.compute_blob_kzg_proof(&kzg_blob, &kzg_commitment.to_bytes())?;

        let is_valid = kzg_settings.verify_blob_kzg_proof(
            &kzg_blob,
            &kzg_commitment.to_bytes(),
            &kzg_proof.to_bytes(),
        )?;
        if is_valid {
            let blob_sidecar: BlobTransactionSidecar = BlobTransactionSidecar::from_kzg(
                vec![kzg_blob],
                vec![kzg_commitment.to_bytes()],
                vec![kzg_proof.to_bytes()],
            );

            Ok(BlobTransactionData {
                kzg_commitment,
                blob_sidecar,
                blob_sha2,
            })
        } else {
            return Err(format_error!("c_kzg error during proof validation"));
        }
    }
}
