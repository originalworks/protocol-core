use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum OwCodecError {
    BlobOverflowError(String),
    EmptyDirectory(String),
    NotADirectory(String),
    NotAFile(String),
    EmptyFile(String),
}

impl fmt::Display for OwCodecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BlobOverflowError(file_path) => {
                write!(f, "Blob overflow error at: {}", file_path)
            }
            Self::EmptyDirectory(dir_path) => {
                write!(f, "No files found at: {}", dir_path)
            }
            Self::NotADirectory(dir_path) => {
                write!(f, "{} is not a directory", dir_path)
            }
            Self::NotAFile(file_path) => {
                write!(f, "{} is not a file", file_path)
            }
            Self::EmptyFile(file_path) => {
                write!(f, "{} is an empty file", file_path)
            }
        }
    }
}

impl Error for OwCodecError {}
