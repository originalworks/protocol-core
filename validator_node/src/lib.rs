mod beacon_chain;
pub mod blob_assignment;
pub mod blob_proofs;
mod constants;
mod contracts;
pub mod ipfs;
mod zip;
use alloy::primitives::Address;
use beacon_chain::BlobFinder;
use blob_assignment::files::BlobAssignmentFiles;
use blob_assignment::manager::{BlobAssignmentManager, BlobAssignmentStartingPoint};
use blob_proofs::BlobProofManager;
use contracts::ContractsManager;
use ipfs::IpfsManager;
use log_macros::log_error;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    pub private_key: String,
    pub environment: String,
    pub username: String,
    pub segment_limit_po2: u32,
    pub ddex_sequencer_address: Address,
    pub disable_telemetry: bool,
    pub storacha_bridge_url: String,
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
        let disable_telemetry: bool = matches!(
            std::env::var("DISABLE_TELEMETRY")
                .unwrap_or_else(|_| "false".to_string())
                .as_str(),
            "1" | "true"
        );
        let mut storacha_bridge_url = env::var("STORACHA_BRIDGE_URL")
            .unwrap_or_else(|_| constants::DEFAULT_STORACHA_BRIDGE_URL.to_string());

        if !storacha_bridge_url.ends_with("/") {
            storacha_bridge_url = format!("{}/", storacha_bridge_url)
        }

        Config {
            rpc_url,
            beacon_rpc_url,
            ws_url,
            private_key,
            environment,
            username,
            segment_limit_po2,
            ddex_sequencer_address,
            disable_telemetry,
            storacha_bridge_url,
        }
    }
}

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let mut blob_assignments_consecutive_error_ct = 0;
    let mut proof_calculation_consecutive_error_ct = 0;
    let threshold = 5;

    let mut next_starting_point = BlobAssignmentStartingPoint::CleanStart;

    let contracts_manager = Arc::new(ContractsManager::build(&config).await?);

    let blob_finder = BlobFinder::new(config.beacon_rpc_url.clone());

    let ipfs_manager = IpfsManager::build(
        Arc::clone(&contracts_manager),
        config.storacha_bridge_url.clone(),
    )?;

    let blob_assignment_files_ptr_1 = Arc::new(Mutex::new(BlobAssignmentFiles::build()?));
    let blob_assignment_files_ptr_2 = Arc::clone(&blob_assignment_files_ptr_1);

    let blob_assignment_manager = BlobAssignmentManager::new(
        blob_assignment_files_ptr_2,
        Arc::clone(&contracts_manager),
        blob_finder,
    );

    let blob_proof_manager = BlobProofManager::new(
        blob_assignment_files_ptr_1,
        ipfs_manager,
        config.segment_limit_po2.clone(),
    );

    let mut current_block_number = blob_assignment_manager.fetch_current_block().await.unwrap();
    blob_assignment_manager.init_clear_inner_queue().await?;

    tokio::spawn(async move {
        loop {
            if blob_assignments_consecutive_error_ct == threshold {
                panic!(
                    "{} consecutive errors occured. Proccess has been terminated",
                    threshold
                );
            }
            let res: Result<BlobAssignmentStartingPoint, anyhow::Error>;
            match next_starting_point {
                BlobAssignmentStartingPoint::NewBlobSubmitted { block_number } => {
                    current_block_number = block_number;
                    let _ = blob_assignment_manager
                        .try_new_assignment(current_block_number)
                        .await;
                    res = blob_assignment_manager.run(current_block_number).await;
                }
                BlobAssignmentStartingPoint::BlobProcessedOrRejected { block_number } => {
                    current_block_number = block_number;

                    res = blob_assignment_manager.run(current_block_number).await;
                }
                BlobAssignmentStartingPoint::CleanStart => {
                    res = blob_assignment_manager.run(current_block_number).await;
                }
            }

            if let Err(e) = res {
                log_error!("{e}");
                blob_assignments_consecutive_error_ct += 1;
            } else {
                blob_assignments_consecutive_error_ct = 0;
                next_starting_point = res.expect("Blob assignment manager failed");
            }
        }
    });

    loop {
        if proof_calculation_consecutive_error_ct == threshold {
            return Err(log_error!(
                "{} consecutive errors occured. Proccess has been terminated",
                threshold
            ));
        }

        let res = blob_proof_manager.run().await;
        if let Err(e) = res {
            log_error!("{e}");
            proof_calculation_consecutive_error_ct += 1;
        } else {
            proof_calculation_consecutive_error_ct = 0;
        }
    }
}
