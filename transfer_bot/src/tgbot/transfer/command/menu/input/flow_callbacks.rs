// `/menu` 多步向导的文本输入与共享选群处理。
// 这里承接 `menu/input.rs` 的主流程分支，让入口文件只负责路由和少量编排。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::text::{
    build_menu_recovery_text, build_menu_target_unavailable_text, build_step_prompt_text,
    build_step_prompt_with_context,
};
use super::flow::{ExistingCommandContext, looks_like_telegram_link, run_existing_command};
use super::simple::send_keyboard_cleanup_notice;
use super::state::{
    DraftTakeResult, MenuInputDraft, MenuInputStep, put_confirm_draft, put_draft,
    put_target_choice_draft, remember_last_target, take_current_draft,
};
use super::target::{
    TargetPromptContext, build_target_choice_buttons_on, confirm_button_rows,
    resolve_default_target_on, resolve_target_by_id_on, resolve_target_input_on,
    send_confirm_prompt, send_target_choice_prompt, send_target_choice_prompt_with_detail,
};
use super::{
    MenuInputKind, TARGET_CHAT_REQUEST_BUTTON_ID, TARGETS_DEFAULT_REQUEST_BUTTON_ID,
    TARGETS_ROUTE_REQUEST_BUTTON_ID,
};

/// 共享选群结果对应的后续状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SharedChatOutcome {
    Ignored,
    Expired,
    WrongStep,
    RechooseTarget,
    ConfirmSelected,
}

/// 目标文本输入解析后的后续状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetInputOutcome {
    ReaskTarget,
    AskConfirm { target_chat_id: i64 },
}

/// 源链接输入解析后的后续状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceLinkOutcome {
    ReaskSource,
    ChooseTarget,
    ChooseTargetWithMissingDefault,
    DispatchDefaultTarget { target_chat_id: i64 },
}

/// 多步向导阶段动作最终要触发的 UI 行为。
#[derive(Debug, Clone, PartialEq, Eq)]
enum FlowUiAction {
    ReaskSource,
    ShowTargetChoice {
        kind: MenuInputKind,
        source_link: String,
        detail: Option<&'static str>,
    },
    ReaskTarget {
        kind: MenuInputKind,
        source_link: String,
    },
    ShowConfirm {
        kind: MenuInputKind,
        source_link: String,
        target_chat_id: i64,
    },
    DispatchDefaultTarget {
        kind: MenuInputKind,
        source_link: String,
        target_chat_id: i64,
    },
    RechooseTargetAfterShared {
        kind: MenuInputKind,
        source_link: String,
    },
}

/// 目标不可用时的统一提示标题。
fn build_target_unavailable_title() -> &'static str {
    "目标不可用"
}

/// 目标不可用时的统一提示说明。
fn build_target_unavailable_detail() -> &'static str {
    "选中的群不在 allowed_target_chat_ids 允许列表中，请重新选择或手动输入。"
}

/// 多步向导执行上下文。
///
/// 把请求、配置和执行者信息收拢到一个小接口，减少每个阶段函数的参数泄漏。
#[derive(Clone)]
pub(super) struct FlowRequestContext {
    pub(super) key: (i64, i64),
    pub(super) config: Arc<BotConfig>,
    pub(super) request_chat_id: i64,
    pub(super) request_message_id: i64,
    pub(super) actor: crate::config::RequestActor,
    pub(super) client_id: i32,
}

/// 重新发送多步向导当前阶段的提示。
///
/// 只处理“源链接 -> 目标 -> 确认”这条链；如果草稿属于单步输入，返回 `false` 交给外层简单分支处理。
pub(super) async fn continue_flow_input_on(
    app: &crate::app_context::AppContext,
    draft: &MenuInputDraft,
    config: &BotConfig,
    chat_id: i64,
    user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    match &draft.step {
        MenuInputStep::SourceLink { kind } => {
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_text("1/3", kind.source_title(), kind.source_detail()),
                chat_id,
                "输入源链接，或发送 /cancel",
                client_id,
            )
            .await?;
            Ok(true)
        }
        MenuInputStep::TargetChoice { kind, source_link } => {
            send_target_choice_prompt(
                config,
                TargetPromptContext {
                    app,
                    request_chat_id: chat_id,
                    sender_user_id: user_id,
                    client_id,
                },
                *kind,
                source_link,
            )
            .await?;
            Ok(true)
        }
        MenuInputStep::TargetChat { kind, .. } => {
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_with_context(
                    "waiting-input",
                    "2/3",
                    "输入目标 chat",
                    "请回复目标 chat_id、别名或 default，或发送 /cancel。",
                    None,
                    None,
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
            Ok(true)
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
            Ok(true)
        }
        MenuInputStep::Confirm {
            kind,
            source_link,
            target_chat_id,
        } => {
            send_confirm_prompt(*kind, source_link, *target_chat_id, chat_id, client_id).await?;
            Ok(true)
        }
        MenuInputStep::JobId { .. }
        | MenuInputStep::AdminInput { .. }
        | MenuInputStep::AdminChatPicker { .. }
        | MenuInputStep::PointLedgerUserId
        | MenuInputStep::PointsAdjust { .. } => Ok(false),
    }
}

