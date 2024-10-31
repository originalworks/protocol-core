use methods::DDEX_PARSER_GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    let data = include_str!("../../res/0Audio.json");
    let mut writer = Vec::new();

    let env = ExecutorEnv::builder()
        .segment_limit_po2(19)
        .write(&data)
        .unwrap()
        .stdout(&mut writer) // 'Private' data sharing between guest and host, data is not stored in the receipt
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let _receipt = prover.prove(env, DDEX_PARSER_GUEST_ELF).unwrap().receipt;
    // let outputsFromReceipt: Outputs = receipt.journal.decode().unwrap(); // This reads data from receipt
    // let outputsFromStdout: Outputs = from_slice(&writer).unwrap(); // This reads private data
}
