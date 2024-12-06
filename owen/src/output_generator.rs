use crate::constants::OUTPUT_FILES_DIR;
use crate::errors::OwenCliError;
use crate::ipfs::{pin_file_ipfs_kubo, pin_file_pinata};
use crate::{Config, IpfsInterface};
use ddex_schema::{
    ddex_parse_xml_file, DdexMessage, File, Image, ImageType, TechnicalImageDetails,
};
use serde_valid::json::ToJsonString;
use std::fs;
use std::path::PathBuf;
use std::{error::Error, path::Path};

#[derive(Debug)]
pub struct MessageDirProcessingContext {
    input_xml_path: String,
    input_image_path: String,
    image_cid: String,
    image_kind: String,
    output_json_path: String,
    empty: bool,
}

async fn attach_cid_and_save(input: &MessageDirProcessingContext) -> Result<(), Box<dyn Error>> {
    let mut ddex_message = ddex_parse_xml_file(&input.input_xml_path).unwrap();
    match ddex_message {
        DdexMessage::NewRelease(ref mut value) => {
            let image: Image = Image {
                resource_reference: "IPFS image file CID".to_string(),
                kind: ImageType {
                    content: input.image_kind.clone(),
                    namespace: None,
                    user_defined_value: None,
                },
                resource_ids: vec![],
                parental_warning_types: vec![],
                technical_detailss: vec![TechnicalImageDetails {
                    technical_resource_details_reference: "IPFS image file CID".to_string(),
                    file: Some(File {
                        uri: input.image_cid.clone(),
                    }),
                    fingerprints: vec![],
                }],
            };
            value.resource_list.images.push(image);

            let json_output = value.to_json_string_pretty().unwrap();
            fs::write(&input.output_json_path, json_output).unwrap();
        }
    }

    Ok(())
}

async fn pin_file(path: &String, config: &Config) -> Result<String, Box<dyn Error>> {
    if config.default_ipfs_interface == IpfsInterface::KUBO {
        Ok(pin_file_ipfs_kubo(path).await?)
    } else {
        Ok(pin_file_pinata(path, &config.pinata_jwt).await?)
    }
}

async fn process_asset_folder(
    asset_folder_path: PathBuf,
    config: &Config,
) -> Result<MessageDirProcessingContext, Box<dyn Error>> {
    let mut file_processing_context = MessageDirProcessingContext {
        input_xml_path: String::new(),
        input_image_path: String::new(),
        output_json_path: String::new(),
        image_cid: String::new(),
        image_kind: String::new(),
        empty: true,
    };
    if asset_folder_path.is_dir() {
        let asset_files = fs::read_dir(&asset_folder_path)?;

        for asset_file in asset_files {
            let asset_path = asset_file?.path();
            if asset_path.is_dir() == false {
                let kind = match infer::get_from_path(&asset_path)? {
                    Some(v) => v,
                    None => continue,
                };

                if kind.mime_type().starts_with("image/") {
                    file_processing_context.input_image_path =
                        asset_path.to_string_lossy().to_string();
                    file_processing_context.image_cid =
                        pin_file(&file_processing_context.input_image_path, config).await?;
                    file_processing_context.image_kind = kind.mime_type().to_string();
                }
                if kind.extension() == "xml" {
                    file_processing_context.input_xml_path =
                        asset_path.to_string_lossy().to_string();
                }
            }
        }

        if !file_processing_context.image_cid.is_empty()
            && !file_processing_context.input_xml_path.is_empty()
            && !file_processing_context.input_image_path.is_empty()
        {
            file_processing_context.output_json_path = format!(
                "{}/{}.json",
                OUTPUT_FILES_DIR,
                &asset_folder_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| {
                        Box::new(OwenCliError::InvalidAssetFolderName(
                            asset_folder_path.to_string_lossy().to_string(),
                        ))
                    })?
            );
            file_processing_context.empty = false;
            attach_cid_and_save(&file_processing_context).await?;
        }
    }
    Ok(file_processing_context)
}

pub async fn create_output_files(
    config: &Config,
) -> Result<Vec<MessageDirProcessingContext>, Box<dyn Error>> {
    let mut result: Vec<MessageDirProcessingContext> = Vec::new();
    let output_files_path = Path::new(OUTPUT_FILES_DIR);
    if output_files_path.is_dir() {
        fs::remove_dir_all(output_files_path)?;
    }
    fs::create_dir_all(output_files_path)?;
    let root_folder_dir = Path::new(&config.folder_path);
    let mut empty_folder = true;

    if root_folder_dir.is_dir() {
        let asset_folders = fs::read_dir(root_folder_dir)?;

        for asset_folder in asset_folders {
            let asset_folder_path = asset_folder?.path();
            let asset_dir_processing_context =
                process_asset_folder(asset_folder_path, &config).await?;
            if !asset_dir_processing_context.empty {
                result.push(asset_dir_processing_context);
                empty_folder = false;
            }
        }
    } else {
        return Err(Box::new(OwenCliError::SourcePathIsNotDir(
            root_folder_dir.to_string_lossy().to_string(),
        )));
    }
    if empty_folder {
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
        };
        let processing_context_vec = create_output_files(&config).await?;

        let processed_count = processing_context_vec.len();

        assert_eq!(
            processing_context_vec.len(),
            2,
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
        };
        fs::create_dir_all(&config.folder_path).unwrap();

        create_output_files(&config).await.unwrap();
        fs::remove_dir_all(&config.folder_path).unwrap();
        ()
    }
}
