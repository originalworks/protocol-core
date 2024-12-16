use alloy::primitives::{address, Address, FixedBytes};

pub const DDEX_SEQUENCER_ADDRESS: Address = address!("00c042C4D5D913277CE16611a2ce6e9003554aD5");
pub const GET_BEACON_BLOCK_API_PATH: &str = "/eth/v2/beacon/blocks/";
pub const GET_SIDECARS_API_PATH: &str = "/eth/v1/beacon/blob_sidecars/";
pub const EMPTY_QUEUE_HEAD: FixedBytes<32> = FixedBytes::repeat_byte(0);
pub const IMAGE_FILE_CID_TAG: &str = "ImageIpfsCid";
pub const IPFS_API_BASE_URL: &str = "http://localhost:5001";
pub const IPFS_API_CAT_FILE: &str = "/api/v0/cat";
