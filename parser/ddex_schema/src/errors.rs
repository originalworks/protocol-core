macro_rules! loc {
    () => {
        format!("{}:{}", file!(), line!())
    };
}

pub(crate) use loc;

#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error("Error while reading a file at {path} - {source}")]
    Io {
        source: std::io::Error,
        path: String,
        loc: String,
    },
    #[error("Error while compiling regex: {source}")]
    Regex { source: regex::Error, loc: String },
    #[error("Error while identifying message header {msg}")]
    DDEXHeader { msg: String, loc: String },
    #[error("Error while parsing/validating XML: {msg}")]
    XMLParse { msg: String, loc: String },
    #[error("Error while parsing JSON: {msg}")]
    JSONParse { msg: String, loc: String },
    #[error("Error while validating JSON: {msg}")]
    JSONValidate { msg: String, loc: String },
}
