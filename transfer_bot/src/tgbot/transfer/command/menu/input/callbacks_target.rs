// `/menu` 目标流程相关的 inline callback。
// 这里集中处理“选目标 -> 确认 -> 返回”的多步向导按钮。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::text::{
    build_menu_recovery_text, build_step_prompt_text, build_step_prompt_with_context,
    build_target_input_prompt_text,
};
use super::flow::{ExistingCommandContext, ExistingCommandOrigin, run_existing_command};
use super::state::{
    ConfirmContextTakeResult, DraftKey, TargetContext, TargetContextAdvanceResult,
    TargetDraftAdvance, advance_target_context_with_cleanup, remember_last_target,
    take_confirm_context,
};
use super::target::{
    TargetPromptContext, edit_confirm_prompt, edit_target_choice_prompt, resolve_default_target_on,
};

/// 目标选择 callback 的公共上下文。
///
/// 多个目标按钮都需要同一组 TDLib 消息坐标；收拢后目标推进逻辑更容易保持一致。
#[derive(Clone)]
struct TargetCallbackContext {
    /// Telegram 回调查询唯一标识 ID
    callback_query_id: i64,
    /// 触发回调所在的 Telegram 会话 ID（群组/私聊/频道）
    chat_id: i64,
    /// 触发回调的内联键盘所在消息 ID
    message_id: i64,
    /// 点击回调按钮的 Telegram 用户 ID
    sender_user_id: i64,
    /// TDLib 客户端实例句柄 ID
    client_id: i32,
}

impl TargetCallbackContext {
    /// 构造 callback 共享上下文实例。
    ///
    /// # 参数
    /// - `callback_query_id`: Telegram 回调查询唯一 ID
    /// - `chat_id`: 所在聊天会话 ID
    /// - `message_id`: 原消息 ID
    /// - `sender_user_id`: 操作者用户 ID
    /// - `client_id`: 客户端实例 ID
    fn new(
        callback_query_id: i64,
        chat_id: i64,
        message_id: i64,
        sender_user_id: i64,
        client_id: i32,
    ) -> Self {
        Self {
            callback_query_id,
            chat_id,
            message_id,
            sender_user_id,
            client_id,
        }
    }

    /// 当前草稿的会话隔离键，按 `(chat_id, sender_user_id)` 组合。
    fn draft_key(&self) -> DraftKey {
        (self.chat_id, self.sender_user_id)
    }

    /// 向 Telegram ACK 当前按钮点击并弹出顶部气泡提示。
    ///
    /// # 参数
    /// - `text`: 提示文案内容
    async fn answer(&self, text: &'static str) -> anyhow::Result<()> {
        send::answer_callback_query(self.callback_query_id, Some(text), self.client_id).await
    }
}

/// 处理“使用默认目标”按钮的回调请求。
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `callback_query_id`: 回调查询 ID
/// - `chat_id`: 当前聊天会话 ID
/// - `message_id`: 消息 ID
/// - `sender_user_id`: 点击按钮的用户 ID
/// - `config`: Bot 全局配置
/// - `client_id`: TDLib 客户端实例 ID
pub(in crate::tgbot::transfer::command::menu) async fn target_default_callback_query(
    app: &crate::app_context::AppContext,
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    config: Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<()> {
    // 构造回调通用上下文
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    // 解析当前会话对应的默认目标聊天 ID（未配置时回退到当前私聊）
    let target_chat_id = resolve_default_target_on(app, &config, chat_id);

    // 将草稿推进到确认步骤并刷新卡片
    select_target_for_callback_on(
        ctx,
        target_chat_id,
        default_target_selected_tip(target_chat_id, chat_id),
    )
    .await
}

/// 默认目标 callback 的提示文案，应如实描述最终解析到的实际位置。
///
/// # 参数
/// - `target_chat_id`: 解析得到的目标聊天 ID
/// - `request_chat_id`: 发起请求的会话 ID
fn default_target_selected_tip(target_chat_id: i64, request_chat_id: i64) -> &'static str {
    // 若解析出的目标就是当前私聊会话本身
    if target_chat_id == request_chat_id {
        "已选择当前私聊"
    } else {
        "已选择默认目标"
    }
}

