use ddex_schema::DdexParser;
use std::fs;
fn main() {
    // Read and validate xml file against the schema & save it to a file in json format
    // let parsed_xml = DdexParser::from_xml_file("../resources/example_messages/0Audio_lite.xml");

    // match parsed_xml {
    //     Ok(_) => {
    //         dbg!("SUCCESS");
    //         ()
    //     }
    //     Err(e) => {
    //         println!("NORMAL\n  {e}");
    //         println!("DEBUG \n {e:?}")
    //     }
    // }
    // match parsed_xml {
    //     DdexMessage::NewRelease(res) => {
    //         let json_output = res.to_json_string_pretty().unwrap();
    //         fs::write("0Audio_lite.json", json_output).unwrap();
    //     }
    // }

    // Read and validate json file against the schema

    let json_string = fs::read_to_string("../resources/example_messages/0Audio_lite.json").unwrap();
    let parsed_json = DdexParser::from_json_string(&json_string);

    match parsed_json {
        Ok(_) => {
            dbg!("SUCCESS");
            ()
        }
        Err(e) => {
            println!("NORMAL\n  {e}");
            println!("DEBUG \n {e:?}")
        }
    }
}
