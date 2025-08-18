use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, Region, SdkConfig};
use std::env;

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
        let aws_default_region = LocalS3Config::get_env_var("AWS_DEFAULT_REGION");

        let region_provider =
            RegionProviderChain::default_provider().or_else(Some(Region::new(aws_default_region)));

        let aws_sdk_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        Self {
            owen_config,
            aws_sdk_config,
        }
    }
}
