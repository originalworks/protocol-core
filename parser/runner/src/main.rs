use ddex_schema::NewReleaseMessage;
use serde_valid::json::FromJsonStr;
use std::fs;
use subgraph_schema::new_release_message_to_subgraph;
fn main() {
    // Read and validate xml file against the schema & save it to a file in json format
    // let parsed_xml = ddex_parse_xml_file("resources/example_messages/0Audio_lite.xml").unwrap();

    // match parsed_xml {
    //     DdexMessage::NewRelease(res) => {
    //         let json_output = res.to_json_string_pretty().unwrap();
    //         fs::write("0Audio_lite.json", json_output).unwrap();
    //     }
    // }

    // Read and validate json file against the schema

    let json_string = fs::read_to_string("../resources/example_messages/0Audio_lite.json").unwrap();
    let parsed_json = NewReleaseMessage::from_json_str(&json_string).unwrap();
    let sg_schema = new_release_message_to_subgraph(parsed_json);
    dbg!(sg_schema);
}
