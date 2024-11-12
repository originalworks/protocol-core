use crate::blob::BlobTransactionData;
use crate::constants;
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
    ) -> Result<DdexSequencerContext, Box<dyn Error>> {
        let contract = DdexSequencer::new(constants::DDEX_SEQUENCER_ADDRESS, provider);
        let result = DdexSequencerContext { contract };
        Ok(result)
    }

    pub async fn send_blob(
        &self,
        transaction_data: BlobTransactionData,
    ) -> Result<(), Box<dyn Error>> {
        let receipt = self
            .contract
            .submitNewBlob(Bytes::from(transaction_data.kzg_commitment.to_vec()))
            .sidecar(transaction_data.blob_sidecar)
            // TODO make gas settings optional CLI/setting file parameters
            .max_fee_per_blob_gas(10000000)
            .max_fee_per_gas(100000000)
            .max_priority_fee_per_gas(100000000)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        println!("{receipt:?}");
        Ok(())
    }
}
