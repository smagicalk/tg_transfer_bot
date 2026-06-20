// `/menu` ForceReply 输入流程。
// 本文件只保留普通输入事件处理；按钮、草稿状态和目标选择视图分别放到子模块。

mod admin;
mod callbacks;
mod callbacks_simple;
mod callbacks_target;
mod flow;
mod flow_callbacks;
mod simple;
mod state;
mod target;

use crate::config::BotConfig;
use crate::tgbot::send;

use self::admin::{
    AdminCommandKind, admin_command_kind, parse_admin_input_payload, run_existing_acl_command,
    run_existing_billing_command, run_existing_config_command, run_existing_targets_command,
};
pub(in crate::tgbot::transfer::command::menu) use self::flow_callbacks::handle_shared_chat_input_on;
use self::flow_callbacks::{FlowRequestContext, continue_flow_input_on, handle_flow_input};
use self::simple::{
    expired_input_detail_on, is_cancel_text, parse_job_id_input, parse_user_id_input,
    run_existing_job_command, run_existing_points_history_command, send_cancelled_notice,
};
use super::text::{build_menu_recovery_text, build_step_prompt_text};
pub(super) use callbacks::{
    admin_input_callback_query, cancel_input_callback_query, job_id_input_callback_query,
    point_ledger_user_input_callback_query, target_alias_callback_query,
    target_back_callback_query, target_confirm_callback_query, target_default_callback_query,
    target_manual_callback_query, target_request_chat_callback_query,
};
pub(in crate::tgbot::transfer::command) use state::AdminInputAction;
use state::{
    DraftTakeResult, MenuInputDraft, MenuInputStep, peek_current_draft, put_draft,
    step_uses_reply_keyboard, take_current_draft,
};
pub(super) use state::{
    MenuInputKind, MenuJobAction, cancel_menu_input, cancel_menu_input_with_state, start_menu_input,
};

use self::callbacks_simple::send_targets_chat_picker_prompt;

/// Telegram 原生选群按钮 ID。
///
/// 同一私聊里同时只保留一个草稿，因此固定 ID 足够；收到 `MessageChatShared` 时仍会校验这个 ID。
const TARGET_CHAT_REQUEST_BUTTON_ID: i32 = 7001;
/// `/targets` 配置页“选择默认目标”按钮 ID。
pub(super) const TARGETS_DEFAULT_REQUEST_BUTTON_ID: i32 = 7101;
/// `/targets` 配置页“选择请求路由目标”按钮 ID。
pub(super) const TARGETS_ROUTE_REQUEST_BUTTON_ID: i32 = 7102;

/// 当前输入草稿的首页摘要。
#[derive(Debug, Clone)]
pub(super) struct MenuDraftSummary {
    /// 首页按钮标题。
    pub(super) title: &'static str,
}

/// “继续输入”按钮读取当前草稿后的纯决策。
///
/// 入口层只根据这个结果决定发送哪类提示，避免“无草稿 / 过期 / 活跃草稿”分支散落在多个地方。
#[derive(Debug, Clone)]
enum ContinueInputDecision {
    None,
    Expired,
    Active(MenuInputDraft),
}

/// 把状态层的草稿读取结果映射为继续输入流程决策。
fn continue_input_decision(result: DraftTakeResult) -> ContinueInputDecision {
    match result {
        DraftTakeResult::None => ContinueInputDecision::None,
        DraftTakeResult::Expired => ContinueInputDecision::Expired,
        DraftTakeResult::Active(draft) => ContinueInputDecision::Active(draft),
    }
}

/// 构造“继续输入时已过期”的恢复文案。
fn build_continue_input_expired_text_on(app: &crate::app_context::AppContext) -> String {
    build_menu_recovery_text("输入已过期", "expired", &expired_input_detail_on(app))
}

