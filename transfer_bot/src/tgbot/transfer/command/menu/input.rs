// `/menu` ForceReply 输入流程。
// 本文件只保留普通输入事件处理；按钮、草稿状态和目标选择视图分别放到子模块。

mod callbacks;
mod state;
mod target;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::{job, lookup, points, transfer_cmd};
use super::text::{build_menu_status_text, build_step_prompt_text, build_transfer_prompt_text};
pub(super) use callbacks::{
    cancel_input_callback_query, job_id_input_callback_query,
    point_ledger_user_input_callback_query, target_alias_callback_query,
    target_back_callback_query, target_confirm_callback_query, target_default_callback_query,
    target_manual_callback_query, target_request_chat_callback_query,
};
use state::{
    DraftTakeResult, MenuInputDraft, MenuInputStep, peek_current_draft, put_confirm_draft,
    put_draft, put_target_choice_draft, remember_last_target, step_uses_reply_keyboard,
    take_current_draft,
};
pub(super) use state::{
    MenuInputKind, MenuJobAction, cancel_menu_input, cancel_menu_input_with_state, start_menu_input,
};
use target::{
    build_target_choice_buttons, confirm_button_rows, resolve_default_target, resolve_target_by_id,
    resolve_target_input, send_confirm_prompt, send_target_choice_prompt,
};

/// Telegram 原生选群按钮 ID。
///
/// 同一私聊里同时只保留一个草稿，因此固定 ID 足够；收到 `MessageChatShared` 时仍会校验这个 ID。
const TARGET_CHAT_REQUEST_BUTTON_ID: i32 = 7001;

/// 当前输入草稿的首页摘要。
#[derive(Debug, Clone)]
pub(super) struct MenuDraftSummary {
    /// 首页按钮标题。
    pub(super) title: &'static str,
}

/// 读取当前输入草稿摘要，不消费草稿。
pub(super) async fn current_draft_summary(
    chat_id: i64,
    user_id: i64,
) -> anyhow::Result<Option<MenuDraftSummary>> {
    match peek_current_draft((chat_id, user_id)).await? {
        DraftTakeResult::Active(draft) => Ok(Some(MenuDraftSummary {
            title: draft.continue_title(),
        })),
        DraftTakeResult::Expired | DraftTakeResult::None => Ok(None),
    }
}

/// 重新发送当前草稿所在阶段的提示。
///
/// 这里不消费草稿；后续文本回复或确认按钮仍会使用原草稿继续执行。
pub(super) async fn continue_current_input(
    chat_id: i64,
    user_id: i64,
    config: std::sync::Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<bool> {
    let draft = match peek_current_draft((chat_id, user_id)).await? {
        DraftTakeResult::Active(draft) => draft,
        DraftTakeResult::Expired => {
            send::ReplyPanel::card(build_transfer_prompt_text(
                "输入已过期",
                &expired_input_detail(),
            ))
            .row(vec![send::build_copy_button(
                "复制 /menu",
                "/menu",
                tdlib_rs::enums::ButtonStyle::Primary,
            )])
            .send(chat_id, client_id)
            .await?;
            return Ok(true);
        }
        DraftTakeResult::None => return Ok(false),
    };

    match draft.step {
        MenuInputStep::SourceLink { kind } => {
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_text("1/3", kind.source_title(), kind.source_detail()),
                chat_id,
                "输入源链接，或发送 /cancel",
                client_id,
            )
            .await?;
        }
        MenuInputStep::TargetChoice { kind, source_link } => {
            send_target_choice_prompt(
                config.as_ref(),
                chat_id,
                user_id,
                client_id,
                kind,
                &source_link,
            )
            .await?;
        }
        MenuInputStep::TargetChat { kind, .. } => {
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_text(
                    "2/3",
                    "输入目标 chat",
                    "请回复目标 chat_id、别名或 default，或发送 /cancel。",
                ),
                chat_id,
                "输入目标 chat_id、alias 或 default",
                client_id,
            )
            .await?;
            tracing::debug!(
                chat_id,
                user_id,
                input_kind = kind.log_name(),
                "continued menu target chat input"
            );
        }
        MenuInputStep::ChatPicker { .. } => {
            send::send_card_message_with_chat_request_keyboard_returning(
                build_step_prompt_text(
                    "2/3",
                    "选择目标群组",
                    "点击输入框下方的“选择群组”，Telegram 会打开原生群组选择器；不想继续就点“取消”。",
                ),
                chat_id,
                TARGET_CHAT_REQUEST_BUTTON_ID,
                "选择群组",
                "选择目标群组，或发送 /cancel",
                client_id,
            )
            .await?;
        }
        MenuInputStep::Confirm {
            kind,
            source_link,
            target_chat_id,
        } => {
            send_confirm_prompt(kind, &source_link, target_chat_id, chat_id, client_id).await?;
        }
        MenuInputStep::JobId { action } => {
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_text("1/1", action.input_title(), action.input_detail()),
                chat_id,
                "输入 job_id，或发送 /cancel",
                client_id,
            )
            .await?;
        }
        MenuInputStep::PointLedgerUserId => {
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_text(
                    "1/1",
                    "用户积分流水",
                    "请回复 Telegram 用户 ID，例如 123456789；或发送 /cancel 取消。",
                ),
                chat_id,
                "输入 Telegram user_id，或发送 /cancel",
                client_id,
            )
            .await?;
        }
    }
    Ok(true)
}

