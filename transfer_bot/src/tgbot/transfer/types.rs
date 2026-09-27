// transfer 内部共用类型定义。

use crate::config::{ClientRole, RequestActor};

/// 源消息输入类型枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceKind {
    /// 命令参数里直接提供了 Telegram 消息链接（形如 `https://t.me/c/123/456` 或公开链接）。
    Link,
    /// 用户通过回复（Reply）Bot 所在当前聊天中的某条消息来触发转存。
    BotMessage,
}

impl SourceKind {
    /// 将源类型转换为存入数据库的固定字符串标识。
    ///
    /// # 返回值
    /// - `"link"`: 对应 `SourceKind::Link`。
    /// - `"bot_message"`: 对应 `SourceKind::BotMessage`。
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Link => "link",
            Self::BotMessage => "bot_message",
        }
    }

    /// 从数据库字符串值解析恢复源输入类型枚举。
    ///
    /// # 参数
    /// - `value`: 存储在数据库列中的字符串。
    ///
    /// # 返回值
    /// - `Some(SourceKind)`: 匹配成功的枚举值。
    /// - `None`: 未知非法字符串。
    pub(super) fn from_str(value: &str) -> Option<Self> {
        match value {
            "link" => Some(Self::Link),
            "bot_message" => Some(Self::BotMessage),
            _ => None,
        }
    }
}

/// 将客户端角色转换为存入数据库的固定字符串。
///
/// # 参数
/// - `role`: 客户端角色（User 或 Bot）。
///
/// # 返回值
/// - `"user"` 或 `"bot"`。
pub(super) fn client_role_as_str(role: ClientRole) -> &'static str {
    role.as_str()
}

/// 从数据库持久化的客户端角色字符串解析恢复 `ClientRole` 枚举。
///
/// # 参数
/// - `value`: 角色字符串。
///
/// # 返回值
/// - `Some(ClientRole)`: 对应的客户端角色枚举。
/// - `None`: 无法识别的角色类型。
pub(super) fn client_role_from_str(value: &str) -> Option<ClientRole> {
    match value {
        "user" => Some(ClientRole::User),
        "bot" => Some(ClientRole::Bot),
        _ => None,
    }
}

/// 一次转存任务的核心输入规划参数结构体。
#[derive(Debug, Clone)]
pub(crate) struct TransferPlan {
    /// 发起本任务的请求者身份上下文（所有者或白名单授权用户）。
    pub actor: RequestActor,
    /// 源链接地址（爬虫抓取的起始入口链接）。
    pub source_link: String,
    /// 源消息的输入形式（链接爬取还是 Bot 聊天内回复）。
    pub source_kind: SourceKind,
    /// 当前计划优先使用哪个客户端角色读取源消息（例如优先使用 Bot 还是 User）。
    pub preferred_source_client_role: ClientRole,
    /// 是否允许在首选角色（如 Bot）读取私有源或失败时，自动回退到用户客户端（User）继续读取。
    ///
    /// 只有具备所有者权限的请求才允许 fallback 到 user 客户端处理私有群/频道。
    pub allow_user_fallback: bool,
    /// 当 `source_kind` 为 `BotMessage` 时，记录源消息所在会话的 `chat_id`；若为普通链接源则为 `None`。
    pub source_message_chat_id: Option<i64>,
    /// 当 `source_kind` 为 `BotMessage` 时，记录源消息的 `message_id`；若为普通链接源则为 `None`。
    pub source_message_id: Option<i64>,
    /// 转存的目标 Telegram 聊天 ID（频道或群组）。
    pub target_chat_id: i64,
    /// 发起转存命令请求的会话 ID（通常为用户与 Bot 的私聊或指令触发群组）。
    pub request_chat_id: i64,
    /// 发起转存命令请求的具体消息 ID（用于回复或定位进度卡片）。
    pub request_message_id: i64,
    /// 用户是否明确指定 `--force` 忽略历史相同消息的转存成功缓存，强制重新拉取并转发。
    pub force_retransfer: bool,
}

/// 爬虫抓取解析后的源消息数据包（可能包含单条消息或多条相册媒体消息）。
#[derive(Debug, Clone)]
pub(super) struct TransferBundle {
    /// 最终成功读取该批源消息的 TDLib 客户端角色。
    pub source_client_role: ClientRole,
    /// 爬虫解析出的源消息所属会话 ID。
    pub source_chat_id: i64,
    /// 爬虫解析出的入口首条源消息 ID。
    pub source_message_id: i64,
    /// 爬虫解析出的相册媒体组 ID（非媒体相册单条消息时为 0）。
    pub source_album_id: i64,
    /// 待转存处理的具体 TDLib 消息对象列表（若为相册则包含整个组内的所有子消息）。
    pub messages: Vec<tdlib_rs::types::Message>,
}
