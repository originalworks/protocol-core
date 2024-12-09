use c_kzg::{ethereum_kzg_settings, Blob, Bytes32, KzgCommitment, KzgProof};
use std::error::Error;

pub fn calculate_kzg(
    blob_bytes: [u8; 131072],
    z_point: [u8; 32],
) -> Result<Bytes32, Box<dyn Error>> {
    let z_bytes: Bytes32 = Bytes32::new(z_point);

    let settings = ethereum_kzg_settings();

    let blob = Blob::new(blob_bytes);
    let commitment = KzgCommitment::blob_to_kzg_commitment(&blob, settings).unwrap();
    let (kzg_proof, y_bytes) = KzgProof::compute_kzg_proof(&blob, &z_bytes, &settings).unwrap();

    let is_valid = KzgProof::verify_kzg_proof(
        &commitment.to_bytes(),
        &z_bytes,
        &y_bytes,
        &kzg_proof.to_bytes(),
        &settings,
    )
    .unwrap();

    assert!(is_valid);
    Ok(y_bytes)
}
