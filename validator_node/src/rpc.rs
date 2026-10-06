use alloy::{contract::Error as ContractError, transports::TransportError};
use log_macros::log_warn;
use std::{future::Future, time::Duration};

/// Retry an RPC read without restarting the surrounding operation.
/// Do not wrap transaction submission: a failed response may hide an accepted transaction.
pub(crate) async fn retry_rpc_call<T, E, F, Fut>(operation: &str, mut call: F) -> anyhow::Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: Into<anyhow::Error>,
{
    let max_attempts: u32 = 5;
    let initial_delay_seconds: u64 = 5;
    let mut attempt: u32 = 1;

    loop {
        match call().await {
            Ok(value) => return Ok(value),
            Err(error) => {
                let error = error.into();
                if !is_rpc_error(&error) || attempt == max_attempts {
                    return Err(error.context(format!(
                        "RPC call {operation} failed after {attempt} attempt(s)"
                    )));
                }

                let retry_index: u32 = attempt - 1;
                let backoff_multiplier: u64 = 2u64.pow(retry_index);
                let retry_delay_seconds = initial_delay_seconds * backoff_multiplier;
                let retry_delay = Duration::from_secs(retry_delay_seconds);

                log_warn!(
                    "RPC call {}: attempt {}/{} failed: {:#}; retrying in {} seconds",
                    operation,
                    attempt,
                    max_attempts,
                    error,
                    retry_delay_seconds
                );
                tokio::time::sleep(retry_delay).await;
                attempt += 1;
            }
        }
    }
}

fn is_rpc_error(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        // The transparent contract wrapper can hide TransportError from source().
        cause.is::<TransportError>()
            || matches!(
                cause.downcast_ref::<ContractError>(),
                Some(ContractError::TransportError(_))
            )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::transports::TransportErrorKind;
    use tokio::time::Instant;

    #[tokio::test(start_paused = true)]
    async fn retries_only_the_call_and_returns_its_value() {
        let mut attempts = 0;
        let started = Instant::now();
        let result = retry_rpc_call("queue head", || {
            attempts += 1;
            let attempt = attempts;
            async move {
                if attempt < 3 {
                    Err(ContractError::TransportError(
                        TransportErrorKind::http_error(503, "no healthy upstream".into()),
                    ))
                } else {
                    Ok(42)
                }
            }
        })
        .await
        .unwrap();

        assert_eq!(result, 42);
        assert_eq!(attempts, 3);
        assert_eq!(started.elapsed(), Duration::from_secs(15));
    }

    #[tokio::test(start_paused = true)]
    async fn stops_after_five_attempts_without_a_final_sleep() {
        let mut attempts = 0;
        let started = Instant::now();
        let error = retry_rpc_call("queue head", || {
            attempts += 1;
            async { Err::<(), _>(TransportErrorKind::http_error(502, "Bad Gateway".into())) }
        })
        .await
        .unwrap_err();

        assert_eq!(attempts, 5);
        assert_eq!(started.elapsed(), Duration::from_secs(75));
        assert!(error.downcast_ref::<TransportError>().is_some());
    }

    #[tokio::test(start_paused = true)]
    async fn does_not_retry_local_errors() {
        let mut attempts = 0;
        let started = Instant::now();
        let result = retry_rpc_call("queue head", || {
            attempts += 1;
            async { Err::<(), _>(ContractError::UnknownFunction("queue head".into())) }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempts, 1);
        assert_eq!(started.elapsed(), Duration::ZERO);
    }
}
