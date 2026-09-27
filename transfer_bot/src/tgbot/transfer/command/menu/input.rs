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

/// Telegram 原生目标群组选择按钮 ID。
pub(super) const TARGET_GROUP_CHAT_REQUEST_BUTTON_ID: i32 = 7001;
/// Telegram 原生目标频道选择按钮 ID。
pub(super) const TARGET_CHANNEL_CHAT_REQUEST_BUTTON_ID: i32 = 7002;
/// 原生聊天 reply keyboard 的手动输入兜底按钮文案。
pub(super) const TARGET_CHAT_MANUAL_INPUT_TEXT: &str = "手动输入目标";

/// 判断按钮 ID 是否属于转存目标的原生选聊请求按钮。
///
/// # 参数
/// - `button_id`: 按钮 ID
/// - `bool`: 若匹配群组或频道按钮 ID 返回 true
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
///
/// # 参数
/// - `input`: 输入文本内容
///
/// # 返回值
/// - `bool`: 若完全匹配手动输入按钮固定文本返回 true
fn is_target_chat_manual_input(input: &str) -> bool {
    input == TARGET_CHAT_MANUAL_INPUT_TEXT
}

/// 原生选聊消息输入的决策结果枚举。
#[derive(Debug, Clone, PartialEq, Eq)]
enum SharedChatInputDecision {
    /// 忽略不相关的按钮事件
    Ignore,
    /// 推进至目标确认步骤
    Confirm(TargetContext),
    /// 草稿已过期
    Expired,
    /// 未找到匹配草稿
    Missing,
    /// 处于错误或已过时的步骤
    Stale,
}

/// 评估原生选聊返回后的处理决策。
///
/// # 参数
/// - `button_id`: 按钮 ID
/// - `result`: 状态层目标上下文推进结果
///
/// # 返回值
/// - `SharedChatInputDecision`: 最终决策
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
///
/// # 参数
/// - `request_chat_id`: 会话 ID
/// - `sender_user_id`: 用户 ID
/// - `message_id`: 消息 ID
/// - `client_id`: TDLib 客户端 ID
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
///
/// # 参数
/// - `request_chat_id`: 会话 ID
/// - `sender_user_id`: 用户 ID
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `bool`: 若清理了至少一个残留 picker 消息返回 true
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

/// 发送转存目标原生选择器提示卡片及附带键盘。
///
/// # 参数
/// - `request_chat_id`: 会话 ID
/// - `sender_user_id`: 用户 ID
/// - `source_link`: 转存源链接文本
/// - `stale_message_id`: 可选的旧卡片消息 ID（将被删除）
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 发送成功返回 Ok(())
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
///
/// # 参数
/// - `chat`: Telegram 原生分享的聊天对象引用
///
/// # 返回值
/// - `Option<String>`: 解析得到的展示名称
fn shared_chat_display_name(chat: &tdlib_rs::types::SharedChat) -> Option<String> {
    let title = chat.title.trim();
    if !title.is_empty() {
        return Some(title.to_owned());
    }
    let username = chat.username.trim().trim_start_matches('@');
    (!username.is_empty()).then(|| format!("@{username}"))
}

