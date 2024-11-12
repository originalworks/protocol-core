use c_kzg::{ethereum_kzg_settings, Blob, KzgCommitment, KzgProof};
use ow_blob_codec::decoder;
use ow_blob_codec::{blob_from_dir, blob_from_file};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::error::Error;
use std::fs;
use std::{fs::File, io::Read};

const TEMP_DIR: &str = "./tests/temp";
const VALID_XML_DIR: &str = "./tests/assets/valid_xml";
const VALID_XML_PATH: &str = "./tests/assets/valid_xml/ERN_example_1.xml";

#[test]
fn single_file_roundtrip() {
    let mut file = File::open(VALID_XML_PATH).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();

    let blob = blob_from_file(VALID_XML_PATH).unwrap();
    let recovered = decoder::blob_to_vecs(blob).unwrap();

    assert_eq!(file_buffer, recovered[0]);
}

#[test]
fn dir_roundtrip() {
    let blob = blob_from_dir(VALID_XML_DIR).unwrap();

    let files = fs::read_dir(VALID_XML_DIR).unwrap();
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
    let blob = blob_from_dir(VALID_XML_DIR).unwrap();
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
    let blob = blob_from_file(VALID_XML_PATH).unwrap();
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

fn generate_large_xml(element_count: usize) -> Result<String, Box<dyn Error>> {
    let file_path = format!("{}{}", TEMP_DIR, "/file.xml");
    let file = File::create(&file_path).unwrap();
    let mut writer = Writer::new(file);

    writer
        .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
        .unwrap();

    writer
        .write_event(Event::Start(BytesStart::new("root")))
        .unwrap();

    for i in 0..element_count {
        writer
            .write_event(Event::Start(BytesStart::new("item")))
            .unwrap();
        let content = format!("This is item number {}", i);
        writer
            .write_event(Event::Text(BytesText::new(&content)))
            .unwrap();
        writer
            .write_event(Event::End(BytesEnd::new("item")))
            .unwrap();
    }
    writer
        .write_event(Event::End(BytesEnd::new("root")))
        .unwrap();

    Ok(file_path)
}

#[test]
#[should_panic]
fn panic_for_blob_overflow() {
    let very_big_file_path = generate_large_xml(100000).unwrap();
    blob_from_file(very_big_file_path.as_str()).unwrap();
}
