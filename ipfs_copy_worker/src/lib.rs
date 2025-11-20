mod cid;
mod constants;
mod database;
mod files;
mod ipfs_gateway;
mod upload;
mod zip;

use crate::database::DatabaseManager;
use crate::files::clear_temp_folder;
use crate::ipfs_gateway::{CidType, DownloadFileResult};
use crate::upload::UploadManager;
use crate::{files::CidFilesManager, ipfs_gateway::Gateway};
use std::env;
use std::fmt;

#[derive(Debug, PartialEq)]
enum ProcessingStatus {
    Ingested,
    Downloaded,
    Uploaded,
    FailedDownload,
    FailedUpload,
    Processed,
}

impl fmt::Display for ProcessingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProcessingStatus::Ingested => "Ingested",
                ProcessingStatus::Downloaded => "Downloaded",
                ProcessingStatus::Uploaded => "Uploaded",
                ProcessingStatus::FailedDownload => "FailedDownload",
                ProcessingStatus::FailedUpload => "FailedUpload",
                ProcessingStatus::Processed => "Processed",
            }
        )
    }
}

impl ProcessingStatus {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Ingested" => Some(ProcessingStatus::Ingested),
            "Downloaded" => Some(ProcessingStatus::Downloaded),
            "Uploaded" => Some(ProcessingStatus::Uploaded),
            "FailedDownload" => Some(ProcessingStatus::FailedDownload),
            "FailedUpload" => Some(ProcessingStatus::FailedUpload),
            "Processed" => Some(ProcessingStatus::Processed),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ProcessingContext {
    pub cid: String,
    pub cid_type: CidType,
    processing_status: ProcessingStatus,
    pub files: Vec<DownloadFileResult>,
}

pub struct Config {
    pub upload_bucket_name: String,
}

impl Config {
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }
    pub fn build() -> anyhow::Result<Config> {
        dotenvy::dotenv().ok();
        let upload_bucket_name = Self::get_env_var("UPLOAD_BUCKET_NAME");

        Ok(Config { upload_bucket_name })
    }
}

pub async fn run() -> anyhow::Result<()> {
    println!("Building...");
    let config = Config::build()?;
    let cid_files_manager = CidFilesManager::build()?;
    let upload_manager = UploadManager::build(config).await?;
    let gateway = Gateway::build()?;
    let database_manager = DatabaseManager::build();

    loop {
        clear_temp_folder()?;
        let Some(queue_head) = cid_files_manager.read_queue_head()? else {
            println!("Queue file is empty, process was terminated");
            break Ok(());
        };

        println!("Queue head: {queue_head}");

        match database_manager.read_cid(&queue_head)? {
            Some(record) => {
                println!("Skipping {}", queue_head);
                print!("existing record: {record:#?}");
                cid_files_manager.move_queue()?;
            }
            None => {
                database_manager.insert_cid(&queue_head, ProcessingStatus::Ingested)?;

                let mut processing_context = gateway.download_by_cid(&queue_head).await?;

                if processing_context.processing_status == ProcessingStatus::Downloaded {
                    database_manager.update_cid(
                        &processing_context.cid,
                        &processing_context.processing_status,
                    )?;
                    processing_context = upload_manager.upload_to_s3(processing_context).await?;
                    database_manager.update_cid(
                        &processing_context.cid,
                        &processing_context.processing_status,
                    )?;
                }

                cid_files_manager.sync_processing_results(&processing_context)?;
            }
        };
    }
}
