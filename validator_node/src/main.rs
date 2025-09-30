use anyhow::Context;
use log_macros::{log_error, log_info};
use sentry::User;
use serde_json::json;
use validator_node::Config;

fn init_sentry(config: &Config) -> Option<sentry::ClientInitGuard> {
    if !config.disable_telemetry {
        let guard: sentry::ClientInitGuard = sentry::init((
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
        Some(guard)
    } else {
        log_info!("Telemetry has been disabled due to env var flag");
        None
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

async fn init(config: Config) -> anyhow::Result<()> {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(User {
            username: Some(config.username.to_owned()),
            ..Default::default()
        }));

        let mut cloned_config = config.clone();
        cloned_config.private_key = "***".to_string();
        scope.set_extra("config", json!(cloned_config));
    });

    validator_node::run(&config).await.map_err(|e| {
        sentry::configure_scope(|scope| {
            scope.set_extra("error_object", json!(format!("{e:#?}")));
        });
        log_error!("{e}")
    })?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_logging()?;

    dotenvy::dotenv().ok();

    let config = Config::build();
    let _guard = init_sentry(&config);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(init(config))
}
