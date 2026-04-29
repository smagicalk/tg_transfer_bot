// transfer 内部共用类型定义。

/// 一次转存任务的输入参数。
#[derive(Debug, Clone)]
pub(super) struct TransferPlan {
    /// 源链接（爬取入口）。
    pub source_link: String,
    /// 目标 chat_id。
    pub target_chat_id: i64,
    /// 请求侧：发起命令的 chat_id。
    pub request_chat_id: i64,
    /// 请求侧：发起命令的 message_id。
    pub request_message_id: i64,
}

/// 链接抓取后的源消息集合。
#[derive(Debug, Clone)]
pub(super) struct TransferBundle {
    /// 爬虫侧：源消息所属 chat。
    pub source_chat_id: i64,
    /// 爬虫侧：入口源消息 ID。
    pub source_message_id: i64,
    /// 爬虫侧：源相册 ID（非相册为 0）。
    pub source_album_id: i64,
    /// 待处理消息列表（单条或相册多条）。
    pub messages: Vec<tdlib_rs::types::Message>,
}
