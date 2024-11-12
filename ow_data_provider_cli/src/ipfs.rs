use reqwest::{multipart, Body};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::{
    error::Error,
    io::{BufReader, BufWriter},
    path::Path,
};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::constants::{IMAGE_FILE_CID_TAG, IPFS_CIDS_ROOT_TAG, OUTPUT_FILES_DIR};
use crate::{
    constants::{IPFS_API_ADD_FILE, IPFS_API_BASE_URL},
    errors::OwDataProviderCliError,
};

use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct IpfsResponse {
    Hash: String,
}
#[derive(Debug)]
pub struct AssetDirProcessingContext {
    input_xml_path: String,
    input_image_path: String,
    image_cid: String,
    output_xml_path: String,
    empty: bool,
}

async fn file_to_multipart_form(file_path: &String) -> Result<multipart::Form, Box<dyn Error>> {
    let file = tokio::fs::File::open(file_path).await?;
    let file_stream = FramedRead::new(file, BytesCodec::new());
    let multipart_stream = multipart::Part::stream(Body::wrap_stream(file_stream));
    let multipart_form = multipart::Form::new().part("file", multipart_stream);
    Ok(multipart_form)
}

pub async fn pin_file(file_path: &String) -> Result<String, Box<dyn Error>> {
    let multipart_form = file_to_multipart_form(file_path).await?;
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}{}", IPFS_API_BASE_URL, IPFS_API_ADD_FILE))
        .multipart(multipart_form)
        .send()
        .await?;

    let result = response.json::<IpfsResponse>().await?;

    Ok(result.Hash)
}

async fn attach_cid_and_save(input: &AssetDirProcessingContext) -> Result<(), Box<dyn Error>> {
    let mut buf = Vec::new();

    let file = fs::File::open(&input.input_xml_path)?;
    let reader = BufReader::new(file);
    let mut reader = Reader::from_reader(reader);

    let output_file = fs::File::create(&input.output_xml_path)?;
    let writer = BufWriter::new(output_file);
    let mut writer = Writer::new(writer);

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => {
                writer.write_event(Event::Start(e.to_owned()))?;
                if e == &BytesStart::new(IPFS_CIDS_ROOT_TAG) {
                    let tag = BytesStart::new(IMAGE_FILE_CID_TAG);
                    writer.write_event(Event::Start(tag.to_owned()))?;
                    writer.write_event(Event::Text(BytesText::new(&input.image_cid)))?;
                    writer.write_event(Event::End(tag.to_end()))?;
                }
            }
            Ok(Event::Text(ref e)) => {
                writer.write_event(Event::Text(e.to_owned()))?;
            }
            Ok(Event::End(ref e)) => {
                writer.write_event(Event::End(e.to_owned()))?;
            }

            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

async fn process_asset_folder(
    asset_folder_path: PathBuf,
    folder_index: usize,
) -> Result<AssetDirProcessingContext, Box<dyn Error>> {
    let mut file_processing_context = AssetDirProcessingContext {
        input_xml_path: String::new(),
        input_image_path: String::new(),
        output_xml_path: String::new(),
        image_cid: String::new(),
        empty: true,
    };
    if asset_folder_path.is_dir() {
        let asset_files = fs::read_dir(asset_folder_path)?;

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
                        pin_file(&file_processing_context.input_image_path).await?;
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
            file_processing_context.output_xml_path =
                format!("{}/{}.xml", OUTPUT_FILES_DIR, folder_index);
            file_processing_context.empty = false;
            attach_cid_and_save(&file_processing_context).await?;
        }
    }
    Ok(file_processing_context)
}

