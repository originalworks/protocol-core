#[test]
fn prover_playground() {
    std::env::set_var("RISC0_DEV_MODE", "1");
    let path = "tests/resources/playground/2024-12-05T15-45-38";
    let parsed_xml =
        ddex_parser::DdexParser::from_xml_file(format!("{}.xml", path).as_str()).unwrap();
    let parsed_json = serde_json::to_string_pretty(&parsed_xml).unwrap();
    std::fs::write(format!("{}.json", path).as_str(), parsed_json).unwrap();

    let blob = blob_codec::BlobCodec::from_file(format!("{}.json", path).as_str()).unwrap();
    let result = validator_node::prover_wrapper::run(&blob.to_bytes().into(), 18);

    if let Ok(res) = result {
        println!("Is valid: {}", res.public_outputs.valid);
    } else {
        dbg!("Nope");
    }
}
