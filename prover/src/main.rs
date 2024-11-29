use blob_codec::BlobCodec;
use core::str;
use methods::{DDEX_PARSER_GUEST_ELF, DDEX_PARSER_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use shared::PublicOutputs;
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

        println!(
            "It took {}h{}m{}s to {}",
            (secs / 60) / 60,
            (secs / 60) % 60,
            secs % 60,
            topic
        );
    }
}
fn main() {
    env_logger::init();
    let mut timer = StopWatch::start();
    // let blob = BlobCodec::from_file("res/0Audio_lite.json").unwrap();
    let blob = BlobCodec::from_dir("res").unwrap();
    let env = ExecutorEnv::builder()
        .segment_limit_po2(19)
        .write_slice(&blob.to_bytes().to_vec())
        .build()
        .unwrap();

    let prover = default_prover();

    let receipt = prover.prove(env, DDEX_PARSER_GUEST_ELF).unwrap().receipt;

    let public_outputs: PublicOutputs = receipt.journal.decode().unwrap();

    println!(
        "Values decoded from receipt:: Verified: {}, Digest: {}",
        public_outputs.is_valid,
        String::from_utf8_lossy(&public_outputs.digest),
    );

    timer.stop("produce the proof");

    timer = StopWatch::start();

    match receipt.verify(DDEX_PARSER_GUEST_ID) {
        Ok(_) => {
            println!("Receipt has been verified to be computed with DDEX_PARSER_GUEST code image")
        }
        Err(_) => println!("Receipt failed to be verified"),
    }

    timer.stop("verify the proof.");
}
