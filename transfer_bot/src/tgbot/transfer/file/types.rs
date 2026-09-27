// 文件准备阶段的共享类型。
// 这些结构会被下载、上传构建和 workflow 模块共同使用。

/// 上传项媒体类型枚举（用于相册兼容性校验与拆分）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot::transfer) enum UploadKind {
    /// GIF 动图或无声动画。
    Animation,
    /// 静态图片或相片。
    Photo,
    /// 视频文件。
    Video,
    /// 任意通用文档、压缩包或未识别媒体。
    Document,
    /// 音乐音频文件（含标签如歌手、歌名）。
    Audio,
    /// 语音备忘录或语音留言。
    Voice,
    /// 纯文本消息。
    Text,
}

/// 一条消息经下载落盘并完成输入结构包装后的上传准备数据。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct PreparedUpload {
    /// 上传消息内容枚举（可直接用于 TDLib 的 `sendMessage` 或 `sendMessageAlbum`）。
    pub input_content: tdlib_rs::enums::InputMessageContent,
    /// 上传媒体类型分类（用于媒体组校验与兼容性分段）。
    pub kind: UploadKind,
    /// 关联的本地缓存元数据（仅包含实际媒体文件的消息有值，纯文本等为 None）。
    pub cache_meta: Option<PreparedCacheMeta>,
}

/// 文件缓存回填与维护信息（用于 `file_cache` 数据库更新、引用计数与延迟 GC 删除）。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct PreparedCacheMeta {
    /// 跨任务的稳定唯一文件缓存键（基于 remote_id 或哈希计算）。
    pub file_key: String,
    /// TDLib 内部维护的文件句柄 ID（可用于调用 `delete_file` 释放缓存）。
    pub td_file_id: i32,
    /// 文件在本地磁盘上的绝对文件系统路径（用于直接物理删除或校验存在性）。
    pub local_path: String,
    /// 文件总大小（单位字节，若远端元数据未知则为 None）。
    pub size_bytes: Option<i64>,
}

/// 下载开始前即可从源消息元数据中解析出的文件种子信息。
///
/// 用于在文件真正下载完成前，提前在 `file_cache` 表中插入或占位 `td_file_id` 和预计大小。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct DownloadSeed {
    /// 跨任务稳定的唯一文件键 `file_key`。
    pub file_key: String,
    /// TDLib 客户端内部为该文件分配的文件对象 ID。
    pub td_file_id: i32,
    /// 文件预计大小（单位字节，未知时为 None）。
    pub size_bytes: Option<i64>,
}
