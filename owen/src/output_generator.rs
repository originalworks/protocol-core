use crate::errors::OwenCliError;
use crate::ipfs::{pin_file_ipfs_kubo, pin_file_pinata};
use crate::{Config, IpfsInterface};
use ddex_schema::{ddex_parse_xml_file, DdexMessage};
use serde_valid::json::ToJsonString;
use std::fs;
use std::path::PathBuf;
use std::{error::Error, path::Path};

#[derive(Debug)]
pub struct MessageDirProcessingContext {
    message_dir_path: String,
    input_xml_path: String,
    input_image_path: String,
    image_cid: String,
    output_json_path: String,
    empty: bool,
}

async fn attach_cid_and_save(
    message_dir_processing_context: &mut MessageDirProcessingContext,
    ddex_message: DdexMessage,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    match ddex_message {
        DdexMessage::NewRelease(mut new_release_message) => {
            for image_resource in &mut new_release_message.resource_list.images {
                if let Some(technical_details) = image_resource.technical_details.get_mut(0) {
                    if let Some(file) = &mut technical_details.file {
                        let input_image_file = format!(
                            "{}/{}",
                            message_dir_processing_context.message_dir_path, file.uri
                        );
                        message_dir_processing_context.input_image_path = input_image_file;
                        let file_uri =
                            pin_file(&message_dir_processing_context.input_image_path, &config)
                                .await?;

                        message_dir_processing_context.image_cid = file_uri.clone();
                        file.uri = file_uri;
                    }
                }
            }

            let json_output = new_release_message.to_json_string_pretty()?;
            fs::write(
                &message_dir_processing_context.output_json_path,
                json_output,
            )?;
        }
    }

    Ok(())
}

fn is_xml_file_empty(file_path: &Path) -> Result<bool, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content.trim().is_empty())
}

async fn pin_file(path: &String, config: &Config) -> Result<String, Box<dyn Error>> {
    if config.default_ipfs_interface == IpfsInterface::KUBO {
        Ok(pin_file_ipfs_kubo(path).await?)
    } else {
        Ok(pin_file_pinata(path, &config.pinata_jwt).await?)
    }
}

async fn process_message_folder(
    message_folder_path: PathBuf,
    config: &Config,
) -> Result<MessageDirProcessingContext, Box<dyn Error>> {
    let mut message_dir_processing_context = MessageDirProcessingContext {
        input_xml_path: String::new(),
        input_image_path: String::new(),
        output_json_path: String::new(),
        image_cid: String::new(),
        message_dir_path: String::new(),
        empty: true,
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

                    message_dir_processing_context.output_json_path = format!(
                        "{}/{}.json",
                        &config.output_files_dir,
                        &message_folder_path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .ok_or_else(|| {
                                Box::new(OwenCliError::InvalidAssetFolderName(
                                    message_folder_path.to_string_lossy().to_string(),
                                ))
                            })?
                    );

                    message_dir_processing_context.empty = false;
                    let ddex_message: DdexMessage =
                        ddex_parse_xml_file(&message_dir_processing_context.input_xml_path)?;
                    attach_cid_and_save(&mut message_dir_processing_context, ddex_message, &config)
                        .await?;
                }
            }
        }
    }
    Ok(message_dir_processing_context)
}

pub async fn create_output_files(
    config: &Config,
) -> Result<Vec<MessageDirProcessingContext>, Box<dyn Error>> {
    let mut result: Vec<MessageDirProcessingContext> = Vec::new();
    let output_files_path = Path::new(&config.output_files_dir);
    if output_files_path.is_dir() {
        fs::remove_dir_all(output_files_path)?;
    }
    fs::create_dir_all(output_files_path)?;
    let input_folder_path = Path::new(&config.folder_path);
    let mut empty_root_folder = true;

    if input_folder_path.is_dir() {
        let message_folders = fs::read_dir(input_folder_path)?;

        for message_folder in message_folders {
            let message_folder_path = message_folder?.path();
            let message_dir_processing_context =
                process_message_folder(message_folder_path, &config).await?;
            if !message_dir_processing_context.empty {
                result.push(message_dir_processing_context);
                empty_root_folder = false;
            }
        }
    } else {
        return Err(Box::new(OwenCliError::SourcePathIsNotDir(
            input_folder_path.to_string_lossy().to_string(),
        )));
    }
    if empty_root_folder {
        return Err(Box::new(OwenCliError::EmptySourcePathFolder(
            config.folder_path.to_string(),
        )));
    }

    print_output(&result)?;
    Ok(result)
}

fn print_output(output: &Vec<MessageDirProcessingContext>) -> Result<(), Box<dyn Error>> {
    for entry in output {
        println!("PROCESSED DDEX MESSAGE:");
        println!(
            "Source files: image: {}; XML: {}",
            entry.input_image_path, entry.input_xml_path
        );
        println!(
            "Image file {} was pined to IPFS under CID: {}",
            entry.input_image_path, entry.image_cid
        );
        println!(
            "CID: {} was included in the output file: {}",
            entry.image_cid, entry.output_json_path
        );
        println!("----------")
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn find_cid_in_file(
        processing_context: &MessageDirProcessingContext,
    ) -> Result<bool, Box<dyn Error>> {
        let file = fs::read_to_string(&processing_context.output_json_path)?;
        let found = file.contains(&processing_context.image_cid);

        Ok(found)
    }

    #[tokio::test]
    async fn create_output_files_with_cids() -> Result<(), Box<dyn Error>> {
        let config = Config {
            rpc_url: String::new(),
            private_key: String::new(),
            folder_path: String::from_str("./tests").unwrap(),
            default_ipfs_interface: IpfsInterface::KUBO,
            ipfs_kubo_url: String::from_str("http://localhost:5001").unwrap(),
            pinata_jwt: String::new(),
            max_fee_per_gas: 1,
            output_files_dir: "./output_files".to_string(),
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
            max_fee_per_gas: 1,
            output_files_dir: "./output_files".to_string(),
        };
        fs::create_dir_all(&config.folder_path).unwrap();

        create_output_files(&config).await.unwrap();
        fs::remove_dir_all(&config.folder_path).unwrap();
        ()
    }
}
