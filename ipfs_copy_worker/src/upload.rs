use std::path::Path;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, primitives::ByteStream};
use zip::CompressionMethod;

use crate::{
    Config, ProcessingContext, ProcessingStatus, constants::TEMP_FOLDER_PATH,
    ipfs_gateway::CidType, zip::zip_directory,
};

pub struct UploadManager {
    client: Client,
    bucket_name: String,
}

impl UploadManager {
    pub async fn build(config: Config) -> anyhow::Result<Self> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let version = aws_config::BehaviorVersion::latest();
        let aws_config = aws_config::defaults(version)
            .region(region_provider)
            .load()
            .await;
        let client: Client = Client::new(&aws_config);

        Ok(Self {
            client,
            bucket_name: config.upload_bucket_name,
        })
    }

    async fn upload_file(
        &self,
        mut processing_context: ProcessingContext,
    ) -> anyhow::Result<ProcessingContext> {
        let file_path = Path::new(TEMP_FOLDER_PATH).join(&processing_context.cid);
        let bytes = std::fs::read(&file_path)?;
        match self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&processing_context.cid)
            .body(ByteStream::from(bytes))
            .send()
            .await
        {
            Ok(_) => {
                println!(
                    "{} was uploaded to S3",
                    &file_path.to_string_lossy().to_string()
                );
                processing_context.processing_status = ProcessingStatus::Uploaded;
                return Ok(processing_context);
            }
            Err(e) => {
                println!(
                    "{} failed to upload to S3",
                    &file_path.to_string_lossy().to_string()
                );
                println!("{e:?}");
                processing_context.processing_status = ProcessingStatus::FailedUpload;
                return Ok(processing_context);
            }
        }
    }

    async fn upload_folder(
        &self,
        mut processing_context: ProcessingContext,
    ) -> anyhow::Result<ProcessingContext> {
        let folder_path = Path::new(TEMP_FOLDER_PATH).join(&processing_context.cid);
        let zip_file_local_path = Path::new(TEMP_FOLDER_PATH)
            .join(&processing_context.cid)
            .join(format!("{}.zip", &processing_context.cid));
        zip_directory(
            &folder_path,
            &zip_file_local_path,
            CompressionMethod::Deflated,
        )?;

        let bytes = std::fs::read(&zip_file_local_path)?;
        match self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(format!("{}.zip", &processing_context.cid))
            .body(ByteStream::from(bytes))
            .send()
            .await
        {
            Ok(_) => {
                println!(
                    "{} folder was uploaded to S3",
                    &zip_file_local_path.to_string_lossy().to_string()
                );
                processing_context.processing_status = ProcessingStatus::Uploaded;
                return Ok(processing_context);
            }
            Err(e) => {
                println!(
                    "{} folder failed to upload to S3",
                    &zip_file_local_path.to_string_lossy().to_string()
                );
                println!("{e:?}");
                processing_context.processing_status = ProcessingStatus::FailedUpload;
                return Ok(processing_context);
            }
        }

        // Ok(processing_context)
    }

    pub async fn upload_to_s3(
        &self,
        mut processing_context: ProcessingContext,
    ) -> anyhow::Result<ProcessingContext> {
        if processing_context.cid_type == CidType::File {
            processing_context = self.upload_file(processing_context).await?;
        } else if processing_context.cid_type == CidType::Folder {
            processing_context = self.upload_folder(processing_context).await?;
        }
        Ok(processing_context)
    }
}
