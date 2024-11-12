use crate::{constants, errors::OwValidatorNodeError, Config};
use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::{Bytes, FixedBytes};
use alloy::providers::fillers::{
    ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider, RootProvider},
    rpc::types::Filter,
    sol,
    sol_types::SolEvent,
};
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use std::error::Error;

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
                    JoinFill<JoinFill<alloy::providers::Identity, GasFiller>, NonceFiller>,
                    ChainIdFiller,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<alloy::transports::http::Http<reqwest::Client>>,
            alloy::transports::http::Http<reqwest::Client>,
            Ethereum,
        >,
    >,
}

pub struct QueueHeadData {
    pub commitment: Bytes,
    pub parent_beacon_block_root: FixedBytes<32>,
}

impl DdexSequencerContext<'_> {
    pub async fn build(
        provider: &FillProvider<
            JoinFill<
                JoinFill<
                    JoinFill<JoinFill<alloy::providers::Identity, GasFiller>, NonceFiller>,
                    ChainIdFiller,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<alloy::transports::http::Http<reqwest::Client>>,
            alloy::transports::http::Http<reqwest::Client>,
            Ethereum,
        >,
    ) -> Result<DdexSequencerContext, Box<dyn Error>> {
        let contract = DdexSequencer::new(constants::DDEX_SEQUENCER_ADDRESS, provider);
        let result = DdexSequencerContext { contract };
        Ok(result)
    }

    fn commitment_to_blobhash(commitment: &Bytes) -> Result<FixedBytes<32>, Box<dyn Error>> {
        let mut hasher = Sha256::new();
        hasher.update(commitment);
        let mut hashed_commitment = hasher.finalize();
        hashed_commitment[0] = 1;

        let mut fixed_bytes_input: [u8; 32] = [0u8; 32];
        fixed_bytes_input.copy_from_slice(&hashed_commitment);

        Ok(FixedBytes::<32>::from(fixed_bytes_input))
    }

    async fn get_parent_beacon_block_root(
        &self,
        block_number: u64,
    ) -> Result<FixedBytes<32>, Box<dyn Error>> {
        let parent_beacon_block_root = self
            .contract
            .provider()
            .get_block_by_number(BlockNumberOrTag::Number(block_number), true)
            .await?
            .ok_or_else(|| return Box::new(OwValidatorNodeError::BlockNotFound(block_number)))?
            .header
            .parent_beacon_block_root
            .ok_or_else(|| return Box::new(OwValidatorNodeError::BlockNotFound(block_number)))?;

        Ok(parent_beacon_block_root)
    }

    pub async fn subscribe_to_queue(
        &self,
        config: &Config,
    ) -> Result<QueueHeadData, Box<dyn Error>> {
        let ws_url = WsConnect::new(&config.ws_url);
        let ws_provider = ProviderBuilder::new().on_ws(ws_url).await?;

        let filter = Filter::new()
            .address(constants::DDEX_SEQUENCER_ADDRESS)
            .event(DdexSequencer::NewBlobSubmitted::SIGNATURE);

        println!("Subscribed to queue, waiting for new blobs...");
        let subscription = ws_provider.subscribe_logs(&filter).await?;
        let mut stream = subscription.into_stream();

        let mut queue_head_commitment = Bytes::new();
        let mut parent_beacon_block_root = FixedBytes::<32>::new([0u8; 32]);

        while let Some(log) = stream.next().await {
            println!("New blob detected!");
            let DdexSequencer::NewBlobSubmitted { commitment } = log.log_decode()?.inner.data;
            let block_number = log.block_number.ok_or_else(|| {
                return Box::new(OwValidatorNodeError::BlockNotFoundInLog());
            })?;
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
    ) -> Result<QueueHeadData, Box<dyn Error>> {
        let filter = Filter::new()
            .address(constants::DDEX_SEQUENCER_ADDRESS)
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
                    let current_blobhash = Self::commitment_to_blobhash(&commitment)?;
                    if queue_head == current_blobhash {
                        let block_number = log.block_number.ok_or_else(|| {
                            return Box::new(OwValidatorNodeError::BlockNotFoundInLog());
                        })?;
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
            return Err(Box::new(OwValidatorNodeError::QueueHeadNotFound()));
        }
        Ok(QueueHeadData {
            parent_beacon_block_root,
            commitment: queue_head_commitment,
        })
    }
}
