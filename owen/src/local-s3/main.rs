mod config;

use anyhow::{Ok, Result};
use config::LocalS3Config;
use owen::logger::{init_logging, init_sentry};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;

    let LocalS3Config {
        owen_config,
        aws_sdk_config,
    } = LocalS3Config::build().await;

    let _guard = init_sentry(&owen_config);

    let _storage = owen::s3_message_storage::MessageStorage::build(&aws_sdk_config);

    Ok(())
}
