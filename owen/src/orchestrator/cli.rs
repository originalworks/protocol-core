use ow_wallet_adapter::{wallet::OwWallet, OwWalletConfig};

use crate::{
    blob::BlobTransactionData,
    config::core::Config,
    contracts::ContractsManager,
    ipfs::IpfsManager,
    output_generator::{DdexMessage, OutputFilesGenerator},
};

pub async fn run(config: &Config) -> anyhow::Result<Vec<DdexMessage>> {
    let ow_wallet_config = OwWalletConfig::from(config)?;
    let ow_wallet = OwWallet::build(&ow_wallet_config).await?;
    let contracts_manager = ContractsManager::build(&config, &ow_wallet).await?;
    contracts_manager.check_image_compatibility().await?;

    let ipfs_manager = IpfsManager::build(&config).await?;
    let output_files_generator = OutputFilesGenerator::build(&config, ipfs_manager)?;
    let ddex_messages = output_files_generator.generate_files(&ow_wallet).await?;

    let blob_transaction_data = BlobTransactionData::build(&config.output_files_dir)?;
    contracts_manager.send_blob(blob_transaction_data).await?;

    Ok(ddex_messages)
}
