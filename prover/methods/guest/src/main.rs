use core::Outputs;
use ddex_schema::DdexMessage;
use risc0_zkvm::guest::env;
use serde_json;

fn main() {
    let ddex_message_str: String = env::read();
    let parsed: DdexMessage = serde_json::from_str(&ddex_message_str).unwrap();
    let out = Outputs { message: parsed };

    env::write(&out);

    // env::commit(&out);
}
