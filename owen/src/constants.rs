pub const DDEX_SEQUENCER_ADDRESS: &str = "75AbeCf07C26368F0f4AA0b0d3637A732E25467e";
pub const IPFS_API_BASE_URL: &str = "http://localhost:5001";
pub const IPFS_API_ADD_FILE: &str = "/api/v0/add";
pub const IPFS_API_CAT_FILE: &str = "/api/v0/cat";
pub const DEFAULT_STORACHA_BRIDGE_URL: &str =
    "https://qb9qzwgenl.execute-api.us-east-1.amazonaws.com/prod/";
pub const CLIENT: &[u8] = b"OWEN";
pub const MAX_DDEX_PER_BLOB: i32 = 50;
#[cfg(any(feature = "local-s3"))]
pub const DEFAULT_DATABASE_NAME: &str = "sqlite";

#[cfg(any(feature = "local-s3"))]
pub const DEFAULT_TABLE_NAME: &str = "message_folders";

pub static REQWEST_CLIENT: once_cell::sync::Lazy<reqwest::Client> =
    once_cell::sync::Lazy::new(|| reqwest::Client::new());
