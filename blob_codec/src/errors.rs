#[derive(thiserror::Error, Debug)]
pub enum OwCodecError {
    #[error("Blob overflow error at: {path}")]
    BlobOverflow { path: String, loc: String },
    #[error("Error while reading a file at {path} - {source}")]
    Io {
        #[source]
        source: std::io::Error,
        path: String,
        loc: String,
    },
    #[error("No files found at: {path}")]
    EmptyDirectory { path: String, loc: String },
    #[error("{path} is not a directory")]
    NotADirectory { path: String, loc: String },
    #[error("{path} is not a file")]
    NotAFile { path: String, loc: String },
    #[error("{path} is an empty file")]
    EmptyFile { path: String, loc: String },
    #[error("Error while decompressing the blob: {msg}")]
    Decompress { msg: String, loc: String },
}