/// 把当前目标选择草稿推进到确认页，并就地编辑原 inline 卡片。
///
/// # 参数
/// - `ctx`: 回调通用上下文
/// - `target_chat_id`: 所选目标聊天 ID
/// - `selected_tip`: 成功选择后的轻提示文本
async fn select_target_for_callback_on(
    ctx: TargetCallbackContext,
    target_chat_id: i64,
    selected_tip: &'static str,
) -> anyhow::Result<()> {
    // 尝试推进状态机到确认阶段
    let Some(context) = advance_target_context_for_callback(
        ctx.draft_key(),
        TargetDraftAdvance::Confirm { target_chat_id },
        ctx.callback_query_id,
        ctx.chat_id,
        "没有等待选择目标的输入",
        ctx.client_id,
    )
    .await?
    else {
        // 状态无效或已过期，终止后续操作
        return Ok(());
    };
    // 发送回调应答提示
    ctx.answer(selected_tip).await?;
    // 原地更新消息卡片为步骤三（确认执行）
    edit_confirm_prompt(
        context.kind,
        &context.source_link,
        target_chat_id,
        ctx.chat_id,
        ctx.message_id,
        ctx.client_id,
    )
    .await
}

/// 处理“常用目标”（别名）按钮的回调请求。
///
/// # 参数
/// - `callback_query_id`: 回调查询 ID
/// - `chat_id`: 当前会话 ID
/// - `message_id`: 原消息 ID
/// - `sender_user_id`: 操作用户 ID
/// - `target_chat_id`: 别名解析出的目标聊天 ID
/// - `client_id`: 客户端实例 ID
pub(in crate::tgbot::transfer::command::menu) async fn target_alias_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    target_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    select_target_for_callback_on(ctx, target_chat_id, "已选择目标").await
}

/// 处理“手动输入目标”按钮的回调请求。
///
/// # 参数
/// - `callback_query_id`: 回调查询 ID
/// - `chat_id`: 当前会话 ID
/// - `message_id`: 原消息 ID
/// - `sender_user_id`: 操作用户 ID
/// - `client_id`: 客户端实例 ID
pub(in crate::tgbot::transfer::command::menu) async fn target_manual_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    // 推进草稿状态到等待手动输入 TargetChat 阶段
    let Some(context) = advance_target_context_for_callback(
        ctx.draft_key(),
        TargetDraftAdvance::TargetChat,
        callback_query_id,
        chat_id,
        "没有等待选择目标的输入",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };

    // 弹出“请输入目标”提示
    ctx.answer("请输入目标").await?;
    // 原地将原卡片变更为等待手动输入的占位卡片（包含“返回目标选择”和“取消”按钮）
    edit_target_input_waiting_card(
        ctx.chat_id,
        ctx.message_id,
        ctx.client_id,
        "2/3",
        "等待手动输入",
        "请回复目标 chat_id、配置别名，或回复 default。",
        &context.source_link,
    )
    .await;
    // 发送带有 ForceReply 的提示消息，引导用户输入
    send::send_card_message_with_force_reply_returning(
        build_target_input_prompt_text(
            &context.source_link,
            "输入目标",
            "请回复数字 chat_id、配置里的目标别名，或回复 default 使用配置默认目标。",
        ),
        ctx.chat_id,
        "输入目标 chat_id、别名或 default",
        ctx.client_id,
    )
    .await?;
    Ok(())
}

