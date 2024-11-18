use ow_data_provider_cli::Config;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let config = match Config::build(env::args()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Configuration error: {}", err);
            std::process::exit(1);
        }
    };
    ow_data_provider_cli::run(config).await?;

    Ok(())
}
