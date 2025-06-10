// use validator_node::prover_wrapper::ProverRunResults;

use validator_node::blob_proofs::{BlobProofManager, ProverRunResults};

fn produce_proof(dir: &str) -> Result<ProverRunResults, anyhow::Error> {
    std::env::set_var("RISC0_DEV_MODE", "1");
    let blob =
        blob_codec::BlobCodec::from_dir(dir, Some(blob_codec::CalldataLimitConfig::default()))
            .unwrap();
    BlobProofManager::run_prover(&blob.to_bytes().into(), prover::CURRENT_DDEX_GUEST_ELF, 18)
}

#[test]
fn prover_valid() {
    let output = produce_proof("tests/resources/valid").unwrap();

    assert_eq!(output.public_outputs.valid, true);
    assert_eq!(output.public_outputs.rejected_messages.len(), 0);
    assert_eq!(output.public_outputs.messages.len(), 2);
}

#[test]
fn prover_rejected() {
    let output = produce_proof("tests/resources/rejected").unwrap();
    assert_eq!(output.public_outputs.valid, false);
    assert_eq!(output.public_outputs.rejected_messages.len(), 3);
    assert_eq!(output.public_outputs.messages.len(), 0);
    assert!(vec![
        "225cd629-6e6e-4e24-a824-65879dd90cbf",
        "unidentified",
        "**]]Some other format[[**"
    ]
    .iter()
    .all(|item| output
        .public_outputs
        .rejected_messages
        .contains(&item.to_string())));
}

#[test]
fn prover_mixed() {
    let output = produce_proof("tests/resources/mixed").unwrap();

    assert_eq!(output.public_outputs.valid, false);
    assert_eq!(output.public_outputs.rejected_messages.len(), 1);
    assert_eq!(output.public_outputs.messages.len(), 0);
    assert_eq!(
        output.public_outputs.rejected_messages,
        vec!["225cd629-6e6e-4e24-a824-65879dd90cbf"]
    );
}

#[test]
fn prover_faulty_blob() {
    std::env::set_var("RISC0_DEV_MODE", "1");
    let output =
        BlobProofManager::run_prover(&[1; 131072].into(), prover::CURRENT_DDEX_GUEST_ELF, 18)
            .unwrap();

    assert_eq!(output.public_outputs.valid, false);
    assert_eq!(output.public_outputs.rejected_messages.len(), 0);
    assert_eq!(output.public_outputs.messages.len(), 0);
}