/// 处理菜单输入。
///
/// 返回 true 表示本条消息已被输入流程消费；返回 false 表示没有匹配草稿。
pub(super) async fn handle_menu_input(
    text: &str,
    config: std::sync::Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    sender_user_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<bool> {
    let input = text.trim();
    if input.is_empty() {
        return Ok(false);
    }

    let key = (request_chat_id, sender_user_id);
    let draft = match take_current_draft(key).await? {
        DraftTakeResult::Active(draft) => draft,
        DraftTakeResult::Expired => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                "menu input draft expired"
            );
            send::ReplyPanel::card(build_transfer_prompt_text(
                "输入已过期",
                &expired_input_detail(),
            ))
            .row(vec![send::build_copy_button(
                "复制 /menu",
                "/menu",
                tdlib_rs::enums::ButtonStyle::Primary,
            )])
            .send(request_chat_id, client_id)
            .await?;
            return Ok(true);
        }
        DraftTakeResult::None => {
            tracing::trace!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                "menu input draft not found"
            );
            return Ok(false);
        }
    };

    if is_cancel_text(input) {
        tracing::debug!(
            request_chat_id,
            sender_user_id,
            request_message_id,
            needs_reply_keyboard_cleanup = step_uses_reply_keyboard(&draft.step),
            "menu input cancelled by text"
        );
        send_cancelled_notice(
            request_chat_id,
            client_id,
            step_uses_reply_keyboard(&draft.step),
        )
        .await?;
        return Ok(true);
    }

    match draft.step {
        MenuInputStep::SourceLink { kind } => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                input_kind = kind.log_name(),
                "menu input source link received"
            );
            if !looks_like_telegram_link(input) {
                put_draft(key, MenuInputDraft::source_link(kind)).await?;
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    input_kind = kind.log_name(),
                    "menu input source link rejected"
                );
                send::send_card_message_with_force_reply_returning(
                    build_step_prompt_text(
                        "1/3",
                        "源链接格式不正确",
                        "请回复 t.me 消息链接，或发送 /cancel 取消。",
                    ),
                    request_chat_id,
                    "输入 https://t.me/... 链接",
                    client_id,
                )
                .await?;
                return Ok(true);
            }

            if kind.uses_default_target() {
                let Some(target_chat_id) = resolve_default_target(&config, request_chat_id) else {
                    put_draft(
                        key,
                        MenuInputDraft::target_choice(kind.command_kind(), input.to_owned()),
                    )
                    .await?;
                    tracing::debug!(
                        request_chat_id,
                        sender_user_id,
                        request_message_id,
                        input_kind = kind.log_name(),
                        "menu input default target missing, asking target choice"
                    );
                    send_target_choice_prompt(
                        &config,
                        request_chat_id,
                        sender_user_id,
                        client_id,
                        kind.command_kind(),
                        input,
                    )
                    .await?;
                    return Ok(true);
                };
                let command_owned = vec![
                    kind.command_name().to_owned(),
                    input.to_owned(),
                    target_chat_id.to_string(),
                ];
                remember_last_target(request_chat_id, sender_user_id, target_chat_id);
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    target_chat_id,
                    input_kind = kind.log_name(),
                    "menu input resolved default target"
                );
                run_existing_command(
                    kind,
                    command_owned,
                    config,
                    request_chat_id,
                    request_message_id,
                    actor,
                    client_id,
                )
                .await?;
                return Ok(true);
            }

            put_draft(key, MenuInputDraft::target_choice(kind, input.to_owned())).await?;
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                input_kind = kind.log_name(),
                "menu input asking target choice"
            );
            send_target_choice_prompt(
                &config,
                request_chat_id,
                sender_user_id,
                client_id,
                kind,
                input,
            )
            .await?;
            Ok(true)
        }
        MenuInputStep::TargetChoice { kind, source_link }
        | MenuInputStep::TargetChat { kind, source_link }
        | MenuInputStep::ChatPicker { kind, source_link } => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                input_kind = kind.log_name(),
                "menu input target chat received"
            );
            let Some(target_chat_id) = resolve_target_input(input, &config, request_chat_id) else {
                put_draft(key, MenuInputDraft::target_chat(kind, source_link)).await?;
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    input_kind = kind.log_name(),
                    "menu input target chat rejected"
                );
                send::send_card_message_with_force_reply_returning(
                    build_step_prompt_text(
                        "2/3",
                        "目标 chat 格式不正确",
                        "请回复数字 chat_id、配置里的目标别名，或回复 default 使用配置默认目标。",
                    ),
                    request_chat_id,
                    "输入目标 chat_id、别名或 default",
                    client_id,
                )
                .await?;
                return Ok(true);
            };

            put_confirm_draft(key, kind, source_link.clone(), target_chat_id).await?;
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                input_kind = kind.log_name(),
                target_chat_id,
                "menu input target resolved, asking confirmation"
            );
            send_confirm_prompt(
                kind,
                &source_link,
                target_chat_id,
                request_chat_id,
                client_id,
            )
            .await?;
            Ok(true)
        }
        MenuInputStep::Confirm {
            kind,
            source_link,
            target_chat_id,
        } => {
            put_confirm_draft(key, kind, source_link, target_chat_id).await?;
            send::ReplyPanel::card(build_step_prompt_text(
                "3/3",
                "等待确认",
                "请点击确认卡片里的“执行”，或发送 /cancel 取消。",
            ))
            .rows(confirm_button_rows())
            .send(request_chat_id, client_id)
            .await?;
            Ok(true)
        }
        MenuInputStep::JobId { action } => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                job_action = action.log_name(),
                "menu input job id received"
            );
            let Some(job_id) = parse_job_id_input(input) else {
                put_draft(key, MenuInputDraft::job_id(action)).await?;
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    job_action = action.log_name(),
                    "menu input job id rejected"
                );
                send::send_card_message_with_force_reply_returning(
                    build_step_prompt_text(
                        "1/1",
                        "job_id 格式不正确",
                        "请回复纯数字 job_id，例如 42；或发送 /cancel 取消。",
                    ),
                    request_chat_id,
                    "输入数字 job_id，或发送 /cancel",
                    client_id,
                )
                .await?;
                return Ok(true);
            };

            tracing::info!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                job_id,
                job_action = action.log_name(),
                "menu input dispatching job command"
            );
            run_existing_job_command(action, job_id, actor, client_id).await?;
            Ok(true)
        }
        MenuInputStep::PointLedgerUserId => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                "menu input point ledger user id received"
            );
            let Some(user_id) = parse_user_id_input(input) else {
                put_draft(key, MenuInputDraft::point_ledger_user_id()).await?;
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    "menu input point ledger user id rejected"
                );
                send::send_card_message_with_force_reply_returning(
                    build_step_prompt_text(
                        "1/1",
                        "用户 ID 格式不正确",
                        "请回复纯数字 Telegram 用户 ID，例如 123456789；或发送 /cancel 取消。",
                    ),
                    request_chat_id,
                    "输入数字 user_id，或发送 /cancel",
                    client_id,
                )
                .await?;
                return Ok(true);
            };

            tracing::info!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                target_user_id = user_id,
                "menu input dispatching points history command"
            );
            run_existing_points_history_command(user_id, actor, client_id).await?;
            Ok(true)
        }
    }
}

