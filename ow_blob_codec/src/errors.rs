use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum OwCodecError {
    FileExtensionExtraction(String),
    BlobOverflowError(String),
    NoXmlInFolder(String),
    NotADirectory(String),
    NotAnXmlFile(String),
    EmptyFile(String),
    CorruptedXmlFile(String),
}

impl fmt::Display for OwCodecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileExtensionExtraction(file_path) => {
                write!(
                    f,
                    "Error while extracting extension from the file: {}",
                    file_path
                )
            }
            Self::BlobOverflowError(file_path) => {
                write!(f, "Blob overflow error at: {}", file_path)
            }
            Self::NoXmlInFolder(dir_path) => {
                write!(f, "No xml files found at: {}", dir_path)
            }
            Self::NotADirectory(dir_path) => {
                write!(f, "{} is not a directory", dir_path)
            }
            Self::NotAnXmlFile(file_path) => {
                write!(f, "{} is not an xml file", file_path)
            }
            Self::EmptyFile(file_path) => {
                write!(f, "{} is an empty file", file_path)
            }
            Self::CorruptedXmlFile(file_path) => {
                write!(f, "{} is corrupted xml", file_path)
            }
        }
    }
}

impl Error for OwCodecError {}
