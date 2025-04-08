use alloy::primitives::FixedBytes;

pub const DDEX_SEQUENCER_ADDRESS: &str = "75AbeCf07C26368F0f4AA0b0d3637A732E25467e";
pub const GET_BEACON_BLOCK_API_PATH: &str = "/eth/v2/beacon/blocks/";
pub const GET_SIDECARS_API_PATH: &str = "/eth/v1/beacon/blob_sidecars/";
pub const EMPTY_QUEUE_HEAD: FixedBytes<32> = FixedBytes::repeat_byte(0);
pub const TEMP_FOLDER: &str = "./temp";
#[allow(dead_code)]
pub const IPFS_API_BASE_URL: &str = "https://ipfs.original.works";
#[allow(dead_code)]
pub const IPFS_API_CAT_FILE: &str = "/ipfs/";

pub const fn network_name(chain_id: &u64) -> &'static str {
    match chain_id {
        1 => "Ethereum",
        100 => "Gnosis",
        10200 => "Chiado",
        17000 => "Holesky",
        11155111 => "Sepolia",
        _ => panic!("Unrecognized chain"),
    }
}

pub static REQWEST_CLIENT: once_cell::sync::Lazy<reqwest::Client> =
    once_cell::sync::Lazy::new(|| reqwest::Client::new());
