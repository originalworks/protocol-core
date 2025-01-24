use anyhow::Result;
use log_macros::{log_error, log_info};
use owen_cli::Config;
use sentry::ClientInitGuard;
use std::env;

fn init_sentry() -> ClientInitGuard {
    // Initialize Sentry
    sentry::init((
        "https://af48238f81e0f9513c7604e43781bf86@o4508699010859008.ingest.de.sentry.io/4508699013087312",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            attach_stacktrace: true,
            ..Default::default()
        },
    ))
}

fn init_logging() -> anyhow::Result<()> {
    let mut log_builder = pretty_env_logger::formatted_timed_builder();
    log_builder.format_module_path(true);
    log_builder.parse_filters("info");
    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    Ok(())
}

fn main() -> Result<()> {
    init_logging()?;
    let _guard = init_sentry();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run())
}

async fn run() -> Result<()> {
    match dotenvy::dotenv() {
        Ok(_) => log_info!("Config loaded from .env file"),
        Err(_) => log_info!("Config loaded from env"),
    }

    let config = Config::build(env::args()).map_err(|e| log_error!("{e:?}"))?;
    owen_cli::run(config)
        .await
        .map_err(|e| log_error!("{e:?}"))?;

    Ok(())
}
