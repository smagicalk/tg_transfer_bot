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
    pub(super) actor: crate::config::RequestActor,
    pub(super) client_id: i32,
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
                ctx.request_message_id,
                ctx.actor,
                ctx.client_id,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // Telegram 链接预检查只做粗筛，最终解析仍由 spider 负责。
    #[test]
    fn test_looks_like_telegram_link() {
        assert!(looks_like_telegram_link("https://t.me/c/1/2"));
        assert!(looks_like_telegram_link("t.me/c/1/2"));
        assert!(!looks_like_telegram_link("https://example.com"));
    }
}
