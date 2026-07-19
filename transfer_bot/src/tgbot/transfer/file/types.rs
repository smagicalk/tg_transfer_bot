// 文件准备阶段的共享类型。
// 这些结构会被下载、上传构建和 workflow 模块共同使用。

/// 上传项媒体类型（用于相册兼容性校验）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot::transfer) enum UploadKind {
    Animation,
    Photo,
    Video,
    Document,
    Audio,
    Voice,
    Text,
}

/// 一条消息准备后的上传数据。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct PreparedUpload {
    /// 上传消息内容（可直接用于 send_message / send_message_album）。
    pub input_content: tdlib_rs::enums::InputMessageContent,
    /// 上传类型（用于分组和兼容性判断）。
    pub kind: UploadKind,
    /// 缓存元信息（仅媒体消息有值）。
    pub cache_meta: Option<PreparedCacheMeta>,
}

/// 文件缓存回填信息（用于 file_cache 更新与延迟删除）。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct PreparedCacheMeta {
    /// 跨任务稳定 key。
    pub file_key: String,
    /// TDLib 文件 ID（用于 delete_file）。
    pub td_file_id: i32,
    /// 本地路径（用于后续直接删除）。
    pub local_path: String,
    /// 文件大小（未知时为 None）。
    pub size_bytes: Option<i64>,
}

/// 下载前即可确定的文件标识信息。
/// 用于在文件真正下载完成前，把 `td_file_id` 和预计大小写入 file_cache。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct DownloadSeed {
    /// 跨任务稳定 file_key。
    pub file_key: String,
    /// TDLib 文件 ID。
    pub td_file_id: i32,
    /// 文件大小（未知时为 None）。
    pub size_bytes: Option<i64>,
}
