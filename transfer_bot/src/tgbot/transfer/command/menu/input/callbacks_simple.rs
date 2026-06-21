// `/menu` 单步输入相关的 inline callback。
// 这里集中处理 job_id / user_id 这类一步完成的输入，以及取消当前输入流程。

use crate::tgbot::send;
use crate::tgbot::transfer::command::menu::build_menu_home_callback_data;

use super::super::text::{build_menu_status_text, build_step_prompt_text};
use super::simple::send_keyboard_cleanup_notice;
use super::state::{
    AdminInputAction, MenuInputDraft, MenuJobAction, cancel_menu_input_with_state, put_draft,
};
use super::{TARGETS_DEFAULT_REQUEST_BUTTON_ID, TARGETS_ROUTE_REQUEST_BUTTON_ID};

/// 需要原生选群器的 targets 管理输入动作。
fn is_targets_chat_picker_action(action: AdminInputAction) -> bool {
    matches!(
        action,
        AdminInputAction::TargetsPickDefault | AdminInputAction::TargetsPickRoute
    )
}

/// 把 targets 选群动作映射到对应的 requestChat 按钮 ID。
fn targets_chat_picker_button_id(action: AdminInputAction) -> Option<i32> {
    match action {
        AdminInputAction::TargetsPickDefault => Some(TARGETS_DEFAULT_REQUEST_BUTTON_ID),
        AdminInputAction::TargetsPickRoute => Some(TARGETS_ROUTE_REQUEST_BUTTON_ID),
        _ => None,
    }
}

/// 发送 targets 配置页的原生选群提示。
pub(super) async fn send_targets_chat_picker_prompt(
    chat_id: i64,
    client_id: i32,
    action: AdminInputAction,
    request_chat_id_input: Option<i64>,
) -> anyhow::Result<()> {
    let (title, detail) = match action {
        AdminInputAction::TargetsPickDefault => (
            "选择默认目标",
            "点击输入框下方的“选择群组”，选中的群会写入 default_chat_id。",
        ),
        AdminInputAction::TargetsPickRoute => (
            "选择请求路由目标",
            "点击输入框下方的“选择群组”，选中的群会写入该 request_chat_id 的路由目标。",
        ),
        _ => anyhow::bail!(
            "unsupported targets chat picker action: {}",
            action.log_name()
        ),
    };
    let context_detail = request_chat_id_input
        .map(|request_chat_id| {
            format!(
                "当前 request_chat_id：{}",
                crate::tgbot::transfer::card::code(request_chat_id)
            )
        })
        .unwrap_or_default();
    let text = if context_detail.is_empty() {
        build_step_prompt_text("1/1", title, detail)
    } else {
        format!(
            "{}\n{}",
            build_step_prompt_text("1/1", title, detail),
            context_detail
        )
    };
    send::send_card_message_with_chat_request_keyboard_returning(
        text,
        chat_id,
        targets_chat_picker_button_id(action)
            .expect("targets chat picker action should map button id"),
        "选择群组",
        "选择目标群组，或发送 /cancel",
        client_id,
    )
    .await?;
    Ok(())
}

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

pub(in crate::tgbot::transfer::command::menu) async fn points_adjust_input_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    action: AdminInputAction,
    target_user_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    if !actor.is_admin() {
        send::answer_callback_query(callback_query_id, Some("没有权限调整积分"), client_id)
            .await?;
        return Ok(());
    }

    put_draft(
        (chat_id, sender_user_id),
        MenuInputDraft::points_adjust(action, target_user_id),
    )
    .await?;
    send::answer_callback_query(callback_query_id, Some("请输入积分和理由"), client_id).await?;
    super::callbacks_target::edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        "1/1",
        action.input_title(),
        action.input_detail(),
    )
    .await;
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text("1/1", action.input_title(), action.input_detail()),
        chat_id,
        action.input_placeholder(),
        client_id,
    )
    .await?;
    Ok(())
}

/// 处理管理配置页里的“输入参数”按钮。
///
/// 这里只启动输入草稿，不直接更新数据库；用户回复后会复用 `/targets`、`/acl`、`/billing`
/// 现有命令入口，避免菜单输入流复制配置写库逻辑。
pub(in crate::tgbot::transfer::command::menu) async fn admin_input_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    action: AdminInputAction,
    client_id: i32,
) -> anyhow::Result<()> {
    if action == AdminInputAction::TargetsPickDefault {
        put_draft(
            (chat_id, sender_user_id),
            MenuInputDraft::admin_chat_picker(action, None),
        )
        .await?;
        send::answer_callback_query(callback_query_id, Some("请选择目标群组"), client_id).await?;
        super::callbacks_target::edit_input_waiting_card(
            chat_id,
            message_id,
            client_id,
            "1/1",
            "等待选择默认目标",
            "请使用输入框下方的 Telegram 原生选群按钮。",
        )
        .await;
        return send_targets_chat_picker_prompt(chat_id, client_id, action, None).await;
    }
    if action == AdminInputAction::TargetsPickRoute {
        put_draft(
            (chat_id, sender_user_id),
            MenuInputDraft::admin_chat_picker(action, None),
        )
        .await?;
        send::answer_callback_query(callback_query_id, Some("请输入 request_chat_id"), client_id)
            .await?;
        super::callbacks_target::edit_input_waiting_card(
            chat_id,
            message_id,
            client_id,
            "1/2",
            "等待 request_chat_id",
            "请先回复 request_chat_id，随后会弹出 Telegram 原生选群器。",
        )
        .await;
        send::send_card_message_with_force_reply_returning(
            build_step_prompt_text(
                "1/2",
                "输入 request_chat_id",
                "请回复 request_chat_id，随后会弹出目标群组选择器；或发送 /cancel 取消。",
            ),
            chat_id,
            "输入 request_chat_id，或发送 /cancel",
            client_id,
        )
        .await?;
        return Ok(());
    }

    put_draft(
        (chat_id, sender_user_id),
        MenuInputDraft::admin_input(action),
    )
    .await?;
    send::answer_callback_query(callback_query_id, Some("请输入参数"), client_id).await?;
    super::callbacks_target::edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        "1/1",
        action.input_title(),
        if is_targets_chat_picker_action(action) {
            "请先回复 request_chat_id，随后会弹出 Telegram 原生选群器。"
        } else {
            action.input_detail()
        },
    )
    .await;
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text(
            "1/1",
            action.input_title(),
            if is_targets_chat_picker_action(action) {
                "请先回复 request_chat_id，随后会弹出 Telegram 原生选群器。"
            } else {
                action.input_detail()
            },
        ),
        chat_id,
        if is_targets_chat_picker_action(action) {
            "输入 request_chat_id，或发送 /cancel"
        } else {
            action.input_placeholder()
        },
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
