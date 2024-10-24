use core::Outputs;
use methods::DDEX_PARSER_GUEST_ELF;
use risc0_zkvm::{default_prover, serde::from_slice, ExecutorEnv};

fn main() {
    let data = include_str!("../../res/2Video.xml");
    let mut writer = Vec::new();

    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .stdout(&mut writer) // 'Private' data sharing between guest and host, data is not stored in the receipt
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, DDEX_PARSER_GUEST_ELF).unwrap().receipt;
    // let outputsFromReceipt: Outputs = receipt.journal.decode().unwrap(); // This reads data from receipt
    let outputsFromStdout: Outputs = from_slice(&writer).unwrap();
    dbg!(outputsFromStdout); // This reads private data
}
