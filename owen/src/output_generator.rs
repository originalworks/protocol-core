use crate::ipfs::{pin_file_ipfs_kubo, pin_file_pinata};
use crate::{Config, IpfsInterface};
use anyhow::Context;
use ddex_schema::{DdexParser, NewReleaseMessage};
use log_macros::{format_error, log_info, log_warn};
use sentry::protocol::Attachment;
use serde_json::json;
use serde_valid::json::ToJsonString;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct MessageDirProcessingContext {
    pub message_dir_path: String,
    input_xml_path: String,
    input_image_path: String,
    image_cid: String,
    output_json_path: String,
    pub excluded: bool,
    reason: Option<String>,
}

async fn pin_and_write_cid(
    message_dir_processing_context: &mut MessageDirProcessingContext,
    new_release_message: &mut NewReleaseMessage,
    config: &Config,
) -> anyhow::Result<()> {
    for image_resource in &mut new_release_message.resource_list.images {
        if let Some(technical_details) = image_resource.technical_details.get_mut(0) {
            if let Some(file) = &mut technical_details.file {
                let input_image_file = format!(
                    "{}/{}",
                    message_dir_processing_context.message_dir_path, file.uri
                );
                message_dir_processing_context.input_image_path = input_image_file;
                let file_uri =
                    pin_file(&message_dir_processing_context.input_image_path, &config).await?;

                message_dir_processing_context.image_cid = file_uri.clone();
                file.uri = file_uri;
            }
        }
    }

    Ok(())
}

fn is_xml_file_empty(file_path: &Path) -> anyhow::Result<bool> {
    let content = fs::read_to_string(file_path)?;
    Ok(content.trim().is_empty())
}

async fn pin_file(path: &String, config: &Config) -> anyhow::Result<String> {
    if config.default_ipfs_interface == IpfsInterface::KUBO {
        Ok(pin_file_ipfs_kubo(path).await?)
    } else {
        Ok(pin_file_pinata(path, &config.pinata_jwt).await?)
    }
}

fn add_attachment(xml_input_path: &String) -> () {
    let mut buffer: Vec<u8> = Vec::new();
    let attachment_added = std::fs::File::open(xml_input_path)
        .ok()
        .and_then(|mut f| f.read_to_end(&mut buffer).ok())
        .and_then(|_| {
            sentry::configure_scope(|scope| {
                scope.add_attachment(Attachment {
                    filename: xml_input_path
                        .split("/")
                        .last()
                        .unwrap_or_else(|| "unknown")
                        .to_string(),
                    buffer,
                    content_type: Some("text/xml".to_string()),
                    ..Default::default()
                });
            });
            Some(())
        });
    if attachment_added.is_none() {
        log_warn!("Failed to add attachment");
    }
}

async fn process_message_folder(
    message_folder_path: PathBuf,
    config: &Config,
) -> anyhow::Result<MessageDirProcessingContext> {
    let mut message_dir_processing_context = MessageDirProcessingContext {
        input_xml_path: String::new(),
        input_image_path: String::new(),
        output_json_path: String::new(),
        image_cid: String::new(),
        message_dir_path: message_folder_path.to_string_lossy().to_string(),
        excluded: true,
        reason: None,
    };
    if message_folder_path.is_dir() {
        let message_files = fs::read_dir(&message_folder_path)?;

        for message_file in message_files {
            let message_file_path = message_file?.path();
            if message_file_path.is_dir() == false {
                let kind = match infer::get_from_path(&message_file_path)? {
                    Some(v) => v,
                    None => continue,
                };
                if kind.extension() == "xml" && is_xml_file_empty(&message_file_path)? == false {
                    message_dir_processing_context.message_dir_path =
                        message_folder_path.to_string_lossy().to_string();
                    message_dir_processing_context.input_xml_path =
                        message_file_path.to_string_lossy().to_string();

                    sentry::configure_scope(|scope| {
                        scope.set_extra(
                            "filename",
                            json!(message_file_path
                                .to_string_lossy()
                                .to_string()
                                .split("/")
                                .last()
                                .expect("Invalid filename")),
                        );
                    });

                    message_dir_processing_context.output_json_path = format!(
                        "{}/{}.json",
                        &config.output_files_dir,
                        &message_folder_path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .ok_or_else(|| format_error!(
                                "Wrong folder name: {}",
                                message_folder_path.to_string_lossy().to_string()
                            ))?
                    );

                    log_info!(
                        "Parsing XML at {}",
                        &message_dir_processing_context.input_xml_path.to_string()
                    );

                    let mut new_release_message = match DdexParser::from_xml_file(
                        &message_dir_processing_context.input_xml_path,
                    ) {
                        Ok(result) => result,
                        Err(err) => {
                            log_warn!("XML parsing error");
                            add_attachment(
                                &message_dir_processing_context.input_xml_path.to_string(),
                            );
                            message_dir_processing_context.reason = Some(err.to_string());
                            return Ok(message_dir_processing_context);
                        }
                    };

                    log_info!("Parsing JSON");
                    let mut json_output = match new_release_message.to_json_string_pretty() {
                        Ok(result) => result,
                        Err(err) => {
                            log_warn!("JSON parsing error");
                            add_attachment(
                                &message_dir_processing_context.input_xml_path.to_string(),
                            );
                            message_dir_processing_context.reason = Some(err.to_string());
                            return Ok(message_dir_processing_context);
                        }
                    };

                    new_release_message = match DdexParser::from_json_string(&json_output) {
                        Ok(result) => result,
                        Err(err) => {
                            add_attachment(
                                &message_dir_processing_context.input_xml_path.to_string(),
                            );
                            message_dir_processing_context.reason = Some(err.to_string());
                            return Ok(message_dir_processing_context);
                        }
                    };
                    pin_and_write_cid(
                        &mut message_dir_processing_context,
                        &mut new_release_message,
                        &config,
                    )
                    .await?;

                    json_output = new_release_message.to_json_string_pretty()?;
                    message_dir_processing_context.excluded = false;
                    log_info!("Media files URIs have been replaced with CIDs");

                    fs::write(
                        &message_dir_processing_context.output_json_path,
                        json_output,
                    )?;

                    log_info!(
                        "Validated JSON saved at {}",
                        &message_dir_processing_context.output_json_path
                    )
                }
            }
        }
    } else {
        message_dir_processing_context.reason =
            Some("Message folder path is not a dir".to_string());
    }
    Ok(message_dir_processing_context)
}

