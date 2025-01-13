use ddex_schema::DdexMessage;
use serde::{Deserialize, Serialize};

alloy_sol_types::sol!(
    #[sol(rpc, all_derives)]
    "../../contracts/contracts/interfaces/IProverPublicOutputs.sol"
);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateOutputs {
    pub full_content: Option<DdexMessage>,
    pub error: Option<String>,
}
