use ddex_schema::DdexMessage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Outputs {
    pub message: DdexMessage,
}
