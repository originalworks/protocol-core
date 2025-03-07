mod beacon_chain;
mod constants;
mod contracts;
mod ipfs;
pub mod prover_wrapper;
use alloy::primitives::Address;
use constants::EMPTY_QUEUE_HEAD;
use contracts::ContractsManager;
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
        let segment_limit_po2: u32 = env::var("SEGMENT_LIMIT_PO2")
            .unwrap_or_else(|_| "18".to_string())
            .parse()
            .expect("Failed to parse SEGMENT_LIMIT_PO2");
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
            environment,
            username,
            segment_limit_po2,
            ddex_sequencer_address,
        }
    }
}

async fn validate_blobs(
    config: &Config,
    contracts_manager: &ContractsManager,
) -> anyhow::Result<()> {
    let tx_context = sentry::TransactionContext::new("blob_processing", "process_blob");
    let tx = sentry::start_transaction(tx_context);

    let mut span = tx.start_child("queue_head_processing", "Get queue head data");

    let queue_head = contracts_manager.sequencer.blobQueueHead().call().await?._0;

    let queue_head_data;

    if queue_head == EMPTY_QUEUE_HEAD {
        log_info!("Queue head is empty");
        queue_head_data = contracts_manager.subscribe_to_queue(&config).await?;
    } else {
        log_info!("Queue head points to {}", queue_head.to_string());
        queue_head_data = contracts_manager
            .get_queue_head_data(&config, queue_head)
            .await?;
    }

    log_debug!("**Queue head data: {}", json!(queue_head_data));

    let image_elf = contracts_manager
        .select_image_elf(&queue_head_data.image_id)
        .await?;

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

    let prover_run_results =
        prover_wrapper::run(&blob.into(), image_elf, config.segment_limit_po2)?;

    span.finish();

    log_info!("Sending tx...");
    span = tx.start_child("transaction_sending", "Sending transaction");

    let _ = contracts_manager
        .submit_proof(
            queue_head_data.image_id,
            prover_run_results.journal.into(),
            prover_run_results.seal.into(),
        )
        .await?;

    span.finish();
    tx.finish();

    Ok(())
}

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let mut consecutive_error_ct = 0;
    let threshold = 5;

    let contracts_manager = ContractsManager::build(
        config.ddex_sequencer_address,
        &config.private_key,
        &config.rpc_url,
    )
    .await?;

    loop {
        if consecutive_error_ct == threshold {
            return Err(log_error!(
                "{} consecutive errors occured. Proccess has been terminated",
                threshold
            ));
        }

        let res = validate_blobs(&config, &contracts_manager).await;
        if let Err(e) = res {
            log_error!("{e}");
            consecutive_error_ct += 1;
        } else {
            consecutive_error_ct = 0;
        }
    }
}
