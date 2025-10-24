mod contract;
mod event_handler;
mod s3;
use event_handler::function_handler;
use lambda_runtime::{Error, run, service_fn, tracing};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}