pub async fn create_output_files(
    folder_path: &String,
) -> Result<Vec<AssetDirProcessingContext>, Box<dyn Error>> {
    let mut result: Vec<AssetDirProcessingContext> = Vec::new();
    let output_files_path = Path::new(OUTPUT_FILES_DIR);
    if output_files_path.is_dir() {
        fs::remove_dir_all(output_files_path)?;
    }
    fs::create_dir_all(output_files_path)?;
    let root_folder_dir = Path::new(&folder_path);
    let mut empty_folder = true;

    if root_folder_dir.is_dir() {
        let asset_folders = fs::read_dir(root_folder_dir)?;

        for (index, asset_folder) in asset_folders.into_iter().enumerate() {
            let asset_folder_path = asset_folder?.path();
            let asset_dir_processing_context =
                process_asset_folder(asset_folder_path, index).await?;
            if !asset_dir_processing_context.empty {
                result.push(asset_dir_processing_context);
                empty_folder = false;
            }
        }
    } else {
        return Err(Box::new(OwDataProviderCliError::SourcePathIsNotDir(
            root_folder_dir.to_string_lossy().to_string(),
        )));
    }
    if empty_folder {
        return Err(Box::new(OwDataProviderCliError::EmptySourcePathFolder(
            folder_path.to_string(),
        )));
    }

    print_output(&result)?;
    Ok(result)
}

fn print_output(output: &Vec<AssetDirProcessingContext>) -> Result<(), Box<dyn Error>> {
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
            entry.image_cid, entry.output_xml_path
        );
        println!("----------")
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::IPFS_API_CAT_FILE;

    fn read_file_tag(path: &Path, tag: &String) -> Result<String, Box<dyn Error>> {
        let mut reader = Reader::from_file(&path)?;
        let mut buffer = Vec::new();

        let mut inside_id_tag = false;

        let mut tag_value = String::new();

        loop {
            match reader.read_event_into(&mut buffer) {
                Ok(Event::Eof) => break,
                Ok(Event::Start(ref e)) => {
                    if e == &BytesStart::new(tag) {
                        inside_id_tag = true;
                    }
                }
                Ok(Event::Text(ref e)) if inside_id_tag => {
                    tag_value = String::from_utf8(e.to_vec())?;
                    break;
                }

                Err(e) => return Err(Box::new(e)),
                _ => {}
            }
            buffer.clear();
        }
        Ok(tag_value)
    }

    async fn fetch_ipfs_file(cid: &String) -> Result<tokio_util::bytes::Bytes, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client
            .post(format!(
                "{}{}?arg={}",
                IPFS_API_BASE_URL, IPFS_API_CAT_FILE, cid
            ))
            .send()
            .await?;

        if response.status() != 200 {
            panic!("Image CID not found {cid}");
        }
        let bytes = response.bytes().await?;

        Ok(bytes)
    }

    #[tokio::test]
    async fn pin_image_and_add_tag() -> Result<(), Box<dyn Error>> {
        let test_folder = "./tests";
        let processing_context_vec = create_output_files(&test_folder.to_string()).await?;

        let processed_count = processing_context_vec.len();

        assert_eq!(
            processing_context_vec.len(),
            2,
            "Wrong output size. Expected 2, got: {processed_count}"
        );

        for processing_context in processing_context_vec {
            let expected_image = fs::read(processing_context.input_image_path)?;

            let fetched_image = fetch_ipfs_file(&processing_context.image_cid).await?;
            assert_eq!(
                fetched_image.to_vec(),
                expected_image,
                "Fetched image doesn't match"
            );
            let output_xml_path = Path::new(&processing_context.output_xml_path);
            let cid_from_xml = read_file_tag(output_xml_path, &IMAGE_FILE_CID_TAG.to_string())?;
            assert_eq!(
                cid_from_xml, processing_context.image_cid,
                "Missing ipfs cid tag in xml"
            );
        }

        Ok(())
    }

    #[should_panic]
    #[tokio::test]
    async fn error_when_empty_directory() {
        let test_folder = "./tests2";
        tokio::fs::create_dir_all(test_folder).await.unwrap();

        create_output_files(&test_folder.to_string()).await.unwrap();
        tokio::fs::remove_dir_all(test_folder).await.unwrap();
        ()
    }
}
