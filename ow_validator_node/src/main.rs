use ow_validator_node::Config;
use std::error::Error;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Configuration error: {err}");
        process::exit(1);
    });

    ow_validator_node::run(config).await?;

    Ok(())
}
