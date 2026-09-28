//! 文件下载实时进度快照模块。
//!
//! # 核心职责
//! 桥接全局应用上下文（`AppContext`）中的 `DownloadProgressStore`，提供：
//! - 查询指定 TDLib 客户端下指定文件的下载大小、总大小及完成状态快照。
//! - 在测试环境下向进度存储写入模拟进度数据以供单元验证。

use crate::app_context::DownloadProgressSnapshot;

/// 更新指定客户端下指定文件的实时下载进度（仅在测试中使用）。
///
/// # 参数
/// * `client_id` - TDLib 客户端标识
/// * `file` - TDLib 返回的最新文件元数据对象
#[cfg(test)]
pub fn update_download_progress(client_id: i32, file: &tdlib_rs::types::File) {
    crate::app_context::app_context()
        .download_progress
        .update_download_progress(client_id, file);
}

/// 获取指定 TDLib 客户端下指定文件的当前下载进度快照。
///
/// # 参数
/// * `client_id` - 负责下载该文件的 TDLib 客户端标识
/// * `file_id` - TDLib 中的文件整型标识
///
/// # 返回
/// 若存在已记录的进度则返回 `Some(DownloadProgressSnapshot)`，否则返回 `None`
pub fn get_download_progress(client_id: i32, file_id: i32) -> Option<DownloadProgressSnapshot> {
    crate::app_context::app_context()
        .download_progress
        .get_download_progress(client_id, file_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试不同 TDLib client_id 之间的下载进度数据完全隔离，互不干扰。
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

    /// 构造测试用的 `tdlib_rs::types::File` 实例。
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
