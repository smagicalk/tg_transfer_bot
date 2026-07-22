// `/menu` 多步向导的文本输入处理。
// 这里承接 `menu/input.rs` 的主流程分支，让入口文件只负责路由和少量编排。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::text::{
    build_step_prompt_text, build_step_prompt_with_context, build_target_input_prompt_text,
};
use super::MenuInputKind;
use super::flow::{
    ExistingCommandContext, ExistingCommandOrigin, looks_like_telegram_link, run_existing_command,
};
use super::state::{
    MenuInputDraft, MenuInputStep, put_confirm_draft, put_draft, remember_last_target,
};
use super::target::{
    TargetPromptContext, confirm_button_rows, resolve_default_target_on, resolve_target_input_on,
    send_confirm_prompt, send_target_choice_prompt,
};

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
    DispatchDefaultTarget { target_chat_id: i64 },
}

/// 多步向导阶段动作最终要触发的 UI 行为。
#[derive(Debug, Clone, PartialEq, Eq)]
enum FlowUiAction {
    ReaskSource,
    ShowTargetChoice {
        kind: MenuInputKind,
        source_link: String,
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
                build_step_prompt_text(
                    kind.source_step_label(),
                    kind.source_title(),
                    kind.source_detail(),
                ),
                chat_id,
                "输入源链接（回复“取消”可退出）",
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
        MenuInputStep::TargetChat { kind, source_link } => {
            send::send_card_message_with_force_reply_returning(
                build_target_input_prompt_text(
                    source_link,
                    "输入目标 chat",
                    "请回复目标 chat_id、别名或 default；回复“取消”可退出。",
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
        MenuInputStep::Confirm {
            kind,
            source_link,
            target_chat_id,
        } => {
            send_confirm_prompt(
                *kind,
                source_link,
                *target_chat_id,
                None,
                chat_id,
                client_id,
            )
            .await?;
            Ok(true)
        }
        MenuInputStep::ChatPicker { source_link, .. } => {
            super::send_target_chat_picker_prompt(chat_id, user_id, source_link, None, client_id)
                .await?;
            Ok(true)
        }
        MenuInputStep::JobId { .. } | MenuInputStep::AdminInput { .. } => Ok(false),
    }
}

/// 处理菜单多步输入。
///
/// 仅处理：
/// - 源链接
/// - 目标选择 / 手输目标 / 选聊结果
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
        | MenuInputStep::TargetChat { kind, source_link } => {
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
                "请点击确认卡片里的“执行”，回复“取消”可退出。",
                None,
                Some(target_chat_id),
            ))
            .rows(confirm_button_rows())
            .send(ctx.request_chat_id, ctx.client_id)
            .await?;
            Ok(Some(true))
        }
        MenuInputStep::ChatPicker { kind, source_link } => {
            put_draft(
                ctx.key,
                MenuInputDraft::chat_picker(kind, source_link.clone()),
            )
            .await?;
            super::send_target_chat_picker_prompt(
                ctx.request_chat_id,
                ctx.key.1,
                &source_link,
                None,
                ctx.client_id,
            )
            .await?;
            Ok(Some(true))
        }
        MenuInputStep::JobId { .. } | MenuInputStep::AdminInput { .. } => Ok(None),
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
    default_target_chat_id: i64,
) -> SourceLinkOutcome {
    if !looks_like_link {
        return SourceLinkOutcome::ReaskSource;
    }
    if !kind.uses_default_target() {
        return SourceLinkOutcome::ChooseTarget;
    }
    SourceLinkOutcome::DispatchDefaultTarget {
        target_chat_id: default_target_chat_id,
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
                    kind.source_step_label(),
                    "源链接格式不正确",
                    "请回复 t.me 消息链接，回复“取消”可退出。",
                ),
                ctx.request_chat_id,
                "输入 https://t.me/... 链接",
                ctx.client_id,
            )
            .await?;
            Ok(Some(true))
        }
        FlowUiAction::ShowTargetChoice { kind, source_link } => {
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
                    origin: ExistingCommandOrigin::TextInput,
                    actor: ctx.actor,
                    client_id: ctx.client_id,
                },
            )
            .await?;
            Ok(Some(true))
        }
        FlowUiAction::ReaskTarget { .. } | FlowUiAction::ShowConfirm { .. } => {
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
                    kind.source_step_label(),
                    "源链接格式不正确",
                    "请回复 t.me 消息链接，回复“取消”可退出。",
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
            let invalid_target_text = build_target_input_prompt_text(
                &source_link,
                "目标 chat 格式不正确",
                "请回复数字 chat_id、配置里的目标别名，或回复 default 使用配置默认目标。",
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
                None,
                ctx.request_chat_id,
                ctx.client_id,
            )
            .await?;
            Ok(Some(true))
        }
        FlowUiAction::ReaskSource
        | FlowUiAction::ShowTargetChoice { .. }
        | FlowUiAction::DispatchDefaultTarget { .. } => {
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
                build_target_input_prompt_text(
                    &source_link,
                    "输入目标 chat",
                    "请回复目标 chat_id、别名或 default；回复“取消”可退出。",
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
        ));
    }

    // 源链接输入应能统一表达：重新输入、改为选目标或直接走默认目标执行。
    #[test]
    fn test_source_link_outcome_routes_main_branches() {
        assert_eq!(
            source_link_outcome(MenuInputKind::Transfer, false, -100),
            SourceLinkOutcome::ReaskSource
        );
        assert_eq!(
            source_link_outcome(MenuInputKind::Transfer, true, -100),
            SourceLinkOutcome::ChooseTarget
        );
        assert_eq!(
            source_link_outcome(MenuInputKind::TransferDefault, true, -100),
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
                source_link: "https://t.me/c/1/2".to_owned()
            }
        );
        assert!(!matches!(
            source_link_ui_action(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2",
                SourceLinkOutcome::ChooseTarget
            ),
            FlowUiAction::ReaskTarget { .. } | FlowUiAction::ShowConfirm { .. }
        ));
    }
}
