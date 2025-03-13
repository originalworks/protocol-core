use crate::blob::BlobTransactionData;
use crate::is_local;
use alloy::primitives::{Address, FixedBytes};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::private::Bytes;
use alloy::{
    network::{Ethereum, EthereumWallet},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, RootProvider,
    },
    sol,
    transports::http::{reqwest, Client, Http},
};
use anyhow::Context;
use log_macros::{format_error, log_info, log_warn};
use serde_json::json;
use DdexEmitter::getSupportedBlobImageIdsReturn;

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
    pub image_id: alloy::primitives::FixedBytes<32>,
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
        let emitter = DdexEmitter::new(emitter_address, provider);

        let image_id_parsed = alloy::primitives::FixedBytes::<32>::from_slice(
            &prover::CURRENT_DDEX_GUEST_ID
                .map(|word| word.to_le_bytes())
                .concat(),
        );

        Ok(Self {
            sequencer,
            emitter,
            image_id: image_id_parsed,
        })
    }

    pub async fn check_image_compatibility(&self) -> anyhow::Result<()> {
        let getSupportedBlobImageIdsReturn {
            _0: current_image_id,
            _1: previous_image_id,
        } = self.emitter.getSupportedBlobImageIds().call().await?;
        if self.image_id == current_image_id {
            log_info!("Using current version of image id");
            return Ok(());
        } else if self.image_id == previous_image_id {
            log_warn!("Using previous version of image id. Remember to update Owen to latest version before previous version sunsets");
            return Ok(());
        } else {
            log_warn!(
                "Current sequencer blob image id: {}",
                current_image_id.to_string()
            );
            log_warn!(
                "Previous sequencer blob image id: {}",
                previous_image_id.to_string()
            );
            log_warn!("Owen image id: {}", self.image_id.to_string());
            return Err(format_error!(
                "ImageId: {} is not supported by Sequencer/Emitter",
                &self.image_id
            ));
        }
    }

    pub async fn send_blob(&self, transaction_data: BlobTransactionData) -> anyhow::Result<()> {
        log_info!("Sending tx...");
        let mut tx_builder = self
            .sequencer
            .submitNewBlob(
                self.image_id,
                Bytes::from(transaction_data.kzg_commitment.to_vec()),
                FixedBytes::<32>::from(transaction_data.blob_sha2),
            )
            .sidecar(transaction_data.blob_sidecar);

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

        Ok(())
    }
}
