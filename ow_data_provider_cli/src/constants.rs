use alloy::primitives::{address, Address};

pub const DDEX_SEQUENCER_ADDRESS: Address = address!("00c042C4D5D913277CE16611a2ce6e9003554aD5");

#[cfg(any(not(test), rust_analyzer))]
pub const IPFS_API_BASE_URL: &str = "http://localhost:5001";
pub const IPFS_API_ADD_FILE: &str = "/api/v0/add";
pub const OUTPUT_FILES_DIR: &str = "./output_files";
pub const IPFS_CIDS_ROOT_TAG: &str = "MessageHeader";
pub const IMAGE_FILE_CID_TAG: &str = "ImageIpfsCid";

#[cfg(all(test, not(rust_analyzer)))]
pub const IPFS_API_BASE_URL: &str = "http://ipfs:5001";
#[cfg(all(test, not(rust_analyzer)))]
pub const IPFS_API_CAT_FILE: &str = "/api/v0/cat";
