#[test]
fn prover() {
    let path = "tests/resources/2024-12-05T21-35-15";
    let parsed_xml =
        ddex_parser::DdexParser::from_xml_file(format!("{}.xml", path).as_str()).unwrap();
    let parsed_json = serde_json::to_string_pretty(&parsed_xml).unwrap();
    std::fs::write(format!("{}.json", path).as_str(), parsed_json).unwrap();

    let blob = blob_codec::BlobCodec::from_file(format!("{}.json", path).as_str()).unwrap();
    let result = validator_node::prover_wrapper::run(&blob.to_bytes().into(), 18);

    if let Ok(_) = result {
        dbg!("Ok");
    } else {
        dbg!("Nope");
    }
}
