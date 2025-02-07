use std::error::Error;
use std::process;
use validator_node::{run, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build();

    run(config).await?;

    Ok(())
}

fn init_sentry(config: &Config) -> ClientInitGuard {
    sentry::init(("https://bfe1b70cabd02efab4e7045fb803bc79@o4508699010859008.ingest.de.sentry.io/4508777331359824",
        sentry::ClientOptions {
            environment: Some(config.environment.to_owned().into()),
            release: sentry::release_name!(),
            attach_stacktrace: false,
            auto_session_tracking: true,
            ..Default::default()
        },
    ))
}

fn init_logging() -> anyhow::Result<()> {
    let mut log_builder = pretty_env_logger::formatted_timed_builder();
    log_builder.parse_filters("info");

    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());

    log::set_boxed_logger(Box::new(logger)).with_context(|| "Failed to set boxed logger")?;
    log::set_max_level(log::LevelFilter::Info);

    Ok(())
}

async fn run(config: Config) -> Result<()> {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(User {
            username: Some(config.username.to_owned()),
            ..Default::default()
        }));

        let mut cloned_config = config.clone();
        cloned_config.pinata_jwt = "***".to_string();
        cloned_config.private_key = "***".to_string();
        scope.set_extra("config", json!(cloned_config));
    });

    owen_cli::run(&config).await.map_err(|e| {
        sentry::configure_scope(|scope| {
            scope.set_tag("error_type", {
                if e.is::<ParserError>() {
                    "parser"
                } else {
                    "other"
                }
            });
            scope.set_extra("error_object", json!(format!("{e:#?}")));
        });

        log_error!("{e}")
    })?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    init_logging()?;
    let config = Config::build();
    let _guard = init_sentry(&config);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run(config))
}
