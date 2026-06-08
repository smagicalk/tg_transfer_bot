// 转存运行时配置与后台任务动态并发控制。
// `/config set` 修改配置后会刷新这里的内存值，等待中的后台任务会被唤醒重新抢占执行槽。

use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

// 转存运行配置：
// - 从 config.json 初始化一次
// - 供并发控制、延迟删除、GC 轮询等共享使用
static TRANSFER_RUNTIME_CONFIG: LazyLock<std::sync::RwLock<crate::config::TransferConfig>> =
    LazyLock::new(|| std::sync::RwLock::new(crate::config::TransferConfig::default()));

// TDLib 文件目录：
// - 只在启动时从 tdlib_config.files_directory 注入
// - GC 删除本地文件前用它做路径边界校验，避免数据库 local_path 被污染后误删其他文件
static TDLIB_FILES_DIRECTORY: LazyLock<std::sync::RwLock<Option<PathBuf>>> =
    LazyLock::new(|| std::sync::RwLock::new(None));

// 全局重任务动态并发限制：
// - 运行时按当前配置读取并发上限
// - 修改配置后无需重启即可生效
static ACTIVE_TRANSFER_JOBS: AtomicUsize = AtomicUsize::new(0);
static TRANSFER_SLOT_NOTIFY: LazyLock<tokio::sync::Notify> =
    LazyLock::new(tokio::sync::Notify::new);

/// 初始化转存运行配置。
/// 该函数应在读取 config.json 后调用一次。
pub fn init_runtime_config(
    config: crate::config::TransferConfig,
    tdlib_files_directory: impl Into<PathBuf>,
) {
    update_runtime_config(config);
    update_tdlib_files_directory(tdlib_files_directory.into());
}

/// 更新转存运行配置。
/// `/config set` 修改成功后，会立即刷新这里的内存值。
pub fn update_runtime_config(config: crate::config::TransferConfig) {
    if let Ok(mut guard) = TRANSFER_RUNTIME_CONFIG.write() {
        *guard = config;
    }
    // 配置变化后唤醒等待中的后台任务，重新按最新并发上限抢占执行槽。
    TRANSFER_SLOT_NOTIFY.notify_waiters();
}

/// 更新 TDLib 文件目录。
///
/// 该目录属于启动级配置，不开放 `/config set` 动态修改；如果配置为空，GC 会拒绝
/// 直接 `remove_file`，只尝试 TDLib 自身的 `delete_file`。
fn update_tdlib_files_directory(path: PathBuf) {
    if let Ok(mut guard) = TDLIB_FILES_DIRECTORY.write() {
        *guard = if path.as_os_str().is_empty() {
            None
        } else {
            Some(path)
        };
    }
}

/// 获取运行时转存配置。
pub(in crate::tgbot::transfer) fn runtime_config() -> crate::config::TransferConfig {
    TRANSFER_RUNTIME_CONFIG
        .read()
        .expect("transfer runtime config rwlock poisoned")
        .clone()
}

/// 获取 TDLib 文件目录。
pub(in crate::tgbot::transfer) fn tdlib_files_directory() -> Option<PathBuf> {
    TDLIB_FILES_DIRECTORY
        .read()
        .expect("tdlib files directory rwlock poisoned")
        .clone()
}

/// 读取转存重任务并发数。
/// 从 config.json 读取 `transfer_config.job_concurrency`。
fn transfer_job_concurrency() -> usize {
    runtime_config().job_concurrency.max(1)
}

/// 获取一个后台任务执行槽。
/// 这是动态并发限制的核心：
/// - 当前活跃任务数小于配置上限时立即进入
/// - 否则等待，直到有任务结束或配置放宽
pub(in crate::tgbot::transfer) async fn acquire_transfer_slot() -> TransferExecGuard {
    loop {
        let limit = transfer_job_concurrency();
        let active = ACTIVE_TRANSFER_JOBS.load(Ordering::SeqCst);
        if active < limit {
            if ACTIVE_TRANSFER_JOBS
                .compare_exchange(active, active + 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return TransferExecGuard;
            }
            continue;
        }
        TRANSFER_SLOT_NOTIFY.notified().await;
    }
}

/// 后台任务执行槽 guard。
/// drop 时释放活跃计数并唤醒等待者。
pub(in crate::tgbot::transfer) struct TransferExecGuard;

impl Drop for TransferExecGuard {
    fn drop(&mut self) {
        ACTIVE_TRANSFER_JOBS.fetch_sub(1, Ordering::SeqCst);
        TRANSFER_SLOT_NOTIFY.notify_one();
    }
}
