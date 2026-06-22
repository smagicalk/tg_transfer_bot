// `/transfer` 命令实现。

use std::sync::Arc;

use crate::config::ClientRole;
use crate::config::{BotConfig, RequestActor};
use crate::tgbot::send;
use crate::tgbot::transfer::billing_runtime_config_on;
use crate::tgbot::transfer::card;

use super::build_downloads_status_button_data;
use super::build_menu_home_button_data;
use super::common::{CommandStyle, downloads_command, lookup_command, resolve_target_chat_id_on};
use super::menu;
use crate::tgbot::transfer::types::{SourceKind, TransferPlan};

/// 在指定上下文上执行 `/transfer` 命令。
pub async fn transfer_command_on(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    text: Vec<&str>,
    config: Arc<BotConfig>,
    request_message: &tdlib_rs::types::Message,
    actor: RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let request_chat_id = request_message.chat_id;
    let request_message_id = request_message.id;
    let source = match resolve_transfer_source(&text, request_message) {
        Ok(source) => source,
        Err(_) if text.len() == 1 => {
            menu::start_transfer_input_from_command(request_chat_id, actor.user_id, client_id)
                .await?;
            tracing::debug!(
                request_chat_id,
                request_message_id,
                sender_user_id = actor.user_id,
                "transfer command without args entered interactive wizard"
            );
            return Ok(());
        }
        Err(err) => return Err(err),
    };
    run_transfer_plan_on(
        app_context,
        text,
        source,
        config,
        request_message_id,
        actor,
        client_id,
    )
    .await
}

/// 在指定上下文上执行菜单/向导收集好的链接转存。
pub(in crate::tgbot::transfer::command) async fn transfer_link_command_on(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    text: Vec<&str>,
    config: Arc<BotConfig>,
    _request_chat_id: i64,
    request_message_id: i64,
    actor: RequestActor,
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
    run_transfer_plan_on(
        app_context,
        text,
        source,
        config,
        request_message_id,
        actor,
        client_id,
    )
    .await
}

/// 在指定上下文上创建计划、发送进度卡片并派发后台任务。
async fn run_transfer_plan_on(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    text: Vec<&str>,
    mut source: ResolvedTransferSource,
    config: Arc<BotConfig>,
    request_message_id: i64,
    actor: RequestActor,
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

    let target_chat_id = resolve_transfer_target_chat_id_on(
        app_context.as_ref(),
        &text,
        &source,
        &config,
        actor.request_chat_id,
    )?;

    let plan = TransferPlan {
        billing: billing_runtime_config_on(app_context.as_ref()),
        actor,
        source_link: source.source_link,
        source_kind: source.source_kind,
        preferred_source_client_role: source.preferred_source_client_role,
        allow_user_fallback: actor.is_admin(),
        source_message_chat_id: source.source_message_chat_id,
        source_message_id: source.source_message_id,
        target_chat_id,
        request_chat_id: actor.request_chat_id,
        request_message_id,
    };
    dispatch_transfer_plan(
        app_context,
        plan,
        config,
        actor.request_chat_id,
        request_message_id,
        client_id,
    )
    .await
}

/// 发送初始回执并启动后台转存任务。
async fn dispatch_transfer_plan(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
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
        owner_user_id = plan.actor.user_id,
        actor_role = plan.actor.role.as_str(),
        "transfer command accepted"
    );

    // 先给用户一个即时反馈，避免长时间下载/上传期间命令看起来像“卡住了”。
    let progress_message = send::send_card_message_with_buttons_returning(
        format_transfer_accepted_text(&plan),
        request_chat_id,
        build_transfer_accepted_button_rows(&plan.source_link),
        client_id,
    )
    .await?;
    // 后台任务会持续编辑这条消息，把它变成转存进度面板。
    super::super::spawn_transfer_job(
        app_context,
        plan,
        request_chat_id,
        Some(progress_message.id),
        config.transfer_client_ids()?,
    );
    Ok(())
}

