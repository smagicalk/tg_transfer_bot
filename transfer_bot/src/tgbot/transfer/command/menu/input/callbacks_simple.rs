//! `/menu` 单步简单输入相关的 Telegram Inline Callback 回调处理模块。
//!
//! 负责响应用户在任务控制卡片、系统管理配置卡片中触发的单步输入（如输入任务 ID、修改并发数、设置默认目标等），
//! 以及用户主动点击“取消输入”内联按钮时的草稿清理与卡片状态刷新。

use crate::tgbot::send;
use crate::tgbot::transfer::command::menu::build_menu_home_callback_data;

use super::super::text::{build_menu_status_text, build_step_prompt_text};
use super::state::{
    AdminInputAction, MenuInputDraft, MenuJobAction, admin_input_prompt_meta,
    cancel_menu_input_with_result, put_draft,
};

/// 响应任务控制面板中“输入 job_id”相关的内联按钮回调点击。
///
/// 业务流程：
/// 1. 创建并存储对应动作（如暂停、恢复、停止等）的输入草稿；
/// 2. 应答 Telegram Callback Query 弹窗；
/// 3. 将原交互卡片编辑为“等待输入任务编号”状态；
/// 4. 发送带有 ForceReply 的输入提示卡片，引导用户输入纯数字任务编号。
///
/// # 参数说明
/// - `callback_query_id`: Telegram 回调查询唯一标识 ID
/// - `chat_id`: 触发回调所在的会话聊天 ID
/// - `message_id`: 原交互卡片的消息 ID
/// - `sender_user_id`: 点击按钮的用户 Telegram ID
/// - `action`: 任务操作动作类型（`MenuJobAction`）
/// - `client_id`: TDLib 客户端实例标识
pub(in crate::tgbot::transfer::command::menu) async fn job_id_input_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    action: MenuJobAction,
    client_id: i32,
) -> anyhow::Result<()> {
    // 写入等待输入任务 ID 的草稿状态
    put_draft((chat_id, sender_user_id), MenuInputDraft::job_id(action)).await?;
    // 应答弹窗提示
    send::answer_callback_query(callback_query_id, Some("请输入 job_id"), client_id).await?;
    // 将原卡片更新为等待输入的过渡展示
    super::callbacks_target::edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        "1/1",
        "等待任务编号",
        "请回复纯数字 job_id，或点击取消结束当前输入。",
    )
    .await;
    // 发送带有 ForceReply 的输入引导卡片
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text("1/1", action.input_title(), action.input_detail()),
        chat_id,
        "输入数字 job_id（回复“取消”可退出）",
        client_id,
    )
    .await?;
    Ok(())
}

/// 响应管理员配置面板中的“输入参数”按钮回调（无初始上下文版本）。
///
/// 内部委托调用 `admin_input_callback_query_with_context`。
///
/// # 参数说明
/// - `callback_query_id`: 回调查询 ID
/// - `chat_id`: 会话聊天 ID
/// - `message_id`: 原消息 ID
/// - `sender_user_id`: 操作者用户 ID
/// - `action`: 管理员交互动作枚举
/// - `client_id`: 客户端实例 ID
pub(in crate::tgbot::transfer::command::menu) async fn admin_input_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    action: AdminInputAction,
    client_id: i32,
) -> anyhow::Result<()> {
    admin_input_callback_query_with_context(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        action,
        None,
        None,
        None,
        None,
        None,
        client_id,
    )
    .await
}

