use anyhow::Result;
use owen::{
    config::core::Config,
    logger::{init_logging, init_sentry, use_sentry},
    orchestrator,
};

fn main() -> Result<()> {
    init_logging()?;
    let config = Config::build()?;
    let _guard = init_sentry();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(use_sentry(&config, orchestrator::cli::run(&config)))
        .map(|_| ())
}
