use ddex_schema::DdexMessage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateOutputs {
    pub full_content: Option<DdexMessage>,
    pub error: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicOutputs {
    pub is_valid: bool,
    pub message_id: Option<String>,
}
