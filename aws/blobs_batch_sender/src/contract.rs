use crate::contract::sEOA::SubmitNewBlobInput;
use alloy::{
    consensus::BlobTransactionSidecar,
    network::EthereumWallet,
    primitives::{Address, Bytes, FixedBytes},
    providers::{
        Identity, ProviderBuilder,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
    },
    sol,
};
use blobs_batch_sender::BlobsBatchSenderConfig;
use lambda_runtime::Error;
use owen::blobs_queue::BlobsQueueS3JsonFile;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    sEOA,
    "../../submodules/account-abstraction/artifacts/contracts/sEOA.sol/sEOA.json"
);

type HardlyTypedProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    alloy::providers::RootProvider,
>;

struct SendBatchTxInput {
    tx_params: Vec<SubmitNewBlobInput>,
    sidecar: BlobTransactionSidecar,
}

pub struct SmartEoaManager {
    ddex_sequencer_address: Address,
    s_eoa: sEOA::sEOAInstance<HardlyTypedProvider>, // rpc_url: String,
}

impl SmartEoaManager {
    pub fn build(config: &BlobsBatchSenderConfig, wallet: EthereumWallet) -> Result<Self, Error> {
        let provider = ProviderBuilder::new()
            .wallet(wallet)
            .connect_http(config.rpc_url.parse()?);

        let s_eoa = sEOA::new(config.s_eoa_address, provider);

        Ok(Self {
            ddex_sequencer_address: config.ddex_sequencer_address,
            s_eoa,
        })
    }

    fn parse_batch_input(
        blob_tx_data_vec: Vec<BlobsQueueS3JsonFile>,
    ) -> Result<SendBatchTxInput, Error> {
        let tx_params: Vec<SubmitNewBlobInput> = blob_tx_data_vec
            .iter()
            .map(|blob_json_file| SubmitNewBlobInput {
                imageId: blob_json_file.image_id,
                commitment: Bytes::from(blob_json_file.tx_data.kzg_commitment.to_vec()),
                blobSha2: FixedBytes::<32>::from(blob_json_file.tx_data.blob_sha2),
            })
            .collect();

        let mut sidecar = BlobTransactionSidecar {
            blobs: Vec::new(),
            commitments: Vec::new(),
            proofs: Vec::new(),
        };

        for blob_json_file in blob_tx_data_vec {
            sidecar.blobs.push(
                blob_json_file
                    .tx_data
                    .blob_sidecar
                    .blobs
                    .first()
                    .expect("Blob missing in input")
                    .clone(),
            );
            sidecar.commitments.push(
                blob_json_file
                    .tx_data
                    .blob_sidecar
                    .commitments
                    .first()
                    .expect("commitment missing in input")
                    .clone(),
            );
            sidecar.proofs.push(
                blob_json_file
                    .tx_data
                    .blob_sidecar
                    .proofs
                    .first()
                    .expect("proof missing in input")
                    .clone(),
            );
        }
        Ok(SendBatchTxInput { tx_params, sidecar })
    }

    pub async fn send_batch(
        &self,
        blob_tx_data_vec: Vec<BlobsQueueS3JsonFile>,
    ) -> Result<(), Error> {
        let batch_input = Self::parse_batch_input(blob_tx_data_vec)?;
        let mut _tx_builder = self
            .s_eoa
            .submitNewBlobBatch(batch_input.tx_params, self.ddex_sequencer_address)
            .sidecar(batch_input.sidecar)
            .max_fee_per_blob_gas(1000000001);
        Ok(())
    }
}
