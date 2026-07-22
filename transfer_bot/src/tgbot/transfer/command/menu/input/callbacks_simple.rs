// `/menu` 单步输入相关的 inline callback。
// 这里集中处理 job_id / user_id 这类一步完成的输入，以及取消当前输入流程。

use crate::tgbot::send;
use crate::tgbot::transfer::command::menu::build_menu_home_callback_data;

use super::super::text::{build_menu_status_text, build_step_prompt_text};
use super::state::{
    AdminInputAction, MenuInputDraft, MenuJobAction, admin_input_prompt_meta,
    cancel_menu_input_with_result, put_draft,
};

/// 处理任务页的“输入 job_id”按钮。
///
/// 这里只启动输入草稿，不直接改任务状态；用户回复 job_id 后会复用 `/job` 命令入口执行。
pub(in crate::tgbot::transfer::command::menu) async fn job_id_input_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    action: MenuJobAction,
    client_id: i32,
) -> anyhow::Result<()> {
    put_draft((chat_id, sender_user_id), MenuInputDraft::job_id(action)).await?;
    send::answer_callback_query(callback_query_id, Some("请输入 job_id"), client_id).await?;
    super::callbacks_target::edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        "1/1",
        "等待任务编号",
        "请回复纯数字 job_id，或点击取消结束当前输入。",
    )
    .await;
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text("1/1", action.input_title(), action.input_detail()),
        chat_id,
        "输入数字 job_id（回复“取消”可退出）",
        client_id,
    )
    .await?;
    Ok(())
}

/// 处理管理配置页里的“输入参数”按钮。
///
/// 这里只启动输入草稿，不直接更新数据库；用户回复后会复用 `/targets`、`/config`
/// 现有命令入口，避免菜单输入流复制配置写库逻辑。
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

/// 处理带上下文的管理输入入口。
///
/// `targets` 里的“选中现有别名/路由后再改目标”会通过这里把 alias 或 request_chat_id
/// 存进草稿，随后只需要用户输入新的 target_chat_id。
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
    // 先算提示文案，再把上下文写进草稿；否则 `context_text` 会被移动掉。
    // 目标 picker 需要把本次 callback 绑定到草稿，防止旧消息的共享聊天结果串线。
    let picker_token = if action.uses_chat_picker() {
        Some(context_i64.unwrap_or(callback_query_id))
    } else {
        context_i64
    };
    let meta = admin_input_prompt_meta(action, context_text.as_deref(), picker_token);
    let step_label = super::admin_input_step_label(action, context_text.as_deref(), picker_token);
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
    send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;
    super::callbacks_target::edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        step_label,
        &prompt_title,
        &prompt_detail,
    )
    .await;
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
    // 目标管理 picker 使用新消息承载 reply keyboard；旧 inline 卡片只会造成重复入口。
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

/// 处理输入流取消按钮。
pub(in crate::tgbot::transfer::command::menu) async fn cancel_input_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let cancelled = cancel_menu_input_with_result(chat_id, sender_user_id).await?;
    send::answer_callback_query(callback_query_id, Some("已取消"), client_id).await?;
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