/// 响应携带前置上下文的管理员配置输入回调。
///
/// 典型应用场景：
/// - targets 目标管理中选中已有别名后再修改目标：此时 `context_text` 保存既有別名名称；
/// - 原生选聊场景：利用 `callback_query_id` 或传入的 `context_i64` 派生本次选聊交互的唯一令牌 `picker_token`，
///   防止旧消息的回调结果或共享会话跨消息串线。
///
/// # 参数说明
/// - `callback_query_id`: Telegram 回调查询 ID
/// - `chat_id`: 来源会话 ID
/// - `message_id`: 原卡片消息 ID
/// - `sender_user_id`: 发起用户 ID
/// - `action`: 管理员操作动作枚举
/// - `context_text`: 前置文本上下文（如别名）
/// - `context_i64`: 前置整型上下文（如指定的会话 ID 或选聊 token）
/// - `prompt_title`: 自定义卡片标题（可选覆盖）
/// - `prompt_detail`: 自定义详细说明（可选覆盖）
/// - `prompt_placeholder`: 自定义输入框占位符（可选覆盖）
/// - `client_id`: 客户端实例标识符
#[allow(clippy::too_many_arguments)]
pub(in crate::tgbot::transfer::command::menu) async fn admin_input_callback_query_with_context(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    action: AdminInputAction,
    context_text: Option<String>,
    context_i64: Option<i64>,
    prompt_title: Option<String>,
    prompt_detail: Option<String>,
    prompt_placeholder: Option<String>,
    client_id: i32,
) -> anyhow::Result<()> {
    // 原生选聊需要将本次 callback 关联的 token 固化在草稿中，隔离不同选聊流程
    let picker_token = if action.uses_chat_picker() {
        Some(context_i64.unwrap_or(callback_query_id))
    } else {
        context_i64
    };
    // 计算提示元数据与步骤指示标签（如 "1/2" 或 "1/1"）
    let meta = admin_input_prompt_meta(action, context_text.as_deref(), picker_token);
    let step_label = super::admin_input_step_label(action, context_text.as_deref(), picker_token);

    // 将管理员输入草稿持久化存储
    put_draft(
        (chat_id, sender_user_id),
        MenuInputDraft::admin_input(action, context_text, picker_token),
    )
    .await?;

    let prompt_title = prompt_title.unwrap_or(meta.title);
    let prompt_detail = prompt_detail.unwrap_or(meta.detail);
    let prompt_placeholder = prompt_placeholder.unwrap_or(meta.placeholder);
    let callback_tip = if action.uses_chat_picker() {
        "请选择目标聊天"
    } else {
        "请输入参数"
    };
    // 应答点击通知
    send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;

    // 编辑旧卡片进入等待状态
    super::callbacks_target::edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        step_label,
        &prompt_title,
        &prompt_detail,
    )
    .await;

    // 发送管理员输入引导（若是原生选聊则发送选聊键盘，否则发送带 ForceReply 的输入提示）
    super::send_admin_input_prompt(
        action,
        picker_token,
        step_label,
        &prompt_title,
        &prompt_detail,
        &prompt_placeholder,
        chat_id,
        sender_user_id,
        client_id,
    )
    .await?;

    // 目标管理 picker 使用新消息承载 reply keyboard；旧 inline 卡片只会造成重复入口，尝试将其删除清理
    if action.uses_chat_picker()
        && let Err(error) = send::delete_message(chat_id, message_id, client_id).await
    {
        tracing::debug!(
            chat_id,
            sender_user_id,
            message_id,
            error = %error,
            "stale admin input card could not be deleted"
        );
    }
    Ok(())
}

/// 响应输入流程中的“取消输入”内联按钮回调点击。
///
/// 业务流程：
/// 1. 取消并物理清理数据库中的草稿行；
/// 2. 应答 Telegram Callback Query 为“已取消”；
/// 3. 将原交互卡片编辑为“已取消”通知卡片，并附带“返回菜单”按钮；
/// 4. 若原流程启用了原生回复选聊键盘，同步发送一条消息移除底部的回复键盘。
///
/// # 参数说明
/// - `callback_query_id`: Telegram 回调查询 ID
/// - `chat_id`: 会话聊天 ID
/// - `message_id`: 原消息 ID
/// - `sender_user_id`: 操作用户 ID
/// - `client_id`: 客户端实例 ID
pub(in crate::tgbot::transfer::command::menu) async fn cancel_input_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    // 原子性取消当前用户的挂起草稿
    let cancelled = cancel_menu_input_with_result(chat_id, sender_user_id).await?;
    send::answer_callback_query(callback_query_id, Some("已取消"), client_id).await?;

    // 组装已取消卡片内容
    let (text, keyboard) = send::ReplyPanel::card(build_menu_status_text(
        "已取消",
        "cancelled",
        if cancelled.removed {
            "当前输入流程已取消。"
        } else {
            "没有正在进行的输入流程。"
        },
    ))
    .row(vec![send::build_callback_button(
        "返回菜单",
        &build_menu_home_callback_data(),
        tdlib_rs::enums::ButtonStyle::Primary,
    )])
    .into_card_parts()?;

    // 原位编辑交互卡片
    send::edit_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        "取消输入刷新失败",
        "输入流程已处理，但原消息编辑失败；请使用错误卡片上的“菜单”按钮重新进入。",
    )
    .await?;

    // 若此前激活了原生选聊 reply keyboard，则清理该回复键盘
    if cancelled.remove_reply_keyboard {
        let cleared = super::clear_native_picker_messages(chat_id, sender_user_id, client_id).await;
        if !cleared {
            send::send_card_message_with_remove_keyboard(
                build_menu_status_text(
                    "聊天选择已关闭",
                    "cancelled",
                    "输入框下方的目标聊天选择按钮已移除。",
                ),
                chat_id,
                client_id,
            )
            .await?;
        }
    }
    Ok(())
}
