use anyhow::Result;
use owen::{
    logger::{init_logging, init_sentry},
    run_with_sentry, Config,
};

fn main() -> Result<()> {
    init_logging()?;
    let config = Config::build()?;
    let _guard = init_sentry();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run_with_sentry(&config))
        .map(|_| ())
}
