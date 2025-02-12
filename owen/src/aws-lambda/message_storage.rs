use std::env;

// use crate::lambda_config::OwenLambdaConfig;

pub struct MessageStorage {
    client: aws_sdk_s3::Client,
    bucket_name: String,
}

impl MessageStorage {
    pub fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }
    pub fn build(aws_main_config: &aws_config::SdkConfig) -> Self {
        Self {
            client: aws_sdk_s3::Client::new(aws_main_config),
            bucket_name: MessageStorage::get_env_var("MESSAGES_BUCKET_NAME"),
        }
    }
}