/// 处理菜单多步输入。
///
/// 仅处理：
/// - 源链接
/// - 目标选择 / 手输目标 / 选群结果
/// - 执行前确认
///
/// `job_id` / `user_id` 这类单步输入仍留给外层简单分支处理。
pub(super) async fn handle_flow_input(
    app: &crate::app_context::AppContext,
    draft: MenuInputDraft,
    input: &str,
    ctx: FlowRequestContext,
) -> anyhow::Result<Option<bool>> {
    match draft.step {
        MenuInputStep::SourceLink { kind } => handle_source_link_input(app, kind, input, ctx).await,
        MenuInputStep::TargetChoice { kind, source_link }
        | MenuInputStep::TargetChat { kind, source_link }
        | MenuInputStep::ChatPicker { kind, source_link } => {
            handle_target_input(app, kind, source_link, input, ctx).await
        }
        MenuInputStep::Confirm {
            kind,
            source_link,
            target_chat_id,
        } => {
            put_confirm_draft(ctx.key, kind, source_link, target_chat_id).await?;
            send::ReplyPanel::card(build_step_prompt_with_context(
                "waiting-confirm",
                "3/3",
                "等待确认",
                "请点击确认卡片里的“执行”，或发送 /cancel 取消。",
                None,
                Some(target_chat_id),
            ))
            .rows(confirm_button_rows())
            .send(ctx.request_chat_id, ctx.client_id)
            .await?;
            Ok(Some(true))
        }
        MenuInputStep::JobId { .. }
        | MenuInputStep::AdminInput { .. }
        | MenuInputStep::AdminChatPicker { .. }
        | MenuInputStep::PointLedgerUserId
        | MenuInputStep::PointsAdjust { .. } => Ok(None),
    }
}

/// 共享选群回调的纯决策。
fn shared_chat_outcome(
    button_id: i32,
    draft: Option<&MenuInputStep>,
    request_button_id: i32,
    target_allowed: bool,
) -> SharedChatOutcome {
    if button_id != request_button_id {
        return SharedChatOutcome::Ignored;
    }
    let Some(step) = draft else {
        return SharedChatOutcome::Expired;
    };
    match step {
        MenuInputStep::ChatPicker { .. } => {
            if target_allowed {
                SharedChatOutcome::ConfirmSelected
            } else {
                SharedChatOutcome::RechooseTarget
            }
        }
        _ => SharedChatOutcome::WrongStep,
    }
}

/// 目标文本输入的纯决策。
fn target_input_outcome(target_chat_id: Option<i64>) -> TargetInputOutcome {
    match target_chat_id {
        Some(target_chat_id) => TargetInputOutcome::AskConfirm { target_chat_id },
        None => TargetInputOutcome::ReaskTarget,
    }
}

/// 源链接输入的纯决策。
fn source_link_outcome(
    kind: MenuInputKind,
    looks_like_link: bool,
    default_target_chat_id: Option<i64>,
) -> SourceLinkOutcome {
    if !looks_like_link {
        return SourceLinkOutcome::ReaskSource;
    }
    if !kind.uses_default_target() {
        return SourceLinkOutcome::ChooseTarget;
    }
    match default_target_chat_id {
        Some(target_chat_id) => SourceLinkOutcome::DispatchDefaultTarget { target_chat_id },
        None => SourceLinkOutcome::ChooseTargetWithMissingDefault,
    }
}

