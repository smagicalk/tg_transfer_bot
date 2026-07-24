// `/menu` ForceReply 输入流程。
// 本文件只保留普通输入事件处理；按钮、草稿状态和目标选择视图分别放到子模块。

mod admin;
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
    AdminCommandKind, admin_command_kind, parse_admin_input_payload, run_existing_config_command,
    run_existing_targets_command,
};
use self::flow_callbacks::{FlowRequestContext, continue_flow_input_on, handle_flow_input};
use self::simple::{
    expired_input_detail_on, is_cancel_text, parse_job_id_input, run_existing_job_command,
    send_cancelled_notice,
};
use super::text::{
    build_menu_recovery_text, build_step_prompt_text, build_step_prompt_with_context,
    build_target_input_prompt_text,
};
pub(super) use callbacks_simple::{
    admin_input_callback_query, admin_input_callback_query_with_context,
    cancel_input_callback_query, job_id_input_callback_query,
};
pub(super) use callbacks_target::{
    target_alias_callback_query, target_back_callback_query, target_confirm_callback_query,
    target_default_callback_query, target_manual_callback_query,
    target_request_chat_callback_query, target_source_back_callback_query,
};
pub(in crate::tgbot::transfer::command) use state::AdminInputAction;
use state::{
    AdminInputContextTakeResult, DraftTakeResult, MenuInputDraft, MenuInputStep, TargetContext,
    TargetContextAdvanceResult, admin_chat_request_button_ids, admin_input_prompt_meta,
    advance_shared_target_context, is_admin_chat_request_button, peek_current_draft, put_draft,
    remember_admin_picker_message, remember_target_picker_message, take_admin_picker_message,
    take_current_draft, take_shared_admin_input_context, take_target_picker_message,
};
pub(super) use state::{
    MenuInputKind, MenuJobAction, cancel_menu_input, cancel_menu_input_with_result,
    start_menu_input,
};

use self::target::{TargetPromptContext, send_confirm_prompt, send_target_choice_prompt};

/// Telegram 原生目标群组/频道选择按钮 ID。
pub(super) const TARGET_GROUP_CHAT_REQUEST_BUTTON_ID: i32 = 7001;
pub(super) const TARGET_CHANNEL_CHAT_REQUEST_BUTTON_ID: i32 = 7002;
/// 原生聊天 reply keyboard 的手动输入兜底按钮文案。
pub(super) const TARGET_CHAT_MANUAL_INPUT_TEXT: &str = "手动输入目标";

fn is_target_chat_request_button(button_id: i32) -> bool {
    matches!(
        button_id,
        TARGET_GROUP_CHAT_REQUEST_BUTTON_ID | TARGET_CHANNEL_CHAT_REQUEST_BUTTON_ID
    )
}

