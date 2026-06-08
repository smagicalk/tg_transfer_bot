// single-flight 下载去重。
// 同一 file_key 并发出现时，只有第一个调用者真正下载，其他调用者等待并复用结果。

use std::collections::HashMap;
use std::future::Future;
use std::sync::LazyLock;
use std::sync::Mutex;

// single-flight 广播值：执行者写入成功/失败，等待者读取后复用结果。
type DownloadResult = Result<(), String>;

// 每个 file_key 对应一个 watch 通道，用于唤醒等待同一文件的任务。
type DownloadNotifier = tokio::sync::watch::Sender<Option<DownloadResult>>;

// 全局 single-flight 表的真实结构。
type InflightDownloadMap = HashMap<String, DownloadNotifier>;

// 全局 single-flight 表：file_key -> 下载结果通知通道。
static INFLIGHT_DOWNLOADS: LazyLock<Mutex<InflightDownloadMap>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// 针对 `file_key` 执行“单飞下载”。
///
/// 行为：
/// - 若当前 key 没有进行中的下载：当前调用者成为执行者，真正执行 `task`。
/// - 若已有进行中的下载：当前调用者不重复下载，等待执行者结果。
/// - 结果会广播给所有等待者；失败会原样返回。
pub async fn run_singleflight<F, Fut>(file_key: String, task: F) -> anyhow::Result<()>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    // 先尝试注册自己为执行者，或拿到已有执行者的订阅。
    let role = {
        let mut guard = INFLIGHT_DOWNLOADS
            .lock()
            .expect("inflight downloads mutex poisoned");
        if let Some(tx) = guard.get(&file_key) {
            tracing::debug!(file_key = %file_key, "join inflight file download");
            SingleflightRole::Waiter(tx.subscribe())
        } else {
            let (tx, _rx) = tokio::sync::watch::channel(None);
            guard.insert(file_key.clone(), tx);
            tracing::debug!(file_key = %file_key, "start inflight file download");
            SingleflightRole::Executor(InflightExecutionGuard::new(file_key.clone()))
        }
    };

    // 当前调用者是执行者，执行下载任务并广播结果。
    // guard 的 Drop 会兜底清理全局表，避免 task 被取消或 panic 后留下永久占用。
    let mut rx = match role {
        SingleflightRole::Executor(mut execute_guard) => {
            let result = task().await;
            let send_value = result.as_ref().map(|_| ()).map_err(|e| format!("{:#}", e));
            if let Err(err) = &send_value {
                tracing::warn!(
                    file_key = %execute_guard.file_key,
                    error = %err,
                    "inflight file download failed"
                );
            } else {
                tracing::debug!(
                    file_key = %execute_guard.file_key,
                    "inflight file download completed"
                );
            }
            execute_guard.finish(send_value);
            return result;
        }
        SingleflightRole::Waiter(rx) => rx,
    };

    // 非执行者：等待执行者广播结果。
    loop {
        {
            let borrowed = rx.borrow();
            if let Some(value) = borrowed.as_ref() {
                return value
                    .as_ref()
                    .map(|_| ())
                    .map_err(|e| anyhow::anyhow!("{}", e));
            }
        }

        if rx.changed().await.is_err() {
            anyhow::bail!("singleflight channel closed unexpectedly");
        }
    }
}

/// single-flight 调用者角色。
enum SingleflightRole {
    /// 当前调用者负责真正执行下载。
    Executor(InflightExecutionGuard),
    /// 当前调用者只等待已有下载结果。
    Waiter(tokio::sync::watch::Receiver<Option<DownloadResult>>),
}

/// single-flight 执行者 guard。
///
/// 作用：
/// - 正常完成时负责广播结果并移除 inflight 记录；
/// - 异常 drop 时广播失败并移除 inflight 记录，避免等待者永久卡住。
struct InflightExecutionGuard {
    file_key: String,
    finished: bool,
}

impl InflightExecutionGuard {
    /// 创建执行者清理 guard。
    fn new(file_key: String) -> Self {
        Self {
            file_key,
            finished: false,
        }
    }

    /// 正常完成下载并广播结果。
    fn finish(&mut self, result: DownloadResult) {
        self.finished = true;
        self.remove_and_notify(result);
    }

    /// 从全局 inflight 表删除当前 key，并通知所有等待者。
    fn remove_and_notify(&self, result: DownloadResult) {
        let mut guard = INFLIGHT_DOWNLOADS
            .lock()
            .expect("inflight downloads mutex poisoned");
        if let Some(tx) = guard.remove(&self.file_key) {
            let _ = tx.send(Some(result));
        }
    }
}

impl Drop for InflightExecutionGuard {
    fn drop(&mut self) {
        if self.finished {
            return;
        }

        self.remove_and_notify(Err(
            "singleflight executor dropped before completion".to_owned()
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    /// single-flight 执行者被取消时，必须清理 inflight 表并唤醒等待者。
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

        // inflight 表已清理后，同一个 key 应允许新的执行者正常进入。
        run_singleflight(file_key, || async { Ok(()) })
            .await
            .expect("singleflight key should be reusable after abort");
    }
}
