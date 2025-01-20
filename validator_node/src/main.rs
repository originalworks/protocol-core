use std::error::Error;
use std::process;
use validator_node::{run, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Configuration error: {err}");
        process::exit(1);
    });

    run(config).await?;

    Ok(())
}
