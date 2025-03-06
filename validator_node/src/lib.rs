mod beacon_chain;
mod constants;
mod ddex_sequencer;
mod ipfs;
pub mod prover_wrapper;

use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::Address;
use alloy::providers::fillers::{
    ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{ProviderBuilder, RootProvider};
use alloy::signers::local::PrivateKeySigner;
use anyhow::Ok;
use constants::EMPTY_QUEUE_HEAD;
use ddex_sequencer::DdexSequencerContext;
use log_macros::{log_debug, log_error, log_info};
use serde_json::json;
use std::cell::RefCell;
use std::env;
use std::str::FromStr;

pub fn is_local() -> bool {
    matches!(
        std::env::var("LOCAL")
            .unwrap_or_else(|_| "false".to_string())
            .as_str(),
        "1" | "true"
    )
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub beacon_rpc_url: String,
    pub ws_url: String,
    pub start_block: RefCell<u64>,
    pub private_key: String,
    pub environment: String,
    pub username: String,
    pub segment_limit_po2: u32,
    pub ddex_sequencer_address: Address,
    #[serde(skip_serializing)]
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
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }

    pub fn build() -> Config {
        if is_local() {
            println!("Running local setup");
            dotenvy::from_filename(".env.local").unwrap();
        } else {
            dotenvy::dotenv().ok();
        }

        let private_key = Config::get_env_var("PRIVATE_KEY");
        let rpc_url = Config::get_env_var("RPC_URL");
        let beacon_rpc_url = Config::get_env_var("BEACON_RPC_URL");
        let ws_url = Config::get_env_var("WS_URL");
        let start_block = RefCell::new(
            Config::get_env_var("START_BLOCK")
                .parse::<u64>()
                .expect("Failed to parse START_BLOCK"),
        );
        let environment = Config::get_env_var("ENVIRONMENT");
        let username = Config::get_env_var("USERNAME");
        let private_key_signer: PrivateKeySigner =
            private_key.parse().expect("Failed to parse PRIVATE_KEY");
        let wallet = EthereumWallet::from(private_key_signer);
        let segment_limit_po2: u32 = env::var("SEGMENT_LIMIT_PO2")
            .unwrap_or_else(|_| "18".to_string())
            .parse()
            .expect("Failed to parse SEGMENT_LIMIT_PO2");
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url.parse().expect("RPC_URL parsing error"));

        let ddex_sequencer_address = Address::from_str(
            std::env::var("DDEX_SEQUENCER_ADDRESS")
                .unwrap_or_else(|_| constants::DDEX_SEQUENCER_ADDRESS.to_string())
                .as_str(),
        )
        .expect("Could not parse ddex sequencer address");

        Config {
            rpc_url,
            beacon_rpc_url,
            ws_url,
            start_block,
            private_key,
            provider,
            environment,
            username,
            segment_limit_po2,
            ddex_sequencer_address,
        }
    }
}

async fn validate_blobs(
    config: &Config,
    ddex_sequencer_context: &DdexSequencerContext<'_>,
) -> anyhow::Result<()> {
    let tx_context = sentry::TransactionContext::new("blob_processing", "process_blob");
    let tx = sentry::start_transaction(tx_context);

    let mut span = tx.start_child("queue_head_processing", "Get queue head data");

    let queue_head = ddex_sequencer_context
        .contract
        .blobQueueHead()
        .call()
        .await?
        ._0;

    let queue_head_data;

    if queue_head == EMPTY_QUEUE_HEAD {
        log_info!("Queue head is empty");
        queue_head_data = ddex_sequencer_context.subscribe_to_queue(&config).await?;
    } else {
        log_info!("Queue head points to {}", queue_head.to_string());
        queue_head_data = ddex_sequencer_context
            .get_queue_head_data(&config, queue_head)
            .await?;
    }

    log_debug!("**Queue head data: {}", json!(queue_head_data));

    span.finish();

    span = tx.start_child("blob_discovery", "Get blob");

    let blob = beacon_chain::find_blob(
        &config.beacon_rpc_url,
        &queue_head_data.commitment,
        &queue_head_data.parent_beacon_block_root,
    )
    .await?;

    span.finish();

    span = tx.start_child(
        "message_jsons_ipfs_pin",
        "Pinning message JSON files to IPFS",
    );

    ipfs::prepare_blob_folder(blob, &queue_head_data).await?;

    span.finish();

    span = tx.start_child("proving", "Proving");

    let prover_run_results = prover_wrapper::run(&blob.into(), config.segment_limit_po2)?;

    span.finish();

    log_info!("Sending tx...");
    span = tx.start_child("transaction_sending", "Sending transaction");

    let _ = ddex_sequencer_context
        .submit_proof(
            prover_run_results.journal.into(),
            prover_run_results.seal.into(),
        )
        .await?;

    span.finish();
    tx.finish();

    Ok(())
}

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let ddex_sequencer_context = ddex_sequencer::DdexSequencerContext::build(
        &config.provider,
        config.ddex_sequencer_address,
    )
    .await;

    let mut consecutive_error_ct = 0;
    let threshold = 5;

    loop {
        if consecutive_error_ct == threshold {
            return Err(log_error!(
                "{} consecutive errors occured. Proccess has been terminated",
                threshold
            ));
        }

        let res = validate_blobs(&config, &ddex_sequencer_context).await;
        if let Err(e) = res {
            log_error!("{e}");
            consecutive_error_ct += 1;
        } else {
            consecutive_error_ct = 0;
        }
    }
}
