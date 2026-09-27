//! `/menu` 多步骤流式向导文本输入处理与状态推进模块。
//!
//! 承接“输入源链接 -> 选择/输入目标会话 -> 最终操作确认”多步向导中的文本输入事件，
//! 通过纯决策函数与 UI 动作映射解耦状态判断与消息渲染逻辑。

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

/// 目标会话文本输入解析后的后续逻辑流转决策枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetInputOutcome {
    /// 目标格式不合法或未匹配到别名，需要重新提示用户输入目标
    ReaskTarget,
    /// 目标解析成功，推进到最终确认卡片
    AskConfirm {
        /// 解析出的目标聊天会话 ID
        target_chat_id: i64,
    },
}

/// 来源链接文本输入解析后的后续逻辑流转决策枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceLinkOutcome {
    /// 链接外观校验不通过，需要重新提示用户输入来源链接
    ReaskSource,
    /// 来源链接合法，需要引导用户进入选择/输入目标会话阶段
    ChooseTarget,
    /// 属于快速转存/查询流程（使用默认目标），直接使用默认目标派发执行
    DispatchDefaultTarget {
        /// 默认目标会话 ID
        target_chat_id: i64,
    },
}

/// 多步骤向导在各阶段动作完成后最终需要触发的 Telegram 界面行为枚举。
#[derive(Debug, Clone, PartialEq, Eq)]
enum FlowUiAction {
    /// 发送重新输入来源链接的 ForceReply 引导卡片
    ReaskSource,
    /// 发送目标选择键盘（包括预置常用目标、快速按钮及手动输入入口）
    ShowTargetChoice {
        kind: MenuInputKind,
        source_link: String,
    },
    /// 发送重新输入目标会话的 ForceReply 引导卡片
    ReaskTarget {
        kind: MenuInputKind,
        source_link: String,
    },
    /// 发送操作前最终确认卡片（附带执行与取消内联按钮）
    ShowConfirm {
        kind: MenuInputKind,
        source_link: String,
        target_chat_id: i64,
    },
    /// 直接调用底层命令执行（快速转存/默认目标链路）
    DispatchDefaultTarget {
        kind: MenuInputKind,
        source_link: String,
        target_chat_id: i64,
    },
}

/// 多步骤流式向导请求处理上下文。
///
/// 封装单个向导交互流程所需的所有依赖项与会话信息。
#[derive(Clone)]
pub(super) struct FlowRequestContext {
    /// 会话唯一标识元组 `(request_chat_id, sender_user_id)`
    pub(super) key: (i64, i64),
    /// 当前生效的机器人配置快照弧指针
    pub(super) config: Arc<BotConfig>,
    /// 发起交互的群组/频道或私聊会话 ID
    pub(super) request_chat_id: i64,
    /// 触发本轮交互的用户消息 ID
    pub(super) request_message_id: i64,
    /// 操作发起者的身份角色与权限
    pub(super) actor: crate::config::RequestActor,
    /// TDLib 客户端实例标识符
    pub(super) client_id: i32,
}

/// 重新发送多步骤向导当前阶段的引导卡片与输入提示。
///
/// 当用户点击“继续输入：xxx”时，根据草稿当前所处的步骤重新呈现相应的提示：
/// - `SourceLink`: 重新发送第一步输入链接的 ForceReply 提示；
/// - `TargetChoice`: 重新展示目标会话选择键盘；
/// - `TargetChat`: 重新发送手动输入 target_chat_id 的 ForceReply 提示；
/// - `Confirm`: 重新展示最终确认卡片；
/// - `ChatPicker`: 重新发送原生选聊键盘；
/// - 若草稿属于单步输入（如 JobId、AdminInput），则返回 `Ok(false)` 交由外层简单分支处理。
///
/// # 参数说明
/// - `app`: 全局应用上下文引用
/// - `draft`: 当前用户的草稿对象引用
/// - `config`: 机器人配置引用
/// - `chat_id`: 目标聊天会话 ID
/// - `user_id`: 目标用户 ID
/// - `client_id`: 客户端实例标识符
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
        // 非向导多步流程，放行给外部处理
        MenuInputStep::JobId { .. } | MenuInputStep::AdminInput { .. } => Ok(false),
    }
}

