use core::PublicOutputs;
use ddex_schema::{ddex_parse_json_str, DdexMessage};
use risc0_zkvm::guest::env;

fn main() {
    let ddex_message_str: String = env::read();
    let parsed = ddex_parse_json_str(ddex_message_str);

    match parsed {
        Ok(content) =>
        {
            #[allow(irrefutable_let_patterns)]
            if let DdexMessage::NewRelease(new_release) = content {
                env::commit(&PublicOutputs {
                    message_id: Some(new_release.message_header.message_id),
                    is_valid: true,
                })
            }
        }
        Err(_e) => {
            env::commit(&PublicOutputs {
                message_id: None,
                is_valid: false,
            });
        }
    }
}