/// 处理 Telegram 原生选群结果。
///
/// 返回 true 表示这条 `MessageChatShared` 已被输入流消费。
pub(super) async fn handle_shared_chat_input(
    shared: &tdlib_rs::types::MessageChatShared,
    config: std::sync::Arc<BotConfig>,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    if shared.button_id != TARGET_CHAT_REQUEST_BUTTON_ID {
        return Ok(false);
    }

    let key = (request_chat_id, sender_user_id);
    let draft = match take_current_draft(key).await? {
        DraftTakeResult::Active(draft) => draft,
        DraftTakeResult::Expired => {
            send::send_card_message_with_remove_keyboard(
                build_menu_status_text(
                    "输入已过期",
                    "expired",
                    "上一次选群操作已超过有效时间，请重新打开 /menu。",
                ),
                request_chat_id,
                client_id,
            )
            .await?;
            return Ok(true);
        }
        DraftTakeResult::None => return Ok(false),
    };

    let MenuInputStep::ChatPicker { kind, source_link } = draft.step else {
        put_draft(key, draft).await?;
        return Ok(false);
    };

    let target_chat_id = shared.chat.chat_id;
    if let Err(err) = resolve_target_by_id(target_chat_id, &config, request_chat_id) {
        put_target_choice_draft(key, kind, source_link).await?;
        tracing::warn!(
            request_chat_id,
            sender_user_id,
            target_chat_id,
            error = %err,
            "shared target chat rejected"
        );
        send_keyboard_cleanup_notice(
            request_chat_id,
            client_id,
            "目标不可用",
            "已移除选群键盘；选中的群不在允许列表，请重新选择或手动输入。",
        )
        .await?;
        send::ReplyPanel::card(build_transfer_prompt_text(
            "目标不可用",
            "选中的群不在 allowed_target_chat_ids 允许列表中，请重新选择或手动输入。",
        ))
        .rows(build_target_choice_buttons(
            &config,
            request_chat_id,
            sender_user_id,
            kind,
        ))
        .send(request_chat_id, client_id)
        .await?;
        return Ok(true);
    }

    put_confirm_draft(key, kind, source_link.clone(), target_chat_id).await?;
    send_keyboard_cleanup_notice(
        request_chat_id,
        client_id,
        "已选择目标",
        "已移除输入框下方的选群键盘，请在确认卡片中继续。",
    )
    .await?;
    send_confirm_prompt(
        kind,
        &source_link,
        target_chat_id,
        request_chat_id,
        client_id,
    )
    .await?;
    Ok(true)
}

