use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum OwValidatorNodeError {
    BlockNotFound(u64),
    BlockNotFoundInLog(),
    QueueHeadNotFound(),
    FailedToFindBlobSidecar(String),
    InvalidBlobLength(usize),
    InvalidHexStringValue(String),
    ImageFileNotFoundInIpfs(String),
}

impl fmt::Display for OwValidatorNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BlockNotFound(block_no) => {
                write!(
                    f,
                    "Could not fetch the block info for block no: {}",
                    block_no
                )
            }
            Self::BlockNotFoundInLog() => {
                write!(f, "Failed to extract block number from log")
            }
            Self::QueueHeadNotFound() => {
                write!(f, "Details about the queue head could not be found")
            }
            Self::FailedToFindBlobSidecar(commitment) => {
                write!(
                    f,
                    "Failed to find BlobSidecar in the beacon chain for the commitment: {}",
                    commitment
                )
            }
            Self::InvalidBlobLength(blob_len) => {
                write!(
                    f,
                    "Invalid blob length: expected 262146 ( = 131072 * 2 + 2), got: {}",
                    blob_len
                )
            }
            Self::InvalidHexStringValue(hex_byte) => {
                write!(f, "Invalid hex value in blob String: {}", hex_byte)
            }
            Self::ImageFileNotFoundInIpfs(cid) => {
                write!(f, "Image file not found in IPFS node: {}", cid)
            }
        }
    }
}

impl Error for OwValidatorNodeError {}
