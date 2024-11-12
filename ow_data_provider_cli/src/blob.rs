use crate::{constants::OUTPUT_FILES_DIR, errors::OwDataProviderCliError};
use alloy::consensus::BlobTransactionSidecar;
use c_kzg::{ethereum_kzg_settings, Blob, KzgCommitment, KzgProof};
use std::error::Error;

pub struct BlobTransactionData {
    pub kzg_commitment: KzgCommitment,
    pub blob_sidecar: BlobTransactionSidecar,
}

impl BlobTransactionData {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let blob: [u8; 131072] = ow_blob_codec::blob_from_dir(OUTPUT_FILES_DIR)?;

        let kzg_blob = Blob::new(blob);

        let kzg_settings = ethereum_kzg_settings();

        let kzg_commitment = KzgCommitment::blob_to_kzg_commitment(&kzg_blob, kzg_settings)?;
        let kzg_proof =
            KzgProof::compute_blob_kzg_proof(&kzg_blob, &kzg_commitment.to_bytes(), &kzg_settings)?;

        let is_valid = KzgProof::verify_blob_kzg_proof(
            &kzg_blob,
            &kzg_commitment.to_bytes(),
            &kzg_proof.to_bytes(),
            &kzg_settings,
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
            })
        } else {
            return Err(Box::new(OwDataProviderCliError::InvalidBlobProof()));
        }
    }
}
