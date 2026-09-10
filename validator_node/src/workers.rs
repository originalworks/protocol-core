use std::future::Future;
use tokio::task::JoinSet;

/// Both workers must remain running. Any completion stops the validator.
pub(crate) async fn supervise<A, P>(assignment: A, proving: P) -> anyhow::Result<()>
where
    A: Future<Output = anyhow::Result<()>> + Send + 'static,
    P: Future<Output = anyhow::Result<()>> + Send + 'static,
{
    // Dropping the set also aborts its workers if the supervisor is cancelled.
    let mut workers = JoinSet::new();
    let assignment_id = workers
        .spawn(async move { ("Assignment", assignment.await) })
        .id();
    workers.spawn(async move { ("Proving", proving.await) });

    let result = workers.join_next().await.expect("Two workers were started");
    workers.abort_all();

    match result {
        Ok((name, Ok(()))) => Err(anyhow::anyhow!("{name} worker exited unexpectedly")),
        Ok((name, Err(error))) => Err(error.context(format!("{name} worker failed"))),
        Err(error) => {
            let name = if error.id() == assignment_id {
                "Assignment"
            } else {
                "Proving"
            };
            Err(anyhow::Error::new(error)
                .context(format!("{name} worker panicked or was cancelled")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::sync::oneshot;

    struct NotifyOnDrop(Option<oneshot::Sender<()>>);

    impl Drop for NotifyOnDrop {
        fn drop(&mut self) {
            let _ = self.0.take().unwrap().send(());
        }
    }

    fn idle_worker() -> (
        impl Future<Output = anyhow::Result<()>> + Send,
        oneshot::Receiver<()>,
    ) {
        let (sender, receiver) = oneshot::channel();
        let guard = NotifyOnDrop(Some(sender));
        let worker = async move {
            let _guard = guard;
            std::future::pending().await
        };
        (worker, receiver)
    }

    async fn assert_sibling_stopped(stopped: oneshot::Receiver<()>) {
        tokio::time::timeout(Duration::from_secs(1), stopped)
            .await
            .unwrap()
            .unwrap();
    }

    #[tokio::test]
    async fn assignment_failure_stops_proving() {
        let (proving, stopped) = idle_worker();
        let error = supervise(async { anyhow::bail!("RPC retries exhausted") }, proving)
            .await
            .unwrap_err();
        assert!(format!("{error:#}").contains("RPC retries exhausted"));
        assert_sibling_stopped(stopped).await;
    }

    #[tokio::test]
    async fn proving_failure_stops_assignment() {
        let (assignment, stopped) = idle_worker();
        let error = supervise(assignment, async { anyhow::bail!("Proof failed") })
            .await
            .unwrap_err();
        assert!(format!("{error:#}").contains("Proof failed"));
        assert_sibling_stopped(stopped).await;
    }

    #[tokio::test]
    async fn worker_panics_are_reported_and_stop_the_sibling() {
        for assignment_panics in [true, false] {
            let (idle, stopped) = idle_worker();
            let panic_worker = async { panic!("worker panic") };
            let error = if assignment_panics {
                supervise(panic_worker, idle).await.unwrap_err()
            } else {
                supervise(idle, panic_worker).await.unwrap_err()
            };
            assert!(error
                .downcast_ref::<tokio::task::JoinError>()
                .unwrap()
                .is_panic());
            assert_sibling_stopped(stopped).await;
        }
    }

    #[tokio::test]
    async fn cancelling_the_supervisor_stops_both_workers() {
        let (assignment, assignment_stopped) = idle_worker();
        let (proving, proving_stopped) = idle_worker();
        let supervisor = tokio::spawn(supervise(assignment, proving));
        tokio::task::yield_now().await;
        supervisor.abort();
        assert!(supervisor.await.unwrap_err().is_cancelled());
        assert_sibling_stopped(assignment_stopped).await;
        assert_sibling_stopped(proving_stopped).await;
    }

    #[tokio::test]
    async fn unexpected_success_stops_the_sibling() {
        let (proving, stopped) = idle_worker();
        let error = supervise(async { Ok(()) }, proving).await.unwrap_err();
        assert!(error.to_string().contains("exited unexpectedly"));
        assert_sibling_stopped(stopped).await;
    }
}
