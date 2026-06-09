// transfer 内部共用类型定义。

use crate::config::ClientRole;

/// 源输入类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SourceKind {
    /// 命令里提供 Telegram 消息链接。
    Link,
    /// 命令回复 bot 当前聊天中的一条消息。
    BotMessage,
}

impl SourceKind {
    /// 数据库存储值。
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Link => "link",
            Self::BotMessage => "bot_message",
        }
    }

    /// 从数据库值恢复。
    pub(super) fn from_str(value: &str) -> Option<Self> {
        match value {
            "link" => Some(Self::Link),
            "bot_message" => Some(Self::BotMessage),
            _ => None,
        }
    }
}

/// 数据库里保存的 client role 字符串。
pub(super) fn client_role_as_str(role: ClientRole) -> &'static str {
    role.as_str()
}

/// 从数据库里的 client role 字符串恢复枚举。
pub(super) fn client_role_from_str(value: &str) -> Option<ClientRole> {
    match value {
        "user" => Some(ClientRole::User),
        "bot" => Some(ClientRole::Bot),
        _ => None,
    }
}

/// 一次转存任务的输入参数。
#[derive(Debug, Clone)]
pub(super) struct TransferPlan {
    /// 源链接（爬取入口）。
    pub source_link: String,
    /// 源输入类型。
    pub source_kind: SourceKind,
    /// 当前计划优先使用哪个 client 读取源消息。
    pub preferred_source_client_role: ClientRole,
    /// bot 可见消息源的 chat_id；链接源为空。
    pub source_message_chat_id: Option<i64>,
    /// bot 可见消息源的 message_id；链接源为空。
    pub source_message_id: Option<i64>,
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
    /// 实际读取源消息的 client 角色。
    pub source_client_role: ClientRole,
    /// 爬虫侧：源消息所属 chat。
    pub source_chat_id: i64,
    /// 爬虫侧：入口源消息 ID。
    pub source_message_id: i64,
    /// 爬虫侧：源相册 ID（非相册为 0）。
    pub source_album_id: i64,
    /// 待处理消息列表（单条或相册多条）。
    pub messages: Vec<tdlib_rs::types::Message>,
}
