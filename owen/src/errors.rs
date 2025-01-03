use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum OwenCliError {
    MissingEnvVar(String),
    MissingCliArg(String),
    InvalidBlobProof(),
    SourcePathIsNotDir(String),
    EmptySourcePathFolder(String),
    InvalidAssetFolderName(String),
}

impl fmt::Display for OwenCliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingEnvVar(var_name) => {
                write!(f, "Missing variable in .env file: {}", var_name)
            }
            Self::MissingCliArg(arg_name) => {
                write!(f, "Missing command line argument: {}", arg_name)
            }
            Self::InvalidBlobProof() => {
                write!(f, "c_kzg error during proof validation")
            }
            Self::SourcePathIsNotDir(path) => {
                write!(f, "Provided folder_path is not a directory: {}", path)
            }
            Self::EmptySourcePathFolder(path) => {
                write!(f, "Folder under provided folder_path is empty: {}", path)
            }
            Self::InvalidAssetFolderName(folder_name) => {
                write!(f, "Wrong folder name: {}", folder_name)
            }
        }
    }
}

impl Error for OwenCliError {}