/// 处理“选择聊天”按钮，切换到 Telegram 原生群组/频道选择器。
///
/// # 参数
/// - `callback_query_id`: 回调查询 ID
/// - `chat_id`: 当前会话 ID
/// - `message_id`: 原消息 ID
/// - `sender_user_id`: 操作用户 ID
/// - `client_id`: 客户端实例 ID
pub(in crate::tgbot::transfer::command::menu) async fn target_request_chat_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    // 推进草稿状态到 ChatPicker 原生选择器阶段
    let Some(context) = advance_target_context_for_callback(
        ctx.draft_key(),
        TargetDraftAdvance::ChatPicker,
        callback_query_id,
        chat_id,
        "没有等待选择目标的输入",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };

    // 弹出提示
    ctx.answer("请选择目标聊天").await?;
    // 将原卡片更新为等待原生选择器输入状态
    edit_target_input_waiting_card(
        ctx.chat_id,
        ctx.message_id,
        ctx.client_id,
        "2/3",
        "等待选择聊天",
        "请使用输入框下方的 Telegram 原生按钮选择群组或频道。",
        &context.source_link,
    )
    .await;
    // 发送原生底部键盘请求卡片
    super::send_target_chat_picker_prompt(
        ctx.chat_id,
        ctx.sender_user_id,
        &context.source_link,
        Some(ctx.message_id),
        ctx.client_id,
    )
    .await?;
    Ok(())
}

/// 处理“确认页执行”按钮的回调请求。
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `callback_query_id`: 回调查询 ID
/// - `chat_id`: 当前会话 ID
/// - `message_id`: 原消息 ID
/// - `sender_user_id`: 操作用户 ID
/// - `config`: Bot 全局配置
/// - `actor`: 发起请求的操作者身份
/// - `client_id`: 客户端实例 ID
#[allow(clippy::too_many_arguments)]
pub(in crate::tgbot::transfer::command::menu) async fn target_confirm_callback_query(
    app: &crate::app_context::AppContext,
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    config: Arc<BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let key = (chat_id, sender_user_id);
    // 取出并消费确认草稿上下文，映射为确认决策
    let confirm = match confirm_callback_decision(take_confirm_context(key).await?) {
        ConfirmCallbackDecision::Run(confirm) => confirm,
        ConfirmCallbackDecision::Recover {
            callback_tip,
            title,
            status,
            detail,
        } => {
            // 输入过期或不存在，向用户反馈并发送恢复卡片
            send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;
            send_input_recovery_card(chat_id, client_id, title, status, detail).await?;
            return Ok(());
        }
        ConfirmCallbackDecision::WaitForTarget { callback_tip } => {
            // 当前处于错误阶段，提示先选择目标
            send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;
            return Ok(());
        }
    };

    // 记住此用户在该会话下最新成功使用的目标
    remember_last_target(chat_id, sender_user_id, confirm.target_chat_id);
    // 应答“开始执行”
    send::answer_callback_query(callback_query_id, Some("开始执行"), client_id).await?;
    // 执行底层命令派发
    run_existing_command(
        confirm.kind,
        vec![
            confirm.kind.command_name().to_owned(),
            confirm.source_link,
            confirm.target_chat_id.to_string(),
        ],
        config,
        ExistingCommandContext {
            // 确认按钮要把执行上下文 move 给现有转存/查询命令入口；这里局部取一次全局 Arc
            // 比在确认状态结构里长期持有整份运行态更简单，也能维持状态表纯净。
            app: std::sync::Arc::new(app.clone()),
            request_chat_id: chat_id,
            request_message_id: message_id,
            origin: ExistingCommandOrigin::CallbackMessage(message_id),
            actor,
            client_id,
        },
    )
    .await
}

/// 确认按钮消费结果对应的后续动作枚举。
#[derive(Debug, Clone, PartialEq, Eq)]
enum ConfirmCallbackDecision {
    /// 状态正常，获取到确认上下文并准备执行
    Run(super::state::ConfirmContext),
    /// 状态异常（过期或不存在），需向用户展示恢复卡片
    Recover {
        /// 回调弹窗提示
        callback_tip: &'static str,
        /// 恢复卡片标题
        title: &'static str,
        /// 恢复卡片状态标识
        status: &'static str,
        /// 恢复卡片说明详情
        detail: &'static str,
    },
    /// 阶段错误，需要先等待目标选择
    WaitForTarget {
        /// 回调弹窗提示
        callback_tip: &'static str,
    },
}

