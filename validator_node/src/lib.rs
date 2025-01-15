mod beacon_chain;
mod constants;
mod ddex_sequencer;
mod errors;
mod ipfs;
mod prover_wrapper;

use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::{Bytes, FixedBytes};
use alloy::providers::fillers::{
    ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{ProviderBuilder, RootProvider};
use alloy::signers::local::PrivateKeySigner;
use constants::EMPTY_QUEUE_HEAD;
use ddex_sequencer::{DdexSequencerContext, QueueHeadData};
use std::cell::RefCell;
use std::env;
use std::error::Error;

pub struct Config {
    pub rpc_url: String,
    pub beacon_rpc_url: String,
    pub ws_url: String,
    pub start_block: RefCell<u64>,
    pub private_key: String,
    pub provider: FillProvider<
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
}

impl Config {
    fn get_env_var(key: &str) -> Result<String, Box<dyn Error>> {
        env::var(key).map_err(|err| {
            format!("Error getting environment variable `{}`: {:?}", key, err).into()
        })
    }

    pub fn build() -> Result<Config, Box<dyn Error>> {
        let private_key = Config::get_env_var("PRIVATE_KEY")?;
        let rpc_url = Config::get_env_var("RPC_URL")?;
        let beacon_rpc_url = Config::get_env_var("BEACON_RPC_URL")?;
        let ws_url = Config::get_env_var("WS_URL")?;
        let start_block = RefCell::new(Config::get_env_var("START_BLOCK")?.parse::<u64>()?);

        let private_key_signer: PrivateKeySigner =
            private_key.parse().expect("Failed to parse PRIVATE_KEY:");
        let wallet = EthereumWallet::from(private_key_signer);

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url.parse().expect("RPC_URL parsing error:"));

        Ok(Config {
            rpc_url,
            beacon_rpc_url,
            ws_url,
            start_block,
            private_key,
            provider,
        })
    }
}

async fn validate_blobs(
    config: &Config,
    ddex_sequencer_context: &DdexSequencerContext<'_>,
) -> Result<(), Box<dyn Error>> {
    let queue_head = ddex_sequencer_context
        .contract
        .blobQueueHead()
        .call()
        .await?
        ._0;

    let mut queue_head_data: QueueHeadData = QueueHeadData {
        commitment: Bytes::new(),
        parent_beacon_block_root: FixedBytes::<32>::new([0u8; 32]),
    };

    if queue_head == EMPTY_QUEUE_HEAD {
        queue_head_data = ddex_sequencer_context.subscribe_to_queue(&config).await?;
    } else {
        queue_head_data = ddex_sequencer_context
            .get_queue_head_data(&config, queue_head)
            .await?;
    }
    let blob = beacon_chain::find_blob(
        &config.beacon_rpc_url,
        queue_head_data.commitment,
        queue_head_data.parent_beacon_block_root,
    )
    .await?;

    // let ipfs_cids = ddex_messages_data
    //     .iter()
    //     .map(|emittable_values| emittable_values.image_ipfs_cid.clone())
    //     .collect();

    // ipfs::check_file_accessibility(ipfs_cids).await?;

    let prover_run_results = prover_wrapper::run(&blob.into())?;

    println!("sending tx...");
    let receipt = ddex_sequencer_context
        .submit_proof(
            prover_run_results.journal.into(),
            prover_run_results.seal.into(),
        )
        .await?;

    println!("Receipt tx hash: {}", receipt.transaction_hash);
    Ok(())
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let ddex_sequencer_context =
        ddex_sequencer::DdexSequencerContext::build(&config.provider).await?;

    loop {
        validate_blobs(&config, &ddex_sequencer_context).await?;
    }
}
