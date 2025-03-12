use crate::Config;
use anyhow::Context;
use log_macros::log_warn;
use sentry::{protocol::Attachment, ClientInitGuard};
use serde_json::json;
use std::io::Read;

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

pub fn report_validation_error<E: std::error::Error>(
    err: &E,
    attachment_path: &String,
    raw_err_object: bool,
) {
    sentry::with_scope(
        |scope| {
            let mut buffer: Vec<u8> = Vec::new();
            let attachment_added = std::fs::File::open(attachment_path)
                .ok()
                .and_then(|mut f| f.read_to_end(&mut buffer).ok())
                .and_then(|_| {
                    scope.add_attachment(Attachment {
                        filename: attachment_path
                            .split("/")
                            .last()
                            .unwrap_or_else(|| "unknown")
                            .to_string(),
                        buffer,
                        content_type: Some("text/xml".to_string()),
                        ..Default::default()
                    });
                    Some(())
                });
            if attachment_added.is_none() {
                log_warn!("Failed to add attachment");
            }

            scope.set_tag("error_type", "parser");
            scope.set_extra("error_object", {
                if raw_err_object {
                    json!(format!("{err:#?}"))
                } else {
                    json!(err.to_string())
                }
            });
            scope.clear_breadcrumbs();
        },
        || {
            sentry::capture_message("Validation error", sentry::Level::Warning);
        },
    );
}