/// 处理 Telegram 原生目标聊天选择器返回的共享聊天消息。
///
/// 支持转存目标选择以及管理配置中设置别名/默认目标的选择结果。
///
/// # 参数
/// - `shared`: Telegram 原生分享的聊天结构体
/// - `request_chat_id`: 会话 ID
/// - `sender_user_id`: 用户 ID
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<bool>`: 若匹配并消费该事件返回 Ok(true)
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
                        build_input_retry_detail(&format!("执行失败：{err}。"), &meta.detail);
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
    /// 当前无草稿
    None,
    /// 草稿已过期
    Expired,
    /// 存在有效的活跃草稿
    Active(MenuInputDraft),
}

/// 把状态层的草稿读取结果映射为继续输入流程决策。
///
/// # 参数
/// - `result`: 草稿取出结果
///
/// # 返回值
/// - `ContinueInputDecision`: 继续输入决策
fn continue_input_decision(result: DraftTakeResult) -> ContinueInputDecision {
    match result {
        DraftTakeResult::None => ContinueInputDecision::None,
        DraftTakeResult::Expired => ContinueInputDecision::Expired,
        DraftTakeResult::Active(draft) => ContinueInputDecision::Active(draft),
    }
}

/// 构造“继续输入时已过期”的恢复文案。
///
/// # 参数
/// - `app`: 全局应用上下文引用
///
/// # 返回值
/// - `String`: 恢复提示卡片文本
fn build_continue_input_expired_text_on(app: &crate::app_context::AppContext) -> String {
    build_menu_recovery_text("输入已过期", "expired", &expired_input_detail_on(app))
}

/// 构造输入失败后的重试说明。
///
/// 失败原因和下一步格式分开展示，避免用户只看到“失败”但不知道应该继续回复什么。
///
/// # 参数
/// - `reason`: 失败原因文本
/// - `next_detail`: 下一步指引说明
///
/// # 返回值
/// - `String`: 拼接后的重试说明文本
fn build_input_retry_detail(reason: &str, next_detail: &str) -> String {
    format!("{reason}\n\n{next_detail}")
}

/// 解析单个别名输入。
///
/// 这里明确不接受空白分隔的多词别名，保持和 `/targets set-alias <alias> <target>` 的命令格式一致。
///
/// # 参数
/// - `input`: 输入字符串
///
/// # 返回值
/// - `Option<String>`: 解析后的有效单词别名
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
///
/// # 参数
/// - `context`: 管理输入上下文引用
/// - `target_chat_id`: 选中的目标 chat_id
///
/// # 返回值
/// - `Option<Vec<String>>`: 构造好的命令参数集合
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
///
/// # 参数
/// - `action`: 管理输入动作
/// - `context_text`: 附带的文本上下文（如别名）
/// - `_context_i64`: 附带的数字上下文
///
/// # 返回值
/// - `&'static str`: 当前步骤说明标签（如 "1/2"）
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
///
/// # 参数
/// - `action`: 管理输入动作
/// - `picker_token`: 原生选择器关联 token
/// - `step_label`: 步骤标签（如 "1/2"）
/// - `title`: 卡片标题
/// - `detail`: 详细说明
/// - `placeholder`: 输入占位符
/// - `request_chat_id`: 会话 ID
/// - `sender_user_id`: 用户 ID
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 发送成功返回 Ok(())
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
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `config`: Bot 配置引用
/// - `chat_id`: 会话 ID
/// - `sender_user_id`: 用户 ID
/// - `kind`: 菜单输入类型
/// - `source_link`: 转存源定位
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功启动返回 Ok(())
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
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `config`: Bot 配置引用
/// - `chat_id`: 会话 ID
/// - `sender_user_id`: 用户 ID
/// - `source_link`: 来源链接
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功启动返回 Ok(())
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
///
/// # 参数
/// - `chat_id`: 会话 ID
/// - `user_id`: 用户 ID
///
/// # 返回值
/// - `anyhow::Result<Option<MenuDraftSummary>>`: 成功读取时返回草稿摘要
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
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `chat_id`: 会话 ID
/// - `user_id`: 用户 ID
/// - `config`: Bot 配置引用
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<bool>`: 若成功继续返回 Ok(true)
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

/// 处理用户通过普通消息或 ForceReply 提交的菜单交互输入。
///
/// 当用户处于菜单的某一等待输入步骤（例如输入来源链接、任务 ID、目标会话或管理参数）并发送文本时，
/// 路由分发器会调用本函数尝试匹配并消费该输入。
///
/// # 参数说明
/// - `app`: 全局应用程序上下文引用，用于访问配置、数据库连接池及服务组件
/// - `text`: 用户输入的文本内容字符串
/// - `config`: 当前生效的机器人配置快照弧指针
/// - `key`: 会话唯一标识键元组 `(request_chat_id, sender_user_id)`
/// - `request_message_id`: 用户本次发送输入内容的消息 ID
/// - `actor`: 请求操作者上下文（包含角色权限校验信息）
/// - `client_id`: TDLib 客户端实例标识符
///
/// # 返回值
/// - `Ok(true)`: 表示本条输入已成功被菜单输入会话草稿消费，无需其他处理器进一步处理
/// - `Ok(false)`: 表示当前会话不存在有效草稿，未消费本条输入，可交由其他文本处理分支处理
/// - `Err(...)`: 处理过程中发生不可恢复的底层错误
pub(super) async fn handle_menu_input_on(
    app: &crate::app_context::AppContext,
    text: &str,
    config: std::sync::Arc<BotConfig>,
    key: (i64, i64),
    request_message_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<bool> {
    // 解构出聊天会话 ID 与发送者用户 ID
    let (request_chat_id, sender_user_id) = key;
    // 去除用户输入首尾的空白字符
    let input = text.trim();
    // 若输入内容纯为空白，则直接忽略并不视为菜单输入消费
    if input.is_empty() {
        return Ok(false);
    }

    // 从存储层取出当前用户的输入草稿状态并原子性移除
    let draft = match take_current_draft(key).await? {
        // 存在活跃且未过期的输入草稿
        DraftTakeResult::Active(draft) => draft,
        // 草稿已超时过期
        DraftTakeResult::Expired => {
            // 清理可能遗留的原生选聊交互消息
            clear_native_picker_messages(request_chat_id, sender_user_id, client_id).await;
            // 记录草稿过期的调试追踪日志
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                "menu input draft expired"
            );
            // 向用户发送输入会话已超时的卡片提示，并附带返回主菜单按钮
            send::ReplyPanel::card(build_continue_input_expired_text_on(app))
                .row(vec![send::build_callback_button(
                    "返回菜单",
                    &super::build_menu_home_callback_data(),
                    tdlib_rs::enums::ButtonStyle::Primary,
                )])
                .send(request_chat_id, client_id)
                .await?;
            // 消费掉本次输入事件
            return Ok(true);
        }
        // 当前用户没有任何关联的菜单输入草稿
        DraftTakeResult::None => {
            // 记录未匹配到草稿的跟踪日志并放行给后续分发器
            tracing::trace!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                "menu input draft not found"
            );
            return Ok(false);
        }
    };

    // 检查用户输入的文本是否表示主动取消（如“取消”、“/cancel”等）
    if is_cancel_text(input) {
        // 判断当前步骤是否使用了原生选聊键盘或原生会话请求按钮
        let had_picker = matches!(&draft.step, MenuInputStep::ChatPicker { .. })
            || matches!(
                &draft.step,
                MenuInputStep::AdminInput {
                    action,
                    context_i64,
                    ..
                } if admin_chat_request_button_ids(*action, *context_i64).is_some()
            );
        // 清除任何可能激活的原生选聊按钮或等待提示
        let picker_message_cleared =
            clear_native_picker_messages(request_chat_id, sender_user_id, client_id).await;
        // 记录用户通过文本取消输入的调试日志
        tracing::debug!(
            request_chat_id,
            sender_user_id,
            request_message_id,
            "menu input cancelled by text"
        );
        // 发送操作已取消提示消息
        send_cancelled_notice(
            request_chat_id,
            client_id,
            had_picker && !picker_message_cleared,
        )
        .await?;
        return Ok(true);
    }

    // 检查用户是否点击或输入了“手动输入”文本指令
    if is_target_chat_manual_input(input) {
        match &draft.step {
            // 普通转发/下载流程中的原生选聊阶段转为手动文本输入
            MenuInputStep::ChatPicker { kind, source_link } => {
                let kind = *kind;
                let source_link = source_link.clone();
                // 若存在旧的原生选聊提示消息，先予以删除清理
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
                // 发送附带 ForceReply 的手动目标输入提示卡片
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
            // 管理员配置流程中原生选聊转为手动文本输入
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
                // 重新发送 ForceReply 手动输入提示
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

    // 首先委托给通用的多步骤流程输入处理器（涵盖来源链接、目标会话选择、确认等流转）
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

    // 根据输入草稿所处的特定状态步骤进行分发处理
    match draft.step {
        // 等待输入任务 ID 步骤（如针对特定任务执行暂停、恢复、取消、重试等操作）
        MenuInputStep::JobId { action } => {
            tracing::debug!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                job_action = action.log_name(),
                "menu input job id received"
            );
            // 尝试将用户输入的文本解析为合法有效的正整数任务 ID
            let Some(job_id) = parse_job_id_input(input) else {
                // 解析失败：将草稿重新写回存储，保持等待用户输入任务 ID 状态
                put_draft(key, MenuInputDraft::job_id(action)).await?;
                tracing::debug!(
                    request_chat_id,
                    sender_user_id,
                    request_message_id,
                    job_action = action.log_name(),
                    "menu input job id rejected"
                );
                // 发送 ForceReply 卡片提示用户格式不正确并引导重新输入
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

            // 记录正在分发执行任务命令的日志
            tracing::info!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                job_id,
                job_action = action.log_name(),
                "menu input dispatching job command"
            );
            // 调用底层的既有任务操作分发处理逻辑
            if let Err(err) = run_existing_job_command(app, action, job_id, actor, client_id).await
            {
                // 命令执行失败（如任务不存在或状态不支持该操作）：保留草稿允许用户重试
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
                // 拼接失败原因与操作输入格式说明
                let detail =
                    build_input_retry_detail(&format!("执行失败：{err}。"), action.input_detail());
                // 发送重试提示卡片
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
        // 管理员参数配置输入步骤
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
            // 针对具有多步流转特性的管理员动作进行前置处理
            match action {
                // 别名绑定的第一步：输入目标别名名称
                AdminInputAction::TargetsAliasName => {
                    // 解析单个别名标识符
                    let Some(alias) = parse_single_alias_input(input) else {
                        // 校验失败，保留草稿并重新发送第一步输入提示
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

                    // 第一步别名校验通过，推进到第二步：设置目标会话 ID
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
                    // 发送第二步输入提示（支持原生选聊或手动输入）
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
                // 搜索别名动作
                AdminInputAction::TargetsAliasSearch => {
                    let Some(query) = parse_single_alias_input(input) else {
                        // 搜索词不合法，提示重新输入
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
                    // 直接渲染并展示目标别名搜索结果的第一页
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

            // 将用户输入解析为对应的管理员子命令参数列表
            let Some(command_owned) = parse_admin_input_payload(
                action,
                input,
                None,
                context_text.as_deref(),
                context_i64,
            ) else {
                // 解析失败：恢复草稿状态并引导重新输入
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

            // 记录正在分发管理员配置指令日志
            tracing::info!(
                request_chat_id,
                sender_user_id,
                request_message_id,
                admin_action = action.log_name(),
                "menu input dispatching admin config command"
            );
            // 根据命令大类分别转发给 targets 或 config 模块的分发函数执行
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
                // 命令执行报错：保留草稿并展示错误信息供用户重试
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
                let detail = build_input_retry_detail(&format!("执行失败：{err}。"), &meta.detail);
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
        // 属于通用多步骤流转的草稿，正常情况下应在前方的 handle_flow_input 中被消费。
        // 若意外落入此兜底分支，记录警告并引导用户返回主菜单，同时保留草稿防止数据丢失。
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
            // 重新写回草稿
            put_draft(key, draft).await?;
            // 发送过期/重置卡片，并附带返回主菜单按钮
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

    /// 测试原生选聊确认展示名称策略：
    /// 原生选聊确认页优先显示聊天群组/频道标题（title），标题缺失或为空时降级使用用户名（@username）。
    #[test]
    fn test_shared_chat_display_name_prefers_title_then_username() {
        let mut chat = tdlib_rs::types::SharedChat {
            chat_id: -100,
            title: "归档群".to_owned(),
            username: "archive_channel".to_owned(),
            photo: None,
        };

        // 优先展示自定义标题
        assert_eq!(shared_chat_display_name(&chat).as_deref(), Some("归档群"));
        // 标题清空后应降级展示带有 @ 前缀的 username
        chat.title.clear();
        assert_eq!(
            shared_chat_display_name(&chat).as_deref(),
            Some("@archive_channel")
        );
    }

    /// 测试继续输入决策映射：
    /// “继续输入”按钮应先把底层状态层的获取结果规整为纯决策枚举，避免入口散落多个 match 分支。
    #[test]
    fn test_continue_input_decision_maps_draft_results() {
        // 无草稿 -> None
        assert!(matches!(
            continue_input_decision(DraftTakeResult::None),
            ContinueInputDecision::None
        ));
        // 超时草稿 -> Expired
        assert!(matches!(
            continue_input_decision(DraftTakeResult::Expired),
            ContinueInputDecision::Expired
        ));
        // 活跃草稿 -> Active 并保留具体步骤数据
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

    /// 测试继续输入的过期提示文本格式：
    /// 继续输入的过期提示应是明确的恢复态（如包含 ‹expired› 状态），而不是普通等待输入态。
    #[test]
    fn test_build_continue_input_expired_text_uses_recovery_status() {
        let text = build_continue_input_expired_text();

        assert!(text.contains("输入已过期"));
        assert!(text.contains("状态：‹expired›"));
        assert!(!text.contains("/menu"));
    }

    /// 测试目标会话选择请求按钮 ID 是否限定在固定范围内（群组与频道两个固定 ID）。
    #[test]
    fn test_target_chat_request_button_ids_are_scoped() {
        // 群组选择按钮 ID 应命中
        assert!(is_target_chat_request_button(
            TARGET_GROUP_CHAT_REQUEST_BUTTON_ID
        ));
        // 频道选择按钮 ID 应命中
        assert!(is_target_chat_request_button(
            TARGET_CHANNEL_CHAT_REQUEST_BUTTON_ID
        ));
        // 无关 ID 不应被误判
        assert!(!is_target_chat_request_button(7999));
    }

    /// 测试手动输入目标会话的触发文案判断：
    /// 只有原生目标选择键盘的精确按钮文案才允许切换到手动输入，
    /// 防止普通目标文本被误判为流程控制动作。
    #[test]
    fn test_target_chat_manual_input_text_is_scoped() {
        // 精确匹配“✍️ 手动输入”
        assert!(is_target_chat_manual_input(TARGET_CHAT_MANUAL_INPUT_TEXT));
        // 非完全匹配项不应触发
        assert!(!is_target_chat_manual_input("手动输入"));
        assert!(!is_target_chat_manual_input("-100123456"));
    }

    /// 测试选聊提示消息 ID 的追踪与替换清理：
    /// 同一草稿重复打开选聊时只保留最新消息，完成选择后可一次性取出旧消息 ID 并删除。
    #[test]
    fn test_target_picker_message_tracking_replaces_and_consumes() {
        let key = (i64::MIN + 7001, i64::MIN + 7002);
        // 初始为空
        assert_eq!(take_target_picker_message(key), None);
        // 第一次记录，此前无旧消息
        assert_eq!(remember_target_picker_message(key, 101), None);
        // 第二次记录，返回被替换的旧消息 ID 101
        assert_eq!(remember_target_picker_message(key, 202), Some(101));
        // 消费取出当前最新的消息 ID 202
        assert_eq!(take_target_picker_message(key), Some(202));
        // 再次获取应已被清空
        assert_eq!(take_target_picker_message(key), None);
    }

    /// 测试共享会话输入决策：
    /// 共享会话输入需要严格匹配活跃的选聊器状态及对应的按钮 ID。
    #[test]
    fn test_shared_chat_input_decision_requires_active_picker() {
        let context = state::TargetContext {
            kind: MenuInputKind::Transfer,
            source_link: "https://t.me/c/1/2".to_owned(),
        };

        // 非法按钮 ID 应直接被忽略
        assert_eq!(
            shared_chat_input_decision(
                7999,
                state::TargetContextAdvanceResult::Active(context.clone())
            ),
            SharedChatInputDecision::Ignore
        );
        // 合法群组请求按钮且处于活跃上下文，应推进至 Confirm 状态
        assert_eq!(
            shared_chat_input_decision(
                TARGET_GROUP_CHAT_REQUEST_BUTTON_ID,
                state::TargetContextAdvanceResult::Active(context.clone())
            ),
            SharedChatInputDecision::Confirm(context)
        );
        // 步骤不匹配时判定为陈旧消息 (Stale)
        assert_eq!(
            shared_chat_input_decision(
                TARGET_CHANNEL_CHAT_REQUEST_BUTTON_ID,
                state::TargetContextAdvanceResult::WrongStep
            ),
            SharedChatInputDecision::Stale
        );
    }

    /// 测试“继续输入”落入流式步骤时的可恢复性：
    /// continue 输入的流程草稿若意外落到本层，也应正确识别为 SourceLink 步骤并引导流式恢复。
    #[test]
    fn test_continue_input_flow_step_is_still_recoverable() {
        let draft = MenuInputDraft::source_link(MenuInputKind::Transfer);

        assert!(matches!(draft.step, MenuInputStep::SourceLink { .. }));
    }

    /// 测试别名输入的单 token 解析验证：
    /// 别名第一步只接受单个 token，首尾允许空格，中间不允许空格，与 `/targets set-alias <alias> <target>` 保持一致。
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

    /// 测试重试错误提示文案的组装：
    /// 输入失败提示必须同时包含失败原因和下一步格式，用户才能清晰了解如何继续修正。
    #[test]
    fn test_build_input_retry_detail_contains_reason_and_next_step() {
        let detail = build_input_retry_detail("输入格式不正确。", "请回复纯数字 job_id。");

        assert!(detail.contains("输入格式不正确"));
        assert!(detail.contains("请回复纯数字 job_id"));
    }

    /// 测试管理员两步流程的步骤指示标签：
    /// TargetsAliasName 为第一步 "1/2"，TargetsSetDefault 为单步 "1/1"。
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

    /// 测试管理员各配置动作是否使用原生聊天选择器（原生键盘）：
    /// 设置默认目标与设置别名均需选聊，而别名输入、搜索与并发设置则无需原生选聊。
    #[test]
    fn test_admin_target_actions_use_native_chat_picker() {
        assert!(AdminInputAction::TargetsSetDefault.uses_chat_picker());
        assert!(AdminInputAction::TargetsSetAlias.uses_chat_picker());
        assert!(!AdminInputAction::TargetsAliasName.uses_chat_picker());
        assert!(!AdminInputAction::TargetsAliasSearch.uses_chat_picker());
        assert!(!AdminInputAction::ConfigSetJobConcurrency.uses_chat_picker());
    }

    /// 测试管理员原生选聊请求按钮 ID 的隔离性与唯一性：
    /// 确保不同动作、不同消息 token 派生的按钮 ID 互不冲突，且不与通用目标选择按钮 ID 混淆。
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

        // 不同的 token 生成不同的 button_id
        assert_ne!(default_first, default_second);
        // 不同的 action 生成不同的 button_id
        assert_ne!(default_first, alias_first);
        // 不应与目标普通选聊 button_id 冲突
        assert!(!is_target_chat_request_button(default_first.0));
        assert!(!is_target_chat_request_button(default_first.1));
        // 应符合管理员聊天请求按钮的判定
        assert!(state::is_admin_chat_request_button(alias_first.0));
        assert!(state::is_admin_chat_request_button(alias_first.1));
    }

    /// 测试共享管理员聊天回调转化为等价 targets 文本命令：
    /// 验证选择共享聊天后正确复用既有的 `/targets set-default` 与 `/targets set-alias` 命令参数结构。
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
