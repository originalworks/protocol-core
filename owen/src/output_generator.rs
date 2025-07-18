use crate::image_processor::optimize_image;
use crate::ipfs::IpfsManager;
use crate::logger::report_validation_error;
use crate::Config;
use anyhow::Context;
use ddex_parser::{DdexParser, NewReleaseMessage};
use log_macros::{format_error, log_info, log_warn};
use serde_json::json;
use serde_valid::json::ToJsonString;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct MessageDirProcessingContext {
    pub message_dir_path: String,
    input_xml_path: String,
    input_image_path: String,
    image_cid: String,
    output_json_path: String,
    pub excluded: bool,
    pub reason: Option<String>,
}

// fn generate_iscc_code_for_file(file_path: &str) -> anyhow::Result<String> {
//     // Call `idk create file_path`
//     let output = Command::new("idk").arg("create").arg(file_path).output()?;

//     // Check exit status
//     if !output.status.success() {
//         return Err(anyhow::anyhow!(
//             "idk create failed with status: {:?}",
//             output.status.code()
//         ));
//     }

//     // Convert stdout to String and parse as JSON
//     let stdout_str = String::from_utf8(output.stdout)?;
//     let parsed_json: Value = serde_json::from_str(&stdout_str)
//         .map_err(|e| anyhow::anyhow!("Failed to parse idk create output as JSON: {}", e))?;

//     // Extract the "iscc" field
//     if let Some(iscc_code) = parsed_json.get("iscc").and_then(|val| val.as_str()) {
//         Ok(iscc_code.to_string())
//     } else {
//         Err(anyhow::anyhow!(
//             "No 'iscc' field found in idk create output"
//         ))
//     }
// }

async fn pin_and_write_cid(
    message_dir_processing_context: &mut MessageDirProcessingContext,
    new_release_message: &mut NewReleaseMessage,
    ipfs_manager: &IpfsManager,
) -> anyhow::Result<()> {
    for image_resource in &mut new_release_message.resource_list.images {
        if let Some(technical_details) = image_resource.technical_details.get_mut(0) {
            if let Some(file) = &mut technical_details.file {
                let input_image_file = format!(
                    "{}/{}",
                    message_dir_processing_context.message_dir_path, file.uri
                );

                let resized_image_path = optimize_image(&input_image_file)?;

                message_dir_processing_context.input_image_path = resized_image_path.clone();

                let file_uri = ipfs_manager.pin_file(&resized_image_path).await?;

                message_dir_processing_context.image_cid = file_uri.clone();
                file.uri = file_uri;
            }
        }
    }

    // 2) HANDLE AUDIO FILES
    // Within pin_and_write_cid or wherever you need to handle audio:
    /*
    // Iterate over all SoundRecordings in the ResourceList
    for sound_recording in &mut new_release_message.resource_list.sound_recordings {
        // If there's a SoundRecordingEdition (or iterate if it's a Vec)
        if let Some(edition) = &mut sound_recording.sound_recording_edition {
            // Go through each TechnicalDetails
            for tech_detail in &mut edition.technical_details {
                // If DeliveryFile is present
                if let Some(delivery_file) = &mut tech_detail.delivery_file {
                    // If there's a File node
                    if let Some(file_details) = &mut delivery_file.file {
                        // Finally, get the URI string
                        let audio_uri = &file_details.uri;

                        // Build the local path from folder + URI
                        let full_path = format!(
                            "{}/{}",
                            message_dir_processing_context.message_dir_path,
                            audio_uri
                        );

                        // Now do your processing:
                        // e.g., generate ISCC code or pin the file to IPFS
                        match generate_iscc_code_for_file(&full_path) {
                            Ok(iscc_code) => {
                                log_info!("ISCC code for {} is {}", full_path, iscc_code);
                                // do whatever you need with `iscc_code`
                            }
                            Err(e) => {
                                log_warn!("Failed to generate ISCC code for {}: {}", full_path, e);
                            }
                        }
                    }
                }
            }
        }
    }
    */

    Ok(())
}

fn is_xml_file_empty(file_path: &Path) -> anyhow::Result<bool> {
    let content = fs::read_to_string(file_path)?;
    Ok(content.trim().is_empty())
}

async fn process_message_folder(
    message_folder_path: PathBuf,
    ipfs_manager: &IpfsManager,
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
                            report_validation_error(
                                &err,
                                &message_dir_processing_context.input_xml_path.to_string(),
                                true,
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
                            report_validation_error(
                                &err,
                                &message_dir_processing_context.input_xml_path.to_string(),
                                true,
                            );
                            message_dir_processing_context.reason = Some(err.to_string());
                            return Ok(message_dir_processing_context);
                        }
                    };

                    new_release_message = match DdexParser::from_json_string(&json_output) {
                        Ok(result) => result,
                        Err(err) => {
                            report_validation_error(
                                &err,
                                &message_dir_processing_context.input_xml_path.to_string(),
                                false,
                            );
                            message_dir_processing_context.reason = Some(err.to_string());
                            return Ok(message_dir_processing_context);
                        }
                    };
                    let pinning_result = pin_and_write_cid(
                        &mut message_dir_processing_context,
                        &mut new_release_message,
                        &ipfs_manager,
                    )
                    .await;

                    match pinning_result {
                        Err(err) => {
                            message_dir_processing_context.reason = Some(err.to_string());
                            return Ok(message_dir_processing_context);
                        }
                        _ => (),
                    }

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

        let ipfs_manager = IpfsManager::build(config).await?;

        for message_folder in message_folders {
            empty_root_folder = false;
            let message_folder_path = message_folder?.path();
            let message_dir_processing_context =
                process_message_folder(message_folder_path, &ipfs_manager, &config).await?;
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
            local_ipfs: true,
            output_files_dir: "./output_files".to_string(),
            environment: String::from_str("dev").unwrap(),
            username: String::from_str("user").unwrap(),
            ddex_sequencer_address: Address::ZERO,
            disable_telemetry: true,
            storacha_bridge_url: "ABC".to_string(),
            ipfs_api_base_url: "ABC".to_string(),
            use_kms: false,
            signer_kms_id: None,
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
            local_ipfs: true,
            output_files_dir: "./output_files".to_string(),
            environment: String::from_str("dev").unwrap(),
            username: String::from_str("user").unwrap(),
            ddex_sequencer_address: Address::ZERO,
            disable_telemetry: true,
            storacha_bridge_url: "ABC".to_string(),
            ipfs_api_base_url: "ABC".to_string(),
            use_kms: false,
            signer_kms_id: None,
        };
        fs::create_dir_all(&config.folder_path).unwrap();

        create_output_files(&config).await.unwrap();
        fs::remove_dir_all(&config.folder_path).unwrap();
        ()
    }
}
