use crate::{
    blob_assignment::manager::{BlobAssignment, BlobAssignmentStartingPoint, BlobAssignmentStatus},
    is_local, Config,
};
use alloy::{
    eips::BlockNumberOrTag,
    network::EthereumWallet,
    primitives::{Address, Bytes, FixedBytes},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, ProviderBuilder, RootProvider, WsConnect,
    },
    rpc::types::{Filter, TransactionReceipt},
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolEvent,
};
use anyhow::Context;
use futures_util::StreamExt;
use log_macros::{format_error, log_error, log_info, log_warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use DdexEmitter::getSupportedVerifierImageIdsReturn;
use DdexSequencer::getQueueHeadDetailsReturn;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DdexSequencer,
    "../contracts/artifacts/contracts/DdexSequencer.sol/DdexSequencer.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DdexEmitter,
    "../contracts/artifacts/contracts/DdexEmitter.sol/DdexEmitter.json"
);

pub struct BlobSubmissionDetails {
    pub commitment: Bytes,
    pub timestamp: u64,
    pub submission_tx_hash: FixedBytes<32>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum LocalImageVersion {
    Current,
    Previous,
}

impl From<DdexSequencer::blobsReturn> for DdexSequencer::Blob {
    fn from(blob: DdexSequencer::blobsReturn) -> Self {
        DdexSequencer::Blob {
            assignedValidator: blob.assignedValidator,
            imageId: blob.imageId,
            submissionBlock: blob.submissionBlock,
            nextBlob: blob.nextBlob,
            submitted: blob.submitted,
            proposer: blob.proposer,
            blobId: blob.blobId,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SubmitProofInput {
    pub image_id: FixedBytes<32>,
    pub journal: Vec<u8>,
    pub seal: Vec<u8>,
    pub ipfs_cid: String,
}

pub struct BlobOnchainData {
    pub blob_data: DdexSequencer::Blob,
    pub blobhash: FixedBytes<32>,
}

#[derive(Debug, serde::Serialize)]
pub struct QueueHeadData {
    pub commitment: Bytes,
    pub parent_beacon_block_root: FixedBytes<32>,
    pub versioned_blobhash: FixedBytes<32>,
    pub transaction_hash: FixedBytes<32>,
    pub block_number: u64,
    pub timestamp: u64,
    pub image_id: FixedBytes<32>,
    pub chain_id: u64,
}

type HardlyTypedProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

pub struct ContractsManager {
    pub sequencer: DdexSequencer::DdexSequencerInstance<HardlyTypedProvider>,
    pub emitter: DdexEmitter::DdexEmitterInstance<HardlyTypedProvider>,
    pub current_image_id: alloy::primitives::FixedBytes<32>,
    pub previous_image_id: alloy::primitives::FixedBytes<32>,
    pub provider: HardlyTypedProvider,
    pub chain_id: u64,
    pub signer: PrivateKeySigner,
}

impl ContractsManager {
    pub async fn build(
        ddex_sequencer_address: Address,
        private_key: &String,
        rpc_url: &String,
    ) -> anyhow::Result<Self> {
        let private_key_signer: PrivateKeySigner = private_key
            .parse()
            .with_context(|| "Failed to parse PRIVATE_KEY")?;
        let wallet = EthereumWallet::from(private_key_signer.clone());

        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect_http(rpc_url.parse()?);

        let chain_id = provider.get_chain_id().await?;
        let sequencer = DdexSequencer::new(ddex_sequencer_address, provider.clone());

        let emitter_address = sequencer.ddexEmitter().call().await?;
        let emitter = DdexEmitter::new(emitter_address, provider.clone());

        let current_image_id_parsed = prover::parse_guest_id(&prover::CURRENT_DDEX_GUEST_ID);
        let previous_image_id_parsed = prover::parse_guest_id(&prover::PREVIOUS_DDEX_GUEST_ID);

        Ok(Self {
            sequencer,
            emitter,
            current_image_id: current_image_id_parsed,
            previous_image_id: previous_image_id_parsed,
            provider,
            chain_id,
            signer: private_key_signer,
        })
    }

    pub async fn is_queue_head_expired(&self) -> anyhow::Result<bool> {
        Ok(self.sequencer.isQueueHeadExpired().call().await?)
    }

    pub async fn get_queue_head(&self) -> anyhow::Result<BlobOnchainData> {
        let getQueueHeadDetailsReturn {
            _0: queue_head_blob,
            _1: queue_head_blobhash,
        } = self.sequencer.getQueueHeadDetails().call().await?;

        Ok(BlobOnchainData {
            blob_data: queue_head_blob,
            blobhash: queue_head_blobhash,
        })
    }

    pub async fn get_blob_submission_details(
        &self,
        blobhash: FixedBytes<32>,
        block_number: u64,
    ) -> anyhow::Result<BlobSubmissionDetails> {
        let logs = self
            .provider
            .get_logs(
                &Filter::new()
                    .address(self.sequencer.address().clone())
                    .event(DdexSequencer::NewBlobSubmitted::SIGNATURE)
                    .from_block(block_number)
                    .to_block(block_number),
            )
            .await?;

        for log in logs {
            let DdexSequencer::NewBlobSubmitted {
                commitment,
                image_id: _,
            } = log.log_decode()?.inner.data;

            let blobhash_from_commitment = Self::commitment_to_blobhash(&commitment);

            if blobhash_from_commitment == blobhash {
                let timestamp = self
                    .provider
                    .get_block_by_number(BlockNumberOrTag::Number(block_number))
                    .await?
                    .ok_or_else(|| format_error!("Cannot get block info"))?
                    .header
                    .timestamp;
                let submission_tx_hash = log
                    .transaction_hash
                    .expect("Transaction hash not found in log");

                return Ok(BlobSubmissionDetails {
                    commitment,
                    timestamp,
                    submission_tx_hash,
                });
            }
        }

        return Err(format_error!(
            "Event with commitment not found in the block {} for for blobhash: {}",
            block_number,
            blobhash.to_string()
        ));
    }

    pub async fn fetch_current_block(&self) -> anyhow::Result<u64> {
        let current_block = self
            .provider
            .get_block_number()
            .await
            .expect("Error while fetching current block number");
        Ok(current_block)
    }

    pub async fn assign_blob(&self) -> anyhow::Result<BlobAssignment> {
        let mut tx_builder = self.sequencer.assignBlob();

        if is_local() {
            tx_builder = tx_builder.gas(1000000);
        }

        let receipt = tx_builder.send().await?.get_receipt().await?;

        if receipt.status() == false {
            return Err(log_error!(
                "assignBlob tx was rejected: {:?}",
                receipt.transaction_hash
            ));
        }
        let logs = receipt.inner.logs();
        let blob_assignment_tx_hash = receipt.transaction_hash;

        for log in logs {
            match log.topic0().expect("Event log signature not found") {
                &DdexSequencer::BlobAssigned::SIGNATURE_HASH => {
                    let DdexSequencer::BlobAssigned {
                        blob,
                        assignedValidator,
                    } = log.log_decode()?.inner.data;

                    if assignedValidator == self.signer.address() {
                        let blob_assignment = self
                            .build_new_assignment(
                                blob,
                                log.transaction_hash.expect("Tx hash not found in log"),
                            )
                            .await?;

                        return Ok(blob_assignment);
                    }
                }

                _ => (),
            }
        }
        return Err(log_error!(
            "Assignment not found in tx hash: {}",
            blob_assignment_tx_hash
        ));
    }

    pub async fn build_new_assignment(
        &self,
        blobhash: FixedBytes<32>,
        blob_assignment_tx_hash: FixedBytes<32>,
    ) -> anyhow::Result<BlobAssignment> {
        let blob_data: DdexSequencer::Blob = self.sequencer.blobs(blobhash).call().await?.into();
        let local_image_version = self.select_local_image_version(&blob_data.imageId).await?;

        let blob_submission_details = self
            .get_blob_submission_details(blobhash, blob_data.submissionBlock.to::<u64>())
            .await?;

        let blob_assignment = BlobAssignment {
            blobhash,
            commitment: blob_submission_details.commitment,
            status: BlobAssignmentStatus::Assigned,
            assignment_tx_hash: blob_assignment_tx_hash,
            blob_submission_block: blob_data.submissionBlock.to::<u64>(),
            image_id: blob_data.imageId,
            proof_submission_tx_hash: None,
            proof_submission_input: None,
            blob_submission_tx_hash: blob_submission_details.submission_tx_hash,
            blob_submission_timestamp: blob_submission_details.timestamp,
            chain_id: self.chain_id,
            local_image_version: local_image_version,
        };
        Ok(blob_assignment)
    }
    pub async fn subscribe_to_contracts(
        &self,
        config: &Config,
        start_block: u64,
    ) -> anyhow::Result<BlobAssignmentStartingPoint> {
        log_info!("Subscribing to queue");
        let ws_url = WsConnect::new(&config.ws_url);
        let ws_provider = ProviderBuilder::new().connect_ws(ws_url).await?;

        let filter = Filter::new()
            .address(vec![
                config.ddex_sequencer_address,
                self.emitter.address().clone(),
            ])
            .events(vec![
                DdexSequencer::NewBlobSubmitted::SIGNATURE,
                DdexEmitter::BlobProcessed::SIGNATURE,
                DdexEmitter::BlobRejected::SIGNATURE,
            ])
            .from_block(start_block);

        log_info!("Subscribed to queue, waiting for changes...");
        let subscription = ws_provider.subscribe_logs(&filter).await?;
        let mut stream = subscription.into_stream();

        while let Some(log) = stream.next().await {
            match log.topic0().expect("Event log signature not found") {
                &DdexSequencer::NewBlobSubmitted::SIGNATURE_HASH => {
                    let DdexSequencer::NewBlobSubmitted {
                        commitment: _,
                        image_id: _,
                    } = log.log_decode()?.inner.data;
                    let block_number = log
                        .block_number
                        .ok_or_else(|| format_error!("Block not found in log"))?;
                    return Ok(BlobAssignmentStartingPoint::NewBlobSubmitted { block_number });
                }
                &DdexEmitter::BlobProcessed::SIGNATURE_HASH
                | &DdexEmitter::BlobRejected::SIGNATURE_HASH => {
                    let block_number = log
                        .block_number
                        .ok_or_else(|| format_error!("Block not found in log"))?;
                    return Ok(BlobAssignmentStartingPoint::BlobProcessedOrRejected {
                        block_number,
                    });
                }
                _ => (),
            }
        }
        Ok(BlobAssignmentStartingPoint::CleanStart)
    }

    pub async fn get_next_blob_assignment(&self) -> anyhow::Result<FixedBytes<32>> {
        Ok(self.sequencer.nextBlobAssignment().call().await?)
    }

    pub async fn select_local_image_version(
        &self,
        blob_image_id: &FixedBytes<32>,
    ) -> anyhow::Result<LocalImageVersion> {
        let local_image_version: LocalImageVersion;
        let blob_is_local_current;

        if blob_image_id == &self.current_image_id {
            local_image_version = LocalImageVersion::Current;
            blob_is_local_current = true;
        } else if blob_image_id == &self.previous_image_id {
            local_image_version = LocalImageVersion::Previous;
            blob_is_local_current = false;
        } else {
            log_warn!("Current image id: {}", self.current_image_id.to_string());
            log_warn!("Previous image id: {}", self.previous_image_id.to_string());
            log_warn!("Blob image id: {}", blob_image_id.to_string());

            self
                .emitter
                .getSupportedBlobImageIds()
                .call()
                .await
                .ok()
                .and_then(|res| -> Option<()> {
                    log_warn!("Current sequencer blob image id: {}", &res._0.to_string());
                    log_warn!("Previous sequencer blob image id: {}", &res._1.to_string());

                    if blob_image_id == &res._0 || blob_image_id == &res._1 {
                        panic!("Blob ImageId is supported by DdexSequencer and unknown to validator. It means that validator is outdated. Please update it.");
                    } else {
                        panic!("Blob ImageId is unsupported by both DdexSequencer and validator. This shouldn't happen.")
                    }
                });

            panic!("Blob ImageId is unknown and couldn't get supported image ids from DdexSequencer. This can be caused by outdated validator or protocol error");
        }

        let getSupportedVerifierImageIdsReturn {
            _0: current_verifier_image_id,
            _1: previous_verifier_image_id,
        } = self.emitter.getSupportedVerifierImageIds().call().await?;

        if previous_verifier_image_id.is_zero() {
            if current_verifier_image_id == self.current_image_id && blob_is_local_current {
                return Ok(local_image_version);
            } else {
                log_warn!("Current image id: {}", self.current_image_id.to_string());
                log_warn!("Previous image id: {}", self.previous_image_id.to_string());
                log_warn!("Blob image id: {}", blob_image_id.to_string());
                log_warn!(
                    "Current sequencer verifier image id: {}",
                    &current_verifier_image_id.to_string()
                );
                log_warn!(
                    "Previous sequencer verifier image id: {}",
                    &previous_verifier_image_id.to_string()
                );
                panic!("Proving aborted. DdexSequencer will reject prove generated with this imageId as it became outdated. Please update validator ASAP!")
            }
        } else {
            if self.current_image_id == current_verifier_image_id && blob_is_local_current
                || self.previous_image_id == previous_verifier_image_id && !blob_is_local_current
            {
                return Ok(local_image_version);
            } else if self.current_image_id == previous_verifier_image_id && blob_is_local_current {
                log_warn!("Validator supports imageId that will soon become outdated. Please update validator ASAP!");
                return Ok(local_image_version);
            } else {
                log_warn!("Current image id: {}", self.current_image_id.to_string());
                log_warn!("Previous image id: {}", self.previous_image_id.to_string());
                log_warn!("Blob image id: {}", blob_image_id.to_string());
                log_warn!(
                    "Current sequencer verifier image id: {}",
                    &current_verifier_image_id.to_string()
                );
                log_warn!(
                    "Previous sequencer verifier image id: {}",
                    &previous_verifier_image_id.to_string()
                );
                panic!("Proving aborted. DdexSequencer will reject prove generated with this imageId as it became outdated. Please update validator ASAP!");
            }
        }
    }

    pub async fn submit_proof(
        &self,
        input: SubmitProofInput,
    ) -> anyhow::Result<TransactionReceipt> {
        let mut tx_builder = self.sequencer.submitProof(
            input.image_id,
            input.journal.into(),
            input.seal.into(),
            input.ipfs_cid,
        );

        if is_local() {
            tx_builder = tx_builder
                .max_priority_fee_per_gas(500000000)
                .max_fee_per_gas(500000001);
        }

        let receipt = tx_builder.send().await?.get_receipt().await?;

        sentry::configure_scope(|scope| {
            scope.set_extra("transaction", json!(receipt));
        });

        log_info!("Success!");
        log_info!("--From: {}", receipt.from.to_string());
        log_info!("--To: {}", receipt.to.unwrap_or_default().to_string());
        log_info!(
            "--ContractAddress: {}",
            receipt.contract_address.unwrap_or_default().to_string()
        );

        log_info!("--TxHash: {}", receipt.transaction_hash.to_string());
        log_info!("--GasPrice: {}", receipt.blob_gas_price.unwrap_or_default());
        log_info!("--EffGasPrice: {}", receipt.effective_gas_price.to_string());
        log_info!("--GasUsed: {}", receipt.blob_gas_used.unwrap_or_default());

        Ok(receipt)
    }

    pub async fn remove_expired_blob(&self) -> anyhow::Result<()> {
        let mut tx_builder = self.sequencer.removeExpiredBlob();

        if is_local() {
            tx_builder = tx_builder.gas(1000000);
        }

        let receipt = tx_builder.send().await?.get_receipt().await?;

        if receipt.status() == false {
            return Err(log_error!(
                "removeExpiredBlob tx was rejected: {:?}",
                receipt.transaction_hash
            ));
        }

        log_info!(
            "removeExpiredBlob tx was successful: {:?}",
            receipt.transaction_hash
        );
        Ok(())
    }

    pub fn commitment_to_blobhash(commitment: &Bytes) -> FixedBytes<32> {
        let mut hasher = Sha256::new();
        hasher.update(commitment);
        let mut hashed_commitment = hasher.finalize();
        hashed_commitment[0] = 1;

        let mut fixed_bytes_input: [u8; 32] = [0u8; 32];
        fixed_bytes_input.copy_from_slice(&hashed_commitment);

        FixedBytes::<32>::from(fixed_bytes_input)
    }

    pub async fn get_parent_beacon_block_root(
        &self,
        block_number: u64,
    ) -> anyhow::Result<FixedBytes<32>> {
        let parent_beacon_block_root = self
            .provider
            .get_block_by_number(BlockNumberOrTag::Number(block_number))
            .await?
            .ok_or_else(|| format_error!("Block {} not found", block_number))?
            .header
            .parent_beacon_block_root
            .ok_or_else(|| format_error!("Block {} not found", block_number))?;

        Ok(parent_beacon_block_root)
    }
}