/// 处理菜单多步骤流式向导文本输入的主入口。
///
/// 仅负责流式向导阶段：
/// - 源链接输入；
/// - 目标选择 / 手动目标输入；
/// - 执行前确认。
///
/// # 返回值
/// - `Ok(Some(true))`: 本条输入已被流式向导成功消费；
/// - `Ok(None)`: 本草稿不属于流式向导步骤（如 JobId），由外层简单逻辑继续处理。
pub(super) async fn handle_flow_input(
    app: &crate::app_context::AppContext,
    draft: MenuInputDraft,
    input: &str,
    ctx: FlowRequestContext,
) -> anyhow::Result<Option<bool>> {
    match draft.step {
        // 第一步：处理来源链接输入
        MenuInputStep::SourceLink { kind } => handle_source_link_input(app, kind, input, ctx).await,
        // 第二步：处理目标会话输入
        MenuInputStep::TargetChoice { kind, source_link }
        | MenuInputStep::TargetChat { kind, source_link } => {
            handle_target_input(app, kind, source_link, input, ctx).await
        }
        // 第三步：处于等待确认阶段却收到文本输入，重新展示确认卡片
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
        // 原生选聊阶段收到普通文本，重新展示选聊提示
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
        // 单步操作放行
        MenuInputStep::JobId { .. } | MenuInputStep::AdminInput { .. } => Ok(None),
    }
}

/// 目标文本输入解析的纯决策函数（不产生副作用，便于单测）。
fn target_input_outcome(target_chat_id: Option<i64>) -> TargetInputOutcome {
    match target_chat_id {
        Some(target_chat_id) => TargetInputOutcome::AskConfirm { target_chat_id },
        None => TargetInputOutcome::ReaskTarget,
    }
}

/// 来源链接输入解析的纯决策函数。
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

/// 将来源链接阶段的纯决策结果映射为统一的高层 UI 动作。
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

/// 将目标会话阶段的纯决策结果映射为统一的高层 UI 动作。
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

/// 处理向导第一步：来源链接文本输入。
///
/// 业务逻辑：
/// 1. 检验输入是否形如 Telegram 消息链接；
/// 2. 若不合法：保留 `SourceLink` 草稿并发送格式错误卡片；
/// 3. 若合法且为快速转存（使用默认目标）：直接调度既有命令执行并记录最近使用目标；
/// 4. 若合法且需要手动指定目标：推进为 `TargetChoice` 草稿并展示目标选择键盘。
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
        // 链接校验失败：重试
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
        // 链接合法，进入目标选择分支
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
        // 快速链路：直接采用系统默认目标执行转存或查询
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
            // 记忆最近使用的目标会话 ID
            remember_last_target(ctx.request_chat_id, ctx.key.1, target_chat_id);
            tracing::debug!(
                request_chat_id = ctx.request_chat_id,
                sender_user_id = ctx.key.1,
                request_message_id = ctx.request_message_id,
                target_chat_id,
                input_kind = kind.log_name(),
                "menu input resolved default target"
            );
            // 调用底层已有的 transfer_cmd 或 lookup 命令执行器
            run_existing_command(
                kind,
                command_owned,
                ctx.config,
                ExistingCommandContext {
                    // 默认目标分支继续沿用当前请求拿到的运行态，避免回退到全局单例
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
        // 异常分支兜底：重新提示输入来源链接
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

/// 处理向导第二步：目标会话文本输入（支持直接输入数字 ID、配置的目标别名或 `default`）。
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
        // 目标无法解析（非有效 ID 且未匹配任何别名）：提示重新输入
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
        // 目标会话解析成功：持久化确认状态草稿并展示确认卡片
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
        // 兜底分支：重新提示输入目标
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

    /// 测试目标输入纯决策逻辑：
    /// 未解析出目标时重新询问（ReaskTarget），成功解析出 ID 时推进至确认状态（AskConfirm）。
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

    /// 测试目标输入决策映射为统一 UI 动作：
    /// 确保严格映射到 ReaskTarget 或 ShowConfirm，不会误触发来源链接动作。
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

    /// 测试来源链接输入的纯决策逻辑：
    /// 格式错误时重新要求输入（ReaskSource），普通转存进入选目标（ChooseTarget），
    /// 默认转存直接派发执行（DispatchDefaultTarget）。
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

    /// 测试来源链接阶段向高层 UI 动作的映射转换。
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
