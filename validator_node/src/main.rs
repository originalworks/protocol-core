mod telemetry;
use anyhow::Context;
use log_macros::{log_error, log_info};
use sentry::User;
use serde_json::json;
use telemetry::{init_logging, init_sentry};
use validator_node::Config;

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
    init_sentry(&config);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(init(config))
}