/// 判断一条文字消息是否来自目标聊天选择键盘的手动输入按钮。
///
/// 这个按钮不是 inline callback，而是普通文本；因此必须在消费草稿后、
/// 进入通用 ForceReply 解析前单独分流。
fn is_target_chat_manual_input(input: &str) -> bool {
    input == TARGET_CHAT_MANUAL_INPUT_TEXT
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SharedChatInputDecision {
    Ignore,
    Confirm(TargetContext),
    Expired,
    Missing,
    Stale,
}

fn shared_chat_input_decision(
    button_id: i32,
    result: TargetContextAdvanceResult,
) -> SharedChatInputDecision {
    if !is_target_chat_request_button(button_id) {
        return SharedChatInputDecision::Ignore;
    }
    match result {
        TargetContextAdvanceResult::Active(context) => SharedChatInputDecision::Confirm(context),
        TargetContextAdvanceResult::Expired => SharedChatInputDecision::Expired,
        TargetContextAdvanceResult::None => SharedChatInputDecision::Missing,
        TargetContextAdvanceResult::WrongStep => SharedChatInputDecision::Stale,
    }
}

/// 清理原生选聊提示及其默认 reply markup。
///
/// 先移除 chat 级键盘，再删除承载键盘的 bot 消息；任一步失败都只记录日志，
/// 避免一次界面清理失败阻断已经完成的目标选择。
async fn delete_native_picker_prompt(
    request_chat_id: i64,
    sender_user_id: i64,
    message_id: i64,
    client_id: i32,
) {
    if let Err(error) = send::delete_chat_reply_markup(request_chat_id, message_id, client_id).await
    {
        tracing::debug!(
            request_chat_id,
            sender_user_id,
            picker_message_id = message_id,
            error = %error,
            "native picker reply markup could not be deleted"
        );
    }
    if let Err(error) = send::delete_message(request_chat_id, message_id, client_id).await {
        tracing::debug!(
            request_chat_id,
            sender_user_id,
            picker_message_id = message_id,
            error = %error,
            "native picker message could not be deleted"
        );
    }
}

/// 清理当前会话可能遗留的目标/目标管理 picker。
///
/// 命令切换、取消和超时都会调用它；两个 tracker 分开消费，避免旧流程的
/// reply keyboard 在新流程中继续产生共享聊天消息。
pub(super) async fn clear_native_picker_messages(
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> bool {
    let mut found = false;
    if let Some(message_id) = take_target_picker_message((request_chat_id, sender_user_id)) {
        found = true;
        delete_native_picker_prompt(request_chat_id, sender_user_id, message_id, client_id).await;
    }
    if let Some(message_id) = take_admin_picker_message((request_chat_id, sender_user_id)) {
        found = true;
        delete_native_picker_prompt(request_chat_id, sender_user_id, message_id, client_id).await;
    }
    found
}

pub(super) async fn send_target_chat_picker_prompt(
    request_chat_id: i64,
    sender_user_id: i64,
    source_link: &str,
    stale_message_id: Option<i64>,
    client_id: i32,
) -> anyhow::Result<()> {
    let sent = send::send_card_message_with_target_chat_request_keyboard_returning(
        build_step_prompt_with_context(
            "waiting-chat",
            "2/3",
            "选择目标聊天",
            "请选择 Bot 已加入的群组，或 Bot 具有发帖权限的频道；也可点击“手动输入目标”直接填写。",
            Some(source_link),
            None,
        ),
        request_chat_id,
        TARGET_GROUP_CHAT_REQUEST_BUTTON_ID,
        TARGET_CHANNEL_CHAT_REQUEST_BUTTON_ID,
        client_id,
    )
    .await?;
    // callback 打开的 picker 前通常还有一张已编辑为“等待中”的 inline 卡片；
    // 删除它，避免选择器和等待卡同时留在对话里。其他恢复入口没有旧消息 ID。
    if let Some(stale_message_id) = stale_message_id
        && stale_message_id > 0
        && stale_message_id != sent.id
        && let Err(error) = send::delete_message(request_chat_id, stale_message_id, client_id).await
    {
        tracing::debug!(
            request_chat_id,
            sender_user_id,
            stale_message_id,
            error = %error,
            "stale target choice card could not be deleted"
        );
    }
    // 同一个草稿只保留最新 picker；阶段重复进入时先删除旧卡片，避免用户看到多组
    // 看起来都可以点击、实际却只对应当前草稿的按钮。
    if let Some(previous_id) =
        remember_target_picker_message((request_chat_id, sender_user_id), sent.id)
        && previous_id != sent.id
    {
        delete_native_picker_prompt(request_chat_id, sender_user_id, previous_id, client_id).await;
    }
    Ok(())
}

/// 原生选聊返回的目标显示名；标题优先，username 作为降级。
fn shared_chat_display_name(chat: &tdlib_rs::types::SharedChat) -> Option<String> {
    let title = chat.title.trim();
    if !title.is_empty() {
        return Some(title.to_owned());
    }
    let username = chat.username.trim().trim_start_matches('@');
    (!username.is_empty()).then(|| format!("@{username}"))
}

pub(in crate::tgbot::transfer::command::menu) async fn handle_shared_chat_input_on(
    shared: &tdlib_rs::types::MessageChatShared,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    let key = (request_chat_id, sender_user_id);
    // 管理目标 picker 使用独立 ID 段；先校验它，避免旧的转存/alias 键盘误触发当前草稿。
    if is_admin_chat_request_button(shared.button_id) {
        match take_shared_admin_input_context(key, shared.button_id).await? {
            AdminInputContextTakeResult::Active(context) => {
                // 只有当前草稿接受了这次共享结果，才能删除已登记的 picker；
                // 迟到的旧 button_id 不能误删新一轮选择器。
                if let Some(picker_message_id) = take_admin_picker_message(key) {
                    delete_native_picker_prompt(
                        request_chat_id,
                        sender_user_id,
                        picker_message_id,
                        client_id,
                    )
                    .await;
                }
                let Some(command_owned) = shared_admin_chat_command(&context, shared.chat.chat_id)
                else {
                    send::send_card_message_with_remove_keyboard(
                        build_menu_recovery_text(
                            "目标选择已失效",
                            "stale",
                            "没有找到对应的别名或默认目标输入，请返回目标页重新开始。",
                        ),
                        request_chat_id,
                        client_id,
                    )
                    .await?;
                    return Ok(true);
                };
                let app = crate::app_context::app_context();
                if let Err(err) = run_existing_targets_command(
                    app.as_ref(),
                    command_owned,
                    request_chat_id,
                    client_id,
                )
                .await
                {
                    put_draft(
                        key,
                        MenuInputDraft::admin_input(
                            context.action,
                            context.context_text.clone(),
                            context.context_i64,
                        ),
                    )
                    .await?;
                    tracing::warn!(
                        request_chat_id,
                        sender_user_id,
                        target_chat_id = shared.chat.chat_id,
                        admin_action = context.action.log_name(),
                        error = %err,
                        "shared target chat admin command failed, waiting for retry"
                    );
                    let meta = admin_input_prompt_meta(
                        context.action,
                        context.context_text.as_deref(),
                        context.context_i64,
                    );
                    let detail =
                        build_input_retry_detail(&format!("执行失败：{}。", err), &meta.detail);
                    send_admin_input_prompt(
                        context.action,
                        context.context_i64,
                        admin_input_step_label(
                            context.action,
                            context.context_text.as_deref(),
                            context.context_i64,
                        ),
                        "目标未更新",
                        &detail,
                        &meta.placeholder,
                        request_chat_id,
                        sender_user_id,
                        client_id,
                    )
                    .await?;
                }
                return Ok(true);
            }
            AdminInputContextTakeResult::Expired => {
                clear_native_picker_messages(request_chat_id, sender_user_id, client_id).await;
                send::send_card_message_with_remove_keyboard(
                    build_menu_recovery_text(
                        "选择已过期",
                        "expired",
                        "上一次目标设置已过期，请返回目标页重新开始。",
                    ),
                    request_chat_id,
                    client_id,
                )
                .await?;
                return Ok(true);
            }
            AdminInputContextTakeResult::None | AdminInputContextTakeResult::WrongStep => {
                send::send_card_message_with_remove_keyboard(
                    build_menu_recovery_text(
                        "选择已失效",
                        "stale",
                        "这次聊天选择不属于当前目标设置，请返回目标页重新开始。",
                    ),
                    request_chat_id,
                    client_id,
                )
                .await?;
                return Ok(true);
            }
        }
    }

    if !is_target_chat_request_button(shared.button_id) {
        return Ok(false);
    }

    let result = advance_shared_target_context(key, shared.chat.chat_id).await?;
    // Telegram 已把选择结果送达后，旧 picker 消息和 chat 级 reply keyboard 都已过时。
    // 先清理再发送确认卡，界面只保留当前阶段需要的操作。
    if let Some(picker_message_id) = take_target_picker_message(key) {
        delete_native_picker_prompt(
            request_chat_id,
            sender_user_id,
            picker_message_id,
            client_id,
        )
        .await;
    }
    match shared_chat_input_decision(shared.button_id, result) {
        SharedChatInputDecision::Ignore => Ok(false),
        SharedChatInputDecision::Confirm(context) => {
            let display_name = shared_chat_display_name(&shared.chat);
            send_confirm_prompt(
                context.kind,
                &context.source_link,
                shared.chat.chat_id,
                display_name.as_deref(),
                request_chat_id,
                client_id,
            )
            .await?;
            Ok(true)
        }
        SharedChatInputDecision::Expired => {
            send::send_card_message_with_remove_keyboard(
                build_menu_recovery_text(
                    "选择已过期",
                    "expired",
                    "上一次目标选择已过期，请返回菜单重新开始。",
                ),
                request_chat_id,
                client_id,
            )
            .await?;
            Ok(true)
        }
        SharedChatInputDecision::Missing | SharedChatInputDecision::Stale => {
            send::send_card_message_with_remove_keyboard(
                build_menu_recovery_text(
                    "选择已失效",
                    "stale",
                    "当前输入流程已经改变，请点击“继续输入”或返回菜单重新开始。",
                ),
                request_chat_id,
                client_id,
            )
            .await?;
            Ok(true)
        }
    }
}

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

/// 构造输入失败后的重试说明。
///
/// 失败原因和下一步格式分开展示，避免用户只看到“失败”但不知道应该继续回复什么。
fn build_input_retry_detail(reason: &str, next_detail: &str) -> String {
    format!("{reason}\n\n{next_detail}")
}

/// 解析单个别名输入。
///
/// 这里明确不接受空白分隔的多词别名，保持和 `/targets set-alias <alias> <target>` 的命令格式一致。
fn parse_single_alias_input(input: &str) -> Option<String> {
    let mut parts = input.split_whitespace();
    let alias = parts.next()?.trim();
    if alias.is_empty() || parts.next().is_some() {
        return None;
    }
    Some(alias.to_owned())
}

/// 把原生选聊返回的 chat_id 转成现有 `/targets` 命令参数。
///
/// 共享聊天消息和 ForceReply 文本输入必须共用同一套参数解析，避免两条路径
/// 对 alias 上下文或数字格式产生不同语义。
fn shared_admin_chat_command(
    context: &state::AdminInputContext,
    target_chat_id: i64,
) -> Option<Vec<String>> {
    parse_admin_input_payload(
        context.action,
        &target_chat_id.to_string(),
        None,
        context.context_text.as_deref(),
        context.context_i64,
    )
}

/// 管理输入当前阶段标签。
fn admin_input_step_label(
    action: AdminInputAction,
    context_text: Option<&str>,
    _context_i64: Option<i64>,
) -> &'static str {
    match action {
        AdminInputAction::TargetsAliasName => "1/2",
        AdminInputAction::TargetsSetAlias if context_text.is_some() => "2/2",
        _ => "1/1",
    }
}

/// 发送管理输入提示；目标 chat_id 默认使用 Telegram 原生选聊，仍允许直接输入数字。
///
/// 参数同时覆盖“当前动作/上下文”“当前步骤文案”和“发送坐标”三组信息；
/// 保持这些值显式传递，便于 ForceReply 与原生选聊两种 markup 共用同一入口。
#[allow(clippy::too_many_arguments)]
pub(super) async fn send_admin_input_prompt(
    action: AdminInputAction,
    picker_token: Option<i64>,
    step_label: &str,
    title: &str,
    detail: &str,
    placeholder: &str,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    // 没有 token 的旧草稿仍走 ForceReply，兼容升级前已经存在的输入状态。
    if let Some((group_button_id, channel_button_id)) =
        admin_chat_request_button_ids(action, picker_token)
    {
        let detail = format!("{detail}\n\n可点击下方按钮选择群组或频道，也可直接输入 chat_id。");
        let sent = send::send_card_message_with_target_chat_request_keyboard_returning(
            build_step_prompt_text(step_label, title, &detail),
            request_chat_id,
            group_button_id,
            channel_button_id,
            client_id,
        )
        .await?;
        if let Some(previous_id) =
            remember_admin_picker_message((request_chat_id, sender_user_id), sent.id)
            && previous_id != sent.id
        {
            delete_native_picker_prompt(request_chat_id, sender_user_id, previous_id, client_id)
                .await;
        }
    } else {
        // 切回 ForceReply 时，先关闭并删除上一次目标管理选聊卡片。
        if let Some(previous_id) = take_admin_picker_message((request_chat_id, sender_user_id)) {
            delete_native_picker_prompt(request_chat_id, sender_user_id, previous_id, client_id)
                .await;
        }
        send::send_card_message_with_force_reply_returning(
            build_step_prompt_text(step_label, title, detail),
            request_chat_id,
            placeholder,
            client_id,
        )
        .await?;
    }
    Ok(())
}

#[cfg(test)]
fn build_continue_input_expired_text() -> String {
    let app_context = crate::app_context::app_context();
    build_continue_input_expired_text_on(app_context.as_ref())
}

/// 从已知源消息直接启动目标选择流程。
pub(super) async fn start_transfer_target_choice_with_source_on(
    app: &crate::app_context::AppContext,
    config: std::sync::Arc<BotConfig>,
    chat_id: i64,
    sender_user_id: i64,
    kind: MenuInputKind,
    source_link: String,
    client_id: i32,
) -> anyhow::Result<()> {
    state::put_target_choice_draft((chat_id, sender_user_id), kind, source_link.clone()).await?;
    send_target_choice_prompt(
        config.as_ref(),
        TargetPromptContext {
            app,
            request_chat_id: chat_id,
            sender_user_id,
            client_id,
        },
        kind,
        &source_link,
    )
    .await
}

/// 从纯链接文本直接启动目标选择流程。
pub(super) async fn start_transfer_target_choice_from_link_on(
    app: &crate::app_context::AppContext,
    config: std::sync::Arc<BotConfig>,
    chat_id: i64,
    sender_user_id: i64,
    source_link: String,
    client_id: i32,
) -> anyhow::Result<()> {
    start_transfer_target_choice_with_source_on(
        app,
        config,
        chat_id,
        sender_user_id,
        MenuInputKind::Transfer,
        source_link,
        client_id,
    )
    .await
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
                "输入 job_id（回复“取消”可退出）",
                client_id,
            )
            .await?;
        }
        MenuInputStep::AdminInput {
            action,
            context_text,
            context_i64,
        } => {
            let meta = admin_input_prompt_meta(action, context_text.as_deref(), context_i64);
            send_admin_input_prompt(
                action,
                context_i64,
                admin_input_step_label(action, context_text.as_deref(), context_i64),
                &meta.title,
                &meta.detail,
                &meta.placeholder,
                chat_id,
                user_id,
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
            clear_native_picker_messages(request_chat_id, sender_user_id, client_id).await;
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
        let had_picker = matches!(&draft.step, MenuInputStep::ChatPicker { .. })
            || matches!(
                &draft.step,
                MenuInputStep::AdminInput {
                    action,
                    context_i64,
                    ..
                } if admin_chat_request_button_ids(*action, *context_i64).is_some()
            );
        let picker_message_cleared =
            clear_native_picker_messages(request_chat_id, sender_user_id, client_id).await;
        tracing::debug!(
            request_chat_id,
            sender_user_id,
            request_message_id,
            "menu input cancelled by text"
        );
        send_cancelled_notice(
            request_chat_id,
            client_id,
            had_picker && !picker_message_cleared,
        )
        .await?;
        return Ok(true);
    }

    if is_target_chat_manual_input(input) {
        match &draft.step {
            MenuInputStep::ChatPicker { kind, source_link } => {
                let kind = *kind;
                let source_link = source_link.clone();
                if let Some(picker_message_id) = take_target_picker_message(key) {
                    delete_native_picker_prompt(
                        request_chat_id,
                        sender_user_id,
                        picker_message_id,
                        client_id,
                    )
                    .await;
                }
                // 普通文本按钮没有 callback 可供编辑原卡片；改写草稿并发送
                // ForceReply 后，Telegram 会用新的输入提示替换原生选聊键盘。
                put_draft(key, MenuInputDraft::target_chat(kind, source_link.clone())).await?;
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    input_kind = kind.log_name(),
                    "menu target chat picker switched to manual input"
                );
                send::send_card_message_with_force_reply_returning(
                    build_target_input_prompt_text(
                        &source_link,
                        "输入目标",
                        "请回复数字 chat_id、配置里的目标别名，或回复 default 使用配置默认目标。",
                    ),
                    request_chat_id,
                    "输入目标 chat_id、别名或 default",
                    client_id,
                )
                .await?;
                return Ok(true);
            }
            MenuInputStep::AdminInput {
                action,
                context_text,
                context_i64,
            } if admin_chat_request_button_ids(*action, *context_i64).is_some() => {
                let action = *action;
                let context_text = context_text.clone();
                // context_i64 只保存本次原生 picker 的 token；切到手动输入后
                // 必须清掉它，避免旧共享聊天结果继续命中新的 ForceReply 草稿。
                put_draft(
                    key,
                    MenuInputDraft::admin_input(action, context_text.clone(), None),
                )
                .await?;
                let meta = admin_input_prompt_meta(action, context_text.as_deref(), None);
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    admin_action = action.log_name(),
                    "menu admin chat picker switched to manual input"
                );
                send_admin_input_prompt(
                    action,
                    None,
                    admin_input_step_label(action, context_text.as_deref(), None),
                    &meta.title,
                    &meta.detail,
                    &meta.placeholder,
                    request_chat_id,
                    sender_user_id,
                    client_id,
                )
                .await?;
                return Ok(true);
            }
            _ => {}
        }
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
                        "请回复纯数字 job_id，例如 42；回复“取消”可退出。",
                    ),
                    request_chat_id,
                    "输入数字 job_id（回复“取消”可退出）",
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
            if let Err(err) = run_existing_job_command(app, action, job_id, actor, client_id).await
            {
                put_draft(key, MenuInputDraft::job_id(action)).await?;
                tracing::warn!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    job_id,
                    job_action = action.log_name(),
                    error = %err,
                    "menu input job command failed, waiting for retry"
                );
                let detail = build_input_retry_detail(
                    &format!("执行失败：{}。", err),
                    action.input_detail(),
                );
                send::send_card_message_with_force_reply_returning(
                    build_step_prompt_text("1/1", "任务操作未生效", &detail),
                    request_chat_id,
                    "重新输入 job_id（回复“取消”可退出）",
                    client_id,
                )
                .await?;
            }
            Ok(true)
        }
        MenuInputStep::AdminInput {
            action,
            context_text,
            context_i64,
        } => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                admin_action = action.log_name(),
                "menu input admin action received"
            );
            match action {
                AdminInputAction::TargetsAliasName => {
                    let Some(alias) = parse_single_alias_input(input) else {
                        put_draft(
                            key,
                            MenuInputDraft::admin_input(action, context_text.clone(), context_i64),
                        )
                        .await?;
                        let meta =
                            admin_input_prompt_meta(action, context_text.as_deref(), context_i64);
                        let detail = build_input_retry_detail("alias 格式不正确。", &meta.detail);
                        send::send_card_message_with_force_reply_returning(
                            build_step_prompt_text("1/2", "输入格式不正确", &detail),
                            request_chat_id,
                            &meta.placeholder,
                            client_id,
                        )
                        .await?;
                        return Ok(true);
                    };

                    let next_action = AdminInputAction::TargetsSetAlias;
                    // 用当前 alias 回复消息 ID 标记这次 picker，旧 alias 的共享结果不能复用。
                    let picker_token = Some(request_message_id);
                    put_draft(
                        key,
                        MenuInputDraft::admin_input(next_action, Some(alias.clone()), picker_token),
                    )
                    .await?;
                    let meta = admin_input_prompt_meta(next_action, Some(&alias), picker_token);
                    tracing::debug!(
                        request_chat_id,
                        sender_user_id,
                        request_message_id,
                        selected_alias = %alias,
                        "menu input target alias first step accepted"
                    );
                    send_admin_input_prompt(
                        next_action,
                        picker_token,
                        "2/2",
                        &meta.title,
                        &meta.detail,
                        &meta.placeholder,
                        request_chat_id,
                        sender_user_id,
                        client_id,
                    )
                    .await?;
                    return Ok(true);
                }
                AdminInputAction::TargetsAliasSearch => {
                    let Some(query) = parse_single_alias_input(input) else {
                        put_draft(
                            key,
                            MenuInputDraft::admin_input(action, context_text.clone(), context_i64),
                        )
                        .await?;
                        let meta =
                            admin_input_prompt_meta(action, context_text.as_deref(), context_i64);
                        let detail =
                            build_input_retry_detail("搜索关键字格式不正确。", &meta.detail);
                        send::send_card_message_with_force_reply_returning(
                            build_step_prompt_text("1/1", "输入格式不正确", &detail),
                            request_chat_id,
                            &meta.placeholder,
                            client_id,
                        )
                        .await?;
                        return Ok(true);
                    };

                    tracing::debug!(
                        request_chat_id,
                        sender_user_id,
                        request_message_id,
                        query = %query,
                        "menu input target alias search accepted"
                    );
                    super::super::targets::send_alias_search_result_page_on(
                        app,
                        &query,
                        1,
                        request_chat_id,
                        client_id,
                    )
                    .await?;
                    return Ok(true);
                }
                _ => {}
            }

            let Some(command_owned) = parse_admin_input_payload(
                action,
                input,
                None,
                context_text.as_deref(),
                context_i64,
            ) else {
                put_draft(
                    key,
                    MenuInputDraft::admin_input(action, context_text.clone(), context_i64),
                )
                .await?;
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    admin_action = action.log_name(),
                    "menu input admin action rejected"
                );
                let meta = admin_input_prompt_meta(action, context_text.as_deref(), context_i64);
                let detail = build_input_retry_detail("输入格式不正确。", &meta.detail);
                send_admin_input_prompt(
                    action,
                    context_i64,
                    admin_input_step_label(action, context_text.as_deref(), context_i64),
                    "输入格式不正确",
                    &detail,
                    &meta.placeholder,
                    request_chat_id,
                    sender_user_id,
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
            let result = match admin_command_kind(action) {
                Some(AdminCommandKind::Targets) => {
                    run_existing_targets_command(app, command_owned, request_chat_id, client_id)
                        .await
                }
                Some(AdminCommandKind::Config) => {
                    run_existing_config_command(app, command_owned, request_chat_id, client_id)
                        .await
                }
                None => Err(anyhow::anyhow!(
                    "unsupported admin input action: {}",
                    action.log_name()
                )),
            };
            if let Err(err) = result {
                put_draft(
                    key,
                    MenuInputDraft::admin_input(action, context_text.clone(), context_i64),
                )
                .await?;
                tracing::warn!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    admin_action = action.log_name(),
                    error = %err,
                    "menu input admin command failed, waiting for retry"
                );
                let meta = admin_input_prompt_meta(action, context_text.as_deref(), context_i64);
                let detail =
                    build_input_retry_detail(&format!("执行失败：{}。", err), &meta.detail);
                send_admin_input_prompt(
                    action,
                    context_i64,
                    admin_input_step_label(action, context_text.as_deref(), context_i64),
                    "输入未生效",
                    &detail,
                    &meta.placeholder,
                    request_chat_id,
                    sender_user_id,
                    client_id,
                )
                .await?;
            }
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

    // 原生选聊确认页优先显示聊天标题，标题缺失时使用 username。
    #[test]
    fn test_shared_chat_display_name_prefers_title_then_username() {
        let mut chat = tdlib_rs::types::SharedChat {
            chat_id: -100,
            title: "归档群".to_owned(),
            username: "archive_channel".to_owned(),
            photo: None,
        };

        assert_eq!(shared_chat_display_name(&chat).as_deref(), Some("归档群"));
        chat.title.clear();
        assert_eq!(
            shared_chat_display_name(&chat).as_deref(),
            Some("@archive_channel")
        );
    }

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
        assert!(!text.contains("/menu"));
    }

    #[test]
    fn test_target_chat_request_button_ids_are_scoped() {
        assert!(is_target_chat_request_button(
            TARGET_GROUP_CHAT_REQUEST_BUTTON_ID
        ));
        assert!(is_target_chat_request_button(
            TARGET_CHANNEL_CHAT_REQUEST_BUTTON_ID
        ));
        assert!(!is_target_chat_request_button(7999));
    }

    // 只有原生目标选择键盘的精确按钮文案才允许切换到手动输入，
    // 防止普通目标文本被误判为流程控制动作。
    #[test]
    fn test_target_chat_manual_input_text_is_scoped() {
        assert!(is_target_chat_manual_input(TARGET_CHAT_MANUAL_INPUT_TEXT));
        assert!(!is_target_chat_manual_input("手动输入"));
        assert!(!is_target_chat_manual_input("-100123456"));
    }

    // 同一草稿重复打开选聊时只保留最新消息，完成选择后可一次性取出旧消息 ID。
    #[test]
    fn test_target_picker_message_tracking_replaces_and_consumes() {
        let key = (i64::MIN + 7001, i64::MIN + 7002);
        assert_eq!(take_target_picker_message(key), None);
        assert_eq!(remember_target_picker_message(key, 101), None);
        assert_eq!(remember_target_picker_message(key, 202), Some(101));
        assert_eq!(take_target_picker_message(key), Some(202));
        assert_eq!(take_target_picker_message(key), None);
    }

    #[test]
    fn test_shared_chat_input_decision_requires_active_picker() {
        let context = state::TargetContext {
            kind: MenuInputKind::Transfer,
            source_link: "https://t.me/c/1/2".to_owned(),
        };

        assert_eq!(
            shared_chat_input_decision(
                7999,
                state::TargetContextAdvanceResult::Active(context.clone())
            ),
            SharedChatInputDecision::Ignore
        );
        assert_eq!(
            shared_chat_input_decision(
                TARGET_GROUP_CHAT_REQUEST_BUTTON_ID,
                state::TargetContextAdvanceResult::Active(context.clone())
            ),
            SharedChatInputDecision::Confirm(context)
        );
        assert_eq!(
            shared_chat_input_decision(
                TARGET_CHANNEL_CHAT_REQUEST_BUTTON_ID,
                state::TargetContextAdvanceResult::WrongStep
            ),
            SharedChatInputDecision::Stale
        );
    }

    // continue 输入的流程草稿若意外落到本层，也应继续走流程提示，而不是 panic。
    #[test]
    fn test_continue_input_flow_step_is_still_recoverable() {
        let draft = MenuInputDraft::source_link(MenuInputKind::Transfer);

        assert!(matches!(draft.step, MenuInputStep::SourceLink { .. }));
    }

    // 别名第一步只接受单个 token，和 `/targets set-alias <alias> <target>` 保持一致。
    #[test]
    fn test_parse_single_alias_input() {
        assert_eq!(
            parse_single_alias_input("archive"),
            Some("archive".to_owned())
        );
        assert_eq!(
            parse_single_alias_input(" archive "),
            Some("archive".to_owned())
        );
        assert_eq!(parse_single_alias_input("my archive"), None);
        assert_eq!(parse_single_alias_input(""), None);
    }

    // 输入失败提示必须同时包含失败原因和下一步格式，用户才能继续修正。
    #[test]
    fn test_build_input_retry_detail_contains_reason_and_next_step() {
        let detail = build_input_retry_detail("输入格式不正确。", "请回复纯数字 job_id。");

        assert!(detail.contains("输入格式不正确"));
        assert!(detail.contains("请回复纯数字 job_id"));
    }

    #[test]
    fn test_admin_input_step_label_for_targets_two_step_flow() {
        assert_eq!(
            admin_input_step_label(AdminInputAction::TargetsAliasName, None, None),
            "1/2"
        );
        assert_eq!(
            admin_input_step_label(AdminInputAction::TargetsSetDefault, None, None),
            "1/1"
        );
    }

    #[test]
    fn test_admin_target_actions_use_native_chat_picker() {
        assert!(AdminInputAction::TargetsSetDefault.uses_chat_picker());
        assert!(AdminInputAction::TargetsSetAlias.uses_chat_picker());
        assert!(!AdminInputAction::TargetsAliasName.uses_chat_picker());
        assert!(!AdminInputAction::TargetsAliasSearch.uses_chat_picker());
        assert!(!AdminInputAction::ConfigSetJobConcurrency.uses_chat_picker());
    }

    #[test]
    fn test_admin_chat_picker_button_ids_are_scoped_to_flow() {
        let default_first =
            state::admin_chat_request_button_ids(AdminInputAction::TargetsSetDefault, Some(100))
                .expect("default target supports chat picker");
        let default_second =
            state::admin_chat_request_button_ids(AdminInputAction::TargetsSetDefault, Some(101))
                .expect("default target supports chat picker");
        let alias_first =
            state::admin_chat_request_button_ids(AdminInputAction::TargetsSetAlias, Some(100))
                .expect("target alias supports chat picker");

        assert_ne!(default_first, default_second);
        assert_ne!(default_first, alias_first);
        assert!(!is_target_chat_request_button(default_first.0));
        assert!(!is_target_chat_request_button(default_first.1));
        assert!(state::is_admin_chat_request_button(alias_first.0));
        assert!(state::is_admin_chat_request_button(alias_first.1));
    }

    #[test]
    fn test_shared_admin_chat_command_reuses_targets_commands() {
        let default_context = state::AdminInputContext {
            action: AdminInputAction::TargetsSetDefault,
            context_text: None,
            context_i64: None,
        };
        assert_eq!(
            shared_admin_chat_command(&default_context, -100123),
            Some(vec![
                "/targets".to_owned(),
                "set-default".to_owned(),
                "-100123".to_owned(),
            ])
        );

        let alias_context = state::AdminInputContext {
            action: AdminInputAction::TargetsSetAlias,
            context_text: Some("archive".to_owned()),
            context_i64: None,
        };
        assert_eq!(
            shared_admin_chat_command(&alias_context, -100456),
            Some(vec![
                "/targets".to_owned(),
                "set-alias".to_owned(),
                "archive".to_owned(),
                "-100456".to_owned(),
            ])
        );
    }
}
