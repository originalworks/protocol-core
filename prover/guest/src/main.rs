use alloy_sol_types::SolValue;
use anyhow::Context;
use blob_codec::BlobCodec;
use ddex_parser::DdexParser;
use prover_interface::{ProvedMessage, ProverPublicOutputs};
use risc0_zkvm::guest::env;
use std::io::{Cursor, Read};

fn get_messages_and_digest() -> anyhow::Result<(Box<[u8; 32]>, Vec<Vec<u8>>)> {
    let mut before_cycle = env::cycle_count();
    let mut input = Vec::<u8>::new();

    env::stdin()
        .read_to_end(&mut input)
        .expect("Failed to read input from env. This shouldn't happen!");

    let blob = BlobCodec::from_vec(input)?;

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

    let messages = blob.decode().with_context(|| "Failed to decode blob")?;

    after_cycle = env::cycle_count();

    println!(
        "Decoding blob took roughly {} cycles",
        after_cycle - before_cycle
    );

    Ok((Box::new(digest), messages))
}

fn main() -> anyhow::Result<()> {
    let total_before_cycle = env::cycle_count();
    let messages;
    let digest;

    match get_messages_and_digest() {
        Ok((digest_res, messages_res)) => {
            messages = messages_res;
            digest = *digest_res;
        }
        Err(_) => {
            let prover_public_outputs = ProverPublicOutputs {
                messages: vec![],
                rejected_messages: vec![],
                valid: false,
                digest: [0; 32].into(),
            };

            env::commit_slice(&prover_public_outputs.abi_encode());
            return Ok(());
        }
    }

    let before_cycle = env::cycle_count();

    let mut reader: Cursor<&Vec<u8>>;
    let mut valid_messages: Vec<ProvedMessage> = vec![];
    let mut rejected_messages: Vec<String> = vec![];
    let valid;

    let msg_id_search = r#""message_id": ""#;

    for message in messages {
        reader = Cursor::new(&message);
        let parsed = DdexParser::from_json_reader(reader);

        match parsed {
            Ok(nrm) => {
                valid_messages.push(ProvedMessage::from_ddex(nrm));
            }
            Err(e) => {
                println!("Rejecting message:{}", e);
                let msg_extract = std::str::from_utf8(&message[..100]).unwrap();

                let mut msg_id = "unidentified";

                if let Some(start) = msg_extract.find(msg_id_search) {
                    let start_index = start + msg_id_search.len();
                    if let Some(end_index) = msg_extract[start_index..].find('"') {
                        msg_id = &msg_extract[start_index..start_index + end_index];
                    }
                };

                rejected_messages.push(msg_id.to_string());
            }
        }
    }

    let after_cycle = env::cycle_count();

    println!(
        "Parsing messages took roughly {} cycles",
        after_cycle - before_cycle
    );

    if rejected_messages.len() > 0 {
        valid_messages.clear();
        valid = false;
    } else {
        valid = true;
    }

    let prover_public_outputs = ProverPublicOutputs {
        messages: valid_messages,
        rejected_messages,
        valid,
        digest: digest.into(),
    };

    env::commit_slice(&prover_public_outputs.abi_encode());

    let total_after_cycle = env::cycle_count();

    println!("TOTAL {} cycles", total_after_cycle - total_before_cycle);

    Ok(())
}