/// 构造 `/transfer` 首次回执按钮。
///
/// 查询命令已经在正文里保留；按钮区优先放可直接点击的运行列表和菜单，
/// 只保留源标识复制，方便用户排查 bot-message 伪链接或原始源链接。
fn build_transfer_accepted_button_rows(
    source_link: &str,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "查看运行列表",
                &build_downloads_status_button_data("running", 8),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![send::build_copy_button(
            "复制源标识",
            source_link,
            tdlib_rs::enums::ButtonStyle::Default,
        )],
    ]
}

/// 解析 `/transfer` 的源输入。
///
/// 支持两种输入：
/// - `/transfer <link> [target]`：链接源，优先 bot 读取，失败再 user；
/// - 回复 bot 可见媒体后发送 `/transfer [target]`：bot 消息源，直接读取被回复消息。
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

    if let Some((chat_id, message_id)) = forwarded_message_location(request_message) {
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
/// 回复消息模式下 `/transfer archive` 的第 2 个参数是 target；
/// 链接模式下 `/transfer <link> archive` 的第 3 个参数才是 target，
/// 因此这里需要按 source_kind 重新组装给公共解析器。
#[cfg(test)]
fn resolve_transfer_target_chat_id(
    text: &[&str],
    source: &ResolvedTransferSource,
    _config: &BotConfig,
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    let app_context = crate::app_context::app_context();
    resolve_transfer_target_chat_id_on(app_context.as_ref(), text, source, _config, request_chat_id)
}

/// 在指定上下文上解析 `/transfer` 目标 chat。
fn resolve_transfer_target_chat_id_on(
    app: &crate::app_context::AppContext,
    text: &[&str],
    source: &ResolvedTransferSource,
    _config: &BotConfig,
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    match source.source_kind {
        SourceKind::Link => resolve_target_chat_id_on(app, text, request_chat_id),
        SourceKind::BotMessage => {
            let target_args = if text.len() >= 2 {
                vec![text[0], "bot-message-source", text[1]]
            } else {
                vec![text[0], "bot-message-source"]
            };
            resolve_target_chat_id_on(app, &target_args, request_chat_id)
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

/// 从转发消息中提取原始消息定位。
///
/// 优先使用 TDLib 给出的 `forward_info.source`，因为它包含“上一次转发来源”的真实 chat/message；
/// 如果没有 source，再退回 channel origin 的 `chat_id/message_id`。匿名来源或普通用户来源没有稳定
/// message_id 时不返回，避免生成伪源标识。
fn forwarded_message_location(message: &tdlib_rs::types::Message) -> Option<(i64, i64)> {
    let forward = message.forward_info.as_ref()?;
    if let Some(source) = &forward.source
        && source.chat_id != 0
        && source.message_id != 0
    {
        return Some((source.chat_id, source.message_id));
    }

    match &forward.origin {
        tdlib_rs::enums::MessageOrigin::Channel(channel)
            if channel.chat_id != 0 && channel.message_id != 0 =>
        {
            Some((channel.chat_id, channel.message_id))
        }
        _ => None,
    }
}

/// 对外提供统一的“媒体消息源定位”提取。
///
/// 优先级：
/// 1. reply_to 指向的原消息
/// 2. forwarded 来源里可还原的原始 chat/message
/// 3. 当前 bot 可见消息本身
///
/// 特殊规则：
/// - 如果消息本身是 forwarded，但转发来源无法还原稳定 message_id，则返回 None，交给上层提示用户
///   改用消息链接或回复 bot 可见媒体，避免把“转发壳消息”的新 message_id 当成伪源。
pub(in crate::tgbot) fn transferable_message_source_location(
    message: &tdlib_rs::types::Message,
) -> Option<(i64, i64)> {
    if let Some(reply) = replied_message_location(message) {
        return Some(reply);
    }

    if message.forward_info.is_some() {
        return forwarded_message_location(message);
    }

    Some((message.chat_id, message.id))
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
            lookup_command(&plan.source_link, plan.target_chat_id, CommandStyle::Long),
        ),
        card::command_line(
            "列表",
            downloads_command(Some("run"), None, None, CommandStyle::Long),
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
        ResolvedTransferSource, bot_message_source_link, build_transfer_accepted_button_rows,
        format_transfer_accepted_text, forwarded_message_location, resolve_transfer_target_chat_id,
    };
    use crate::ClientRole;
    use crate::app_context::app_context;
    use crate::config::{ActorRole, BillingConfig, BotConfig, RequestActor};
    use crate::tgbot::transfer::types::{SourceKind, TransferPlan};

    fn install_target_runtime(
        targets: crate::config::TargetsConfig,
        access_control: crate::config::AccessControlConfig,
    ) {
        let app = app_context();
        app.targets_runtime.update_runtime_config(targets);
        app.access_control_runtime
            .update_runtime_config(access_control);
    }

    // 首次回执应直接使用卡片标记，后续编辑不会从 Markdown 样式跳到 card 样式。
    #[test]
    fn test_format_transfer_accepted_text() {
        let text = format_transfer_accepted_text(&TransferPlan {
            actor: RequestActor {
                request_chat_id: 1,
                user_id: 1,
                role: ActorRole::Admin,
            },
            source_link: "https://t.me/c/1/2".to_owned(),
            source_kind: SourceKind::Link,
            preferred_source_client_role: ClientRole::Bot,
            allow_user_fallback: true,
            billing: BillingConfig::default(),
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

    // 首次回执按钮应直接跳运行列表和菜单，查询命令留在正文，避免按钮区重复复制命令。
    #[test]
    fn test_build_transfer_accepted_button_rows() {
        let rows = build_transfer_accepted_button_rows("https://t.me/c/1/2");
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(rows[0][0].text, "查看运行列表");
        assert_eq!(rows[0][1].text, "菜单");
        assert_eq!(rows[1][0].text, "复制源标识");
        assert!(!labels.contains(&"复制查询命令"));
        assert!(matches!(
            rows[0][0].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    // bot 可见消息源使用稳定伪链接参与查重，避免自动转存和回复命令生成两种 key。
    #[test]
    fn test_bot_message_source_link() {
        assert_eq!(bot_message_source_link(100, 200), "bot-message:100:200");
    }

    #[test]
    fn test_forwarded_message_location_prefers_forward_source() {
        let message = tdlib_rs::types::Message {
            id: 1,
            sender_id: tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
                user_id: 1,
            }),
            chat_id: 1000,
            sending_state: None,
            scheduling_state: None,
            is_outgoing: false,
            is_pinned: false,
            is_from_offline: false,
            can_be_saved: true,
            has_timestamped_media: false,
            is_channel_post: false,
            is_paid_star_suggested_post: false,
            is_paid_ton_suggested_post: false,
            contains_unread_mention: false,
            date: 0,
            edit_date: 0,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::Channel(
                    tdlib_rs::types::MessageOriginChannel {
                        chat_id: -2000,
                        message_id: 88,
                        author_signature: String::new(),
                    },
                ),
                date: 0,
                source: Some(tdlib_rs::types::ForwardSource {
                    chat_id: -3000,
                    message_id: 99,
                    sender_id: None,
                    sender_name: String::new(),
                    date: 0,
                    is_outgoing: false,
                }),
                public_service_announcement_type: String::new(),
            }),
            import_info: None,
            interaction_info: None,
            unread_reactions: vec![],
            fact_check: None,
            suggested_post_info: None,
            reply_to: None,
            topic_id: None,
            self_destruct_type: None,
            self_destruct_in: 0.0,
            auto_delete_in: 0.0,
            via_bot_user_id: 0,
            sender_business_bot_user_id: 0,
            sender_boost_count: 0,
            sender_tag: String::new(),
            paid_message_star_count: 0,
            author_signature: String::new(),
            media_album_id: 0,
            effect_id: 0,
            restriction_info: None,
            summary_language_code: String::new(),
            content: tdlib_rs::enums::MessageContent::MessageText(tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: "test".to_owned(),
                    entities: vec![],
                },
                link_preview: None,
                link_preview_options: None,
            }),
            reply_markup: None,
        };

        assert_eq!(forwarded_message_location(&message), Some((-3000, 99)));
    }

    #[test]
    fn test_forwarded_message_location_falls_back_to_channel_origin() {
        let message = tdlib_rs::types::Message {
            id: 1,
            sender_id: tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
                user_id: 1,
            }),
            chat_id: 1000,
            sending_state: None,
            scheduling_state: None,
            is_outgoing: false,
            is_pinned: false,
            is_from_offline: false,
            can_be_saved: true,
            has_timestamped_media: false,
            is_channel_post: false,
            is_paid_star_suggested_post: false,
            is_paid_ton_suggested_post: false,
            contains_unread_mention: false,
            date: 0,
            edit_date: 0,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::Channel(
                    tdlib_rs::types::MessageOriginChannel {
                        chat_id: -2000,
                        message_id: 88,
                        author_signature: String::new(),
                    },
                ),
                date: 0,
                source: None,
                public_service_announcement_type: String::new(),
            }),
            import_info: None,
            interaction_info: None,
            unread_reactions: vec![],
            fact_check: None,
            suggested_post_info: None,
            reply_to: None,
            topic_id: None,
            self_destruct_type: None,
            self_destruct_in: 0.0,
            auto_delete_in: 0.0,
            via_bot_user_id: 0,
            sender_business_bot_user_id: 0,
            sender_boost_count: 0,
            sender_tag: String::new(),
            paid_message_star_count: 0,
            author_signature: String::new(),
            media_album_id: 0,
            effect_id: 0,
            restriction_info: None,
            summary_language_code: String::new(),
            content: tdlib_rs::enums::MessageContent::MessageText(tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: "test".to_owned(),
                    entities: vec![],
                },
                link_preview: None,
                link_preview_options: None,
            }),
            reply_markup: None,
        };

        assert_eq!(forwarded_message_location(&message), Some((-2000, 88)));
    }

    #[test]
    fn test_forwarded_message_location_rejects_unstable_origin() {
        let message = tdlib_rs::types::Message {
            id: 1,
            sender_id: tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
                user_id: 1,
            }),
            chat_id: 1000,
            sending_state: None,
            scheduling_state: None,
            is_outgoing: false,
            is_pinned: false,
            is_from_offline: false,
            can_be_saved: true,
            has_timestamped_media: false,
            is_channel_post: false,
            is_paid_star_suggested_post: false,
            is_paid_ton_suggested_post: false,
            contains_unread_mention: false,
            date: 0,
            edit_date: 0,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::User(tdlib_rs::types::MessageOriginUser {
                    sender_user_id: 42,
                }),
                date: 0,
                source: None,
                public_service_announcement_type: String::new(),
            }),
            import_info: None,
            interaction_info: None,
            unread_reactions: vec![],
            fact_check: None,
            suggested_post_info: None,
            reply_to: None,
            topic_id: None,
            self_destruct_type: None,
            self_destruct_in: 0.0,
            auto_delete_in: 0.0,
            via_bot_user_id: 0,
            sender_business_bot_user_id: 0,
            sender_boost_count: 0,
            sender_tag: String::new(),
            paid_message_star_count: 0,
            author_signature: String::new(),
            media_album_id: 0,
            effect_id: 0,
            restriction_info: None,
            summary_language_code: String::new(),
            content: tdlib_rs::enums::MessageContent::MessageText(tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: "test".to_owned(),
                    entities: vec![],
                },
                link_preview: None,
                link_preview_options: None,
            }),
            reply_markup: None,
        };

        assert_eq!(forwarded_message_location(&message), None);
    }

    #[test]
    fn test_transferable_message_source_location_prefers_reply_then_forward_then_self() {
        let base_message = tdlib_rs::types::Message {
            id: 7,
            sender_id: tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
                user_id: 1,
            }),
            chat_id: 1000,
            sending_state: None,
            scheduling_state: None,
            is_outgoing: false,
            is_pinned: false,
            is_from_offline: false,
            can_be_saved: true,
            has_timestamped_media: false,
            is_channel_post: false,
            is_paid_star_suggested_post: false,
            is_paid_ton_suggested_post: false,
            contains_unread_mention: false,
            date: 0,
            edit_date: 0,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::Channel(
                    tdlib_rs::types::MessageOriginChannel {
                        chat_id: -2000,
                        message_id: 88,
                        author_signature: String::new(),
                    },
                ),
                date: 0,
                source: Some(tdlib_rs::types::ForwardSource {
                    chat_id: -3000,
                    message_id: 99,
                    sender_id: None,
                    sender_name: String::new(),
                    date: 0,
                    is_outgoing: false,
                }),
                public_service_announcement_type: String::new(),
            }),
            import_info: None,
            interaction_info: None,
            unread_reactions: vec![],
            fact_check: None,
            suggested_post_info: None,
            reply_to: None,
            topic_id: None,
            self_destruct_type: None,
            self_destruct_in: 0.0,
            auto_delete_in: 0.0,
            via_bot_user_id: 0,
            sender_business_bot_user_id: 0,
            sender_boost_count: 0,
            sender_tag: String::new(),
            paid_message_star_count: 0,
            author_signature: String::new(),
            media_album_id: 0,
            effect_id: 0,
            restriction_info: None,
            summary_language_code: String::new(),
            content: tdlib_rs::enums::MessageContent::MessageText(tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: "test".to_owned(),
                    entities: vec![],
                },
                link_preview: None,
                link_preview_options: None,
            }),
            reply_markup: None,
        };

        let mut replied = base_message.clone();
        replied.reply_to = Some(tdlib_rs::enums::MessageReplyTo::Message(
            tdlib_rs::types::MessageReplyToMessage {
                chat_id: -4000,
                message_id: 66,
                quote: None,
                checklist_task_id: 0,
                origin: None,
                origin_send_date: 0,
                content: None,
            },
        ));
        assert_eq!(
            super::transferable_message_source_location(&replied),
            Some((-4000, 66))
        );

        let forwarded = base_message.clone();
        assert_eq!(
            super::transferable_message_source_location(&forwarded),
            Some((-3000, 99))
        );

        let mut self_only = base_message;
        self_only.forward_info = None;
        assert_eq!(
            super::transferable_message_source_location(&self_only),
            Some((1000, 7))
        );
    }

    #[test]
    fn test_transferable_message_source_location_rejects_unstable_forward_shell() {
        let message = tdlib_rs::types::Message {
            id: 7,
            sender_id: tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
                user_id: 1,
            }),
            chat_id: 1000,
            sending_state: None,
            scheduling_state: None,
            is_outgoing: false,
            is_pinned: false,
            is_from_offline: false,
            can_be_saved: true,
            has_timestamped_media: false,
            is_channel_post: false,
            is_paid_star_suggested_post: false,
            is_paid_ton_suggested_post: false,
            contains_unread_mention: false,
            date: 0,
            edit_date: 0,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::User(tdlib_rs::types::MessageOriginUser {
                    sender_user_id: 42,
                }),
                date: 0,
                source: None,
                public_service_announcement_type: String::new(),
            }),
            import_info: None,
            interaction_info: None,
            unread_reactions: vec![],
            fact_check: None,
            suggested_post_info: None,
            reply_to: None,
            topic_id: None,
            self_destruct_type: None,
            self_destruct_in: 0.0,
            auto_delete_in: 0.0,
            via_bot_user_id: 0,
            sender_business_bot_user_id: 0,
            sender_boost_count: 0,
            sender_tag: String::new(),
            paid_message_star_count: 0,
            author_signature: String::new(),
            media_album_id: 0,
            effect_id: 0,
            restriction_info: None,
            summary_language_code: String::new(),
            content: tdlib_rs::enums::MessageContent::MessageText(tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: "test".to_owned(),
                    entities: vec![],
                },
                link_preview: None,
                link_preview_options: None,
            }),
            reply_markup: None,
        };

        assert_eq!(super::transferable_message_source_location(&message), None);
    }

    // 回复媒体模式下第二个参数应当被当成 target，而不是 source link。
    #[test]
    fn test_resolve_target_for_bot_message_source() {
        let config = BotConfig::default();
        install_target_runtime(
            crate::config::TargetsConfig {
                default_chat_id: 0,
                by_request_chat_id: Default::default(),
                aliases: std::collections::HashMap::from([("archive".to_owned(), -100)]),
            },
            crate::config::AccessControlConfig::default(),
        );
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