/// 把状态层结果映射为 UI 层动作，避免确认按钮入口混杂多段提示文案。
///
/// # 参数
/// - `result`: 尝试取出确认上下文的结果
fn confirm_callback_decision(result: ConfirmContextTakeResult) -> ConfirmCallbackDecision {
    match result {
        ConfirmContextTakeResult::Active(confirm) => ConfirmCallbackDecision::Run(confirm),
        ConfirmContextTakeResult::Expired => ConfirmCallbackDecision::Recover {
            callback_tip: "输入已过期",
            title: "输入已过期",
            status: "expired",
            detail: "上一次确认已超过有效时间，请重新打开菜单发起操作。",
        },
        ConfirmContextTakeResult::None => ConfirmCallbackDecision::Recover {
            callback_tip: "没有待执行的输入",
            title: "没有待执行的输入",
            status: "empty",
            detail: "当前没有可确认的菜单输入，请重新打开菜单发起操作。",
        },
        ConfirmContextTakeResult::WrongStep => ConfirmCallbackDecision::WaitForTarget {
            callback_tip: "请先选择目标",
        },
    }
}

/// 目标推进结果对应的 callback UI 动作枚举。
#[derive(Debug, Clone, PartialEq, Eq)]
enum TargetAdvanceCallbackDecision {
    /// 推进成功，继续目标流程
    Continue(TargetContext),
    /// 状态异常，展示恢复卡片
    Recover {
        /// 回调弹窗提示
        callback_tip: &'static str,
        /// 恢复卡片标题
        title: &'static str,
        /// 恢复卡片状态标识
        status: &'static str,
        /// 恢复卡片说明详情
        detail: &'static str,
    },
    /// 阶段错误，等待源链接输入
    WaitForSource {
        /// 回调弹窗提示
        callback_tip: &'static str,
    },
}

/// 把状态层的目标推进结果映射为统一的 callback 提示和恢复动作。
///
/// # 参数
/// - `result`: 状态层目标推进结果
/// - `missing_tip`: 草稿不存在时的专属提示文案
fn target_advance_callback_decision(
    result: TargetContextAdvanceResult,
    missing_tip: &'static str,
) -> TargetAdvanceCallbackDecision {
    match result {
        TargetContextAdvanceResult::Active(context) => {
            TargetAdvanceCallbackDecision::Continue(context)
        }
        TargetContextAdvanceResult::Expired => TargetAdvanceCallbackDecision::Recover {
            callback_tip: "输入已过期",
            title: "输入已过期",
            status: "expired",
            detail: "上一次菜单输入已超过有效时间，请返回菜单重新开始。",
        },
        TargetContextAdvanceResult::None => TargetAdvanceCallbackDecision::Recover {
            callback_tip: missing_tip,
            title: missing_tip,
            status: "empty",
            detail: "当前按钮对应的输入流程已经不存在，请重新打开菜单。",
        },
        TargetContextAdvanceResult::WrongStep => TargetAdvanceCallbackDecision::WaitForSource {
            callback_tip: "请先发送源链接",
        },
    }
}

