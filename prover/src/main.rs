use prover::{PublicOutputs, DDEX_GUEST_ELF, DDEX_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::time::Instant;

fn main() {
    env_logger::init();
    let mut timer = Instant::now();
    let data = include_str!("../res/0Audio_lite.json");
    // let mut writer = Vec::new();

    let env = ExecutorEnv::builder()
        .segment_limit_po2(19)
        .write(&data)
        .unwrap()
        // .stdout(&mut writer) // 'Private' data sharing between guest and host, data is not stored in the receipt
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, DDEX_GUEST_ELF).unwrap().receipt;
    let mut secs = timer.elapsed().as_secs();

    // This reads data from receipt
    let public_outputs: PublicOutputs = receipt.journal.decode().unwrap();
    // OR
    // This reads private data from stdout
    // let private_outputs: PrivateOutputs = from_slice(&writer).unwrap();

    println!(
        "Values decoded from receipt:: Verified: {}, MessageId: {}",
        public_outputs.is_valid,
        public_outputs
            .message_id
            .unwrap_or_else(|| "None".to_string())
    );

    println!(
        "It took {}h{}m{}s to produce the proof.",
        (secs / 60) / 60,
        (secs / 60) % 60,
        secs % 60,
    );

    timer = Instant::now();

    match receipt.verify(DDEX_GUEST_ID) {
        Ok(_) => {
            println!("Receipt has been verified to be computed with DDEX_PARSER_GUEST code image")
        }
        Err(_) => println!("Receipt failed to be verified"),
    }

    secs = timer.elapsed().as_secs();

    println!(
        "It took {}h{}m{}s to verify the proof.",
        (secs / 60) / 60,
        (secs / 60) % 60,
        secs % 60,
    );
}
