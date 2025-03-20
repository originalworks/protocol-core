use alloy::primitives::FixedBytes;

pub const DDEX_SEQUENCER_ADDRESS: &str = "05B077035F2f147543f2Ec69794F922bb5784909";
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
        17000 => "Holesky",
        _ => panic!("Unrecognized chain"),
    }
}
