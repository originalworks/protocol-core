use ddex_schema::DdexMessage;
use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Outputs {
    pub data_hash: Digest,
    pub message: DdexMessage,
}
