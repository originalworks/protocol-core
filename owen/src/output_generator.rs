use crate::image_processor::optimize_image;
use crate::ipfs::IpfsManager;
use crate::logger::report_validation_error;
use crate::Config;
use anyhow::Context;
use blob_codec::BlobEstimator;
use ddex_parser::{DdexParser, NewReleaseMessage};
use log_macros::{format_error, log_info, log_warn};
use serde_json::json;
use serde_valid::json::ToJsonString;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DdexMessage {
    pub message_dir_path: String,
    input_xml_path: Option<String>,
    input_image_paths: Vec<String>,
    image_cids: Vec<String>,
    output_json_path: Option<String>,
    validated: bool,
    pub excluded: bool,
    pub reason: Option<String>,
    pub new_release_message: Option<NewReleaseMessage>,
}

impl DdexMessage {
    pub fn build(message_dir_path: PathBuf) -> anyhow::Result<Self> {
        let mut excluded = false;
        let mut reason = None;
        let mut input_xml_path = None;

        if message_dir_path.is_dir() == false {
            excluded = true;
            reason = Some("Message path is not a dir".to_string());
        } else {
            if let Some(ddex_xml_path_buf) = BlobEstimator::find_ddex_xml(message_dir_path.clone())?
            {
                input_xml_path = Some(ddex_xml_path_buf.to_string_lossy().to_string());
                sentry::configure_scope(|scope| {
                    scope.set_extra(
                        "filename",
                        json!(ddex_xml_path_buf
                            .to_string_lossy()
                            .to_string()
                            .split("/")
                            .last()
                            .expect("Invalid filename")),
                    );
                });
            } else {
                excluded = true;
                reason = Some("DDEX .xml file not found".to_string());
            }
        }

        Ok(Self {
            input_xml_path,
            input_image_paths: vec![],
            image_cids: vec![],
            output_json_path: None,
            message_dir_path: message_dir_path.to_string_lossy().to_string(),
            excluded,
            validated: false,
            reason,
            new_release_message: None,
        })
    }

    fn try_input_xml_path(&self) -> anyhow::Result<&String> {
        self.input_xml_path
            .as_ref()
            .ok_or_else(|| format_error!("Missing input_xml_path"))
    }
    fn try_output_json_path(&self) -> anyhow::Result<&String> {
        self.output_json_path
            .as_ref()
            .ok_or_else(|| format_error!("Missing output_json_path"))
    }

    pub fn validate(mut self) -> anyhow::Result<Self> {
        if self.excluded == true {
            return Ok(self);
        }
        let input_xml_file = self.try_input_xml_path()?.clone();
        log_info!("Parsing XML at {}", input_xml_file);
        self.new_release_message = match DdexParser::from_xml_file(&input_xml_file) {
            Ok(result) => Some(result),
            Err(err) => {
                log_warn!("XML parsing error = {}. Message skipped", err.to_string());
                report_validation_error(&err, &input_xml_file, true);
                self.reason = Some(err.to_string());
                self.excluded = true;
                return Ok(self);
            }
        };

        log_info!("Parsing JSON");
        let json_output = match self.new_release_message.to_json_string_pretty() {
            Ok(result) => result,
            Err(err) => {
                log_warn!("JSON parsing error = {}. Message skipped", err.to_string());
                report_validation_error(&err, &input_xml_file, true);
                self.reason = Some(err.to_string());
                self.excluded = true;
                return Ok(self);
            }
        };

        self.new_release_message = match DdexParser::from_json_string(&json_output) {
            Ok(result) => Some(result),
            Err(err) => {
                log_warn!("Vaidation error. Message skipped");
                report_validation_error(&err, &input_xml_file, false);
                self.reason = Some(err.to_string());
                self.excluded = true;
                return Ok(self);
            }
        };

        self.validated = true;

        Ok(self)
    }
    pub fn save_output_json(mut self, destination_dir: &String) -> anyhow::Result<Self> {
        if self.excluded || !self.validated {
            return Ok(self);
        }
        let message_dir_path = Path::new(&self.message_dir_path);
        let output_json_path = format!(
            "{}/{}.json",
            destination_dir,
            message_dir_path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format_error!(
                    "Wrong folder name: {}",
                    message_dir_path.to_string_lossy().to_string()
                ))?
        );
        let json_output = self.new_release_message.to_json_string_pretty()?;

        fs::write(&output_json_path, json_output)?;

        log_info!("Validated JSON saved at {}", &output_json_path);
        self.output_json_path = Some(output_json_path);
        self.new_release_message = None;
        Ok(self)
    }
}

pub struct OutputFilesGenerator<'a, 'b> {
    output_files_dir: String,
    input_files_dir: String,
    ipfs_manager: &'a IpfsManager<'b>,
}

impl<'a, 'b> OutputFilesGenerator<'a, 'b> {
    pub fn build(config: &Config, ipfs_manager: &'a IpfsManager<'b>) -> anyhow::Result<Self> {
        Ok(Self {
            ipfs_manager,
            output_files_dir: config.output_files_dir.clone(),
            input_files_dir: config.input_files_dir.clone(),
        })
    }

