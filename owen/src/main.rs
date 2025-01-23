use owen_cli::Config;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match dotenvy::dotenv() {
        Ok(_) => println!("Config loaded from .env file"),
        Err(_) => println!("Config loaded from env"),
    }

    let config = match Config::build(env::args()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Configuration error: {}", err);
            std::process::exit(1);
        }
    };
    owen_cli::run(config).await?;

    Ok(())
}
