mod contract;
mod event_handler;
mod s3;
mod secrets;
use event_handler::function_handler;
use lambda_runtime::{Error, run, service_fn, tracing};

use crate::secrets::set_secret_envs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_secret_envs().await?;
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}
