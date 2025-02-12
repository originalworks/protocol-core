use crate::{errors::OwValidatorNodeError, Config};
use crate::{is_local, Config};
use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::providers::fillers::{
    ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::rpc::types::TransactionReceipt;
use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider, RootProvider},
    rpc::types::Filter,
    sol,
    sol_types::SolEvent,
};
use futures_util::StreamExt;
use log_macros::{format_error, log_info};
use serde_json::json;
use sha2::{Digest, Sha256};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DdexSequencer,
    "../contracts/artifacts/contracts/DdexSequencer.sol/DdexSequencer.json"
);

pub struct DdexSequencerContext<'a> {
    pub contract: DdexSequencer::DdexSequencerInstance<
        alloy::transports::http::Http<reqwest::Client>,
        &'a FillProvider<
            JoinFill<
                JoinFill<
                    alloy::providers::Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<
                            alloy::providers::fillers::BlobGasFiller,
                            JoinFill<NonceFiller, ChainIdFiller>,
                        >,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<alloy::transports::http::Http<reqwest::Client>>,
            alloy::transports::http::Http<reqwest::Client>,
            Ethereum,
        >,
    >,
}

#[derive(Debug, serde::Serialize)]
pub struct QueueHeadData {
    pub commitment: Bytes,
    pub parent_beacon_block_root: FixedBytes<32>,
}

impl DdexSequencerContext<'_> {
    pub async fn build(
        provider: &FillProvider<
            JoinFill<
                JoinFill<
                    alloy::providers::Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<
                            alloy::providers::fillers::BlobGasFiller,
                            JoinFill<NonceFiller, ChainIdFiller>,
                        >,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<alloy::transports::http::Http<reqwest::Client>>,
            alloy::transports::http::Http<reqwest::Client>,
            Ethereum,
        >,
        ddex_sequencer_address: Address,
    ) -> DdexSequencerContext {
        let contract = DdexSequencer::new(ddex_sequencer_address, provider);
        let result = DdexSequencerContext { contract };
        result
    }

    pub async fn submit_proof(
        &self,
        journal: Vec<u8>,
        seal: Vec<u8>,
    ) -> anyhow::Result<TransactionReceipt> {
        let mut tx_builder = self.contract.submitProof(journal.into(), seal.into());

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
            .contract
            .provider()
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

        while let Some(log) = stream.next().await {
            println!("New blob detected!");
            let DdexSequencer::NewBlobSubmitted { commitment } = log.log_decode()?.inner.data;
            let block_number = log
                .block_number
                .ok_or_else(|| format_error!("Block not found in log"))?;
            parent_beacon_block_root = self.get_parent_beacon_block_root(block_number).await?;
            queue_head_commitment = commitment;
            *config.start_block.borrow_mut() = block_number;
            break;
        }
        Ok(QueueHeadData {
            parent_beacon_block_root,
            commitment: queue_head_commitment,
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

        let logs = config.provider.get_logs(&filter).await?;

        let mut queue_head_commitment = Bytes::new();
        let mut parent_beacon_block_root = FixedBytes::<32>::new([0u8; 32]);

        for log in logs {
            match log.topic0() {
                Some(&DdexSequencer::NewBlobSubmitted::SIGNATURE_HASH) => {
                    let DdexSequencer::NewBlobSubmitted { commitment } =
                        log.log_decode()?.inner.data;
                    let current_blobhash = Self::commitment_to_blobhash(&commitment);
                    if queue_head == current_blobhash {
                        let block_number = log
                            .block_number
                            .ok_or_else(|| format_error!("Block not found in log"))?;
                        parent_beacon_block_root =
                            self.get_parent_beacon_block_root(block_number).await?;
                        queue_head_commitment = commitment;
                        *config.start_block.borrow_mut() = block_number;
                        break;
                    }
                }
                _ => (),
            }
        }
        if parent_beacon_block_root == FixedBytes::<32>::new([0u8; 32])
            || queue_head_commitment == Bytes::new()
        {
            // return Err(OwValidatorNodeError::QueueHeadNotFound());
            return Err(format_error!("Queue head not found"));
        }

        Ok(QueueHeadData {
            parent_beacon_block_root,
            commitment: queue_head_commitment,
        })
    }
}