/// 处理“返回选择目标”按钮的回调请求。
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `callback_query_id`: 回调查询 ID
/// - `chat_id`: 当前会话 ID
/// - `message_id`: 原消息 ID
/// - `sender_user_id`: 操作用户 ID
/// - `config`: Bot 全局配置
/// - `client_id`: 客户端实例 ID
pub(in crate::tgbot::transfer::command::menu) async fn target_back_callback_query(
    app: &crate::app_context::AppContext,
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    config: Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<()> {
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    // 回退状态机到 TargetChoice 步骤
    let Some(context) = advance_target_context_for_callback(
        ctx.draft_key(),
        TargetDraftAdvance::TargetChoice,
        callback_query_id,
        chat_id,
        "没有可返回的目标选择",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };
    ctx.answer("已返回目标选择").await?;
    // 重新编辑原消息为目标选择提示卡片
    edit_target_choice_prompt(
        &config,
        TargetPromptContext {
            app,
            request_chat_id: ctx.chat_id,
            sender_user_id: ctx.sender_user_id,
            client_id: ctx.client_id,
        },
        ctx.message_id,
        context.kind,
        &context.source_link,
    )
    .await?;
    Ok(())
}

/// 处理确认页“修改来源”按钮的回调请求。
///
/// # 参数
/// - `callback_query_id`: 回调查询 ID
/// - `chat_id`: 当前会话 ID
/// - `message_id`: 原消息 ID
/// - `sender_user_id`: 操作用户 ID
/// - `client_id`: 客户端实例 ID
pub(in crate::tgbot::transfer::command::menu) async fn target_source_back_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    // 回退状态机到 SourceLink 步骤
    let Some(context) = advance_target_context_for_callback(
        ctx.draft_key(),
        TargetDraftAdvance::SourceLink,
        callback_query_id,
        chat_id,
        "没有可修改的来源输入",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };

    ctx.answer("请重新输入来源").await?;
    edit_input_waiting_card(
        ctx.chat_id,
        ctx.message_id,
        ctx.client_id,
        context.kind.source_step_label(),
        "等待源链接",
        "请回复新的源链接，回复“取消”可退出。",
    )
    .await;
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text(
            context.kind.source_step_label(),
            context.kind.source_title(),
            context.kind.source_detail(),
        ),
        ctx.chat_id,
        "输入源链接（回复“取消”可退出）",
        ctx.client_id,
    )
    .await?;
    Ok(())
}

/// 从 callback 按钮操作中原子推进目标上下文状态。
///
/// # 参数
/// - `key`: 草稿会话隔离键 `(chat_id, sender_user_id)`
/// - `advance`: 目标草稿推进类型
/// - `callback_query_id`: 当前回调查询 ID
/// - `chat_id`: 所在聊天会话 ID
/// - `missing_tip`: 草稿丢失或为空时的提示文案
/// - `client_id`: 客户端实例 ID
///
/// # 返回
/// 成功推进时返回 `Some(TargetContext)`，若发生过期或需要恢复提示则返回 `None`
pub(super) async fn advance_target_context_for_callback(
    key: DraftKey,
    advance: TargetDraftAdvance,
    callback_query_id: i64,
    chat_id: i64,
    missing_tip: &'static str,
    client_id: i32,
) -> anyhow::Result<Option<TargetContext>> {
    // 调用状态层原子推进并清理中间关联状态
    let outcome = advance_target_context_with_cleanup(key, advance).await?;
    match target_advance_callback_decision(outcome.result, missing_tip) {
        TargetAdvanceCallbackDecision::Continue(context) => {
            // 若状态机要求清理原生选择器 Reply Keyboard
            if outcome.remove_reply_keyboard {
                let cleared = super::clear_native_picker_messages(chat_id, key.1, client_id).await;
                if !cleared {
                    // 进程重启后 tracker 可能丢失；此时发送一条最小兜底消息移除 chat 级键盘。
                    send::send_card_message_with_remove_keyboard(
                        build_menu_recovery_text(
                            "聊天选择已关闭",
                            "continued",
                            "已离开原生聊天选择器，继续当前目标流程。",
                        ),
                        chat_id,
                        client_id,
                    )
                    .await?;
                }
            }
            Ok(Some(context))
        }
        TargetAdvanceCallbackDecision::Recover {
            callback_tip,
            title,
            status,
            detail,
        } => {
            // 应答气泡提示，并发送独立的恢复指引卡片
            send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;
            send_input_recovery_card(chat_id, client_id, title, status, detail).await?;
            Ok(None)
        }
        TargetAdvanceCallbackDecision::WaitForSource { callback_tip } => {
            // 阶段错误（需要先输入源链接），弹出轻提示
            send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;
            Ok(None)
        }
    }
}

