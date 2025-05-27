fn init_sentry(config: &Config) -> () {
    if !config.disable_telemetry {
        let _ = sentry::init((
            "https://8a5e3e61ac0beb391ad84b32086674df@o4508766269014016.ingest.us.sentry.io/4508805625217024",
            sentry::ClientOptions {
                environment: Some(config.environment.to_owned().into()),
                release: sentry::release_name!(),
                attach_stacktrace: true,
                auto_session_tracking: true,
                traces_sample_rate: 1.0, // lower it in production
                ..Default::default()
            },
        ));
        log_info!("Telemetry has been initiated");
    } else {
        log_info!("Telemetry has been disabled due to env var flag");
    }
}

fn init_logging() -> anyhow::Result<()> {
    let mut log_builder = pretty_env_logger::formatted_timed_builder();
    log_builder.parse_filters("info");

    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());
    log::set_boxed_logger(Box::new(logger)).with_context(|| "Failed to set boxed logger")?;
    log::set_max_level(log::LevelFilter::Info);

    Ok(())
}