/// 判断普通文本是否表示取消。
///
/// `取消` 来自 reply keyboard 的文本按钮；`cancel` 作为英文兜底，`/cancel` 由上层命令路由优先处理。
fn is_cancel_text(input: &str) -> bool {
    input == "取消" || input.eq_ignore_ascii_case("cancel") || input.eq_ignore_ascii_case("/cancel")
}

/// 发送取消提示；如果选群键盘可能残留，则顺带移除 reply keyboard。
async fn send_cancelled_notice(
    request_chat_id: i64,
    client_id: i32,
    needs_reply_keyboard_cleanup: bool,
) -> anyhow::Result<()> {
    let text = build_menu_status_text(
        "已取消",
        "cancelled",
        "当前输入流程已取消，可重新打开 /menu。",
    );
    if needs_reply_keyboard_cleanup {
        return send::send_card_message_with_remove_keyboard(text, request_chat_id, client_id)
            .await;
    }

    send::ReplyPanel::card(text)
        .row(vec![send::build_copy_button(
            "复制 /menu",
            "/menu",
            tdlib_rs::enums::ButtonStyle::Primary,
        )])
        .send(request_chat_id, client_id)
        .await
}

/// 发送“键盘已清理”提示。
///
/// TDLib 不能同时在同一条消息上携带 inline keyboard 和 remove keyboard，
/// 因此清理 reply keyboard 必须单独发一条短卡片，再发送确认/目标选择卡片。
async fn send_keyboard_cleanup_notice(
    request_chat_id: i64,
    client_id: i32,
    title: &str,
    detail: &str,
) -> anyhow::Result<()> {
    send::send_card_message_with_remove_keyboard(
        build_menu_status_text(title, "keyboard-cleared", detail),
        request_chat_id,
        client_id,
    )
    .await
}

/// 调用已有命令入口，避免菜单输入流复制转存/查询业务逻辑。
async fn run_existing_command(
    kind: MenuInputKind,
    command_owned: Vec<String>,
    config: std::sync::Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    match kind {
        MenuInputKind::Transfer | MenuInputKind::TransferDefault => {
            transfer_cmd::transfer_link_command(
                command_refs,
                config,
                request_chat_id,
                request_message_id,
                actor,
                client_id,
            )
            .await
        }
        MenuInputKind::Lookup | MenuInputKind::LookupDefault => {
            lookup::lookup_command(command_refs, config, actor, client_id).await
        }
    }
}

