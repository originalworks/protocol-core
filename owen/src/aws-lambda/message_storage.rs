use std::{collections::HashMap, env, error::Error, fs, path::Path};
use tokio::fs::File;

pub struct MessageStorage {
    client: aws_sdk_s3::Client,
    bucket_name: String,
    input_files_dir: String,
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
        }
    }

    fn build_local_object_path(
        &self,
        s3_folder_object_key: &String,
        s3_message_folder: &String,
        local_message_folder: &String,
    ) -> Result<String, Box<dyn Error>> {
        let (_, s3_parent_dir_file) = s3_folder_object_key
            .split_once(s3_message_folder)
            .expect("Could not split s3 folder key");
        let local_object_path = format!("{local_message_folder}{s3_parent_dir_file}");
        Ok(local_object_path)
    }

    async fn copy_from_s3(&self, key: String, local_path: String) -> Result<(), Box<dyn Error>> {
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

    pub async fn sync_message_folders(
        &self,
        message_folders: &Vec<String>,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut local_to_s3_folder_mapping = HashMap::<String, String>::new();
        for s3_message_folder in message_folders {
            let s3_message_folder_parent_dir = Path::new(&s3_message_folder) // unique
                .file_name()
                .expect(
                    format!("S3 message folder has no filename {}", &s3_message_folder).as_str(),
                )
                .to_str()
                .expect("Parsing to str failed");

            let local_message_folder =
                format!("{}/{}", self.input_files_dir, s3_message_folder_parent_dir);

            local_to_s3_folder_mapping
                .insert(local_message_folder.clone(), s3_message_folder.clone());

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
        }

        Ok(local_to_s3_folder_mapping)
    }

    pub fn clear_input_folder(&self) -> Result<(), Box<dyn Error>> {
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
}
