use alloy::primitives::{Bytes, FixedBytes};
use log_macros::{format_error, log_error, log_info, log_warn};
use prover::SubmitProofInput;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    beacon_chain::BlobFinder,
    constants::{EMPTY_BYTES32, MAX_BLOB_ASSIGNMENTS},
    contracts::{ContractsManager, LocalImageVersion},
    Config,
};

use super::files::BlobAssignmentFiles;

pub enum BlobAssignmentStartingPoint {
    NewBlobSubmitted { block_number: u64 },
    BlobProcessedOrRejected { block_number: u64 },
    CleanStart,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum BlobAssignmentStatus {
    Assigned,
    Discovered,
    Processed,
    Sent,
    Failed,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BlobAssignment {
    pub blobhash: FixedBytes<32>,
    pub commitment: Bytes,
    pub status: BlobAssignmentStatus,
    pub assignment_tx_hash: FixedBytes<32>,
    pub blob_submission_block: u64,
    pub image_id: FixedBytes<32>,
    pub proof_submission_tx_hash: Option<FixedBytes<32>>,
    pub proof_submission_input: Option<SubmitProofInput>,
    pub blob_submission_tx_hash: FixedBytes<32>,
    pub blob_submission_timestamp: u64,
    pub chain_id: u64,
    pub local_image_version: LocalImageVersion,
}

pub struct BlobAssignmentManager {
    pub blob_assignment_files: Arc<Mutex<BlobAssignmentFiles>>,
    pub contracts_manager: Arc<ContractsManager>,
    pub blob_finder: BlobFinder,
}

impl BlobAssignmentManager {
    pub fn new(
        blob_assignment_files: Arc<Mutex<BlobAssignmentFiles>>,
        contracts_manager: Arc<ContractsManager>,
        blob_finder: BlobFinder,
    ) -> Self {
        BlobAssignmentManager {
            blob_assignment_files,
            contracts_manager,
            blob_finder,
        }
    }

    pub async fn init_clear_inner_queue(&self) -> anyhow::Result<()> {
        log_info!(
            "Clearing inner queue assignments if they are not existing on the contract anymore"
        );

        let inner_queue = {
            let blob_assignment_files = self.blob_assignment_files.lock().await;
            blob_assignment_files.inner_queue.clone()
        };
        for assignment in inner_queue {
            let blob_data = self
                .contracts_manager
                .sequencer
                .blobs(assignment)
                .call()
                .await?;
            if blob_data.submitted == false {
                log_info!(
                    "Blob {} doesn't exist anymore on the cotract. Removing...",
                    assignment
                );
                {
                    let mut blob_assignment_files = self.blob_assignment_files.lock().await;
                    blob_assignment_files.remove_from_queue(assignment)?;
                }
            }
        }
        Ok(())
    }

    pub async fn run(
        &self,
        config: &Config,
        start_block: u64,
    ) -> anyhow::Result<BlobAssignmentStartingPoint> {
        let queue_head = self.contracts_manager.get_queue_head().await?;

        if queue_head.blobhash == EMPTY_BYTES32 {
            log_info!("ASSIGNMENT LOOP: Queue head is empty");
            let next_starting_point = self
                .contracts_manager
                .subscribe_to_contracts(start_block)
                .await?;

            Ok(next_starting_point)
        } else {
            log_info!(
                "ASSIGNMENT LOOP: Curent queue head: {} assigned to: {}",
                queue_head.blobhash,
                queue_head.blob_data.assignedValidator
            );

            if queue_head.blob_data.assignedValidator == self.contracts_manager.signer.address() {
                log_info!("ASSIGNMENT LOOP: Queue head is assigned to this validator");

                let assigned_blob = self
                    .get_or_create_assigned_blob(queue_head.blobhash)
                    .await?;

                if assigned_blob.status != BlobAssignmentStatus::Processed {
                    log_info!("ASSIGNMENT LOOP: Queue head not processed yet, waiting for changes");
                    BlobAssignmentFiles::watch_json_file().await?;
                    return Ok(BlobAssignmentStartingPoint::CleanStart);
                }

                if let Some(proof_submission_input) = assigned_blob.proof_submission_input {
                    match self
                        .contracts_manager
                        .submit_proof(proof_submission_input)
                        .await
                    {
                        Ok(receipt) => {
                            if receipt.status() == true {
                                {
                                    let mut blob_assignment_files =
                                        self.blob_assignment_files.lock().await;
                                    blob_assignment_files
                                        .archive_head_assignment(receipt.transaction_hash)?;
                                }
                            } else {
                                log_warn!(
                                    "ASSIGNMENT LOOP: Proof submission tx failed. Blobhash: {} tx hash: {}",
                                    assigned_blob.blobhash,
                                    receipt.transaction_hash
                                );
                            }
                        }
                        Err(err) => {
                            let formated_error = format_error!(
                                "ASSIGNMENT LOOP: Sending tx with proof failed {}",
                                err
                            );

                            if formated_error.to_string().contains("time expired") {
                                log_warn!(
                                    "ASSIGNMENT LOOP: Queue head expired, trying new assignment"
                                );
                                return Ok(self.handle_blob_assignment().await?);
                            } else {
                                return Err(formated_error);
                            }
                        }
                    };
                }

                Ok(BlobAssignmentStartingPoint::CleanStart)
            } else {
                log_info!("ASSIGNMENT LOOP: Queue head is not assigned to this validator, trying new assignment");

                Ok(self.try_new_assignment(config, start_block).await?)
            }
        }
    }

    async fn get_or_create_assigned_blob(
        &self,
        blobhash: FixedBytes<32>,
    ) -> anyhow::Result<BlobAssignment> {
        let mut assigned_blob = {
            let mut blob_assignment_files = self.blob_assignment_files.lock().await;

            if !blob_assignment_files.assignments.contains_key(&blobhash) {
                log_warn!("ASSIGNMENT LOOP: Queue head is assigned to validator but no assignment found! Priority assignment will be created");

                let priority_assigned_blob = self
                    .contracts_manager
                    .build_new_assignment(blobhash, FixedBytes([0u8; 32]))
                    .await?;

                blob_assignment_files.save_assignment_as_priority(priority_assigned_blob)?;
            }
            blob_assignment_files
                .assignments
                .get(&blobhash)
                .expect("Assignment must exist by now")
                .clone()
        };

        let blob_file = {
            let blob_assignment_files = self.blob_assignment_files.lock().await;
            blob_assignment_files.get_blob_file(assigned_blob.blobhash)?
        };

        if blob_file.is_none() {
            log_warn!(
                "ASSIGNMENT LOOP: No blob file found for this priority assignment, fetching blob from beacon chain"
            );
            let blob_array = self
                .find_blob(
                    assigned_blob.commitment,
                    assigned_blob.blob_submission_block,
                )
                .await?
                .ok_or(format_error!(
                    "ASSIGNMENT LOOP: Blob not found for priority assignment"
                ))?;

            assigned_blob = {
                let mut blob_assignment_files = self.blob_assignment_files.lock().await;
                blob_assignment_files.save_discovered_blob(assigned_blob.blobhash, blob_array)?
            };
        }

        Ok(assigned_blob)
    }

    pub async fn fetch_current_block(&self) -> anyhow::Result<u64> {
        Ok(self.contracts_manager.fetch_current_block().await?)
    }

    async fn find_blob(
        &self,
        commitment: Bytes,
        submission_block: u64,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let parent_beacon_block_root = self
            .contracts_manager
            .get_parent_beacon_block_root(submission_block)
            .await?;

        let blob_vec = match self
            .blob_finder
            .find(&commitment, &parent_beacon_block_root)
            .await
        {
            Ok(blob) => blob,
            Err(_) => match self.handle_missing_head_blob(commitment).await {
                Ok(_) => return Ok(None),
                Err(e) => {
                    return Err(log_error!(
                        "ASSIGNMENT LOOP: Failed to remove missing head blob: {:?}",
                        e
                    ));
                }
            },
        };

        Ok(Some(blob_vec))
    }

    pub async fn handle_missing_head_blob(&self, commitment: Bytes) -> anyhow::Result<()> {
        log_warn!("ASSIGNMENT LOOP: BLOB for queue head not found, trying to remove expired blob");
        match self.contracts_manager.remove_expired_blob().await {
            Ok(_) => {
                log_info!("ASSIGNMENT LOOP: Expired blob removed successfully");
                let blobhash = ContractsManager::commitment_to_blobhash(&commitment);
                {
                    let mut blob_assignment_files = self.blob_assignment_files.lock().await;
                    blob_assignment_files.remove_from_queue(blobhash)?;
                }
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn try_new_assignment(
        &self,
        config: &Config,
        start_block: u64,
    ) -> anyhow::Result<BlobAssignmentStartingPoint> {
        let assignment_count: usize;
        {
            let blob_assignment_files = self.blob_assignment_files.lock().await;
            assignment_count = blob_assignment_files.inner_queue.len();
        }
        if assignment_count >= MAX_BLOB_ASSIGNMENTS {
            log_info!("ASSIGNMENT LOOP: Max assignments reached, subscribing to contracts");

            Ok(self
                .contracts_manager
                .subscribe_to_contracts(start_block)
                .await?)
        } else {
            log_info!("ASSIGNMENT LOOP: Max assignments not reached, checking next assignment");

            let next_blob_assignment = self.contracts_manager.get_next_blob_assignment().await?;

            if next_blob_assignment == EMPTY_BYTES32 {
                log_info!(
                    "ASSIGNMENT LOOP: Next assignment is empty, checking for expired queue head"
                );

                let queue_head_expired = self.contracts_manager.is_queue_head_expired().await?;

                if queue_head_expired {
                    log_info!("ASSIGNMENT LOOP: Queue head expired, trying to assing new blob");
                    Ok(self.handle_blob_assignment().await?)
                } else {
                    log_info!("ASSIGNMENT LOOP: Next assignment is empty and queue head not expired, subscribing to contracts");
                    Ok(self
                        .contracts_manager
                        .subscribe_to_contracts(start_block)
                        .await?)
                }
            } else {
                log_info!(
                    "ASSIGNMENT LOOP: Next assignment is not empty, trying to assign new blob: {}",
                    next_blob_assignment.to_string()
                );

                Ok(self.handle_blob_assignment().await?)
            }
        }
    }

    async fn handle_blob_assignment(&self) -> anyhow::Result<BlobAssignmentStartingPoint> {
        match self.contracts_manager.assign_blob().await {
            Ok(assigned_blob) => {
                {
                    let mut blob_assignment_files = self.blob_assignment_files.lock().await;
                    if blob_assignment_files
                        .assignments
                        .contains_key(&assigned_blob.blobhash)
                    {
                        log_info!(
                            "Assignment exists. Skipping {} save. Inner queue state: {:?}",
                            &assigned_blob.blobhash,
                            blob_assignment_files.inner_queue
                        );
                        return Ok(BlobAssignmentStartingPoint::CleanStart);
                    } else {
                        blob_assignment_files.save_assignment(assigned_blob.clone())?;
                    }
                }

                let blob_vec = match self
                    .find_blob(
                        assigned_blob.commitment,
                        assigned_blob.blob_submission_block,
                    )
                    .await?
                {
                    Some(blob) => blob,
                    None => {
                        return Ok(BlobAssignmentStartingPoint::CleanStart);
                    }
                };

                {
                    let mut blob_assignment_files = self.blob_assignment_files.lock().await;
                    blob_assignment_files.save_discovered_blob(assigned_blob.blobhash, blob_vec)?;
                }

                return Ok(BlobAssignmentStartingPoint::CleanStart);
            }
            Err(e) => {
                log_warn!("ASSIGNMENT LOOP: Blob assignment failed: {:?}", e);
                return Ok(BlobAssignmentStartingPoint::CleanStart);
            }
        }
    }
}
