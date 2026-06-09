// 实时下载进度表。
// 进度只用于当前进程内 `/downloads` 查询，不写入数据库，重启后由 TDLib 状态重新驱动。

use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::RwLock;

// 运行时下载进度表：(client_id, td_file_id) -> 最近一次 TDLib 上报的进度快照。
//
// TDLib file_id 只在单个 client 内有意义；bot/user 双 client 同时下载时必须把 client_id
// 放进 key，避免两个账号的同号文件进度互相覆盖。
static DOWNLOAD_PROGRESS: LazyLock<RwLock<HashMap<(i32, i32), DownloadProgressSnapshot>>> =
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
pub fn update_download_progress(client_id: i32, file: &tdlib_rs::types::File) {
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

    let key = (client_id, file.id);
    if file.local.is_downloading_completed {
        guard.remove(&key);
        return;
    }

    if !file.local.is_downloading_active && file.local.downloaded_size <= 0 {
        return;
    }

    guard.insert(
        key,
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
pub fn get_download_progress(client_id: i32, file_id: i32) -> Option<DownloadProgressSnapshot> {
    DOWNLOAD_PROGRESS
        .read()
        .expect("download progress rwlock poisoned")
        .get(&(client_id, file_id))
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 双 client 模式下 bot/user 可能拥有相同 td_file_id，进度必须按 client_id 隔离。
    #[test]
    fn test_download_progress_is_isolated_by_client_id() {
        let file_id = 42;
        update_download_progress(10, &test_file(file_id, 100, 1000, false, true));
        update_download_progress(20, &test_file(file_id, 700, 1000, false, true));

        let first = get_download_progress(10, file_id).expect("first client progress");
        let second = get_download_progress(20, file_id).expect("second client progress");

        assert_eq!(first.downloaded_size, 100);
        assert_eq!(second.downloaded_size, 700);
    }

    fn test_file(
        id: i32,
        downloaded_size: i64,
        size: i64,
        is_downloading_completed: bool,
        is_downloading_active: bool,
    ) -> tdlib_rs::types::File {
        tdlib_rs::types::File {
            id,
            size,
            expected_size: 0,
            local: tdlib_rs::types::LocalFile {
                path: String::new(),
                can_be_downloaded: true,
                can_be_deleted: true,
                is_downloading_active,
                is_downloading_completed,
                download_offset: 0,
                downloaded_prefix_size: 0,
                downloaded_size,
            },
            remote: tdlib_rs::types::RemoteFile {
                id: String::new(),
                unique_id: String::new(),
                is_uploading_active: false,
                is_uploading_completed: false,
                uploaded_size: 0,
            },
        }
    }
}
