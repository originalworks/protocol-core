use alloy_sol_types::SolValue;
use core::str;
use log_macros::log_info;
use prover::ProverPublicOutputs;
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use std::time::Instant;

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

        log_info!(
            "It took {}h{}m{}s to {}",
            (secs / 60) / 60,
            (secs / 60) % 60,
            secs % 60,
            topic,
        );
    }
}

pub struct ProverRunResults {
    pub seal: Vec<u8>,
    pub journal: Vec<u8>,
    #[allow(dead_code)]
    pub public_outputs: ProverPublicOutputs,
}

pub fn run(
    blob: &Vec<u8>,
    image_elf: &[u8],
    segment_limit_po2: u32,
) -> anyhow::Result<ProverRunResults> {
    log_info!("Proving...");
    let timer = StopWatch::start();

    let mut env_builder = ExecutorEnv::builder();

    if segment_limit_po2 != 0 {
        env_builder.segment_limit_po2(segment_limit_po2);
    }

    let env = env_builder.write_slice(blob).build()?;

    let prover = default_prover();

    let receipt = prover
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            image_elf,
            &ProverOpts::groth16(),
        )?
        .receipt;

    let seal = encode_seal(&receipt)?;

    let journal = receipt.journal.bytes.clone();

    let public_outputs: ProverPublicOutputs = ProverPublicOutputs::abi_decode(&journal, true)?;

    log_info!("Public outputs: {:?}", public_outputs);
    log_info!("Journal: {:?}", journal);
    log_info!("Seal: {:?}", seal);

    timer.stop("produce the proof");

    Ok(ProverRunResults {
        seal,
        journal,
        public_outputs,
    })
}
