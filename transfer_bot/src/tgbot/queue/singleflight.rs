use std::future::Future;

pub async fn run_singleflight<F, Fut>(file_key: String, task: F) -> anyhow::Result<()>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    crate::app_context::app_context()
        .inflight_downloads
        .run_singleflight(file_key, task)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_singleflight_executor_abort_unblocks_waiter() {
        let file_key = format!(
            "sf_abort_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should be valid")
                .as_nanos()
        );
        let started = Arc::new(tokio::sync::Notify::new());
        let started_for_executor = started.clone();
        let executor_key = file_key.clone();
        let executor = tokio::spawn(run_singleflight(executor_key, move || {
            let started = started_for_executor.clone();
            async move {
                started.notify_one();
                std::future::pending::<anyhow::Result<()>>().await
            }
        }));

        started.notified().await;

        let waiter_key = file_key.clone();
        let waiter = tokio::spawn(run_singleflight(waiter_key, || async {
            anyhow::bail!("waiter must not become executor while first task is inflight")
        }));

        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        executor.abort();
        let _ = executor.await;

        let waiter_result = tokio::time::timeout(std::time::Duration::from_secs(1), waiter)
            .await
            .expect("waiter should be unblocked")
            .expect("waiter task should not panic");
        assert!(waiter_result.is_err());

        run_singleflight(file_key, || async { Ok(()) })
            .await
            .expect("singleflight key should be reusable after abort");
    }
}
