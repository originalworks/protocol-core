use alloy_sol_types::SolValue;
use log_macros::{format_error, log_info};
use prover::{ProverPublicOutputs, CURRENT_DDEX_GUEST_ELF, PREVIOUS_DDEX_GUEST_ELF};
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    blob_assignment::{
        files::BlobAssignmentFiles,
        manager::{BlobAssignment, BlobAssignmentStatus},
    },
    contracts::{LocalImageVersion, SubmitProofInput},
    ipfs::IpfsManager,
};
use std::time::Instant;

pub struct ProverRunResults {
    pub seal: Vec<u8>,
    pub journal: Vec<u8>,
    #[allow(dead_code)]
    pub public_outputs: ProverPublicOutputs,
}

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

pub struct BlobProofManager {
    blob_assignment_files: Arc<Mutex<BlobAssignmentFiles>>,
    ipfs_manager: IpfsManager,
    segment_limit_po2: u32,
}

impl BlobProofManager {
    pub fn new(
        blob_assignment_files: Arc<Mutex<BlobAssignmentFiles>>,
        ipfs_manager: IpfsManager,
        segment_limit_po2: u32,
    ) -> Self {
        Self {
            blob_assignment_files,
            ipfs_manager,
            segment_limit_po2,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let blob_assignment = {
            let assigned_blob_file_mutex = self.blob_assignment_files.lock().await;
            assigned_blob_file_mutex.get_inner_queue_head()?
        };

        if let Some(blob_assignment) = blob_assignment {
            log_info!(
                "PROVING LOOP: New assignment found, {}",
                blob_assignment.blobhash
            );
            if blob_assignment.status == BlobAssignmentStatus::Discovered {
                log_info!("PROVING LOOP: Assignment ready, generating proof");
                self.generate_proof(blob_assignment).await?;
            } else {
                log_info!(
                    "PROVING LOOP: Assignment is not ready yet, status: {:?}. Waiting for changes",
                    &blob_assignment.status
                );

                BlobAssignmentFiles::watch_json_file().await?;
                return Ok(());
            }
            return Ok(());
        } else {
            log_info!("PROVING LOOP: No assignment found, waiting for changes");
            BlobAssignmentFiles::watch_json_file().await?;
            return Ok(());
        }
    }
    async fn generate_proof(&self, blob_assignment: BlobAssignment) -> anyhow::Result<()> {
        let blob_file = {
            let blob_assignment_files = self.blob_assignment_files.lock().await;
            blob_assignment_files.get_blob_file(blob_assignment.blobhash)?
        };

        if let Some(blob) = blob_file {
            self.ipfs_manager
                .build_blob_folder(&blob, &blob_assignment)
                .await?;

            let ipfs_folder_cid = self.ipfs_manager.upload_blob_folder_and_cleanup().await?;

            let local_image_elf;
            if blob_assignment.local_image_version == LocalImageVersion::Current {
                local_image_elf = CURRENT_DDEX_GUEST_ELF;
            } else if blob_assignment.local_image_version == LocalImageVersion::Previous {
                local_image_elf = PREVIOUS_DDEX_GUEST_ELF;
            } else {
                return Err(format_error!(
                    "Couldn't determine local image to use: {}",
                    blob_assignment.blobhash
                ));
            }

            let prover_run_results =
                Self::run_prover(&blob, local_image_elf, self.segment_limit_po2)?;

            let submit_proof_input = SubmitProofInput {
                image_id: blob_assignment.image_id,
                journal: prover_run_results.journal,
                seal: prover_run_results.seal,
                ipfs_cid: ipfs_folder_cid,
            };
            {
                let mut blob_assignment_files = self.blob_assignment_files.lock().await;
                blob_assignment_files.save_proof(blob_assignment.blobhash, submit_proof_input)?;
            }
        } else {
            return Err(format_error!(
                "Blob file not found for assignment: {}",
                blob_assignment.blobhash
            ));
        }
        Ok(())
    }

    pub fn run_prover(
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
}