#[cfg(test)]
fn build_continue_input_expired_text() -> String {
    let app_context = crate::app_context::app_context();
    build_continue_input_expired_text_on(app_context.as_ref())
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

/// 在指定上下文上重新发送当前草稿所在阶段的提示。
pub(super) async fn continue_current_input_on(
    app: &crate::app_context::AppContext,
    chat_id: i64,
    user_id: i64,
    config: std::sync::Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<bool> {
    let draft = match continue_input_decision(peek_current_draft((chat_id, user_id)).await?) {
        ContinueInputDecision::Active(draft) => draft,
        ContinueInputDecision::Expired => {
            send::ReplyPanel::card(build_continue_input_expired_text_on(app))
                .row(vec![send::build_callback_button(
                    "返回菜单",
                    &super::build_menu_home_callback_data(),
                    tdlib_rs::enums::ButtonStyle::Primary,
                )])
                .send(chat_id, client_id)
                .await?;
            return Ok(true);
        }
        ContinueInputDecision::None => return Ok(false),
    };

    if continue_flow_input_on(app, &draft, config.as_ref(), chat_id, user_id, client_id).await? {
        return Ok(true);
    }

    match draft.step {
        MenuInputStep::JobId { action } => {
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_text("1/1", action.input_title(), action.input_detail()),
                chat_id,
                "输入 job_id，或发送 /cancel",
                client_id,
            )
            .await?;
        }
        MenuInputStep::AdminInput { action } => {
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_text("1/1", action.input_title(), action.input_detail()),
                chat_id,
                action.input_placeholder(),
                client_id,
            )
            .await?;
        }
        MenuInputStep::AdminChatPicker {
            action,
            request_chat_id_input,
        } => {
            send_targets_chat_picker_prompt(chat_id, client_id, action, request_chat_id_input)
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
        MenuInputStep::SourceLink { .. }
        | MenuInputStep::TargetChoice { .. }
        | MenuInputStep::TargetChat { .. }
        | MenuInputStep::ChatPicker { .. }
        | MenuInputStep::Confirm { .. } => {
            tracing::warn!(
                chat_id,
                user_id,
                step = ?draft.step,
                "continue input fell back to flow step unexpectedly"
            );
            return continue_flow_input_on(
                app,
                &draft,
                config.as_ref(),
                chat_id,
                user_id,
                client_id,
            )
            .await;
        }
    }
    Ok(true)
}

/// 处理菜单输入。
///
/// 返回 true 表示本条消息已被输入流程消费；返回 false 表示没有匹配草稿。
pub(super) async fn handle_menu_input_on(
    app: &crate::app_context::AppContext,
    text: &str,
    config: std::sync::Arc<BotConfig>,
    key: (i64, i64),
    request_message_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<bool> {
    let (request_chat_id, sender_user_id) = key;
    let input = text.trim();
    if input.is_empty() {
        return Ok(false);
    }

    let draft = match take_current_draft(key).await? {
        DraftTakeResult::Active(draft) => draft,
        DraftTakeResult::Expired => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                "menu input draft expired"
            );
            send::ReplyPanel::card(build_continue_input_expired_text_on(app))
                .row(vec![send::build_callback_button(
                    "返回菜单",
                    &super::build_menu_home_callback_data(),
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

    if let Some(consumed) = handle_flow_input(
        app,
        draft.clone(),
        input,
        FlowRequestContext {
            key,
            config: config.clone(),
            request_chat_id,
            request_message_id,
            actor,
            client_id,
        },
    )
    .await?
    {
        return Ok(consumed);
    }

    match draft.step {
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
            run_existing_job_command(app, action, job_id, actor, client_id).await?;
            Ok(true)
        }
        MenuInputStep::AdminInput { action } => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                admin_action = action.log_name(),
                "menu input admin action received"
            );
            let Some(command_owned) = parse_admin_input_payload(action, input) else {
                put_draft(key, MenuInputDraft::admin_input(action)).await?;
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    admin_action = action.log_name(),
                    "menu input admin action rejected"
                );
                send::send_card_message_with_force_reply_returning(
                    build_step_prompt_text("1/1", "输入格式不正确", action.input_detail()),
                    request_chat_id,
                    action.input_placeholder(),
                    client_id,
                )
                .await?;
                return Ok(true);
            };

            tracing::info!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                admin_action = action.log_name(),
                "menu input dispatching admin config command"
            );
            match admin_command_kind(action) {
                Some(AdminCommandKind::Targets) => {
                    run_existing_targets_command(app, command_owned, request_chat_id, client_id)
                        .await?;
                }
                Some(AdminCommandKind::Acl) => {
                    run_existing_acl_command(app, command_owned, request_chat_id, client_id)
                        .await?;
                }
                Some(AdminCommandKind::Config) => {
                    run_existing_config_command(app, command_owned, request_chat_id, client_id)
                        .await?;
                }
                Some(AdminCommandKind::Billing) => {
                    run_existing_billing_command(app, command_owned, request_chat_id, client_id)
                        .await?;
                }
                None => anyhow::bail!("unsupported admin input action: {}", action.log_name()),
            }
            Ok(true)
        }
        MenuInputStep::AdminChatPicker {
            action,
            request_chat_id_input,
        } => {
            if action != AdminInputAction::TargetsPickRoute {
                put_draft(
                    key,
                    MenuInputDraft::admin_chat_picker(action, request_chat_id_input),
                )
                .await?;
                anyhow::bail!(
                    "unsupported admin chat picker text step: {}",
                    action.log_name()
                );
            }

            let Some(route_request_chat_id) = parse_job_id_input(input) else {
                put_draft(
                    key,
                    MenuInputDraft::admin_chat_picker(action, request_chat_id_input),
                )
                .await?;
                send::send_card_message_with_force_reply_returning(
                    build_step_prompt_text(
                        "1/1",
                        "request_chat_id 格式不正确",
                        "请回复纯数字 request_chat_id，随后会弹出目标群组选择器；或发送 /cancel 取消。",
                    ),
                    request_chat_id,
                    "输入 request_chat_id，或发送 /cancel",
                    client_id,
                )
                .await?;
                return Ok(true);
            };

            put_draft(
                key,
                MenuInputDraft::admin_chat_picker(action, Some(route_request_chat_id)),
            )
            .await?;
            send_targets_chat_picker_prompt(
                request_chat_id,
                client_id,
                action,
                Some(route_request_chat_id),
            )
            .await?;
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
        MenuInputStep::SourceLink { .. }
        | MenuInputStep::TargetChoice { .. }
        | MenuInputStep::TargetChat { .. }
        | MenuInputStep::ChatPicker { .. }
        | MenuInputStep::Confirm { .. } => {
            tracing::warn!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                step = ?draft.step,
                "menu text input fell through to flow step unexpectedly"
            );
            put_draft(key, draft).await?;
            send::ReplyPanel::card(build_continue_input_expired_text_on(app))
                .row(vec![send::build_callback_button(
                    "返回菜单",
                    &super::build_menu_home_callback_data(),
                    tdlib_rs::enums::ButtonStyle::Primary,
                )])
                .send(request_chat_id, client_id)
                .await?;
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // “继续输入”按钮应先把状态层结果规整为纯决策，避免入口散落多个 match。
    #[test]
    fn test_continue_input_decision_maps_draft_results() {
        assert!(matches!(
            continue_input_decision(DraftTakeResult::None),
            ContinueInputDecision::None
        ));
        assert!(matches!(
            continue_input_decision(DraftTakeResult::Expired),
            ContinueInputDecision::Expired
        ));
        assert!(matches!(
            continue_input_decision(DraftTakeResult::Active(MenuInputDraft::job_id(
                MenuJobAction::Pause
            ))),
            ContinueInputDecision::Active(MenuInputDraft {
                step: MenuInputStep::JobId {
                    action: MenuJobAction::Pause
                }
            })
        ));
    }

    // 继续输入的过期提示应是恢复态，而不是等待态。
    #[test]
    fn test_build_continue_input_expired_text_uses_recovery_status() {
        let text = build_continue_input_expired_text();

        assert!(text.contains("输入已过期"));
        assert!(text.contains("状态：‹expired›"));
        assert!(text.contains("返回菜单：‹/menu›"));
    }

    // continue 输入的流程草稿若意外落到本层，也应继续走流程提示，而不是 panic。
    #[test]
    fn test_continue_input_flow_step_is_still_recoverable() {
        let draft = MenuInputDraft::source_link(MenuInputKind::Transfer);

        assert!(matches!(draft.step, MenuInputStep::SourceLink { .. }));
    }
}
