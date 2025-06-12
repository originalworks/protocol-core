use alloy::primitives::FixedBytes;

pub const DDEX_SEQUENCER_ADDRESS: &str = "75AbeCf07C26368F0f4AA0b0d3637A732E25467e";
pub const GET_BEACON_BLOCK_API_PATH: &str = "/eth/v2/beacon/blocks/";
pub const GET_SIDECARS_API_PATH: &str = "/eth/v1/beacon/blob_sidecars/";
pub const EMPTY_BYTES32: FixedBytes<32> = FixedBytes::repeat_byte(0);
pub const TEMP_FOLDER: &str = "./temp";
pub const BLOB_ASSIGNMENT_FOLDER_NAME: &str = "blob_assignment";
pub const DOWNLOADED_BLOBS_FOLDER_NAME: &str = "downloaded_blobs";
pub const BLOB_ASSIGNMENT_JSON_FILE_NAME: &str = "blob_assignments.json";
pub const IPFS_TEMP_FILES_FOLDER_NAME: &str = "ipfs_temp_files";
pub const MAX_BLOB_ASSIGNMENTS: usize = 5;
pub const MAX_STORED_ASSIGNMENTS: usize = 20;
#[allow(dead_code)]
pub const IPFS_API_BASE_URL: &str = "https://w3s.link";
#[allow(dead_code)]
pub const IPFS_API_CAT_FILE: &str = "/ipfs/";
pub const CLIENT: &[u8] = b"VALIDATOR";
pub const DEFAULT_STORACHA_BRIDGE_URL: &str =
    "https://p1o0h55rwh.execute-api.us-east-1.amazonaws.com/devel/";

pub const fn network_name(chain_id: &u64) -> &'static str {
    match chain_id {
        1 => "Ethereum",
        100 => "Gnosis",
        10200 => "Chiado",
        17000 => "Holesky",
        11155111 => "Sepolia",
        3151908 => "Kurtosis",
        _ => panic!("Unrecognized chain"),
    }
}

pub static REQWEST_CLIENT: once_cell::sync::Lazy<reqwest::Client> =
    once_cell::sync::Lazy::new(|| reqwest::Client::new());
