#![cfg(feature = "aws-integration")]
use std::sync::{Arc, Mutex};

use lambda_runtime::{service_fn, tracing};
use owen::logger::{init_logging, init_sentry};
use owen::orchestrator::aws::aa_lambda::AccountAbstractionLambdaOrchestrator;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    println!("Lambda cold start");

    let _guard = init_sentry();
    init_logging()?;

    tracing::init_default_subscriber();

    let orchestrator = Arc::new(Mutex::new(
        AccountAbstractionLambdaOrchestrator::build().await?,
    ));

    lambda_runtime::run(service_fn(|event| {
        let orchestrator = Arc::clone(&orchestrator);
        async move {
            let mut orchestrator_lock = orchestrator.lock().unwrap();
            orchestrator_lock.lambda_function_handler(event).await
        }
    }))
    .await
}
