use alloy::{eips::eip4844::BYTES_PER_BLOB, primitives::FixedBytes};
use log_macros::{format_error, log_info};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::Path,
    time::{Duration, SystemTime},
};
use tokio::time::sleep;

use crate::{
    constants::{
        BLOB_ASSIGNMENT_FOLDER_NAME, BLOB_ASSIGNMENT_JSON_FILE_NAME, DOWNLOADED_BLOBS_FOLDER_NAME,
        MAX_STORED_ASSIGNMENTS, TEMP_FOLDER,
    },
    contracts::SubmitProofInput,
};

use super::manager::{BlobAssignment, BlobAssignmentStatus};

#[derive(Deserialize, Serialize)]
pub struct BlobAssignmentFiles {
    pub inner_queue: Vec<FixedBytes<32>>,
    pub archived: Vec<FixedBytes<32>>,
    pub assignments: HashMap<FixedBytes<32>, BlobAssignment>,
    pub json_file_path: String,
    pub downloaded_blobs_path: String,
}

impl BlobAssignmentFiles {
    pub fn new() -> Self {
        let blob_assignment_folder_path = Path::new(TEMP_FOLDER).join(BLOB_ASSIGNMENT_FOLDER_NAME);
        let json_file_path = blob_assignment_folder_path.join(BLOB_ASSIGNMENT_JSON_FILE_NAME);
        let downloaded_blobs_path = blob_assignment_folder_path.join(DOWNLOADED_BLOBS_FOLDER_NAME);

        Self {
            inner_queue: vec![],
            archived: vec![],
            assignments: HashMap::new(),
            json_file_path: json_file_path.to_string_lossy().to_string(),
            downloaded_blobs_path: downloaded_blobs_path.to_string_lossy().to_string(),
        }
    }
    pub fn build() -> anyhow::Result<Self> {
        let mut json_str = String::new();
        let mut blob_assignment_files = BlobAssignmentFiles::new();

        fs::create_dir_all(&blob_assignment_files.downloaded_blobs_path)?;
        let json_file_path = Path::new(blob_assignment_files.json_file_path.as_str());

        if json_file_path.exists() {
            json_str = fs::read_to_string(json_file_path)?;
            blob_assignment_files = serde_json::from_str(&json_str)?;
        } else {
            json_str = serde_json::to_string_pretty(&blob_assignment_files)?;
            fs::write(json_file_path, json_str)?;
        }
        Ok(blob_assignment_files)
    }

    pub fn save_assignment(&mut self, assignment: BlobAssignment) -> anyhow::Result<()> {
        log_info!("Saving assignment {}", &assignment.blobhash);
        self.inner_queue.push(assignment.blobhash);
        self.assignments.insert(assignment.blobhash, assignment);
        self.update_file()?;
        log_info!(
            "New assignment saved. Inner queue state: {:?}",
            self.inner_queue
        );
        Ok(())
    }

    pub fn save_discovered_blob(
        &mut self,
        blobhash: FixedBytes<32>,
        blob_array: Vec<u8>,
    ) -> anyhow::Result<BlobAssignment> {
        {
            let mutable_assignment = self.assignments.get_mut(&blobhash).ok_or_else(|| {
                format_error!("Trying to save blob for assignment that doesn't exist")
            })?;
            let blob_path = Path::new(&self.downloaded_blobs_path).join(blobhash.to_string());
            fs::write(blob_path, blob_array)?;
            mutable_assignment.status = BlobAssignmentStatus::Discovered;
        }
        self.update_file()?;
        let blob_assignment = self
            .assignments
            .get(&blobhash)
            .ok_or_else(|| format_error!("Assignment not found after saving discovered blob"))?;
        Ok(blob_assignment.clone())
    }

    pub fn get_blob_file(&self, blobhash: FixedBytes<32>) -> anyhow::Result<Option<Vec<u8>>> {
        match std::fs::read(Path::new(&self.downloaded_blobs_path).join(blobhash.to_string())) {
            Ok(file_vec) => {
                if file_vec.len() != BYTES_PER_BLOB {
                    return Err(format_error!(
                        "Unexpected blob file vec length, got {}",
                        file_vec.len()
                    ));
                }
                return Ok(Some(file_vec));
            }
            _ => return Ok(None),
        };
    }