    fn prepare_folders(&self) -> anyhow::Result<()> {
        let output_files_dir_path = Path::new(&self.output_files_dir);
        if output_files_dir_path.is_dir() {
            fs::remove_dir_all(output_files_dir_path).with_context(|| {
                format_error!("Failed to remove dir at {}", {
                    output_files_dir_path.to_string_lossy().to_string()
                })
            })?;
        }
        fs::create_dir_all(output_files_dir_path).with_context(|| {
            format_error!("Failed to create dir at {}", {
                output_files_dir_path.to_string_lossy().to_string()
            })
        })?;

        let input_files_dir_path = Path::new(&self.input_files_dir);
        if input_files_dir_path.is_dir() == false {
            return Err(format_error!(
                "Provided folder_path is not a directory: {}",
                &self.input_files_dir
            ))?;
        }

        if fs::read_dir(input_files_dir_path)?.count() == 0 {
            return Err(format_error!(
                "Provided input folder is empty: {}",
                &self.input_files_dir
            ))?;
        }
        Ok(())
    }

    pub async fn generate_files(&self) -> anyhow::Result<Vec<DdexMessage>> {
        log_info!("Creating output files");
        self.prepare_folders()?;
        let mut result: Vec<DdexMessage> = Vec::new();

        let input_files_dir_read = fs::read_dir(&self.input_files_dir)?;

        for message_dir in input_files_dir_read {
            let mut ddex_message = DdexMessage::build(message_dir?.path())?;
            ddex_message = ddex_message.validate()?;
            ddex_message = self.pin_images(ddex_message).await?;
            ddex_message = ddex_message.save_output_json(&self.output_files_dir)?;
            result.push(ddex_message);
        }
        Self::print_output(&result)?;
        Ok(result)
    }

    async fn pin_images(&self, mut ddex_message: DdexMessage) -> anyhow::Result<DdexMessage> {
        if ddex_message.excluded || !ddex_message.validated {
            return Ok(ddex_message);
        }

        if let Some(new_release_message) = ddex_message.new_release_message.as_mut() {
            for image_resource in &mut new_release_message.resource_list.images {
                if let Some(technical_details) = image_resource.technical_details.get_mut(0) {
                    if let Some(file) = &mut technical_details.file {
                        let input_image_file =
                            format!("{}/{}", ddex_message.message_dir_path, file.uri);

                        let resized_image_path = match optimize_image(&input_image_file) {
                            Ok(res) => res,
                            Err(err) => {
                                log_warn!(
                                    "Error in optimizing image = {}. Message skipped",
                                    err.to_string()
                                );
                                ddex_message.excluded = true;
                                ddex_message.reason = Some(err.to_string());
                                return Ok(ddex_message);
                            }
                        };

                        let image_cid = match self.ipfs_manager.pin_file(&resized_image_path).await
                        {
                            Ok(res) => res,
                            Err(err) => {
                                log_warn!(
                                    "Error in uploading image to IPFS = {}. Message skipped",
                                    err.to_string()
                                );
                                ddex_message.excluded = true;
                                ddex_message.reason = Some(err.to_string());
                                return Ok(ddex_message);
                            }
                        };
                        ddex_message.image_cids.push(image_cid.clone());
                        ddex_message
                            .input_image_paths
                            .push(resized_image_path.clone());

                        file.uri = image_cid;
                    }
                }
            }
        }
        log_info!("Media files URIs have been replaced with CIDs");
        Ok(ddex_message)
    }

