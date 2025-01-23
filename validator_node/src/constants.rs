use alloy::primitives::{address, Address, FixedBytes};

pub const DDEX_SEQUENCER_ADDRESS: Address = address!("EAdd25177cd288aE9f90596938f934Bd20b3531a");
pub const GET_BEACON_BLOCK_API_PATH: &str = "/eth/v2/beacon/blocks/";
pub const GET_SIDECARS_API_PATH: &str = "/eth/v1/beacon/blob_sidecars/";
pub const EMPTY_QUEUE_HEAD: FixedBytes<32> = FixedBytes::repeat_byte(0);
#[allow(dead_code)]
pub const IPFS_API_BASE_URL: &str = "http://localhost:5001";
#[allow(dead_code)]
pub const IPFS_API_CAT_FILE: &str = "/api/v0/cat";
