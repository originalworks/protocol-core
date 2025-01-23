mod errors;
mod schema;
mod validation;

pub use errors::*;
use regex::Regex;
pub use schema::*;
use serde_valid::json::{FromJsonReader, FromJsonStr};
use std::io::{BufRead, BufReader, Seek};

pub struct DdexParser;

impl DdexParser {
    pub fn from_json_string(str: &String) -> Result<NewReleaseMessage, ParserError> {
        NewReleaseMessage::from_json_str(&str).map_err(|e| match e {
            serde_valid::Error::DeserializeError(err) => ParserError::JSONParse {
                msg: err.to_string(),
                loc: loc!(),
            },
            serde_valid::Error::ValidationError(err) => ParserError::JSONValidate {
                msg: err.to_string(),
                loc: loc!(),
            },
        })
    }

    pub fn from_json_reader<R: std::io::Read>(reader: R) -> Result<NewReleaseMessage, ParserError> {
        NewReleaseMessage::from_json_reader(reader).map_err(|e| match e {
            serde_valid::Error::DeserializeError(err) => ParserError::JSONParse {
                msg: err.to_string(),
                loc: loc!(),
            },
            serde_valid::Error::ValidationError(err) => ParserError::JSONValidate {
                msg: err.to_string(),
                loc: loc!(),
            },
        })
    }

    pub fn from_xml_string(str: String) -> Result<NewReleaseMessage, ParserError> {
        let re = Regex::new(r"NewReleaseMessage").map_err(|e| ParserError::Regex {
            source: e,
            loc: loc!(),
        })?;

        let message_type = re.find(&str).ok_or_else(|| ParserError::DDEXHeader {
            msg: "Failed to identify message type".to_string(),
            loc: loc!(),
        })?;

        match message_type.as_str() {
            "NewReleaseMessage" => {
                let parsed: NewReleaseMessage =
                    yaserde::de::from_str(&str).map_err(|e| ParserError::XMLParse {
                        msg: e,
                        loc: loc!(),
                    })?;
                return Ok(parsed);
            }
            _ => Err(ParserError::DDEXHeader {
                msg: "Unsupported ddex message type".to_string(),
                loc: loc!(),
            })?,
        }
    }

    pub fn from_xml_file(path: &str) -> Result<NewReleaseMessage, ParserError> {
        let file = std::fs::File::open(path).map_err(|e| ParserError::Io {
            source: e,
            path: path.to_string(),
            loc: loc!(),
        })?;
        let mut reader = BufReader::new(file);

        let message_type_line = (&mut reader).lines().nth(1);

        if let Some(Ok(message_type_tag)) = message_type_line {
            let re = Regex::new(r"NewReleaseMessage").map_err(|e| ParserError::Regex {
                source: e,
                loc: loc!(),
            })?;
            let message_type =
                re.find(&message_type_tag)
                    .ok_or_else(|| ParserError::DDEXHeader {
                        msg: "Failed to identify message type".to_string(),
                        loc: loc!(),
                    })?;

            let _ = reader.rewind().map_err(|e| ParserError::XMLParse {
                msg: e.to_string(),
                loc: loc!(),
            });

            match message_type.as_str() {
                "NewReleaseMessage" => {
                    let parsed: NewReleaseMessage =
                        yaserde::de::from_reader(reader).map_err(|e| ParserError::XMLParse {
                            msg: e,
                            loc: loc!(),
                        })?;
                    return Ok(parsed);
                }
                _ => Err(ParserError::DDEXHeader {
                    msg: "Unsupported ddex message type".to_string(),
                    loc: loc!(),
                })?,
            }
        } else {
            Err(ParserError::DDEXHeader {
                msg: "Unable to detect message type".to_string(),
                loc: loc!(),
            })?
        }
    }
}
