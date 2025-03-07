use crate::{is_local, Config};
use alloy::{
    eips::BlockNumberOrTag,
    network::{Ethereum, EthereumWallet},
    primitives::{Address, Bytes, FixedBytes},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, RootProvider,
    },
    providers::{ProviderBuilder, WsConnect},
    rpc::types::{Filter, TransactionReceipt},
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolEvent,
    transports::http::{reqwest, Client, Http},
};
use anyhow::Context;
use log_macros::{format_error, log_info, log_warn};
use prover::{CURRENT_DDEX_GUEST_ELF, PREVIOUS_DDEX_GUEST_ELF};
use serde_json::json;
use sha2::{Digest, Sha256};
use DdexEmitter::getSupportedVerifierImageIdsReturn;

use futures_util::StreamExt;

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

#[derive(Debug, serde::Serialize)]
pub struct QueueHeadData {
    pub commitment: Bytes,
    pub parent_beacon_block_root: FixedBytes<32>,
    pub versioned_blobhash: FixedBytes<32>,
    pub transaction_hash: FixedBytes<32>,
    pub block_number: u64,
    pub timestamp: u64,
    pub image_id: FixedBytes<32>,
}

type HardlyTypedProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

pub struct ContractsManager {
    pub sequencer: DdexSequencer::DdexSequencerInstance<
        alloy::transports::http::Http<reqwest::Client>,
        HardlyTypedProvider,
    >,
    pub emitter: DdexEmitter::DdexEmitterInstance<
        alloy::transports::http::Http<reqwest::Client>,
        HardlyTypedProvider,
    >,
    pub current_image_id: alloy::primitives::FixedBytes<32>,
    pub previous_image_id: alloy::primitives::FixedBytes<32>,
    pub provider: HardlyTypedProvider,
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
        let wallet = EthereumWallet::from(private_key_signer);

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url.parse()?);

        let sequencer = DdexSequencer::new(ddex_sequencer_address, provider.clone());

        let emitter_address = sequencer.ddexEmitter().call().await?._0;
        let emitter = DdexEmitter::new(emitter_address, provider.clone());

        let current_image_id_parsed = alloy::primitives::FixedBytes::<32>::from_slice(
            &prover::CURRENT_DDEX_GUEST_ID
                .map(|word| word.to_le_bytes())
                .concat(),
        );

        let previous_image_id_parsed = alloy::primitives::FixedBytes::<32>::from_slice(
            &prover::PREVIOUS_DDEX_GUEST_ID
                .map(|word| word.to_le_bytes())
                .concat(),
        );

        Ok(Self {
            sequencer,
            emitter,
            current_image_id: current_image_id_parsed,
            previous_image_id: previous_image_id_parsed,
            provider,
        })
    }

    pub async fn select_image_elf(&self, blob_image_id: &FixedBytes<32>) -> anyhow::Result<&[u8]> {
        let selected_elf;
        let blob_is_local_current;

        if blob_image_id == &self.current_image_id {
            selected_elf = CURRENT_DDEX_GUEST_ELF;
            blob_is_local_current = true;
        } else if blob_image_id == &self.previous_image_id {
            selected_elf = PREVIOUS_DDEX_GUEST_ELF;
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
                return Ok(selected_elf);
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
                return Ok(selected_elf);
            } else if self.current_image_id == previous_verifier_image_id && blob_is_local_current {
                log_warn!("Validator supports imageId that will soon become outdated. Please update validator ASAP!");
                return Ok(selected_elf);
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
        image_id: FixedBytes<32>,
        journal: Vec<u8>,
        seal: Vec<u8>,
    ) -> anyhow::Result<TransactionReceipt> {
        let mut tx_builder = self
            .sequencer
            .submitProof(image_id, journal.into(), seal.into());

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

    fn commitment_to_blobhash(commitment: &Bytes) -> FixedBytes<32> {
        let mut hasher = Sha256::new();
        hasher.update(commitment);
        let mut hashed_commitment = hasher.finalize();
        hashed_commitment[0] = 1;

        let mut fixed_bytes_input: [u8; 32] = [0u8; 32];
        fixed_bytes_input.copy_from_slice(&hashed_commitment);

        FixedBytes::<32>::from(fixed_bytes_input)
    }

    async fn get_parent_beacon_block_root(
        &self,
        block_number: u64,
    ) -> anyhow::Result<FixedBytes<32>> {
        let parent_beacon_block_root = self
            .provider
            .get_block_by_number(BlockNumberOrTag::Number(block_number), true)
            .await?
            .ok_or_else(|| {
                format_error!("Block {} not found", block_number)
                // return Box::new(OwValidatorNodeError::BlockNotFound(block_number))
            })?
            .header
            .parent_beacon_block_root
            .ok_or_else(|| {
                format_error!("Block {} not found", block_number)
                // return Box::new(OwValidatorNodeError::BlockNotFound(block_number))
            })?;

        Ok(parent_beacon_block_root)
    }

    pub async fn subscribe_to_queue(&self, config: &Config) -> anyhow::Result<QueueHeadData> {
        log_info!("Subscribing to queue");
        let ws_url = WsConnect::new(&config.ws_url);
        let ws_provider = ProviderBuilder::new().on_ws(ws_url).await?;

        let filter = Filter::new()
            .address(config.ddex_sequencer_address)
            .event(DdexSequencer::NewBlobSubmitted::SIGNATURE);

        log_info!("Subscribed to queue, waiting for new blobs...");
        let subscription = ws_provider.subscribe_logs(&filter).await?;
        let mut stream = subscription.into_stream();

        let mut queue_head_commitment = Bytes::new();
        let mut parent_beacon_block_root = FixedBytes::<32>::new([0u8; 32]);
        let mut transaction_hash = FixedBytes::<32>::new([0u8; 32]);
        let mut block_number: u64 = 0;
        let mut timestamp: u64 = 0;
        let mut blob_image_id = FixedBytes::<32>::new([0u8; 32]);

        while let Some(log) = stream.next().await {
            println!("New blob detected!");
            let DdexSequencer::NewBlobSubmitted {
                commitment,
                image_id,
            } = log.log_decode()?.inner.data;
            block_number = log
                .block_number
                .ok_or_else(|| format_error!("Block not found in log"))?;
            parent_beacon_block_root = self.get_parent_beacon_block_root(block_number).await?;
            queue_head_commitment = commitment;
            blob_image_id = image_id;
            *config.start_block.borrow_mut() = block_number;
            transaction_hash = log
                .transaction_hash
                .ok_or_else(|| format_error!("Transaction hash not found in log"))?;
            timestamp = log
                .block_timestamp
                .ok_or_else(|| format_error!("Block timestamp not found in log"))?;
            break;
        }
        Ok(QueueHeadData {
            parent_beacon_block_root,
            versioned_blobhash: Self::commitment_to_blobhash(&queue_head_commitment),
            commitment: queue_head_commitment,
            image_id: blob_image_id,
            transaction_hash,
            block_number,
            timestamp,
        })
    }

    pub async fn get_queue_head_data(
        &self,
        config: &Config,
        queue_head: FixedBytes<32>,
    ) -> anyhow::Result<QueueHeadData> {
        let filter = Filter::new()
            .address(config.ddex_sequencer_address)
            .event(DdexSequencer::NewBlobSubmitted::SIGNATURE)
            .from_block(*config.start_block.borrow());

        let logs = self.provider.get_logs(&filter).await?;

        let mut queue_head_commitment = Bytes::new();
        let mut blob_image_id = FixedBytes::<32>::new([0u8; 32]);
        let mut parent_beacon_block_root = FixedBytes::<32>::new([0u8; 32]);
        let mut transaction_hash = FixedBytes::<32>::new([0u8; 32]);
        let mut block_number: u64 = 0;
        let mut timestamp: u64 = 0;

        for log in logs {
            match log.topic0() {
                Some(&DdexSequencer::NewBlobSubmitted::SIGNATURE_HASH) => {
                    let DdexSequencer::NewBlobSubmitted {
                        commitment,
                        image_id,
                    } = log.log_decode()?.inner.data;
                    transaction_hash = log
                        .transaction_hash
                        .ok_or_else(|| format_error!("Transaction hash not found in log"))?;
                    let current_blobhash = Self::commitment_to_blobhash(&commitment);
                    if queue_head == current_blobhash {
                        block_number = log
                            .block_number
                            .ok_or_else(|| format_error!("Block not found in log"))?;
                        parent_beacon_block_root =
                            self.get_parent_beacon_block_root(block_number).await?;
                        queue_head_commitment = commitment;
                        blob_image_id = image_id;
                        *config.start_block.borrow_mut() = block_number;
                        timestamp = log
                            .block_timestamp
                            .ok_or_else(|| format_error!("Block timestamp not found in log"))?;
                        break;
                    }
                }
                _ => (),
            }
        }
        if parent_beacon_block_root == FixedBytes::<32>::new([0u8; 32])
            || queue_head_commitment == Bytes::new()
        {
            return Err(format_error!("Queue head not found"));
        }

        if blob_image_id == FixedBytes::<32>::new([0u8; 32]) {
            return Err(format_error!("Corrupted image_id"));
        }

        Ok(QueueHeadData {
            parent_beacon_block_root,
            versioned_blobhash: Self::commitment_to_blobhash(&queue_head_commitment),
            commitment: queue_head_commitment,
            image_id: blob_image_id,
            transaction_hash,
            block_number,
            timestamp,
        })
    }
}
