// `/transfer` 命令实现。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::config::ClientRole;
use crate::tgbot::send;
use crate::tgbot::transfer::card;

use super::build_downloads_status_button_data;
use super::common::{CommandStyle, downloads_command, lookup_command, resolve_target_chat_id};
use crate::tgbot::transfer::types::{SourceKind, TransferPlan};

/// `/transfer` 命令入口。
/// 命令格式：`/transfer <link> [target]`
pub async fn transfer_command(
    text: Vec<&str>,
    config: Arc<BotConfig>,
    request_message: &tdlib_rs::types::Message,
    client_id: i32,
) -> anyhow::Result<()> {
    let request_chat_id = request_message.chat_id;
    let request_message_id = request_message.id;
    let source = resolve_transfer_source(&text, request_message)?;
    run_transfer_plan(
        text,
        source,
        config,
        request_chat_id,
        request_message_id,
        client_id,
    )
    .await
}

/// 链接转存入口。
///
/// 菜单输入流已经收集到明确 source link 和 target，因此不需要依赖 TDLib reply 信息。
pub async fn transfer_link_command(
    text: Vec<&str>,
    config: Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    if text.len() < 2 {
        anyhow::bail!("usage: /transfer <link> [target]");
    }
    let source = ResolvedTransferSource {
        source_link: text[1].to_owned(),
        source_kind: SourceKind::Link,
        preferred_source_client_role: effective_link_source_role(&config),
        source_message_chat_id: None,
        source_message_id: None,
    };
    run_transfer_plan(
        text,
        source,
        config,
        request_chat_id,
        request_message_id,
        client_id,
    )
    .await
}

/// bot 收到可见媒体后自动转存。
///
/// 自动模式只使用配置默认目标，不解析用户输入；没有默认目标时调用方会得到错误并提示手动 `/t <target>`。
pub async fn transfer_bot_message_auto_command(
    config: Arc<BotConfig>,
    request_message: tdlib_rs::types::Message,
    client_id: i32,
) -> anyhow::Result<()> {
    let request_chat_id = request_message.chat_id;
    let request_message_id = request_message.id;
    let source = ResolvedTransferSource {
        source_link: bot_message_source_link(request_message.chat_id, request_message.id),
        source_kind: SourceKind::BotMessage,
        preferred_source_client_role: ClientRole::Bot,
        source_message_chat_id: Some(request_message.chat_id),
        source_message_id: Some(request_message.id),
    };
    run_transfer_plan(
        vec!["/t", "bot-message-source"],
        source,
        config,
        request_chat_id,
        request_message_id,
        client_id,
    )
    .await
}

/// 创建计划、发送进度卡片并派发后台任务。
async fn run_transfer_plan(
    text: Vec<&str>,
    mut source: ResolvedTransferSource,
    config: Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    // 链接源的策略是“bot 优先，user 备用”；如果当前配置没有 bot client，
    // 则直接降级 user，避免单 user 部署下因为缺少 bot client 失败。
    if source.source_kind == SourceKind::Link
        && source.preferred_source_client_role == ClientRole::Bot
        && !config.runtime_clients.contains_key(&ClientRole::Bot)
    {
        source.preferred_source_client_role = ClientRole::User;
    }

    let target_chat_id = resolve_transfer_target_chat_id(&text, &source, &config, request_chat_id)?;

    let plan = TransferPlan {
        source_link: source.source_link,
        source_kind: source.source_kind,
        preferred_source_client_role: source.preferred_source_client_role,
        source_message_chat_id: source.source_message_chat_id,
        source_message_id: source.source_message_id,
        target_chat_id,
        request_chat_id,
        request_message_id,
    };
    dispatch_transfer_plan(plan, config, request_chat_id, request_message_id, client_id).await
}

