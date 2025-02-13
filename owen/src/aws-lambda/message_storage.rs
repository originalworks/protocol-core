use std::{env, error::Error, fs, path::Path};
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

    fn build_local_path(
        &self,
        s3_key: &String,
        message_folder: &String,
    ) -> Result<String, Box<dyn Error>> {
        let s3_message_folder_parent_dir = Path::new(&message_folder)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let local_message_folder =
            format!("{}/{}", self.input_files_dir, s3_message_folder_parent_dir);
        let (_, s3_parent_dir_file) = s3_key.split_once(message_folder).unwrap();
        let local_path = format!("{local_message_folder}{s3_parent_dir_file}");

        Ok(local_path)
    }

    async fn copy_from_s3(&self, key: String, local_path: String) -> Result<(), Box<dyn Error>> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await?;
        fs::create_dir_all(Path::new(&local_path).parent().unwrap())?;
        let mut file = File::create(&local_path).await?;
        let mut stream = resp.body.into_async_read();

        tokio::io::copy(&mut stream, &mut file).await?;

        Ok(())
    }

    pub async fn sync_message_folders(
        &self,
        message_folders: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        for s3_message_folder in message_folders {
            let s3_message_folder_objects = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket_name)
                .prefix(&s3_message_folder)
                .send()
                .await?;

            for s3_folder_object in s3_message_folder_objects.contents.unwrap() {
                let s3_key = s3_folder_object.key.unwrap();
                let local_path = self.build_local_path(&s3_key, &s3_message_folder).unwrap();

                self.copy_from_s3(s3_key, local_path).await?;
            }
        }

        Ok(())
    }
}