/// 把源链接阶段结果映射为统一 UI 动作。
fn source_link_ui_action(
    kind: MenuInputKind,
    input: &str,
    outcome: SourceLinkOutcome,
) -> FlowUiAction {
    match outcome {
        SourceLinkOutcome::ReaskSource => FlowUiAction::ReaskSource,
        SourceLinkOutcome::ChooseTarget => FlowUiAction::ShowTargetChoice {
            kind,
            source_link: input.to_owned(),
            detail: None,
        },
        SourceLinkOutcome::ChooseTargetWithMissingDefault => FlowUiAction::ShowTargetChoice {
            kind: kind.command_kind(),
            source_link: input.to_owned(),
            detail: Some("当前没有默认目标，请改为手动选择目标。"),
        },
        SourceLinkOutcome::DispatchDefaultTarget { target_chat_id } => {
            FlowUiAction::DispatchDefaultTarget {
                kind,
                source_link: input.to_owned(),
                target_chat_id,
            }
        }
    }
}

/// 把目标输入阶段结果映射为统一 UI 动作。
fn target_input_ui_action(
    kind: MenuInputKind,
    source_link: String,
    outcome: TargetInputOutcome,
) -> FlowUiAction {
    match outcome {
        TargetInputOutcome::ReaskTarget => FlowUiAction::ReaskTarget { kind, source_link },
        TargetInputOutcome::AskConfirm { target_chat_id } => FlowUiAction::ShowConfirm {
            kind,
            source_link,
            target_chat_id,
        },
    }
}

/// 把共享选群阶段结果映射为统一 UI 动作。
fn shared_chat_ui_action(
    kind: MenuInputKind,
    source_link: String,
    target_chat_id: i64,
    outcome: SharedChatOutcome,
) -> Option<FlowUiAction> {
    match outcome {
        SharedChatOutcome::RechooseTarget => {
            Some(FlowUiAction::RechooseTargetAfterShared { kind, source_link })
        }
        SharedChatOutcome::ConfirmSelected => Some(FlowUiAction::ShowConfirm {
            kind,
            source_link,
            target_chat_id,
        }),
        SharedChatOutcome::Ignored | SharedChatOutcome::Expired | SharedChatOutcome::WrongStep => {
            None
        }
    }
}

