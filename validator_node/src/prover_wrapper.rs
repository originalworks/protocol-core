use alloy_sol_types::SolValue;
use core::str;
use prover::{ProverPublicOutputs, DDEX_GUEST_ELF, DDEX_GUEST_ID};
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use std::{error::Error, time::Instant};

pub struct StopWatch {
    timer: Instant,
}

impl StopWatch {
    pub fn start() -> Self {
        StopWatch {
            timer: Instant::now(),
        }
    }
    pub fn stop(self, topic: &str) -> () {
        let secs = self.timer.elapsed().as_secs();

        println!(
            "It took {}h{}m{}s to {}",
            (secs / 60) / 60,
            (secs / 60) % 60,
            secs % 60,
            topic
        );
    }
}

pub struct ProverRunResults {
    pub seal: Vec<u8>,
    pub journal: Vec<u8>,
    pub public_outputs: ProverPublicOutputs,
}

pub fn run(blob: &Vec<u8>) -> Result<ProverRunResults, Box<dyn Error>> {
    env_logger::init();
    let mut timer = StopWatch::start();

    let env = ExecutorEnv::builder()
        .segment_limit_po2(19)
        .write_slice(blob)
        .build()
        .unwrap();

    let prover = default_prover();

    let receipt = prover
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            DDEX_GUEST_ELF,
            &ProverOpts::groth16(),
        )
        .unwrap()
        .receipt;

    let seal = encode_seal(&receipt).unwrap();

    let journal = receipt.journal.bytes.clone();

    let public_outputs: ProverPublicOutputs =
        ProverPublicOutputs::abi_decode(&journal, true).unwrap();

    println!("public outputs: {public_outputs:?}");
    println!("journal: {journal:?}");
    println!("seal: {seal:?}");

    timer.stop("produce the proof");

    timer = StopWatch::start();

    match receipt.verify(DDEX_GUEST_ID) {
        Ok(_) => {
            println!("Receipt has been verified to be computed with DDEX_PARSER_GUEST code image")
        }
        Err(_) => println!("Receipt failed to be verified"),
    }

    timer.stop("verify the proof.");

    Ok(ProverRunResults {
        seal,
        journal,
        public_outputs,
    })
}
