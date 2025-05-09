mod config;
mod message_database;

use anyhow::Result;
use blob_codec::errors::OwCodecError;
use config::LocalS3Config;
use owen::logger::{init_logging, init_sentry};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;

    let LocalS3Config {
        owen_config,
        aws_sdk_config,
        messages_per_blob,
    } = LocalS3Config::build().await;

    let _guard = init_sentry(&owen_config);

    let storage = owen::s3_message_storage::MessageStorage::build(&aws_sdk_config);
    let mut database = message_database::MessageDatabase::build();

    let s3_message_folders = storage
        .list_message_folders_with_limit(messages_per_blob)
        .await?;

    let local_to_s3_folder_mapping = storage.sync_message_folders(&s3_message_folders).await?;

    match owen::run_with_sentry(&owen_config).await {
        Ok(message_processing_context) => {
            let s3_folder_to_processing_context_map = database.save_message_folders(
                local_to_s3_folder_mapping,
                message_processing_context,
                &s3_message_folders,
            )?;

            storage
                .clear_s3_folders(s3_folder_to_processing_context_map, &s3_message_folders)
                .await?;
        }
        Err(e)
            if e.to_string()
                .to_lowercase()
                .contains(&"blob already submitted".to_string().to_lowercase()) =>
        {
            database.save_message_folders_with_status(
                &s3_message_folders,
                "processed".to_string(),
                Some("BLOB already submitted".to_string()),
            )?;

            storage
                .clear_processed_s3_folders(&s3_message_folders)
                .await?;
        }
        Err(e) if e.is::<OwCodecError>() => {
            if let Some(e) = e.downcast_ref::<OwCodecError>() {
                match e {
                    OwCodecError::BlobOverflow {
                        path: _path,
                        loc: _loc,
                    } => {
                        panic!("Not enough space in the blob to pack all messages. Lower the MESSAGES_PER_BLOB variable value")
                    }
                    _ => {}
                }
            }
        }
        Err(err) => {
            database.save_message_folders_with_status(
                &s3_message_folders,
                "rejected".to_string(),
                Some(err.to_string()),
            )?;

            storage
                .clear_rejected_s3_folders(&s3_message_folders)
                .await?;
        }
    };

    Ok(())
}
