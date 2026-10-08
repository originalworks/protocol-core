use crate::{
    aws::{
        queue::{egress::ProcessedBlobQueue, ingress::DdexIngestionQueue},
        storage::{egress::ProcessedBlobStorage, ingress::DdexIngestionStorage},
    },
    blob::{commitment_to_blobhash, BlobTransactionData},
    config::{aws_aa_lambda::AwsAaLambdaConfig, core::Config},
    contracts::ContractsManager,
    ipfs::IpfsManager,
    logger::use_sentry,
    output_generator::{DdexMessage, OutputFilesGenerator},
};
use alloy::primitives::Bytes;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_lambda_events::cloudwatch_events::CloudWatchEvent;
use blob_codec::{errors::OwCodecError, BlobEstimator};
use lambda_runtime::{tracing, LambdaEvent};
use log_macros::log_warn;
use ow_wallet_adapter::{wallet::OwWallet, OwWalletConfig};
use std::path::Path;

pub struct AccountAbstractionLambdaOrchestrator {
    pub ddex_ingestion_queue: DdexIngestionQueue,
    pub ddex_ingestion_storage: DdexIngestionStorage,
    pub blob_estimator: BlobEstimator,
    pub contracts_manager: ContractsManager,
    pub output_files_dir: String,
    pub output_files_generator: OutputFilesGenerator,
    pub ow_wallet: OwWallet,
    pub processed_blob_queue: ProcessedBlobQueue,
    pub processed_blob_storage: ProcessedBlobStorage,
    pub core_config: Config,
}

impl AccountAbstractionLambdaOrchestrator {
    pub async fn build() -> anyhow::Result<Self> {
        let core_config = Config::build()?;
        let aws_aa_lambda_config = AwsAaLambdaConfig::build(&core_config)?;
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let aws_main_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        let ddex_ingestion_queue =
            DdexIngestionQueue::build(&aws_main_config, &aws_aa_lambda_config)?;
        let ddex_ingestion_storage =
            DdexIngestionStorage::build(&aws_main_config, &aws_aa_lambda_config)?;

        let ow_wallet_config = OwWalletConfig::from(&core_config)?;
        let ow_wallet = OwWallet::build(&ow_wallet_config).await?;
        let contracts_manager = ContractsManager::build(&core_config, &ow_wallet).await?;
        let ipfs_manager = IpfsManager::build(&core_config).await?;
        let output_files_generator = OutputFilesGenerator::build(&core_config, ipfs_manager)?;
        let processed_blob_queue = ProcessedBlobQueue::build().await?;
        let processed_blob_storage = ProcessedBlobStorage::build().await?;

        Ok(Self {
            ddex_ingestion_queue,
            ddex_ingestion_storage,
            blob_estimator: BlobEstimator::default(),
            contracts_manager,
            output_files_dir: core_config.output_files_dir.clone(),
            output_files_generator,
            ow_wallet,
            processed_blob_queue,
            processed_blob_storage,
            core_config,
        })
    }

    pub async fn build_input_folder(&mut self) -> anyhow::Result<()> {
        self.ddex_ingestion_storage.clear_input_folder()?;

        let mut added_counter = 0;

        loop {
            match self.ddex_ingestion_queue.reserve_message_folder().await? {
                Some(s3_message_folder) => {
                    let local_message_folder = self
                        .ddex_ingestion_storage
                        .sync_message_folder(&s3_message_folder)
                        .await?;
                    match self.blob_estimator.estimate_and_check(Path::new(
                        &self.ddex_ingestion_storage.storage_config.input_files_dir,
                    )) {
                        Ok(_) => {
                            self.ddex_ingestion_storage
                                .local_to_s3_folder_mapping
                                .insert(local_message_folder.clone(), s3_message_folder.clone());
                            added_counter += 1;
                            self.ddex_ingestion_storage
                                .s3_message_folders
                                .push(s3_message_folder)
                        }
                        Err(err) => {
                            log_warn!(err);
                            std::fs::remove_dir_all(Path::new(&local_message_folder))?;
                            let mut status_value = String::new();
                            // just one message exceed the limit - set that one message as rejected
                            if added_counter == 0 {
                                status_value = self
                                    .ddex_ingestion_queue
                                    .queue_config
                                    .rejected_status_value
                                    .clone();
                            } else {
                                status_value = self
                                    .ddex_ingestion_queue
                                    .queue_config
                                    .unprocessed_status_value
                                    .clone();
                            }
                            self.ddex_ingestion_queue
                                .set_single_message_folder_status(s3_message_folder, status_value)
                                .await?;
                            break;
                        }
                    }
                }
                None => break,
            }
        }

        println!(
            "synced directories: {:?}",
            self.ddex_ingestion_storage.s3_message_folders
        );
        Ok(())
    }

    pub async fn lambda_function_handler(
        &mut self,
        event: LambdaEvent<CloudWatchEvent>,
    ) -> Result<(), lambda_runtime::Error> {
        println!("Lambda execution enter");
        let payload = event.payload;
        tracing::info!("Payload: {:?}", payload);

        let core_config = Config::build()?;

        match use_sentry(&core_config, self.run()).await {
            Ok(ddex_messages) => {
                self.ddex_ingestion_queue
                    .sync_message_folder_statuses(
                        self.ddex_ingestion_storage
                            .local_to_s3_folder_mapping
                            .clone(),
                        ddex_messages,
                        self.ddex_ingestion_storage.s3_message_folders.clone(),
                    )
                    .await
                    .map_err(|err| format!("Sync message folder statuses error: {err}"))?;
            }
            Err(e)
                if e.to_string()
                    .to_lowercase()
                    .contains(&"blob already submitted".to_string().to_lowercase()) =>
            {
                self.ddex_ingestion_queue
                    .set_message_folders_status(
                        &self.ddex_ingestion_storage.s3_message_folders,
                        self.ddex_ingestion_queue
                            .queue_config
                            .processed_status_value
                            .clone(),
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
                self.ddex_ingestion_queue
                    .set_message_folders_status(
                        &self.ddex_ingestion_storage.s3_message_folders,
                        self.ddex_ingestion_queue
                            .queue_config
                            .rejected_status_value
                            .clone(),
                    )
                    .await
                    .map_err(|err| {
                        format!("Setting message folder statuses as rejected failed: {err}")
                    })?;
            }
        }

        println!("Lambda execution leave");
        Ok(())
    }

    async fn run(&mut self) -> anyhow::Result<Vec<DdexMessage>> {
        self.build_input_folder().await?;

        self.contracts_manager.check_image_compatibility().await?;
        let ddex_messages = self
            .output_files_generator
            .generate_files(&self.ow_wallet)
            .await?;

        let blob_transaction_data = BlobTransactionData::build(&self.output_files_dir)?;

        let image_id = self.contracts_manager.image_id;

        let blobhash =
            commitment_to_blobhash(&Bytes::from(blob_transaction_data.kzg_commitment.to_vec()));
        self.processed_blob_queue.send(&blobhash).await?;
        self.processed_blob_storage
            .send_to_s3(&blob_transaction_data, image_id, &blobhash)
            .await?;

        Ok(ddex_messages)
    }
}