pub async fn create_output_files(
    config: &Config,
) -> anyhow::Result<Vec<MessageDirProcessingContext>> {
    log_info!("Creating output files");
    let mut result: Vec<MessageDirProcessingContext> = Vec::new();
    let output_files_path = Path::new(&config.output_files_dir);
    if output_files_path.is_dir() {
        fs::remove_dir_all(output_files_path).with_context(|| {
            format_error!("Failed to remove dir at {}", {
                output_files_path.to_string_lossy().to_string()
            })
        })?;
    }
    fs::create_dir_all(output_files_path).with_context(|| {
        format_error!("Failed to create dir at {}", {
            output_files_path.to_string_lossy().to_string()
        })
    })?;

    let input_folder_path = Path::new(&config.folder_path);
    let mut empty_root_folder = true;

    if input_folder_path.is_dir() {
        let message_folders = fs::read_dir(input_folder_path).with_context(|| {
            format_error!(
                "Failed to read dir at {}",
                input_folder_path.to_string_lossy().to_string(),
            )
        })?;

        for message_folder in message_folders {
            empty_root_folder = false;
            let message_folder_path = message_folder?.path();
            let message_dir_processing_context =
                process_message_folder(message_folder_path, &config).await?;
            result.push(message_dir_processing_context);
        }
    } else {
        return Err(format_error!(
            "Provided folder_path is not a directory: {}",
            input_folder_path.to_string_lossy().to_string()
        ))?;
    }
    if empty_root_folder {
        return Err(format_error!(
            "Folder under provided folder_path is empty: {}",
            config.folder_path.to_string()
        ))?;
    }

    print_output(&result)?;
    Ok(result)
}

fn print_output(output: &Vec<MessageDirProcessingContext>) -> anyhow::Result<()> {
    for entry in output {
        if !entry.excluded {
            log_info!("-- PROCESSED DDEX MESSAGE");
            log_info!(
                "-- Source files: image: {}; XML: {}",
                entry.input_image_path,
                entry.input_xml_path
            );
            log_info!(
                "-- Image file {} was pinned to IPFS under CID: {}",
                entry.input_image_path,
                entry.image_cid
            );
            log_info!(
                "-- CID: {} was included in the output file: {}",
                entry.image_cid,
                entry.output_json_path
            );
        } else {
            log_warn!("!!! REJECTED DDEX MESSAGE");
            log_warn!(
                "!!! Rejected folder path: {}; Rejected XML file: {}",
                entry.message_dir_path,
                entry.input_xml_path
            );

            if let Some(rejection_reason) = &entry.reason {
                log_warn!("!!! Rejection reason: {}", rejection_reason);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy::primitives::Address;

    use super::*;

    fn find_cid_in_file(processing_context: &MessageDirProcessingContext) -> anyhow::Result<bool> {
        let file = fs::read_to_string(&processing_context.output_json_path)?;
        let found = file.contains(&processing_context.image_cid);

        Ok(found)
    }

    #[tokio::test]
    async fn create_output_files_with_cids() -> anyhow::Result<()> {
        let config = Config {
            rpc_url: String::new(),
            private_key: String::new(),
            folder_path: String::from_str("./tests").unwrap(),
            default_ipfs_interface: IpfsInterface::KUBO,
            ipfs_kubo_url: String::from_str("http://localhost:5001").unwrap(),
            pinata_jwt: String::new(),
            output_files_dir: "./output_files".to_string(),
            environment: String::from_str("dev").unwrap(),
            username: String::from_str("user").unwrap(),
            ddex_sequencer_address: Address::ZERO,
        };
        let processing_context_vec = create_output_files(&config).await?;

        let processed_count = processing_context_vec.len();

        assert_eq!(
            processing_context_vec.len(),
            4,
            "Wrong output size. Expected 2, got: {processed_count}"
        );

        for processing_context in processing_context_vec {
            assert!(find_cid_in_file(&processing_context)?);
        }

        Ok(())
    }

    #[should_panic]
    #[tokio::test]
    async fn error_when_empty_directory() {
        let config = Config {
            rpc_url: String::new(),
            private_key: String::new(),
            folder_path: String::from_str("./tests/empty_dir").unwrap(),
            default_ipfs_interface: IpfsInterface::KUBO,
            ipfs_kubo_url: String::new(),
            pinata_jwt: String::new(),
            output_files_dir: "./output_files".to_string(),
            environment: String::from_str("dev").unwrap(),
            username: String::from_str("user").unwrap(),
            ddex_sequencer_address: Address::ZERO,
        };
        fs::create_dir_all(&config.folder_path).unwrap();

        create_output_files(&config).await.unwrap();
        fs::remove_dir_all(&config.folder_path).unwrap();
        ()
    }
}