/// 在指定上下文上处理共享选群结果。
pub(in crate::tgbot::transfer::command::menu) async fn handle_shared_chat_input_on(
    app: &crate::app_context::AppContext,
    shared: &tdlib_rs::types::MessageChatShared,
    config: Arc<BotConfig>,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    if shared.button_id != TARGET_CHAT_REQUEST_BUTTON_ID
        && shared.button_id != TARGETS_DEFAULT_REQUEST_BUTTON_ID
        && shared.button_id != TARGETS_ROUTE_REQUEST_BUTTON_ID
    {
        return Ok(false);
    }

    let key = (request_chat_id, sender_user_id);
    let draft = match take_current_draft(key).await? {
        DraftTakeResult::Active(draft) => draft,
        DraftTakeResult::Expired => {
            send::send_card_message_with_remove_keyboard(
                build_menu_recovery_text(
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

    let target_chat_id = shared.chat.chat_id;
    if let MenuInputStep::AdminChatPicker {
        action,
        request_chat_id_input,
    } = draft.step
    {
        match action {
            crate::tgbot::transfer::command::menu::AdminInputAction::TargetsPickDefault => {
                let command_owned = vec![
                    "/targets".to_owned(),
                    "set-default".to_owned(),
                    target_chat_id.to_string(),
                ];
                super::admin::run_existing_targets_command(
                    app,
                    command_owned,
                    request_chat_id,
                    client_id,
                )
                .await?;
                return Ok(true);
            }
            crate::tgbot::transfer::command::menu::AdminInputAction::TargetsPickRoute => {
                let Some(route_request_chat_id) = request_chat_id_input else {
                    put_draft(key, MenuInputDraft::admin_chat_picker(action, None)).await?;
                    send::send_card_message_with_force_reply_returning(
                        build_step_prompt_text(
                            "1/1",
                            "缺少 request_chat_id",
                            "请先回复 request_chat_id，随后再选择目标群组；或发送 /cancel 取消。",
                        ),
                        request_chat_id,
                        "输入 request_chat_id，或发送 /cancel",
                        client_id,
                    )
                    .await?;
                    return Ok(true);
                };
                let command_owned = vec![
                    "/targets".to_owned(),
                    "set-route".to_owned(),
                    route_request_chat_id.to_string(),
                    target_chat_id.to_string(),
                ];
                super::admin::run_existing_targets_command(
                    app,
                    command_owned,
                    request_chat_id,
                    client_id,
                )
                .await?;
                return Ok(true);
            }
            _ => {
                put_draft(
                    key,
                    MenuInputDraft::admin_chat_picker(action, request_chat_id_input),
                )
                .await?;
                return Ok(false);
            }
        }
    }

    let outcome = shared_chat_outcome(
        shared.button_id,
        Some(&draft.step),
        TARGET_CHAT_REQUEST_BUTTON_ID,
        resolve_target_by_id_on(app, shared.chat.chat_id, &config, request_chat_id).is_ok(),
    );

    let MenuInputStep::ChatPicker { kind, source_link } = draft.step else {
        put_draft(key, draft).await?;
        return Ok(matches!(outcome, SharedChatOutcome::WrongStep));
    };

    match shared_chat_ui_action(kind, source_link, target_chat_id, outcome) {
        Some(FlowUiAction::RechooseTargetAfterShared { kind, source_link }) => {
            put_target_choice_draft(key, kind, source_link).await?;
            tracing::warn!(
                request_chat_id,
                sender_user_id,
                target_chat_id,
                "shared target chat rejected"
            );
            send_keyboard_cleanup_notice(
                request_chat_id,
                client_id,
                build_target_unavailable_title(),
                "已移除选群键盘；选中的群不在允许列表，请重新选择或手动输入。",
            )
            .await?;
            send::ReplyPanel::card(build_menu_target_unavailable_text(
                build_target_unavailable_detail(),
            ))
            .rows(build_target_choice_buttons_on(
                app,
                &config,
                request_chat_id,
                sender_user_id,
                kind,
            ))
            .send(request_chat_id, client_id)
            .await?;
            Ok(true)
        }
        Some(FlowUiAction::ShowConfirm {
            kind,
            source_link,
            target_chat_id,
        }) => {
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
        Some(_) | None => Ok(false),
    }
}

/// 处理源链接输入阶段。
async fn handle_source_link_input(
    app: &crate::app_context::AppContext,
    kind: MenuInputKind,
    input: &str,
    ctx: FlowRequestContext,
) -> anyhow::Result<Option<bool>> {
    match source_link_ui_action(
        kind,
        input,
        source_link_outcome(
            kind,
            looks_like_telegram_link(input),
            resolve_default_target_on(app, &ctx.config, ctx.request_chat_id),
        ),
    ) {
        FlowUiAction::ReaskSource => {
            put_draft(ctx.key, MenuInputDraft::source_link(kind)).await?;
            tracing::debug!(
                request_chat_id = ctx.request_chat_id,
                sender_user_id = ctx.key.1,
                request_message_id = ctx.request_message_id,
                input_kind = kind.log_name(),
                "menu input source link rejected"
            );
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_text(
                    "1/3",
                    "源链接格式不正确",
                    "请回复 t.me 消息链接，或发送 /cancel 取消。",
                ),
                ctx.request_chat_id,
                "输入 https://t.me/... 链接",
                ctx.client_id,
            )
            .await?;
            Ok(Some(true))
        }
        FlowUiAction::ShowTargetChoice {
            kind,
            source_link,
            detail,
        } => {
            put_draft(
                ctx.key,
                MenuInputDraft::target_choice(kind, source_link.clone()),
            )
            .await?;
            tracing::debug!(
                request_chat_id = ctx.request_chat_id,
                sender_user_id = ctx.key.1,
                request_message_id = ctx.request_message_id,
                input_kind = kind.log_name(),
                "menu input asking target choice"
            );
            match detail {
                Some(detail) => {
                    send_target_choice_prompt_with_detail(
                        &ctx.config,
                        TargetPromptContext {
                            app,
                            request_chat_id: ctx.request_chat_id,
                            sender_user_id: ctx.key.1,
                            client_id: ctx.client_id,
                        },
                        kind,
                        &source_link,
                        detail,
                    )
                    .await?;
                }
                None => {
                    send_target_choice_prompt(
                        &ctx.config,
                        TargetPromptContext {
                            app,
                            request_chat_id: ctx.request_chat_id,
                            sender_user_id: ctx.key.1,
                            client_id: ctx.client_id,
                        },
                        kind,
                        &source_link,
                    )
                    .await?;
                }
            }
            Ok(Some(true))
        }
        FlowUiAction::DispatchDefaultTarget {
            kind,
            source_link,
            target_chat_id,
        } => {
            let command_owned = vec![
                kind.command_name().to_owned(),
                source_link,
                target_chat_id.to_string(),
            ];
            remember_last_target(ctx.request_chat_id, ctx.key.1, target_chat_id);
            tracing::debug!(
                request_chat_id = ctx.request_chat_id,
                sender_user_id = ctx.key.1,
                request_message_id = ctx.request_message_id,
                target_chat_id,
                input_kind = kind.log_name(),
                "menu input resolved default target"
            );
            run_existing_command(
                kind,
                command_owned,
                ctx.config,
                ExistingCommandContext {
                    // 默认目标分支继续沿用当前请求拿到的运行态，避免回退到全局单例。
                    app: Arc::new(app.clone()),
                    request_chat_id: ctx.request_chat_id,
                    request_message_id: ctx.request_message_id,
                    actor: ctx.actor,
                    client_id: ctx.client_id,
                },
            )
            .await?;
            Ok(Some(true))
        }
        FlowUiAction::ReaskTarget { .. }
        | FlowUiAction::ShowConfirm { .. }
        | FlowUiAction::RechooseTargetAfterShared { .. } => {
            tracing::warn!(
                request_chat_id = ctx.request_chat_id,
                sender_user_id = ctx.key.1,
                request_message_id = ctx.request_message_id,
                input_kind = kind.log_name(),
                "source link stage received unexpected ui action, fallback to reask source"
            );
            put_draft(ctx.key, MenuInputDraft::source_link(kind)).await?;
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_text(
                    "1/3",
                    "源链接格式不正确",
                    "请回复 t.me 消息链接，或发送 /cancel 取消。",
                ),
                ctx.request_chat_id,
                "输入 https://t.me/... 链接",
                ctx.client_id,
            )
            .await?;
            Ok(Some(true))
        }
    }
}

/// 处理目标输入阶段。
async fn handle_target_input(
    app: &crate::app_context::AppContext,
    kind: MenuInputKind,
    source_link: String,
    input: &str,
    ctx: FlowRequestContext,
) -> anyhow::Result<Option<bool>> {
    match target_input_ui_action(
        kind,
        source_link.clone(),
        target_input_outcome(resolve_target_input_on(
            app,
            input,
            &ctx.config,
            ctx.request_chat_id,
        )),
    ) {
        FlowUiAction::ReaskTarget { kind, source_link } => {
            let invalid_target_text = build_step_prompt_with_context(
                "waiting-input",
                "2/3",
                "目标 chat 格式不正确",
                "请回复数字 chat_id、配置里的目标别名，或回复 default 使用配置默认目标。",
                Some(&source_link),
                None,
            );
            put_draft(ctx.key, MenuInputDraft::target_chat(kind, source_link)).await?;
            tracing::debug!(
                request_chat_id = ctx.request_chat_id,
                sender_user_id = ctx.key.1,
                input_kind = kind.log_name(),
                "menu input target chat rejected"
            );
            send::send_card_message_with_force_reply_returning(
                invalid_target_text,
                ctx.request_chat_id,
                "输入目标 chat_id、别名或 default",
                ctx.client_id,
            )
            .await?;
            Ok(Some(true))
        }
        FlowUiAction::ShowConfirm {
            kind,
            source_link,
            target_chat_id,
        } => {
            put_confirm_draft(ctx.key, kind, source_link.clone(), target_chat_id).await?;
            tracing::debug!(
                request_chat_id = ctx.request_chat_id,
                sender_user_id = ctx.key.1,
                input_kind = kind.log_name(),
                target_chat_id,
                "menu input target resolved, asking confirmation"
            );
            send_confirm_prompt(
                kind,
                &source_link,
                target_chat_id,
                ctx.request_chat_id,
                ctx.client_id,
            )
            .await?;
            Ok(Some(true))
        }
        FlowUiAction::ReaskSource
        | FlowUiAction::ShowTargetChoice { .. }
        | FlowUiAction::DispatchDefaultTarget { .. }
        | FlowUiAction::RechooseTargetAfterShared { .. } => {
            tracing::warn!(
                request_chat_id = ctx.request_chat_id,
                sender_user_id = ctx.key.1,
                request_message_id = ctx.request_message_id,
                input_kind = kind.log_name(),
                "target input stage received unexpected ui action, fallback to reask target"
            );
            put_draft(
                ctx.key,
                MenuInputDraft::target_chat(kind, source_link.clone()),
            )
            .await?;
            send::send_card_message_with_force_reply_returning(
                build_step_prompt_with_context(
                    "waiting-input",
                    "2/3",
                    "输入目标 chat",
                    "请回复目标 chat_id、别名或 default，或发送 /cancel。",
                    Some(&source_link),
                    None,
                ),
                ctx.request_chat_id,
                "输入目标 chat_id、alias 或 default",
                ctx.client_id,
            )
            .await?;
            Ok(Some(true))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 共享选群的回调只在 button_id 匹配且当前草稿真的是 ChatPicker 时才应该继续处理。
    #[test]
    fn test_shared_chat_outcome_routes_main_branches() {
        assert_eq!(
            shared_chat_outcome(1, None, TARGET_CHAT_REQUEST_BUTTON_ID, true),
            SharedChatOutcome::Ignored
        );
        assert_eq!(
            shared_chat_outcome(
                TARGET_CHAT_REQUEST_BUTTON_ID,
                None,
                TARGET_CHAT_REQUEST_BUTTON_ID,
                true
            ),
            SharedChatOutcome::Expired
        );
        assert_eq!(
            shared_chat_outcome(
                TARGET_CHAT_REQUEST_BUTTON_ID,
                Some(&MenuInputStep::TargetChoice {
                    kind: MenuInputKind::Transfer,
                    source_link: "https://t.me/c/1/2".to_owned(),
                }),
                TARGET_CHAT_REQUEST_BUTTON_ID,
                true
            ),
            SharedChatOutcome::WrongStep
        );
        assert_eq!(
            shared_chat_outcome(
                TARGET_CHAT_REQUEST_BUTTON_ID,
                Some(&MenuInputStep::ChatPicker {
                    kind: MenuInputKind::Transfer,
                    source_link: "https://t.me/c/1/2".to_owned(),
                }),
                TARGET_CHAT_REQUEST_BUTTON_ID,
                false
            ),
            SharedChatOutcome::RechooseTarget
        );
        assert_eq!(
            shared_chat_outcome(
                TARGET_CHAT_REQUEST_BUTTON_ID,
                Some(&MenuInputStep::ChatPicker {
                    kind: MenuInputKind::Transfer,
                    source_link: "https://t.me/c/1/2".to_owned(),
                }),
                TARGET_CHAT_REQUEST_BUTTON_ID,
                true
            ),
            SharedChatOutcome::ConfirmSelected
        );
    }

    // 目标文本输入只应有两个高层结果：重新询问目标，或进入确认页。
    #[test]
    fn test_target_input_outcome_routes_reask_or_confirm() {
        assert_eq!(target_input_outcome(None), TargetInputOutcome::ReaskTarget);
        assert_eq!(
            target_input_outcome(Some(-100)),
            TargetInputOutcome::AskConfirm {
                target_chat_id: -100
            }
        );
    }

    // 目标输入阶段也应统一映射到重问目标或展示确认页。
    #[test]
    fn test_target_input_ui_action_routes_main_branches() {
        assert_eq!(
            target_input_ui_action(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2".to_owned(),
                TargetInputOutcome::ReaskTarget
            ),
            FlowUiAction::ReaskTarget {
                kind: MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned()
            }
        );
        assert_eq!(
            target_input_ui_action(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2".to_owned(),
                TargetInputOutcome::AskConfirm {
                    target_chat_id: -100
                }
            ),
            FlowUiAction::ShowConfirm {
                kind: MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned(),
                target_chat_id: -100
            }
        );
        assert!(!matches!(
            target_input_ui_action(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2".to_owned(),
                TargetInputOutcome::ReaskTarget
            ),
            FlowUiAction::ReaskSource
                | FlowUiAction::ShowTargetChoice { .. }
                | FlowUiAction::DispatchDefaultTarget { .. }
                | FlowUiAction::RechooseTargetAfterShared { .. }
        ));
    }

    // 源链接输入应能统一表达：重新输入、改为选目标、默认目标缺失回退、直接走默认目标执行。
    #[test]
    fn test_source_link_outcome_routes_main_branches() {
        assert_eq!(
            source_link_outcome(MenuInputKind::Transfer, false, None),
            SourceLinkOutcome::ReaskSource
        );
        assert_eq!(
            source_link_outcome(MenuInputKind::Transfer, true, None),
            SourceLinkOutcome::ChooseTarget
        );
        assert_eq!(
            source_link_outcome(MenuInputKind::TransferDefault, true, None),
            SourceLinkOutcome::ChooseTargetWithMissingDefault
        );
        assert_eq!(
            source_link_outcome(MenuInputKind::TransferDefault, true, Some(-100)),
            SourceLinkOutcome::DispatchDefaultTarget {
                target_chat_id: -100
            }
        );
    }

    // 源链接阶段应能统一映射到高层 UI 动作。
    #[test]
    fn test_source_link_ui_action_routes_main_branches() {
        assert_eq!(
            source_link_ui_action(
                MenuInputKind::Transfer,
                "bad-link",
                SourceLinkOutcome::ReaskSource
            ),
            FlowUiAction::ReaskSource
        );
        assert_eq!(
            source_link_ui_action(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2",
                SourceLinkOutcome::ChooseTarget
            ),
            FlowUiAction::ShowTargetChoice {
                kind: MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned(),
                detail: None
            }
        );
        assert_eq!(
            source_link_ui_action(
                MenuInputKind::TransferDefault,
                "https://t.me/c/1/2",
                SourceLinkOutcome::ChooseTargetWithMissingDefault
            ),
            FlowUiAction::ShowTargetChoice {
                kind: MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned(),
                detail: Some("当前没有默认目标，请改为手动选择目标。")
            }
        );
        assert!(!matches!(
            source_link_ui_action(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2",
                SourceLinkOutcome::ChooseTarget
            ),
            FlowUiAction::ReaskTarget { .. }
                | FlowUiAction::ShowConfirm { .. }
                | FlowUiAction::RechooseTargetAfterShared { .. }
        ));
    }

    // 目标不可用的提示标题和说明应统一抽成单点，避免共享选群回退时风格漂移。
    #[test]
    fn test_target_unavailable_text_is_stable() {
        assert_eq!(build_target_unavailable_title(), "目标不可用");
        assert!(build_target_unavailable_detail().contains("allowed_target_chat_ids"));
    }

    // 共享选群失败时，最终回退应仍然回到“重新选择目标”的语义，而不是继续停在选群阶段。
    #[test]
    fn test_shared_chat_outcome_failed_target_is_rechoose() {
        let outcome = shared_chat_outcome(
            TARGET_CHAT_REQUEST_BUTTON_ID,
            Some(&MenuInputStep::ChatPicker {
                kind: MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned(),
            }),
            TARGET_CHAT_REQUEST_BUTTON_ID,
            false,
        );

        assert_eq!(outcome, SharedChatOutcome::RechooseTarget);
    }

    // 共享选群阶段应只映射到“回退重选目标 / 进入确认”两类 UI 动作。
    #[test]
    fn test_shared_chat_ui_action_routes_main_branches() {
        assert_eq!(
            shared_chat_ui_action(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2".to_owned(),
                -100,
                SharedChatOutcome::RechooseTarget
            ),
            Some(FlowUiAction::RechooseTargetAfterShared {
                kind: MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned()
            })
        );
        assert_eq!(
            shared_chat_ui_action(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2".to_owned(),
                -100,
                SharedChatOutcome::ConfirmSelected
            ),
            Some(FlowUiAction::ShowConfirm {
                kind: MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned(),
                target_chat_id: -100
            })
        );
        assert_eq!(
            shared_chat_ui_action(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2".to_owned(),
                -100,
                SharedChatOutcome::WrongStep
            ),
            None
        );
    }

    // 共享选群在旧按钮或错阶段下不应误进入确认。
    #[test]
    fn test_shared_chat_outcome_does_not_confirm_on_ignored_or_wrong_step() {
        assert_eq!(
            shared_chat_outcome(9999, None, TARGET_CHAT_REQUEST_BUTTON_ID, true),
            SharedChatOutcome::Ignored
        );
        assert_eq!(
            shared_chat_outcome(
                TARGET_CHAT_REQUEST_BUTTON_ID,
                Some(&MenuInputStep::Confirm {
                    kind: MenuInputKind::Transfer,
                    source_link: "https://t.me/c/1/2".to_owned(),
                    target_chat_id: -100,
                }),
                TARGET_CHAT_REQUEST_BUTTON_ID,
                true
            ),
            SharedChatOutcome::WrongStep
        );
    }
}