/// 输入流程无法继续时给出可点击恢复入口。
///
/// 只弹 callback 提示容易被 Telegram 客户端很快收起；额外发一张短卡片能让用户明确知道下一步。
///
/// # 参数
/// - `chat_id`: 目标聊天会话 ID
/// - `client_id`: 客户端实例 ID
/// - `title`: 卡片标题
/// - `status`: 状态标签（如 "expired", "empty"）
/// - `detail`: 引导说明文本
pub(super) async fn send_input_recovery_card(
    chat_id: i64,
    client_id: i32,
    title: &str,
    status: &str,
    detail: &str,
) -> anyhow::Result<()> {
    send::ReplyPanel::card(build_menu_recovery_text(title, status, detail))
        .row(vec![send::build_callback_button(
            "重新打开菜单",
            &super::super::callback::menu_page_callback_data(
                super::super::callback::MenuPage::Home,
            ),
            tdlib_rs::enums::ButtonStyle::Primary,
        )])
        .send(chat_id, client_id)
        .await
}

/// 把旧目标选择卡片改成等待状态。
///
/// ForceReply / reply keyboard 需要单独消息承载；旧 inline 卡片如果继续保留所有目标按钮，
/// 用户容易重复点击造成流程跳转。因此这里原地收敛为“等待 + 取消”。
///
/// # 参数
/// - `chat_id`: 聊天会话 ID
/// - `message_id`: 原卡片消息 ID
/// - `client_id`: 客户端实例 ID
/// - `step`: 步骤序号指示（如 "1/3"）
/// - `title`: 等待标题
/// - `detail`: 操作详情说明
pub(super) async fn edit_input_waiting_card(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    step: &str,
    title: &str,
    detail: &str,
) {
    edit_input_waiting_card_with_navigation(
        chat_id, message_id, client_id, step, title, detail, None, false,
    )
    .await;
}

/// 把手动目标输入卡片改成等待状态，并保留返回目标选择的入口。
///
/// # 参数
/// - `chat_id`: 聊天会话 ID
/// - `message_id`: 原卡片消息 ID
/// - `client_id`: 客户端实例 ID
/// - `step`: 步骤序号指示
/// - `title`: 等待标题
/// - `detail`: 操作详情说明
/// - `source_link`: 已经录入的源链接文本
async fn edit_target_input_waiting_card(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    step: &str,
    title: &str,
    detail: &str,
    source_link: &str,
) {
    edit_input_waiting_card_with_navigation(
        chat_id,
        message_id,
        client_id,
        step,
        title,
        detail,
        Some(source_link),
        true,
    )
    .await;
}

/// 编辑等待输入卡片，并按当前流程提供必要的返回动作。
///
/// # 参数
/// - `chat_id`: 聊天会话 ID
/// - `message_id`: 消息 ID
/// - `client_id`: 客户端实例 ID
/// - `step`: 步骤标识
/// - `title`: 标题
/// - `detail`: 详情
/// - `source_link`: 可选的已选源链接上下文
/// - `can_return_to_target_choice`: 是否包含“返回目标选择”按钮
#[allow(clippy::too_many_arguments)]
async fn edit_input_waiting_card_with_navigation(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    step: &str,
    title: &str,
    detail: &str,
    source_link: Option<&str>,
    can_return_to_target_choice: bool,
) {
    // 构造等待输入卡片的文本
    let prompt_text =
        build_step_prompt_with_context("waiting-input", step, title, detail, source_link, None);
    // 渲染卡片及内联按钮
    let Ok((text, keyboard)) = send::ReplyPanel::card(prompt_text)
        .rows(build_input_waiting_button_rows(can_return_to_target_choice))
        .into_card_parts()
    else {
        tracing::warn!(chat_id, message_id, "build waiting input card failed");
        return;
    };

    // 编辑原有消息
    if let Err(err) =
        send::edit_card_message_with_inline_keyboard(text, chat_id, message_id, keyboard, client_id)
            .await
    {
        tracing::warn!(
            chat_id,
            message_id,
            error = %err,
            "edit waiting input card failed"
        );
    }
}

