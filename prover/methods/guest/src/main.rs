use core::Outputs;
use ddex_schema::ddex_parse_str;
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

fn main() {
    let ddex_message_str: String = env::read();
    let sha = *Impl::hash_bytes(&ddex_message_str.as_bytes());
    let parsed = ddex_parse_str(ddex_message_str).unwrap();

    let out = Outputs {
        message: parsed,
        data_hash: sha,
    };

    env::write(&out);

    // env::commit(&out);
}
