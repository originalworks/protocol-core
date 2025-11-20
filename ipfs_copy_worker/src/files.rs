use std::fs::{self, File, OpenOptions, create_dir_all, remove_dir_all};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::constants::{
    FAILED_UNKNOWN_FILE_PATH, FAILED_UPLOADS_FILE_PATH, LOCAL_PATHS_OF_TIMEOUTS, QUEUE_FILE_PATH,
    TEMP_FOLDER_PATH, TIMEOUTS_FILE_PATH,
};
use crate::ipfs_gateway::DownloadFileResult;
use crate::{ProcessingContext, ProcessingStatus};
pub struct CidFilesManager {
    queue_file_path: String,
    timeouts_file_path: String,
    failed_uploads_file_path: String,
    failed_unknown_file_path: String,
    local_paths_of_timeouts: String,
}

impl CidFilesManager {
    pub fn build() -> anyhow::Result<Self> {
        Self::try_create_file(QUEUE_FILE_PATH)?;
        Self::try_create_file(TIMEOUTS_FILE_PATH)?;
        Self::try_create_file(FAILED_UPLOADS_FILE_PATH)?;
        Self::try_create_file(FAILED_UNKNOWN_FILE_PATH)?;
        Self::try_create_file(LOCAL_PATHS_OF_TIMEOUTS)?;

        Ok(Self {
            queue_file_path: QUEUE_FILE_PATH.to_string(),
            timeouts_file_path: TIMEOUTS_FILE_PATH.to_string(),
            failed_uploads_file_path: FAILED_UPLOADS_FILE_PATH.to_string(),
            failed_unknown_file_path: FAILED_UNKNOWN_FILE_PATH.to_string(),
            local_paths_of_timeouts: LOCAL_PATHS_OF_TIMEOUTS.to_string(),
        })
    }

    fn try_create_file(file_path: &str) -> anyhow::Result<()> {
        if !std::path::Path::new(file_path).exists() {
            std::fs::File::create(file_path)?;
        }
        Ok(())
    }

    pub fn read_queue_head(&self) -> anyhow::Result<Option<String>> {
        let file = File::open(&self.queue_file_path)?;
        let mut reader = BufReader::new(file);

        let mut first_line = String::new();
        let bytes_read = reader.read_line(&mut first_line)?;

        if bytes_read == 0 {
            Ok(None)
        } else {
            Ok(Some(
                first_line.trim_end_matches(&['\r', '\n'][..]).to_string(),
            ))
        }
    }

    pub fn sync_processing_results(
        &self,
        processing_context: &ProcessingContext,
    ) -> anyhow::Result<()> {
        println!("Syncing processing results for {}", processing_context.cid);
        match processing_context.processing_status {
            ProcessingStatus::Uploaded => {
                println!("All good, move the queue");
                self.move_queue()?;
                Ok(())
            }
            ProcessingStatus::FailedDownload => {
                println!("At least one cid timeouted, adding to {TIMEOUTS_FILE_PATH}");
                self.add_to_timeouts(&processing_context)?;
                self.move_queue()?;
                Ok(())
            }

            ProcessingStatus::FailedUpload => {
                println!("Failed to upload file to S3, adding to {FAILED_UPLOADS_FILE_PATH}");
                self.add_to_failed_uploads(&processing_context.cid)?;
                self.move_queue()?;
                Ok(())
            }
            _ => {
                println!("Unknown fail, adding {FAILED_UNKNOWN_FILE_PATH}");
                self.add_to_failed_unknown(&processing_context.cid)?;
                self.move_queue()?;
                Ok(())
            }
        }
    }

    pub fn move_queue(&self) -> anyhow::Result<()> {
        let content = fs::read_to_string(&self.queue_file_path)?;
        let mut lines = content.lines();
        lines.next(); // skip the first line

        let remaining: String = lines.collect::<Vec<_>>().join("\n");
        let mut file = File::create(&self.queue_file_path)?;
        file.write_all(remaining.as_bytes())?;
        Ok(())
    }

    pub fn add_to_timeouts(&self, processing_context: &ProcessingContext) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.timeouts_file_path)?;

        writeln!(file, "{}", processing_context.cid)?; // write line + newline
        self.add_to_timeout_paths(&processing_context.files)?;
        Ok(())
    }

    pub fn add_to_timeout_paths(
        &self,
        download_file_results: &Vec<DownloadFileResult>,
    ) -> anyhow::Result<()> {
        for download_file_result in download_file_results {
            if !download_file_result.status_code.is_success() {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(&self.local_paths_of_timeouts)?;

                writeln!(
                    file,
                    "{} - {}",
                    &download_file_result.local_path.clone(),
                    download_file_result.status_code
                )?; // write line + newline
            }
        }
        Ok(())
    }

    pub fn add_to_failed_uploads(&self, cid: &String) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.failed_uploads_file_path)?;

        writeln!(file, "{}", cid)?; // write line + newline

        Ok(())
    }

    pub fn add_to_failed_unknown(&self, cid: &String) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.failed_unknown_file_path)?;

        writeln!(file, "{}", cid)?; // write line + newline

        Ok(())
    }
}

pub fn clear_temp_folder() -> anyhow::Result<()> {
    let temp_folder_path = Path::new(TEMP_FOLDER_PATH);
    if temp_folder_path.is_dir() {
        remove_dir_all(temp_folder_path)?;
        create_dir_all(temp_folder_path)?;
    }
    Ok(())
}
