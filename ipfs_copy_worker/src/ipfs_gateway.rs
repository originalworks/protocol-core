use crate::constants::TEMP_FOLDER_PATH;
use crate::{
    ProcessingContext, ProcessingStatus, cid::calculate_dir_cid, constants::STORACHA_GATEWAY_URL,
};
use futures_util::StreamExt;
use reqwest::StatusCode;
use std::{fs, io::Write, path::Path};

#[derive(Debug)]
pub struct DownloadFileResult {
    pub status_code: StatusCode,
    pub local_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CidType {
    File,
    Folder,
    Unknown,
}

pub struct Gateway {
    gateway_url: String,
    reqwest_client: reqwest::Client,
}

impl Gateway {
    pub fn build() -> anyhow::Result<Self> {
        Self::prepare_temp_folder()?;
        Ok(Self {
            gateway_url: STORACHA_GATEWAY_URL.to_string(),
            reqwest_client: reqwest::Client::new(),
        })
    }

    fn prepare_temp_folder() -> anyhow::Result<()> {
        let temp_folder_path = Path::new(TEMP_FOLDER_PATH);
        if temp_folder_path.is_dir() {
            fs::remove_dir_all(temp_folder_path)?;
        }
        fs::create_dir_all(temp_folder_path)?;

        Ok(())
    }

    pub async fn download_by_cid(&self, cid: &String) -> anyhow::Result<ProcessingContext> {
        match self.detect_cid_type(cid).await? {
            CidType::File => {
                println!("Processing file...");

                let download_file_result = self.download_file(cid).await?;

                Ok(ProcessingContext {
                    cid: cid.clone(),
                    cid_type: CidType::File,
                    processing_status: if download_file_result.status_code.is_success() {
                        ProcessingStatus::Downloaded
                    } else {
                        ProcessingStatus::FailedDownload
                    },
                    files: vec![download_file_result],
                })
            }
            CidType::Folder => {
                println!("Processing folder...");

                let folder_local_path = Path::new(TEMP_FOLDER_PATH).join(cid);

                let download_file_results = self.download_folder(cid).await?;

                let calculated_cid = calculate_dir_cid(
                    &self.reqwest_client,
                    &folder_local_path.to_string_lossy().to_string(),
                )
                .await?;

                println!("{calculated_cid} - {cid}");

                Ok(ProcessingContext {
                    cid: cid.clone(),
                    cid_type: CidType::Folder,
                    processing_status: if &calculated_cid == cid {
                        ProcessingStatus::Downloaded
                    } else {
                        ProcessingStatus::FailedDownload
                    },
                    files: download_file_results,
                })
            }
            CidType::Unknown => Ok(ProcessingContext {
                cid: cid.clone(),
                cid_type: CidType::Unknown,
                processing_status: ProcessingStatus::FailedDownload,
                files: vec![],
            }),
        }
    }

    async fn download_folder(&self, cid: &String) -> anyhow::Result<Vec<DownloadFileResult>> {
        let blob_folder_path = Path::new(TEMP_FOLDER_PATH).join(cid).join("blob");
        let images_folder_path = Path::new(TEMP_FOLDER_PATH).join(cid).join("images");
        let json_folder_path = Path::new(TEMP_FOLDER_PATH).join(cid).join("json");
        let zips_folder_path = Path::new(TEMP_FOLDER_PATH).join("zips");

        fs::create_dir_all(blob_folder_path)?;
        fs::create_dir_all(images_folder_path)?;
        fs::create_dir_all(json_folder_path)?;
        fs::create_dir_all(zips_folder_path)?;

        let mut downloaded_vec = Vec::new();

        let blob_data_download_result = self
            .download_file(&format!("{}/blob/blob_data.bin", cid))
            .await?;
        let blob_metadata_download_result = self
            .download_file(&format!("{}/blob/metadata.json", cid))
            .await?;

        let mut downloaded_iamges = self.try_download_images_folder(cid).await?;
        let mut downloaded_jsons = self.try_download_json_folder(cid).await?;

        downloaded_vec.push(blob_data_download_result);
        downloaded_vec.push(blob_metadata_download_result);
        downloaded_vec.append(&mut downloaded_iamges);
        downloaded_vec.append(&mut downloaded_jsons);

        Ok(downloaded_vec)
    }

