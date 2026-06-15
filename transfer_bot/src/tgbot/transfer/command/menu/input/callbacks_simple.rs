// `/menu` 单步输入相关的 inline callback。
// 这里集中处理 job_id / user_id 这类一步完成的输入，以及取消当前输入流程。

use crate::tgbot::send;
use crate::tgbot::transfer::command::menu::build_menu_home_callback_data;

use super::super::text::{build_menu_status_text, build_step_prompt_text};
use super::simple::send_keyboard_cleanup_notice;
use super::state::{MenuInputDraft, MenuJobAction, cancel_menu_input_with_state, put_draft};

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
        "输入数字 job_id，或发送 /cancel",
        client_id,
    )
    .await?;
    Ok(())
}

/// 处理 admin “输入用户 ID 查询积分流水”按钮。
///
/// 这里只启动输入草稿，不直接查询；用户回复 user_id 后会复用 `/points history` 命令入口。
pub(in crate::tgbot::transfer::command::menu) async fn point_ledger_user_input_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    if !actor.is_admin() {
        send::answer_callback_query(callback_query_id, Some("没有权限查询用户流水"), client_id)
            .await?;
        return Ok(());
    }
    put_draft(
        (chat_id, sender_user_id),
        MenuInputDraft::point_ledger_user_id(),
    )
    .await?;
    send::answer_callback_query(callback_query_id, Some("请输入 user_id"), client_id).await?;
    super::callbacks_target::edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        "1/1",
        "等待用户 ID",
        "请回复纯数字 Telegram user_id，或点击取消结束当前输入。",
    )
    .await;
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
    let cancelled = cancel_menu_input_with_state(chat_id, sender_user_id).await?;
    let removed = cancelled.is_some();
    send::answer_callback_query(callback_query_id, Some("已取消"), client_id).await?;
    let (text, keyboard) = send::ReplyPanel::card(build_menu_status_text(
        "已取消",
        "cancelled",
        if removed {
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
        "输入流程已处理，但原消息编辑失败；请复制错误或重新打开 /menu。",
    )
    .await?;
    if cancelled
        .map(|cancelled| cancelled.needs_reply_keyboard_cleanup)
        .unwrap_or(false)
    {
        send_keyboard_cleanup_notice(
            chat_id,
            client_id,
            "键盘已收起",
            "已移除输入框下方的选群键盘，可重新打开 /menu。",
        )
        .await?;
    }
    Ok(())
}