/// 发送初始回执并启动后台转存任务。
async fn dispatch_transfer_plan(
    plan: TransferPlan,
    config: Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    // 日志只记录请求定位和目标 chat；源链接会回显给用户，但不写入日志文件。
    tracing::info!(
        request_chat_id,
        request_message_id,
        target_chat_id = plan.target_chat_id,
        source_kind = plan.source_kind.as_str(),
        source_role = plan.preferred_source_client_role.as_str(),
        "transfer command accepted"
    );

    // 先给用户一个即时反馈，避免长时间下载/上传期间命令看起来像“卡住了”。
    let lookup_command =
        lookup_command(&plan.source_link, plan.target_chat_id, CommandStyle::Short);
    let progress_message = send::send_card_message_with_buttons_returning(
        format_transfer_accepted_text(&plan),
        request_chat_id,
        vec![vec![
            send::build_callback_button(
                "查看运行列表",
                &build_downloads_status_button_data("running", 8),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制源标识",
                &plan.source_link,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制查询命令",
                &lookup_command,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ]],
        client_id,
    )
    .await?;
    // 后台任务会持续编辑这条消息，把它变成转存进度面板。
    super::super::spawn_transfer_job(
        plan,
        request_chat_id,
        Some(progress_message.id),
        config.transfer_client_ids()?,
    );
    Ok(())
}

/// 解析 `/transfer` 的源输入。
///
/// 支持两种第一版输入：
/// - `/t <link> [target]`：链接源，优先 bot 读取，失败再 user；
/// - 回复 bot 可见媒体后发送 `/t [target]`：bot 消息源，直接读取被回复消息。
fn resolve_transfer_source(
    text: &[&str],
    request_message: &tdlib_rs::types::Message,
) -> anyhow::Result<ResolvedTransferSource> {
    if text.get(1).is_some_and(|arg| looks_like_telegram_link(arg)) {
        return Ok(ResolvedTransferSource {
            source_link: text[1].to_owned(),
            source_kind: SourceKind::Link,
            preferred_source_client_role: ClientRole::Bot,
            source_message_chat_id: None,
            source_message_id: None,
        });
    }

    if let Some((chat_id, message_id)) = replied_message_location(request_message) {
        return Ok(ResolvedTransferSource {
            source_link: bot_message_source_link(chat_id, message_id),
            source_kind: SourceKind::BotMessage,
            preferred_source_client_role: ClientRole::Bot,
            source_message_chat_id: Some(chat_id),
            source_message_id: Some(message_id),
        });
    }

    anyhow::bail!(
        "usage: /transfer <link> [target], or reply a bot-visible media message with /transfer [target]"
    )
}

/// bot 可见消息的稳定源标识。
fn bot_message_source_link(chat_id: i64, message_id: i64) -> String {
    format!("bot-message:{}:{}", chat_id, message_id)
}

/// 链接源优先使用 bot；如果当前配置没有启用 bot client，则自动退回 user。
fn effective_link_source_role(config: &BotConfig) -> ClientRole {
    if config.runtime_clients.contains_key(&ClientRole::Bot) {
        ClientRole::Bot
    } else {
        ClientRole::User
    }
}

/// 解析目标 chat。
///
/// 回复消息模式下 `/t archive` 的第 2 个参数是 target；链接模式下 `/t <link> archive`
/// 的第 3 个参数才是 target，因此这里需要按 source_kind 重新组装给公共解析器。
fn resolve_transfer_target_chat_id(
    text: &[&str],
    source: &ResolvedTransferSource,
    config: &BotConfig,
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    match source.source_kind {
        SourceKind::Link => resolve_target_chat_id(text, config, request_chat_id),
        SourceKind::BotMessage => {
            let target_args = if text.len() >= 2 {
                vec![text[0], "bot-message-source", text[1]]
            } else {
                vec![text[0], "bot-message-source"]
            };
            resolve_target_chat_id(&target_args, config, request_chat_id)
        }
    }
}

/// 命令解析出的源信息。
struct ResolvedTransferSource {
    source_link: String,
    source_kind: SourceKind,
    preferred_source_client_role: ClientRole,
    source_message_chat_id: Option<i64>,
    source_message_id: Option<i64>,
}

/// 粗略判断参数是否是 Telegram 链接；真正合法性仍由 spider 层负责。
fn looks_like_telegram_link(input: &str) -> bool {
    input.starts_with("https://t.me/")
        || input.starts_with("http://t.me/")
        || input.starts_with("t.me/")
}

/// 从命令消息中提取被回复消息定位。
fn replied_message_location(message: &tdlib_rs::types::Message) -> Option<(i64, i64)> {
    let tdlib_rs::enums::MessageReplyTo::Message(reply) = message.reply_to.as_ref()? else {
        return None;
    };
    let chat_id = if reply.chat_id != 0 {
        reply.chat_id
    } else {
        message.chat_id
    };
    if reply.message_id == 0 {
        return None;
    }
    Some((chat_id, reply.message_id))
}

/// 构造 `/transfer` 首次回执卡片。
///
/// 后台任务启动后会持续编辑同一条消息，因此初始卡片也使用 card 格式，避免样式闪变。
fn format_transfer_accepted_text(plan: &TransferPlan) -> String {
    [
        "已接收转存请求".to_owned(),
        card::status_target("queued", plan.target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("进度"),
        "后台会自动下载并上传，本消息会持续刷新。".to_owned(),
        card::section("命令"),
        card::command_line(
            "查询",
            lookup_command(&plan.source_link, plan.target_chat_id, CommandStyle::Short),
        ),
        card::command_line(
            "列表",
            downloads_command(Some("run"), None, None, CommandStyle::Short),
        ),
        String::new(),
    ]
    .into_iter()
    .chain(card::source_block(&plan.source_link))
    .collect::<Vec<_>>()
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{
        ResolvedTransferSource, bot_message_source_link, format_transfer_accepted_text,
        resolve_transfer_target_chat_id,
    };
    use crate::ClientRole;
    use crate::config::BotConfig;
    use crate::tgbot::transfer::types::{SourceKind, TransferPlan};

    // 首次回执应直接使用卡片标记，后续编辑不会从 Markdown 样式跳到 card 样式。
    #[test]
    fn test_format_transfer_accepted_text() {
        let text = format_transfer_accepted_text(&TransferPlan {
            source_link: "https://t.me/c/1/2".to_owned(),
            source_kind: SourceKind::Link,
            preferred_source_client_role: ClientRole::Bot,
            source_message_chat_id: None,
            source_message_id: None,
            target_chat_id: -100,
            request_chat_id: 1,
            request_message_id: 2,
        });

        assert!(text.contains("状态：‹queued›"));
        assert!(text.contains("目标：‹-100›"));
        assert!(text.contains("‹https://t.me/c/1/2›"));
    }

    // bot 可见消息源使用稳定伪链接参与查重，避免自动转存和回复命令生成两种 key。
    #[test]
    fn test_bot_message_source_link() {
        assert_eq!(bot_message_source_link(100, 200), "bot-message:100:200");
    }

    // 回复媒体模式下第二个参数应当被当成 target，而不是 source link。
    #[test]
    fn test_resolve_target_for_bot_message_source() {
        let mut config = BotConfig::default();
        config.target_aliases.insert("archive".to_owned(), -100);
        let source = ResolvedTransferSource {
            source_link: bot_message_source_link(10, 20),
            source_kind: SourceKind::BotMessage,
            preferred_source_client_role: ClientRole::Bot,
            source_message_chat_id: Some(10),
            source_message_id: Some(20),
        };

        let target = resolve_transfer_target_chat_id(&["/t", "archive"], &source, &config, 1)
            .expect("alias target should resolve");

        assert_eq!(target, -100);
    }
}