    async fn try_download_images_folder(
        &self,
        cid: &String,
    ) -> anyhow::Result<Vec<DownloadFileResult>> {
        let mut downloaded_vec = Vec::new();

        let file_list = self.read_image_files_list_from_html(cid).await?;

        for file_name in file_list {
            let json_cid_path = format!("{}/images/{}", cid, file_name);
            let download_result = self.download_file(&json_cid_path).await?;
            downloaded_vec.push(download_result);
        }

        Ok(downloaded_vec)
    }

    async fn try_download_json_folder(
        &self,
        cid: &String,
    ) -> anyhow::Result<Vec<DownloadFileResult>> {
        let mut downloaded_vec = Vec::new();

        let file_list = self.read_json_files_list_from_html(cid).await?;

        for file_name in file_list {
            let json_cid_path = format!("{}/json/{}", cid, file_name);
            let download_result = self.download_file(&json_cid_path).await?;
            downloaded_vec.push(download_result);
        }

        Ok(downloaded_vec)
    }

    async fn read_image_files_list_from_html(&self, cid: &String) -> anyhow::Result<Vec<String>> {
        let url = format!("{}{}/images", self.gateway_url, cid);
        let response = self.reqwest_client.get(&url).send().await?;
        let response_text = response.text().await?;
        let mut file_list = Vec::new();
        let mut counter = 0;

        loop {
            let file_name = format!("{}.avif", counter);
            if response_text.contains(&file_name) {
                file_list.push(file_name);
                counter += 1;
            } else {
                break;
            }
        }

        Ok(file_list)
    }

    async fn read_json_files_list_from_html(&self, cid: &String) -> anyhow::Result<Vec<String>> {
        let url = format!("{}{}/json", self.gateway_url, cid);
        let response = self.reqwest_client.get(&url).send().await?;
        let response_text = response.text().await?;
        let mut file_list = Vec::new();
        let mut counter = 0;

        loop {
            let file_name = format!("{}.json", counter);
            if response_text.contains(&file_name) {
                file_list.push(file_name);
                counter += 1;
            } else {
                break;
            }
        }

        Ok(file_list)
    }

    async fn download_file(&self, cid: &String) -> anyhow::Result<DownloadFileResult> {
        let download_url = format!("{}{}", self.gateway_url, cid);
        println!("Downloading: {download_url}");
        let new_file_path = Path::new(TEMP_FOLDER_PATH).join(cid);

        let response = self.reqwest_client.get(&download_url).send().await?;

        match response.status() {
            status_code if status_code.is_success() => {
                let mut file = fs::File::create(&new_file_path)?;
                let mut stream = response.bytes_stream();

                while let Some(chunk) = stream.next().await {
                    file.write_all(&chunk?)?;
                }
                return Ok(DownloadFileResult {
                    status_code,
                    local_path: new_file_path.to_string_lossy().to_string(),
                });
            }

            status_code => {
                return Ok(DownloadFileResult {
                    status_code,
                    local_path: new_file_path.to_string_lossy().to_string(),
                });
            }
        }
    }

    pub async fn detect_cid_type(&self, cid: &String) -> anyhow::Result<CidType> {
        let url = format!("{}{}", self.gateway_url, cid);

        if let Ok(resp) = self.reqwest_client.head(&url).send().await {
            let status = resp.status();
            if status.is_success() {
                if let Some(header_value) = resp.headers().get("content-type") {
                    let header_value_str = header_value.to_str().unwrap_or("");
                    if header_value_str.contains("text/html") {
                        return Ok(CidType::Folder);
                    }
                    return Ok(CidType::File);
                }
            }
        }

        Ok(CidType::Unknown)
    }
}
