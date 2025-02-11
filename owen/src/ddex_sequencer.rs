use crate::blob::BlobTransactionData;
use crate::constants;
use alloy::primitives::{Address, FixedBytes};
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
use log_macros::log_info;
use serde_json::json;

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
                    Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<Http<Client>>,
            Http<Client>,
            Ethereum,
        >,
    >,
}

impl DdexSequencerContext<'_> {
    pub async fn build(
        provider: &FillProvider<
            JoinFill<
                JoinFill<
                    Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<Http<Client>>,
            Http<Client>,
            Ethereum,
        >,
        ddex_sequencer_address: Address,
    ) -> anyhow::Result<DdexSequencerContext> {
        log_info!("Creating DdexSequencerContext...");
        let contract = DdexSequencer::new(ddex_sequencer_address, provider);
        let result = DdexSequencerContext { contract };
        Ok(result)
    }

    pub async fn send_blob(&self, transaction_data: BlobTransactionData) -> anyhow::Result<()> {
        log_info!("Sending tx...");
        let receipt = self
            .contract
            .submitNewBlob(
                Bytes::from(transaction_data.kzg_commitment.to_vec()),
                FixedBytes::<32>::from(transaction_data.blob_sha2),
            )
            .sidecar(transaction_data.blob_sidecar)
            .send()
            .await?
            .get_receipt()
            .await?;

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
