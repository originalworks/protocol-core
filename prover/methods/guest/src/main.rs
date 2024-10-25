use core::Outputs;
use ddex_schema::ddex_parse_str;
use risc0_zkvm::guest::env;

fn main() {
    let ddex_message_str: String = env::read();
    let parsed = ddex_parse_str(ddex_message_str).unwrap();

    let out = Outputs { message: parsed };

    env::write(&out);

    // env::commit(&out);
}