/// 等待输入卡片的按钮布局。
///
/// # 参数
/// - `can_return_to_target_choice`: 是否允许返回目标选择菜单
///
/// # 返回
/// 二维内联按钮矩阵
fn build_input_waiting_button_rows(
    can_return_to_target_choice: bool,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut row = Vec::new();
    // 若允许返回上一步目标选择，增加返回按钮
    if can_return_to_target_choice {
        row.push(send::build_callback_button(
            "返回目标选择",
            &super::super::callback::target_back_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    // 添加取消流程按钮
    row.push(send::build_callback_button(
        "取消",
        &super::super::callback::cancel_input_callback_data(),
        tdlib_rs::enums::ButtonStyle::Danger,
    ));
    vec![row]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RequestActor;
    use crate::tgbot::transfer::command::menu::input::MenuJobAction;

    /// 准备测试数据库环境并获取全局锁
    async fn prepare_schema() -> anyhow::Result<tokio::sync::MutexGuard<'static, ()>> {
        let guard = crate::db::TEST_DB_LOCK.lock().await;
        let db = crate::db::get_db().await?;
        crate::db::ensure_test_schema_current(db).await?;
        Ok(guard)
    }

    /// 测试用例：没有确认草稿时，执行按钮应走“empty”分支而不是 panic
    #[tokio::test]
    async fn test_take_confirm_context_without_draft_returns_none() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (990_003, 990_004);
        let result = take_confirm_context(key).await?;

        // 校验草稿不存在时返回 None
        assert!(matches!(result, ConfirmContextTakeResult::None));
        Ok(())
    }

    /// 测试用例：单步任务动作仍应保持公开长动作名称，不因 callback 模块继续分裂语义
    #[test]
    fn test_menu_job_action_public_names_stay_stable() {
        assert_eq!(MenuJobAction::Status.command_action(), "status");
        assert_eq!(MenuJobAction::Pause.command_action(), "pause");
        assert_eq!(MenuJobAction::Resume.command_action(), "resume");
        assert_eq!(MenuJobAction::Stop.command_action(), "stop");
    }

    /// 测试用例：目标确认执行路径复用的 actor 必须仍是请求者本人，避免后续测试误用默认值
    #[test]
    fn test_confirm_path_actor_fixture_is_explicit() {
        let actor = RequestActor {
            request_chat_id: 100,
            user_id: 200,
        };

        assert_eq!(actor.request_chat_id, 100);
        assert_eq!(actor.user_id, 200);
    }

    /// 测试用例：目标 callback 上下文必须用 chat + user 隔离草稿，不能把 message_id 混进主键
    #[test]
    fn test_target_callback_context_draft_key() {
        let ctx = TargetCallbackContext {
            callback_query_id: 1,
            chat_id: 10,
            message_id: 20,
            sender_user_id: 30,
            client_id: 40,
        };

        // 验证草稿隔离键为 (chat_id, sender_user_id)
        assert_eq!(ctx.draft_key(), (10, 30));
    }

    /// 测试用例：默认目标回落当前私聊时，callback 提示应与按钮文案一致
    #[test]
    fn test_default_target_selected_tip_describes_actual_target() {
        assert_eq!(default_target_selected_tip(100, 100), "已选择当前私聊");
        assert_eq!(default_target_selected_tip(-100, 100), "已选择默认目标");
    }

    /// 测试用例：确认按钮的状态结果应映射为稳定 UI 动作，入口函数只负责执行副作用
    #[test]
    fn test_confirm_callback_decision_maps_state_results() {
        // Active 状态映射为 Run
        let active = confirm_callback_decision(ConfirmContextTakeResult::Active(
            super::super::state::ConfirmContext {
                kind: crate::tgbot::transfer::command::menu::input::MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned(),
                target_chat_id: -100,
            },
        ));
        assert!(matches!(active, ConfirmCallbackDecision::Run(_)));

        // Expired 状态映射为 Recover
        assert!(matches!(
            confirm_callback_decision(ConfirmContextTakeResult::Expired),
            ConfirmCallbackDecision::Recover {
                callback_tip: "输入已过期",
                status: "expired",
                ..
            }
        ));
        // None 状态映射为 Recover
        assert!(matches!(
            confirm_callback_decision(ConfirmContextTakeResult::None),
            ConfirmCallbackDecision::Recover {
                callback_tip: "没有待执行的输入",
                status: "empty",
                ..
            }
        ));
        // WrongStep 状态映射为 WaitForTarget
        assert_eq!(
            confirm_callback_decision(ConfirmContextTakeResult::WrongStep),
            ConfirmCallbackDecision::WaitForTarget {
                callback_tip: "请先选择目标"
            }
        );
    }

    /// 测试用例：目标推进按钮也应先规整状态层返回，避免过期/空草稿/错阶段在入口各写一遍
    #[test]
    fn test_target_advance_callback_decision_maps_state_results() {
        // Active 状态映射为 Continue
        let active = target_advance_callback_decision(
            TargetContextAdvanceResult::Active(TargetContext {
                kind: crate::tgbot::transfer::command::menu::input::MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned(),
            }),
            "没有等待选择目标的输入",
        );
        assert!(matches!(
            active,
            TargetAdvanceCallbackDecision::Continue(TargetContext {
                kind: crate::tgbot::transfer::command::menu::input::MenuInputKind::Transfer,
                ..
            })
        ));

        // Expired 状态映射为 Recover
        assert!(matches!(
            target_advance_callback_decision(
                TargetContextAdvanceResult::Expired,
                "没有等待选择目标的输入"
            ),
            TargetAdvanceCallbackDecision::Recover {
                callback_tip: "输入已过期",
                status: "expired",
                ..
            }
        ));
        // None 状态映射为 Recover
        assert!(matches!(
            target_advance_callback_decision(
                TargetContextAdvanceResult::None,
                "没有等待选择目标的输入"
            ),
            TargetAdvanceCallbackDecision::Recover {
                callback_tip: "没有等待选择目标的输入",
                status: "empty",
                ..
            }
        ));
        // WrongStep 状态映射为 WaitForSource
        assert_eq!(
            target_advance_callback_decision(
                TargetContextAdvanceResult::WrongStep,
                "没有等待选择目标的输入"
            ),
            TargetAdvanceCallbackDecision::WaitForSource {
                callback_tip: "请先发送源链接"
            }
        );
    }

    /// 测试用例：手动目标输入等待态应允许返回目标选择，其他输入等待态仍只提供取消
    #[test]
    fn test_input_waiting_button_rows_support_target_back() {
        use base64::{Engine as _, engine::general_purpose};

        // 允许返回目标选择时包含两个按钮
        let target_rows = build_input_waiting_button_rows(true);
        assert_eq!(target_rows.len(), 1);
        assert_eq!(target_rows[0][0].text, "返回目标选择");
        assert_eq!(target_rows[0][1].text, "取消");

        // 验证“返回目标选择”按钮的回调数据为 "m:tb"
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) =
            &target_rows[0][0].r#type
        else {
            panic!("target back must be callback");
        };
        let decoded = String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap())
            .expect("callback should be utf8");
        assert_eq!(decoded, "m:tb");

        // 默认不允许返回时仅包含“取消”按钮
        let default_rows = build_input_waiting_button_rows(false);
        assert_eq!(default_rows.len(), 1);
        assert_eq!(default_rows[0].len(), 1);
        assert_eq!(default_rows[0][0].text, "取消");
    }
}
