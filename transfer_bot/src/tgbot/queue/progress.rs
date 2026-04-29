// 实时下载进度表。
// 进度只用于当前进程内 `/downloads` 查询，不写入数据库，重启后由 TDLib 状态重新驱动。

use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::RwLock;

// 运行时下载进度表：td_file_id -> 最近一次 TDLib 上报的进度快照。
static DOWNLOAD_PROGRESS: LazyLock<RwLock<HashMap<i32, DownloadProgressSnapshot>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// 单个文件的实时下载快照。
#[derive(Debug, Clone, Default)]
pub struct DownloadProgressSnapshot {
    /// 当前已下载字节数。
    pub downloaded_size: i64,
    /// 总字节数（未知时为 None）。
    pub total_size: Option<i64>,
}

/// 根据 TDLib `Update::File` 刷新实时下载进度。
/// 规则：
/// - 下载完成后从内存表移除，避免无界增长；
/// - 下载中则保留最近一次 downloaded/total 快照。
pub fn update_download_progress(file: &tdlib_rs::types::File) {
    let total_size = if file.size > 0 {
        Some(file.size)
    } else if file.expected_size > 0 {
        Some(file.expected_size)
    } else {
        None
    };

    let mut guard = DOWNLOAD_PROGRESS
        .write()
        .expect("download progress rwlock poisoned");

    if file.local.is_downloading_completed {
        guard.remove(&file.id);
        return;
    }

    if !file.local.is_downloading_active && file.local.downloaded_size <= 0 {
        return;
    }

    guard.insert(
        file.id,
        DownloadProgressSnapshot {
            downloaded_size: file
                .local
                .downloaded_size
                .max(file.local.downloaded_prefix_size),
            total_size,
        },
    );
}

/// 获取单个文件的实时下载进度。
pub fn get_download_progress(file_id: i32) -> Option<DownloadProgressSnapshot> {
    DOWNLOAD_PROGRESS
        .read()
        .expect("download progress rwlock poisoned")
        .get(&file_id)
        .cloned()
}
