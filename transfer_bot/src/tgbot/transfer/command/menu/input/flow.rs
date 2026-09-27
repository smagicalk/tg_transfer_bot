//! `/menu` 多步骤流式输入向导基础辅助模块。
//!
//! 聚焦于转存（Transfer）与查重（Lookup）交互向导共用的底层能力，包括已有命令执行上下文封装、
//! 来源入口溯源（文本输入 vs 按钮回调）、Telegram 链接及 Bot 内部消息源格式前置初筛等。

use std::sync::Arc;

use crate::config::BotConfig;

use super::state::MenuInputKind;
use crate::tgbot::transfer::command::{lookup, transfer_cmd};

/// 多步骤流式向导最终触发调用既有命令时的共享上下文结构体。
///
/// 聚合了请求会话定位、触发消息定位、操作者身份与 TDLib 客户端标识，
/// 避免底层执行函数参数列表过度膨胀。
pub(super) struct ExistingCommandContext {
    /// 全局应用上下文强引用
    pub(super) app: std::sync::Arc<crate::app_context::AppContext>,
    /// 触发本次交互请求所在的会话聊天 ID
    pub(super) request_chat_id: i64,
    /// 触发本次交互请求的用户消息 ID（用于幂等定位）
    pub(super) request_message_id: i64,
    /// 命令触发源头分类（区分来自文本输入还是内联按钮回调）
    pub(super) origin: ExistingCommandOrigin,
    /// 当前操作者的权限与角色上下文
    pub(super) actor: crate::config::RequestActor,
    /// TDLib 客户端实例标识符
    pub(super) client_id: i32,
}

/// 现有命令触发入口来源类型枚举。
///
/// 关键设计区别：
/// - `CallbackMessage(message_id)`: 来自用户点击内联按钮，携带机器人自身发送的卡片消息 ID，支持原地编辑（`editMessageText`）；
/// - `TextInput`: 来自用户直接回复的普通文本消息，携带的是用户的消息 ID，绝不能作为卡片消息 ID 进行编辑，只能追加新回复。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ExistingCommandOrigin {
    /// 来自用户发送的纯文本消息输入
    TextInput,
    /// 来自用户点击机器人卡片上的 Callback 按钮，包含该卡片消息 ID
    CallbackMessage(i64),
}

impl ExistingCommandOrigin {
    /// 获取可被原地编辑的机器人交互卡片消息 ID。
    ///
    /// 若来源是文本输入则返回 `None`；若来源是内联按钮回调则返回 `Some(message_id)`。
    fn interaction_message_id(self) -> Option<i64> {
        match self {
            Self::TextInput => None,
            Self::CallbackMessage(message_id) => Some(message_id),
        }
    }
}

/// 调用底层的既有命令入口执行转存或查重业务逻辑。
///
/// # 参数说明
/// - `kind`: 菜单输入流程大类（转存、默认转存、查重、默认查重）
/// - `command_owned`: 组装完成的命令行参数字符串向量
/// - `config`: 当前机器人配置快照
/// - `ctx`: 共享执行上下文对象
pub(super) async fn run_existing_command(
    kind: MenuInputKind,
    command_owned: Vec<String>,
    config: Arc<BotConfig>,
    ctx: ExistingCommandContext,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    match kind {
        // 转存流程：分发给 transfer_cmd 模块
        MenuInputKind::Transfer | MenuInputKind::TransferDefault => {
            transfer_cmd::transfer_link_command_on(
                ctx.app,
                command_refs,
                config,
                ctx.request_chat_id,
                transfer_cmd::TransferCommandContext {
                    request_message_id: ctx.request_message_id,
                    interaction_message_id: ctx.origin.interaction_message_id(),
                    actor: ctx.actor,
                    client_id: ctx.client_id,
                },
            )
            .await
        }
        // 查重流程：分发给 lookup 模块
        MenuInputKind::Lookup | MenuInputKind::LookupDefault => {
            lookup::lookup_command_on(
                ctx.app.as_ref(),
                command_refs,
                config,
                ctx.actor,
                ctx.client_id,
            )
            .await
        }
    }
}

/// 粗略前置校验输入文本是否具有 Telegram 消息链接或合法消息源的基本外观。
///
/// 仅用于在用户输入的第一时间拦截显而易见的不合法输入（如任意普通网页链接）；
/// 严格的链接合法性、消息存在性与实体解析仍交由 spider 爬虫层完成。
///
/// # 参数说明
/// - `input`: 用户输入的文本切片
///
/// # 返回值
/// 若符合基本链接前缀特征则返回 true，否则返回 false。
pub(super) fn looks_like_telegram_link(input: &str) -> bool {
    input.starts_with("https://t.me/")
        || input.starts_with("http://t.me/")
        || input.starts_with("t.me/")
        || parse_bot_message_source(input).is_some()
}

/// 解析 Bot 可见消息的稳定内部标识符（格式形如 `bot-message:<chat_id>:<message_id>`）。
///
/// # 参数说明
/// - `input`: 待解析的标识符文本
///
/// # 返回值
/// 若成功解析出两个合法整数则返回 `Some((chat_id, message_id))`，否则返回 `None`。
pub(super) fn parse_bot_message_source(input: &str) -> Option<(i64, i64)> {
    let payload = input.strip_prefix("bot-message:")?;
    let (chat_id, message_id) = payload.split_once(':')?;
    let chat_id = chat_id.parse::<i64>().ok()?;
    let message_id = message_id.parse::<i64>().ok()?;
    Some((chat_id, message_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 Telegram 链接的初步外观过滤能力：
    /// 包含标准链接、无协议头短链接、bot-message 内部标识，以及非 TG 外部链接的拒绝。
    #[test]
    fn test_looks_like_telegram_link() {
        assert!(looks_like_telegram_link("https://t.me/c/1/2"));
        assert!(looks_like_telegram_link("t.me/c/1/2"));
        assert!(looks_like_telegram_link("bot-message:-100123:456"));
        assert!(!looks_like_telegram_link("https://example.com"));
    }

    /// 测试 Bot 可见消息源前缀字符串的解构解析。
    #[test]
    fn test_parse_bot_message_source() {
        assert_eq!(
            parse_bot_message_source("bot-message:-100123:456"),
            Some((-100123, 456))
        );
        assert_eq!(parse_bot_message_source("bot-message:bad:456"), None);
        assert_eq!(parse_bot_message_source("https://t.me/c/1/2"), None);
    }

    /// 测试交互消息来源分类：
    /// 确保纯文本输入不暴露交互消息 ID（防止误调用编辑），而 Callback 来源能准确返回卡片消息 ID。
    #[test]
    fn test_existing_command_origin_separates_user_input_from_bot_card() {
        assert_eq!(
            ExistingCommandOrigin::TextInput.interaction_message_id(),
            None
        );
        assert_eq!(
            ExistingCommandOrigin::CallbackMessage(321_912_832).interaction_message_id(),
            Some(321_912_832)
        );
    }
}
