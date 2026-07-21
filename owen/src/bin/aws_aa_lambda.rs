#![cfg(feature = "aws-integration")]
use alloy::primitives::Bytes;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use blob_codec::errors::OwCodecError;
use ddex_parser::ParserError;
use lambda_runtime::{service_fn, tracing, LambdaEvent};
use log_macros::log_error;
use ow_wallet::{OwWallet, OwWalletConfig};
use owen::aws::queue::egress::ProcessedBlobQueue;
use owen::aws::storage::egress::ProcessedBlobStorage;
use owen::blob::{commitment_to_blobhash, BlobTransactionData};
use owen::contracts::ContractsManager;
use owen::ipfs::IpfsManager;
use owen::output_generator::OutputFilesGenerator;
use owen::{
    aws::{
        input_folder::InputFolderBuilder, queue::ingress::DdexIngestionQueue,
        storage::ingress::DdexIngestionStorage,
    },
    logger::{init_logging, init_sentry},
    output_generator::DdexMessage,
    Config,
};
use sentry::User;
use serde_json::json;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    println!("Lambda cold start");

    let _guard = init_sentry();
    init_logging()?;

    tracing::init_default_subscriber();

    lambda_runtime::run(service_fn(function_handler)).await
}

async fn function_handler(
    event: LambdaEvent<CloudWatchEvent>,
) -> Result<(), lambda_runtime::Error> {
    println!("Lambda execution enter");
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let owen_config = owen::Config::build()?;

    let queue = Arc::new(DdexIngestionQueue::build(&aws_main_config)?);
    let storage = Arc::new(Mutex::new(DdexIngestionStorage::build(&aws_main_config)?));

    let input_folder_builder = InputFolderBuilder::build(&queue, &storage);

    input_folder_builder.build_input_folder().await?;

    let storage_lock = storage
        .lock()
        .expect("Failed to lock on ddex_ingestion_storage");

    let local_to_s3_folder_mapping = storage_lock.local_to_s3_folder_mapping.clone();
    let s3_message_folders = storage_lock.s3_message_folders.clone();

    if s3_message_folders.is_empty() {
        tracing::info!("No message folders found, queue is empty. Terminating execution.");
        return Ok(());
    }

    println!("synced directories: {s3_message_folders:?}");

    match run_with_sentry(&owen_config).await {
        Ok(ddex_messages) => {
            queue
                .sync_message_folder_statuses(
                    local_to_s3_folder_mapping,
                    ddex_messages,
                    s3_message_folders,
                )
                .await
                .map_err(|err| format!("Sync message folder statuses error: {err}"))?;
        }
        Err(e)
            if e.to_string()
                .to_lowercase()
                .contains(&"blob already submitted".to_string().to_lowercase()) =>
        {
            queue
                .set_message_folders_status(
                    &s3_message_folders,
                    queue.processed_status_value.clone(),
                )
                .await
                .map_err(|err| {
                    format!("Setting message folder statuses as processed failed: {err}")
                })?;
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
        Err(e) => {
            tracing::info!("Unhandled error: {}", e);
            queue
                .set_message_folders_status(
                    &s3_message_folders,
                    queue.rejected_status_value.clone(),
                )
                .await
                .map_err(|err| {
                    format!("Setting message folder statuses as rejected failed: {err}")
                })?;
        }
    };
    println!("Lambda execution leave");
    Ok(())
}

pub async fn run_with_sentry(config: &Config) -> anyhow::Result<Vec<DdexMessage>> {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(User {
            username: Some(config.username.to_owned()),
            ..Default::default()
        }));
    });

    let ddex_messages = run(&config).await.map_err(|e| {
        sentry::configure_scope(|scope| {
            scope.set_tag("error_type", {
                if e.is::<ParserError>() {
                    "parser"
                } else {
                    "other"
                }
            });
            scope.set_extra("error_object", json!(format!("{e:#?}")));
        });

        log_error!("{e}")
    })?;

    anyhow::Ok(ddex_messages)
}

async fn run(config: &Config) -> anyhow::Result<Vec<DdexMessage>> {
    let ow_wallet_config = OwWalletConfig::from(config)?;
    let ow_wallet = OwWallet::build(&ow_wallet_config).await?;
    let contracts_manager = ContractsManager::build(&config, &ow_wallet).await?;
    let ipfs_manager = IpfsManager::build(&config, &ow_wallet).await?;
    let output_files_generator = OutputFilesGenerator::build(&config, &ipfs_manager)?;
    let processed_blob_queue = ProcessedBlobQueue::build().await?;
    let processed_blob_storage = ProcessedBlobStorage::build().await?;

    contracts_manager.check_image_compatibility().await?;
    let ddex_messages = output_files_generator.generate_files().await?;

    let blob_transaction_data = BlobTransactionData::build(&config.output_files_dir)?;

    let image_id = contracts_manager.image_id;

    let blobhash =
        commitment_to_blobhash(&Bytes::from(blob_transaction_data.kzg_commitment.to_vec()));
    processed_blob_queue.send(&blobhash).await?;
    processed_blob_storage
        .send_to_s3(&blob_transaction_data, image_id, &blobhash)
        .await?;

    Ok(ddex_messages)
}