/// 调用已有 `/job` 命令入口，避免菜单输入流复制任务状态迁移逻辑。
async fn run_existing_job_command(
    action: MenuJobAction,
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_owned = [
        "/job".to_owned(),
        action.command_action().to_owned(),
        job_id.to_string(),
    ];
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    job::job_command(command_refs, actor, client_id).await
}

/// 调用已有 `/points history` 命令，避免菜单输入流复制积分流水查询逻辑。
async fn run_existing_points_history_command(
    user_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_owned = [
        "/points".to_owned(),
        "history".to_owned(),
        user_id.to_string(),
    ];
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    points::points_command(command_refs, actor, client_id).await
}

/// 解析用户回复的任务编号。
///
/// job_id 来自数据库自增主键，必须是正整数；这里先过滤空白、符号和混合文本，避免把错误输入传到命令层。
fn parse_job_id_input(input: &str) -> Option<i64> {
    let trimmed = input.trim();
    if trimmed.is_empty() || !trimmed.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    let job_id = trimmed.parse::<i64>().ok()?;
    (job_id > 0).then_some(job_id)
}

/// 解析用户回复的 Telegram 用户 ID。
///
/// user_id 也使用纯数字输入，避免把 `@username` 误当成可解析目标；后续如需 username 解析再单独接 TDLib 查询。
fn parse_user_id_input(input: &str) -> Option<i64> {
    parse_job_id_input(input)
}

/// 粗略判断是否是 Telegram 消息链接。
///
/// 真正合法性仍由 spider 层解析；这里仅避免明显错误输入推进到下一步。
fn looks_like_telegram_link(input: &str) -> bool {
    input.starts_with("https://t.me/")
        || input.starts_with("http://t.me/")
        || input.starts_with("t.me/")
}

/// 菜单输入过期提示，跟随运行时配置展示实际超时时间。
fn expired_input_detail() -> String {
    format!(
        "上一次菜单输入已超过 {}，请重新打开 /menu。",
        format_duration_hint(
            crate::tgbot::transfer::runtime_config()
                .menu_input_timeout_seconds
                .max(1)
        )
    )
}

/// 把秒数压缩成适合卡片展示的短文案。
fn format_duration_hint(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{} 秒", seconds);
    }
    if seconds.is_multiple_of(3600) {
        return format!("{} 小时", seconds / 3600);
    }
    if seconds.is_multiple_of(60) {
        return format!("{} 分钟", seconds / 60);
    }
    format!("{} 秒", seconds)
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

    // reply keyboard 的“取消”按钮会发回普通文本，状态机必须能直接识别。
    #[test]
    fn test_is_cancel_text() {
        assert!(is_cancel_text("取消"));
        assert!(is_cancel_text("cancel"));
        assert!(is_cancel_text("/cancel"));
        assert!(!is_cancel_text("继续"));
    }

    // 任务控制输入只接受正整数 job_id，避免把说明文字或负数传给 `/job`。
    #[test]
    fn test_parse_job_id_input() {
        assert_eq!(parse_job_id_input("42"), Some(42));
        assert_eq!(parse_job_id_input(" 42 "), Some(42));
        assert_eq!(parse_job_id_input("0"), None);
        assert_eq!(parse_job_id_input("-1"), None);
        assert_eq!(parse_job_id_input("job 42"), None);
        assert_eq!(parse_job_id_input(""), None);
    }

    // 用户积分流水输入同样只接受 Telegram 数字 user_id，username 解析后续单独接 TDLib。
    #[test]
    fn test_parse_user_id_input() {
        assert_eq!(parse_user_id_input("123456789"), Some(123456789));
        assert_eq!(parse_user_id_input("@alice"), None);
        assert_eq!(parse_user_id_input("-123"), None);
    }

    // 过期提示应跟随可配置秒数展示，不再写死默认 10 分钟。
    #[test]
    fn test_format_duration_hint() {
        assert_eq!(format_duration_hint(45), "45 秒");
        assert_eq!(format_duration_hint(600), "10 分钟");
        assert_eq!(format_duration_hint(7200), "2 小时");
        assert_eq!(format_duration_hint(95), "95 秒");
    }
}
