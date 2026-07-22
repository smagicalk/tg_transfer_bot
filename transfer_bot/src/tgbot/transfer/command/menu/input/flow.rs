// `/menu` 中的多步向导逻辑辅助函数。
// 这里聚焦转存/查询流程共用的基础能力，不处理 job_id 或 user_id 这类单步输入。

use std::sync::Arc;

use crate::config::BotConfig;

use super::state::MenuInputKind;
use crate::tgbot::transfer::command::{lookup, transfer_cmd};

/// 多步向导最终执行现有命令时的共享上下文。
///
/// 转存与查询都需要同一组请求定位和 actor 信息，收拢后可以避免 helper 参数继续膨胀。
pub(super) struct ExistingCommandContext {
    pub(super) app: std::sync::Arc<crate::app_context::AppContext>,
    pub(super) request_chat_id: i64,
    pub(super) request_message_id: i64,
    pub(super) origin: ExistingCommandOrigin,
    pub(super) actor: crate::config::RequestActor,
    pub(super) client_id: i32,
}

/// 现有命令由哪种交互入口触发。
///
/// callback 携带的是机器人卡片 ID，可以继续原地编辑；文本输入携带的是用户消息 ID，
/// 只能作为请求幂等定位，绝不能传给 `editMessageText`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ExistingCommandOrigin {
    TextInput,
    CallbackMessage(i64),
}

impl ExistingCommandOrigin {
    fn interaction_message_id(self) -> Option<i64> {
        match self {
            Self::TextInput => None,
            Self::CallbackMessage(message_id) => Some(message_id),
        }
    }
}

/// 调用已有命令入口，避免菜单输入流复制转存/查询业务逻辑。
pub(super) async fn run_existing_command(
    kind: MenuInputKind,
    command_owned: Vec<String>,
    config: Arc<BotConfig>,
    ctx: ExistingCommandContext,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    match kind {
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

/// 粗略判断是否是 Telegram 消息链接。
///
/// 真正合法性仍由 spider 层解析；这里仅避免明显错误输入推进到下一步。
pub(super) fn looks_like_telegram_link(input: &str) -> bool {
    input.starts_with("https://t.me/")
        || input.starts_with("http://t.me/")
        || input.starts_with("t.me/")
        || parse_bot_message_source(input).is_some()
}

/// 解析 bot 可见消息的稳定源标识。
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

    // Telegram 链接预检查只做粗筛，最终解析仍由 spider 负责。
    #[test]
    fn test_looks_like_telegram_link() {
        assert!(looks_like_telegram_link("https://t.me/c/1/2"));
        assert!(looks_like_telegram_link("t.me/c/1/2"));
        assert!(looks_like_telegram_link("bot-message:-100123:456"));
        assert!(!looks_like_telegram_link("https://example.com"));
    }

    #[test]
    fn test_parse_bot_message_source() {
        assert_eq!(
            parse_bot_message_source("bot-message:-100123:456"),
            Some((-100123, 456))
        );
        assert_eq!(parse_bot_message_source("bot-message:bad:456"), None);
        assert_eq!(parse_bot_message_source("https://t.me/c/1/2"), None);
    }

    /// 用户输入消息不可编辑；只有 callback 所在的机器人卡片能作为交互消息。
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
