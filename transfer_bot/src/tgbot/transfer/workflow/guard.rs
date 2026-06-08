// 进程内互斥锁：
// - job_id 锁防止同一任务重复执行
// - source_link + target_chat_id 锁防止创建阶段穿透查重窗口

use std::collections::HashSet;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::Duration;

// 进程内任务互斥：防止同一个 job_id 被并发重复执行。
static RUNNING_JOB_IDS: LazyLock<Mutex<HashSet<i64>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

// 进程内创建阶段互斥：防止同一个 source_link + target_chat_id 同时穿透“查重后插入”窗口。
static CREATING_SOURCE_TARGETS: LazyLock<Mutex<HashSet<(String, i64)>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// 查询当前进程内是否已有某个任务正在执行。
pub(in crate::tgbot::transfer) async fn is_job_running_in_process(job_id: i64) -> bool {
    RUNNING_JOB_IDS
        .lock()
        .expect("running job id mutex poisoned")
        .contains(&job_id)
}

/// 获取任务互斥 guard（获取失败表示已有同任务在运行）。
pub(super) async fn acquire_job_guard(job_id: i64) -> Option<JobGuard> {
    let mut guard = RUNNING_JOB_IDS
        .lock()
        .expect("running job id mutex poisoned");
    if guard.contains(&job_id) {
        return None;
    }
    guard.insert(job_id);
    Some(JobGuard { job_id })
}

/// 获取 source-target 创建 guard。
///
/// 这里选择等待同 key 的创建完成，而不是直接返回 running：
/// 等待后可以重新读取数据库，拿到准确的历史完成/进行中任务。
pub(super) async fn acquire_source_target_create_guard(
    source_link: String,
    target_chat_id: i64,
) -> SourceTargetGuard {
    let key = (source_link, target_chat_id);
    loop {
        {
            let mut guard = CREATING_SOURCE_TARGETS
                .lock()
                .expect("source-target mutex poisoned");
            if guard.insert(key.clone()) {
                return SourceTargetGuard { key };
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// 任务互斥 guard，drop 时自动释放占用。
pub(super) struct JobGuard {
    job_id: i64,
}

impl Drop for JobGuard {
    fn drop(&mut self) {
        let mut guard = RUNNING_JOB_IDS
            .lock()
            .expect("running job id mutex poisoned");
        guard.remove(&self.job_id);
    }
}

/// source-target 创建互斥 guard，drop 时释放占用。
pub(super) struct SourceTargetGuard {
    key: (String, i64),
}

impl Drop for SourceTargetGuard {
    fn drop(&mut self) {
        let mut guard = CREATING_SOURCE_TARGETS
            .lock()
            .expect("source-target mutex poisoned");
        guard.remove(&self.key);
    }
}
