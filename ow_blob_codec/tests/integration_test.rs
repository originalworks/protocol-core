use c_kzg::{ethereum_kzg_settings, Blob, KzgCommitment, KzgProof};
use ow_blob_codec::decoder;
use ow_blob_codec::{blob_from_dir, blob_from_file};
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write};
use std::{fs::File, io::Read};

const TEMP_FILE_PATH: &str = "./tests/temp_file.txt";
const VALID_DIR: &str = "./tests/assets";
const VALID_FILE_PATH: &str = "./tests/assets/msg_one.json";

fn generate_large_text_file(line_count: usize) -> Result<(), Box<dyn Error>> {
    let file = File::create(TEMP_FILE_PATH)?;
    let mut writer = BufWriter::new(file);

    for i in 0..line_count {
        let line = format!("This is line number {}\n", i);
        writer.write_all(line.as_bytes())?;
    }

    Ok(())
}

#[test]
fn single_file_roundtrip() {
    let mut file = File::open(VALID_FILE_PATH).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    let blob = blob_from_file(VALID_FILE_PATH).unwrap();
    let recovered = decoder::blob_to_vecs(blob).unwrap();

    assert_eq!(file_buffer, recovered[0]);
}

#[test]
fn dir_roundtrip() {
    let blob = blob_from_dir(VALID_DIR).unwrap();

    let files = fs::read_dir(VALID_DIR).unwrap();
    let mut raw_files = Vec::new();

    for file_entry in files {
        let path = file_entry.unwrap().path();
        let mut file = File::open(path).unwrap();
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer).unwrap();
        raw_files.push(file_buffer);
    }

    let recovered = decoder::blob_to_vecs(blob).unwrap();

    assert_eq!(raw_files, recovered);
}

#[test]
fn pass_kzg_verification_dir() {
    let blob = blob_from_dir(VALID_DIR).unwrap();
    let kzg_settings = ethereum_kzg_settings();

    let kzg_blob = Blob::new(blob);
    let kzg_commitment = KzgCommitment::blob_to_kzg_commitment(&kzg_blob, kzg_settings).unwrap();
    let kzg_proof =
        KzgProof::compute_blob_kzg_proof(&kzg_blob, &kzg_commitment.to_bytes(), &kzg_settings)
            .unwrap();

    let is_valid = KzgProof::verify_blob_kzg_proof(
        &kzg_blob,
        &kzg_commitment.to_bytes(),
        &kzg_proof.to_bytes(),
        &kzg_settings,
    )
    .unwrap();

    assert!(is_valid);
}

#[test]
fn pass_kzg_verification_file() {
    let blob = blob_from_file(VALID_FILE_PATH).unwrap();
    let kzg_settings = ethereum_kzg_settings();

    let kzg_blob = Blob::new(blob);
    let kzg_commitment = KzgCommitment::blob_to_kzg_commitment(&kzg_blob, kzg_settings).unwrap();
    let kzg_proof =
        KzgProof::compute_blob_kzg_proof(&kzg_blob, &kzg_commitment.to_bytes(), &kzg_settings)
            .unwrap();

    let is_valid = KzgProof::verify_blob_kzg_proof(
        &kzg_blob,
        &kzg_commitment.to_bytes(),
        &kzg_proof.to_bytes(),
        &kzg_settings,
    )
    .unwrap();

    assert!(is_valid);
}

#[test]
#[should_panic]
fn panic_for_blob_overflow() {
    generate_large_text_file(100000).unwrap();
    blob_from_file(TEMP_FILE_PATH).unwrap();
}