    pub fn save_assignment_as_priority(
        &mut self,
        new_blob_assignment: BlobAssignment,
    ) -> anyhow::Result<()> {
        if self.assignments.contains_key(&new_blob_assignment.blobhash)
            && self.inner_queue.first().is_some()
            && *self.inner_queue.first().expect("First item should exist")
                == new_blob_assignment.blobhash
        {
            Ok(())
        } else if self.inner_queue.first().is_some()
            && *self.inner_queue.first().expect("First item should exist")
                == new_blob_assignment.blobhash
        {
            self.assignments
                .insert(new_blob_assignment.blobhash, new_blob_assignment);
            self.update_file()?;
            Ok(())
        } else {
            let exist_in_queue = self
                .inner_queue
                .iter()
                .position(|n| n == &new_blob_assignment.blobhash);

            if let Some(index_in_queue) = exist_in_queue {
                self.inner_queue.remove(index_in_queue);
            }

            self.inner_queue.insert(0, new_blob_assignment.blobhash);

            self.assignments
                .insert(new_blob_assignment.blobhash, new_blob_assignment);
            self.update_file()?;

            Ok(())
        }
    }

    pub fn save_proof(
        &mut self,
        blobhash: FixedBytes<32>,
        proof_submission_input: SubmitProofInput,
    ) -> anyhow::Result<()> {
        if let Some(assignment) = self.assignments.get_mut(&blobhash) {
            assignment.proof_submission_input = Some(proof_submission_input);
            assignment.status = BlobAssignmentStatus::Processed;
        } else {
            return Err(format_error!(
                "Computed proof for a blob that doesn't exist in the queue: {}",
                blobhash
            ));
        }
        self.update_file()?;
        Ok(())
    }

    pub fn get_inner_queue_head(&self) -> anyhow::Result<Option<BlobAssignment>> {
        if let Some(inner_queue_head) = self.inner_queue.first() {
            if let Some(assignment) = self.assignments.get(inner_queue_head) {
                return Ok(Some(assignment.clone()));
            }
        }
        Ok(None)
    }

    pub fn archive_head_assignment(&mut self, tx_hash: FixedBytes<32>) -> anyhow::Result<()> {
        let inner_queue_head = self
            .get_inner_queue_head()?
            .expect("Queue empty, can't archive head");
        log_info!("Archiving assignment: {}", inner_queue_head.blobhash);
        self.archived.push(inner_queue_head.blobhash);
        self.inner_queue.remove(0);
        if let Some(assignment) = self.assignments.get_mut(&inner_queue_head.blobhash) {
            assignment.status = BlobAssignmentStatus::Sent;
            assignment.proof_submission_tx_hash = Some(tx_hash);
        }
        self.clear_old_archives()?;

        self.update_file()?;
        Ok(())
    }

    pub fn clear_old_archives(&mut self) -> anyhow::Result<()> {
        while self.archived.len() > MAX_STORED_ASSIGNMENTS {
            log_info!("Max archived assignment reached, removing oldest archive");
            let assignment_to_remove = self.archived.remove(0);
            self.assignments.remove(&assignment_to_remove);
            let blob_path =
                Path::new(&self.downloaded_blobs_path).join(assignment_to_remove.to_string());
            std::fs::remove_file(blob_path)?;
        }
        Ok(())
    }

    pub fn remove_from_queue(&mut self, blobhash: FixedBytes<32>) -> anyhow::Result<()> {
        let position_in_queue = self.inner_queue.iter().position(|n| n == &blobhash);

        if let Some(index) = position_in_queue {
            self.inner_queue.remove(index);
        }

        self.archived.push(blobhash);

        if let Some(assignment) = self.assignments.get_mut(&blobhash) {
            assignment.status = BlobAssignmentStatus::Failed;
        }

        Ok(())
    }

    fn update_file(&self) -> anyhow::Result<()> {
        let json_str = serde_json::to_string_pretty(&self)?;
        fs::write(&self.json_file_path, json_str)?;
        Ok(())
    }

    pub async fn watch_json_file() -> anyhow::Result<()> {
        let json_file_path = Path::new(TEMP_FOLDER)
            .join(BLOB_ASSIGNMENT_FOLDER_NAME)
            .join(BLOB_ASSIGNMENT_JSON_FILE_NAME);

        log_info!(
            "Watching for changes in {}",
            json_file_path.to_string_lossy()
        );
        let last_modified = BlobAssignmentFiles::get_json_modified_time()?;

        loop {
            sleep(Duration::from_millis(1000)).await;
            let new_time = BlobAssignmentFiles::get_json_modified_time()?;
            if last_modified != new_time {
                return Ok(());
            }
        }
    }

    pub fn get_json_modified_time() -> anyhow::Result<SystemTime> {
        let json_file_path = Path::new(TEMP_FOLDER)
            .join(BLOB_ASSIGNMENT_FOLDER_NAME)
            .join(BLOB_ASSIGNMENT_JSON_FILE_NAME);
        Ok(fs::metadata(json_file_path).and_then(|meta| meta.modified())?)
    }
}
