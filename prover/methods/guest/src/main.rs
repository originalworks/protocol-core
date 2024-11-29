use blob_codec::BlobCodec;
use ddex_schema::ddex_parse_json_reader;
use risc0_zkvm::guest::env;
use shared::{Message, PublicOutputs};
use std::io::{Cursor, Read};

fn main() {
    let total_before_cycle = env::cycle_count();
    let mut before_cycle = env::cycle_count();

    let mut input = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input).unwrap();
    let blob = BlobCodec::from_vec(input).unwrap();

    let mut after_cycle = env::cycle_count();
    println!(
        "Parsing blob took roughly {} cycles",
        after_cycle - before_cycle
    );

    before_cycle = env::cycle_count();

    let digest = blob.digest();

    after_cycle = env::cycle_count();
    println!(
        "Calculating digest took roughly {} cycles",
        after_cycle - before_cycle
    );

    before_cycle = env::cycle_count();

    let messages = blob.decode().unwrap();

    after_cycle = env::cycle_count();
    println!(
        "Decoding blob took roughly {} cycles",
        after_cycle - before_cycle
    );

    before_cycle = env::cycle_count();

    let mut reader: Cursor<&Vec<u8>>;
    let mut results: Vec<Message> = vec![];

    for message in messages {
        reader = Cursor::new(&message);
        let parsed = ddex_parse_json_reader(reader).unwrap();
        results.push(Message {
            message_id: parsed.message_header.message_id,
        });
    }

    after_cycle = env::cycle_count();
    println!(
        "Parsing messages took roughly {} cycles",
        after_cycle - before_cycle
    );

    env::commit(&PublicOutputs {
        messages: results,
        is_valid: true,
        digest,
    });

    let total_after_cycle = env::cycle_count();
    println!("TOTAL {} cycles", total_after_cycle - total_before_cycle);
}
