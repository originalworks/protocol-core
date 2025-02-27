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
    #[error("Error(s) while validating JSON:\n   - {msg}")]
    JSONValidate { msg: String, loc: String },
}

pub trait CompactErrors {
    fn compact_errors(&self) -> Vec<String>;
}

impl CompactErrors for serde_valid::validation::Errors {
    fn compact_errors(&self) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        let path = "root".to_string();

        self.inner_compact_errors(path, &mut results);
        results
    }
}

pub trait InnerCompactErrors {
    fn inner_compact_errors(&self, path: String, results: &mut Vec<String>);
}

impl InnerCompactErrors for serde_valid::validation::Errors {
    fn inner_compact_errors(&self, path: String, results: &mut Vec<String>) {
        match self {
            serde_valid::validation::Errors::Object(obj_errs) => {
                for (key, sub_error) in &obj_errs.properties {
                    sub_error.inner_compact_errors(format!("{}.{}", path, key), results);
                }
            }
            serde_valid::validation::Errors::Array(arr_errs) => {
                for (index, sub_error) in &arr_errs.items {
                    sub_error.inner_compact_errors(format!("{}[{}]", path, index), results);
                }
            }
            serde_valid::validation::Errors::NewType(new_type_errs) => {
                for err in new_type_errs {
                    results.push(format!("{} = {}", path.clone(), err.to_string()));
                }
            }
        }
    }
}
