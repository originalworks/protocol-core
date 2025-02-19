use crate::Config;
use anyhow::Context;
use sentry::ClientInitGuard;

pub fn init_sentry(config: &Config) -> ClientInitGuard {
    sentry::init(("https://2cea3d6af1cb8e4bd9c7c39530d390a1@o4508766269014016.ingest.us.sentry.io/4508766275043328",
        sentry::ClientOptions {
            environment: Some(config.environment.to_owned().into()),
            release: sentry::release_name!(),
            attach_stacktrace: false,
            auto_session_tracking: true,
            ..Default::default()
        },
    ))
}

pub fn init_logging() -> anyhow::Result<()> {
    let mut log_builder = pretty_env_logger::formatted_timed_builder();
    log_builder.parse_filters("info");

    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());

    log::set_boxed_logger(Box::new(logger)).with_context(|| "Failed to set boxed logger")?;
    log::set_max_level(log::LevelFilter::Info);

    Ok(())
}