    fn print_output(output: &Vec<DdexMessage>) -> anyhow::Result<()> {
        for entry in output {
            if !entry.excluded {
                log_info!("-- PROCESSED DDEX MESSAGE");
                log_info!("-- Output file: {}", entry.try_output_json_path()?);
                log_info!("-- Input files:");
                log_info!("---- XML: {}", entry.try_input_xml_path()?);

                let len = entry.image_cids.len().max(entry.input_image_paths.len());

                if len > 0 {
                    for i in 0..len {
                        let cid = entry
                            .image_cids
                            .get(i)
                            .map(|s| s.as_str())
                            .unwrap_or("Missing!");
                        let img = entry
                            .input_image_paths
                            .get(i)
                            .map(|s| s.as_str())
                            .unwrap_or("Missing!");

                        log_info!("---- IMG {}: {}", i + 1, img);
                        log_info!("---- CID {}: {}", i + 1, cid);
                    }
                } else {
                    log_info!("---- No more inputs");
                }
            } else {
                log_warn!("!!! REJECTED DDEX MESSAGE");
                log_warn!(
                    "!!! Rejected folder path: {}; Rejected XML file: {}",
                    entry.message_dir_path,
                    entry.input_xml_path.as_deref().unwrap_or("")
                );

                if let Some(rejection_reason) = &entry.reason {
                    log_warn!("!!! Rejection reason: {}", rejection_reason);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use ow_wallet::{OwWallet, OwWalletConfig};
    use pretty_env_logger;
    use std::str::FromStr;

    // !! THEY WILL WORK BETTER WITH IPFS NODE RUNNING
    // closest docker compose is in local_setup/docker

    fn ddex_contains_all_cids(ddex_message: &DdexMessage) -> anyhow::Result<bool> {
        let file_content = fs::read_to_string(ddex_message.try_output_json_path()?)?;

        let all_found = ddex_message
            .image_cids
            .iter()
            .all(|cid| file_content.contains(cid));

        Ok(all_found)
    }

    #[test]
    #[ignore]
    fn print_output_test() {
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Info)
            .try_init();

        let msg1 = DdexMessage {
            excluded: false,
            image_cids: vec!["ABC".to_string(), "ABC".to_string(), "ABC".to_string()],
            input_image_paths: vec!["IMAGE_PATH".to_string(), "IMAGE_PATH".to_string()],
            input_xml_path: Some("XML_PATH".to_string()),
            message_dir_path: "DIR_PATH".to_string(),
            new_release_message: None,
            output_json_path: Some("JSON_PATH".to_string()),
            validated: true,
            reason: None,
        };

        let msg2 = DdexMessage {
            excluded: false,
            image_cids: vec![],
            input_image_paths: vec![],
            input_xml_path: Some("XML_PATH".to_string()),
            message_dir_path: "DIR_PATH".to_string(),
            new_release_message: None,
            output_json_path: Some("JSON_PATH".to_string()),
            validated: true,
            reason: None,
        };

        let _ = OutputFilesGenerator::print_output(&vec![msg1, msg2]);
        ()
    }
    #[tokio::test]
    async fn create_output_files_with_cids() -> anyhow::Result<()> {
        let config = Config {
            rpc_url: String::new(),
            private_key: Some(
                "0x4f8a43547b061fd473aad90fee77a86fb8abf9e30d2107a1b1d844fcb12a009c".to_string(),
            ),
            input_files_dir: String::from_str("./tests").unwrap(),
            local_ipfs: true,
            output_files_dir: "./output_files".to_string(),
            environment: String::from_str("dev").unwrap(),
            username: String::from_str("user").unwrap(),
            ddex_sequencer_address: Address::ZERO,
            disable_telemetry: true,
            storacha_bridge_url: "ABC".to_string(),
            ipfs_api_base_url: "http://127.0.0.1:5001".to_string(),
            use_kms: false,
            signer_kms_id: None,
            use_batch_sender: false,
        };
        let ow_wallet_config = OwWalletConfig::from(&config)?;
        let ow_wallet = OwWallet::build(&ow_wallet_config).await?;
        let ipfs_manager = IpfsManager::build(&config, &ow_wallet).await?;
        let output_files_generator = OutputFilesGenerator::build(&config, &ipfs_manager)?;
        let ddex_messages = output_files_generator.generate_files().await?;

        assert_eq!(
            ddex_messages.len(),
            8,
            "Wrong processed ddex messages count"
        );

        let excluded = ddex_messages.iter().filter(|msg| msg.excluded).count();
        assert_eq!(excluded, 4, "Wrong excluded ddex messages count");

        let included: Vec<&DdexMessage> =
            ddex_messages.iter().filter(|msg| !msg.excluded).collect();

        assert_eq!(included.len(), 4, "Wrong included ddex messages count");

        let entries = fs::read_dir(&config.output_files_dir)?;
        let output_count = entries
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                    && e.path()
                        .extension()
                        .map(|ext| ext == "json")
                        .unwrap_or(false)
            })
            .count();

        assert_eq!(output_count, 4, "Wrong output files count");

        for ddex_message in included {
            assert!(ddex_contains_all_cids(ddex_message)?);
        }

        Ok(())
    }

    #[should_panic]
    #[tokio::test]
    async fn error_when_empty_directory() {
        let config = Config {
            rpc_url: String::new(),
            private_key: Some(
                "0x4f8a43547b061fd473aad90fee77a86fb8abf9e30d2107a1b1d844fcb12a009c".to_string(),
            ),
            input_files_dir: String::from_str("./tests/empty_dir").unwrap(),
            local_ipfs: true,
            output_files_dir: "./output_files".to_string(),
            environment: String::from_str("dev").unwrap(),
            username: String::from_str("user").unwrap(),
            ddex_sequencer_address: Address::ZERO,
            disable_telemetry: true,
            storacha_bridge_url: "ABC".to_string(),
            ipfs_api_base_url: "http://127.0.0.1:5001".to_string(),
            use_kms: false,
            signer_kms_id: None,
            use_batch_sender: false,
        };
        fs::create_dir_all(&config.input_files_dir).unwrap();
        let ow_wallet_config = OwWalletConfig::from(&config).unwrap();
        let ow_wallet = OwWallet::build(&ow_wallet_config).await.unwrap();
        let ipfs_manager = IpfsManager::build(&config, &ow_wallet).await.unwrap();
        let output_files_generator = OutputFilesGenerator::build(&config, &ipfs_manager).unwrap();
        let _ = output_files_generator.generate_files().await.unwrap();

        fs::remove_dir_all(&config.input_files_dir).unwrap();
        ()
    }
}
