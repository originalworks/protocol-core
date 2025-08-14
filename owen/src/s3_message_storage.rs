use anyhow::Result;
use aws_sdk_s3::types::{Delete, ObjectIdentifier};
use blob_codec::BlobEstimator;
use log_macros::log_warn;
use std::{collections::HashMap, env, fs, path::Path};
use tokio::fs::File;

use crate::{constants::MAX_DDEX_PER_BLOB, output_generator::MessageDirProcessingContext};

pub struct MessageStorage {
    client: aws_sdk_s3::Client,
    bucket_name: String,
    input_files_dir: String,
    message_bucket_prefix: String,
    fallback_bucket_name: String,
    pub local_to_s3_folder_mapping: HashMap<String, String>,
    pub s3_message_folders: Vec<String>,
}

impl MessageStorage {
    pub fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }
    pub fn build(aws_main_config: &aws_config::SdkConfig) -> Self {
        Self {
            client: aws_sdk_s3::Client::new(aws_main_config),
            bucket_name: MessageStorage::get_env_var("MESSAGES_BUCKET_NAME"),
            input_files_dir: MessageStorage::get_env_var("INPUT_FILES_DIR"),
            message_bucket_prefix: MessageStorage::get_env_var("MESSAGE_BUCKET_PREFIX"),
            fallback_bucket_name: MessageStorage::get_env_var("FALLBACK_BUCKET_NAME"),
            local_to_s3_folder_mapping: HashMap::new(),
            s3_message_folders: vec![],
        }
    }

    fn build_local_object_path(
        &self,
        s3_folder_object_key: &String,
        s3_message_folder: &String,
        local_message_folder: &String,
    ) -> Result<String> {
        let (_, s3_parent_dir_file) = s3_folder_object_key
            .split_once(s3_message_folder)
            .expect("Could not split s3 folder key");
        let local_object_path = format!("{local_message_folder}{s3_parent_dir_file}");
        Ok(local_object_path)
    }

    async fn copy_from_s3(&self, key: String, local_path: String) -> Result<()> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await?;
        fs::create_dir_all(
            Path::new(&local_path)
                .parent()
                .expect("Local message folder path has no parent"),
        )?;
        let mut file = File::create(&local_path).await?;
        let mut stream = resp.body.into_async_read();

        tokio::io::copy(&mut stream, &mut file).await?;

        Ok(())
    }

    pub async fn download_message_folders(&mut self) -> Result<()> {
        let max_s3_message_folders = self
            .list_message_folders_with_limit(MAX_DDEX_PER_BLOB)
            .await?;
        let blob_estimator = BlobEstimator::default();

        for s3_message_folder in max_s3_message_folders {
            let local_message_folder = self.sync_message_folder(&s3_message_folder).await?;
            match blob_estimator.estimate_and_check(Path::new(&self.input_files_dir)) {
                Ok(_) => {
                    self.local_to_s3_folder_mapping
                        .insert(local_message_folder.clone(), s3_message_folder.clone());

                    self.s3_message_folders.push(s3_message_folder)
                }
                Err(err) => {
                    log_warn!(err);
                    std::fs::remove_dir_all(Path::new(&local_message_folder))?;
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn sync_message_folder(&self, s3_message_folder: &String) -> Result<String> {
        let s3_message_folder_parent_dir = Path::new(&s3_message_folder) // unique
            .file_name()
            .expect(format!("S3 message folder has no filename {}", &s3_message_folder).as_str())
            .to_str()
            .expect("Parsing to str failed");

        let local_message_folder =
            format!("{}/{}", self.input_files_dir, s3_message_folder_parent_dir);

        let s3_message_folder_objects = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket_name)
            .prefix(s3_message_folder)
            .send()
            .await?;

        for s3_folder_object in s3_message_folder_objects
            .contents
            .expect("S3 message folder object has no 'contents'")
        {
            let s3_folder_object_key = s3_folder_object
                .key
                .expect("S3 message folder object has no 'key'");
            let local_object_path = self.build_local_object_path(
                &s3_folder_object_key,
                &s3_message_folder,
                &local_message_folder,
            )?;

            self.copy_from_s3(s3_folder_object_key, local_object_path)
                .await?;
        }
        Ok(local_message_folder)
    }

    pub fn clear_input_folder(&self) -> Result<()> {
        let input_files_path = Path::new(&self.input_files_dir);
        if input_files_path.is_dir() {
            // Debugging issue with leftovers between lambda runs:
            let mut empty = false;
            while empty != true {
                fs::remove_dir_all(input_files_path)?;
                if input_files_path.is_dir() == false {
                    empty = true;
                } else {
                    println!("Input folder is not cleared yet, retrying...");
                }
            }
        }
        Ok(())
    }

    pub async fn list_message_folders_with_limit(&self, limit: i32) -> Result<Vec<String>> {
        let s3_objects = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket_name)
            .prefix(&self.message_bucket_prefix)
            .delimiter("/")
            .max_keys(limit)
            .send()
            .await?;
        let message_folders: Vec<String> = s3_objects
            .common_prefixes
            .expect("Could not get message folders")
            .iter()
            .map(|s3_object| {
                let mut message_folder = s3_object
                    .prefix
                    .as_ref()
                    .expect("Could not get message folder prefix")
                    .clone();
                message_folder.pop(); // remove trailing slash
                message_folder
            })
            .collect();
        Ok(message_folders)
    }

    pub async fn clear_processed_s3_folders(&self, s3_message_folders: &Vec<String>) -> Result<()> {
        let mut objects_to_delete = Vec::new();
        for s3_message_folder in s3_message_folders {
            let folder_objects = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket_name)
                .prefix(s3_message_folder)
                .send()
                .await?;

            if let Some(folder_contents) = folder_objects.contents {
                for folder_object in folder_contents {
                    if let Some(key) = folder_object.key {
                        objects_to_delete.push(ObjectIdentifier::builder().key(key).build()?);
                    }
                }
            }
        }

        if !objects_to_delete.is_empty() {
            self.client
                .delete_objects()
                .bucket(&self.bucket_name)
                .delete(
                    Delete::builder()
                        .set_objects(Some(objects_to_delete))
                        .build()?,
                )
                .send()
                .await?;
        }
        Ok(())
    }

    pub async fn clear_rejected_s3_folders(&self, s3_message_folders: &Vec<String>) -> Result<()> {
        let mut objects_to_delete = Vec::new();
        for s3_message_folder in s3_message_folders {
            let folder_objects = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket_name)
                .prefix(s3_message_folder)
                .send()
                .await?;

            if let Some(folder_contents) = folder_objects.contents {
                for folder_object in folder_contents {
                    if let Some(key) = folder_object.key {
                        objects_to_delete.push(ObjectIdentifier::builder().key(key).build()?);
                    }
                }
            }
        }

        if !objects_to_delete.is_empty() {
            for object_key in &objects_to_delete {
                let source = format!("{}/{}", self.bucket_name, &object_key.key);
                self.client
                    .copy_object()
                    .copy_source(source)
                    .bucket(&self.fallback_bucket_name)
                    .key(&object_key.key)
                    .send()
                    .await?;
                println!("Object copied to fallback bucket: {}", &object_key.key);
            }

            self.client
                .delete_objects()
                .bucket(&self.bucket_name)
                .delete(
                    Delete::builder()
                        .set_objects(Some(objects_to_delete))
                        .build()?,
                )
                .send()
                .await?;
        }
        Ok(())
    }

    pub async fn clear_s3_folders(
        &self,
        s3_folder_to_processing_context_map: HashMap<String, MessageDirProcessingContext>,
        s3_message_folders: &Vec<String>,
    ) -> Result<()> {
        let mut objects_to_delete = Vec::new();
        let mut objects_to_copy_to_fallback_bucket = Vec::new();
        for s3_message_folder in s3_message_folders {
            let folder_objects = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket_name)
                .prefix(s3_message_folder)
                .send()
                .await?;

            let processing_context = s3_folder_to_processing_context_map
                .get(s3_message_folder)
                .expect("Could not retrieve processing context");
            if let Some(folder_contents) = folder_objects.contents {
                for folder_object in folder_contents {
                    if let Some(key) = folder_object.key {
                        objects_to_delete.push(ObjectIdentifier::builder().key(&key).build()?);
                        if processing_context.excluded {
                            objects_to_copy_to_fallback_bucket.push(key);
                        }
                    }
                }
            }
        }

        if !objects_to_copy_to_fallback_bucket.is_empty() {
            for object_key in objects_to_copy_to_fallback_bucket {
                let source = format!("{}/{}", self.bucket_name, &object_key);
                self.client
                    .copy_object()
                    .copy_source(source)
                    .bucket(&self.fallback_bucket_name)
                    .key(&object_key)
                    .send()
                    .await?;
                println!("Object copied to fallback bucket: {}", &object_key);
            }
        }

        if !objects_to_delete.is_empty() {
            self.client
                .delete_objects()
                .bucket(&self.bucket_name)
                .delete(
                    Delete::builder()
                        .set_objects(Some(objects_to_delete))
                        .build()?,
                )
                .send()
                .await?;
        }

        Ok(())
    }
}
