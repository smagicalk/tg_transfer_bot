//! 单飞防并发击穿（Singleflight）模块。
//!
//! # 核心职责
//! 桥接全局应用上下文（`AppContext`）中的 `InflightDownloadRegistry`，提供：
//! - 针对相同的 `file_key`（通常为远程文件标识），同一时刻只允许一个异步下载协程实际执行。
//! - 其他并发请求自动挂起等待首个协程执行完毕并共享结果，防止高并发场景下重复下载击穿带宽与本地存储。

use std::future::Future;

/// 使用 Singleflight 机制执行指定文件的下载任务。
///
/// 若已有相同 `file_key` 的任务在执行，当前协程将进入等待队列；
/// 当首个任务成功时，所有等待协程直接返回成功；若首个任务失败，则等待协程也会感知到失败。
///
/// # 参数
/// * `file_key` - 唯一标识文件的键值（例如 `tdlib_file_unique_id`）
/// * `task` - 实际下载逻辑闭包，仅在当前调用者抢占为 Executor 时被调用
///
/// # 返回
/// 任务成功完成返回 `Ok(())`，失败返回相应错误
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

    /// 验证当首个执行任务被强行中止（Abort）时，等待队列中的协程能被及时唤醒并报错，不会永久死锁。
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

