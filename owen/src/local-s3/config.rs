use std::env;

use aws_config::{meta::region::RegionProviderChain, Region, SdkConfig};
use aws_sdk_s3::config::Credentials;

pub struct LocalS3Config {
    pub owen_config: owen::Config,
    pub aws_sdk_config: SdkConfig,
}

impl LocalS3Config {
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(format!("Missing env variable: {key}").as_str())
    }

    pub async fn build() -> Self {
        let owen_config = owen::Config::build();

        let aws_access_key_id = LocalS3Config::get_env_var("AWS_ACCESS_KEY_ID");
        let aws_secret_access_key = LocalS3Config::get_env_var("AWS_SECRET_ACCESS_KEY");
        let aws_default_region = LocalS3Config::get_env_var("AWS_DEFAULT_REGION");

        let aws_credentials = Credentials::new(
            aws_access_key_id,
            aws_secret_access_key,
            None,
            None,
            "owen-s3",
        );

        let region_provider =
            RegionProviderChain::default_provider().or_else(Some(Region::new(aws_default_region)));

        let aws_sdk_config = aws_config::from_env()
            .credentials_provider(aws_credentials)
            .region(region_provider)
            .load()
            .await;

        Self {
            owen_config,
            aws_sdk_config,
        }
    }
}
