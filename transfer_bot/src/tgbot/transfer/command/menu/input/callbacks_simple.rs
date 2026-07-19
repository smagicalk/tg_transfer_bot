// `/menu` 单步输入相关的 inline callback。
// 这里集中处理 job_id / user_id 这类一步完成的输入，以及取消当前输入流程。

use crate::tgbot::send;
use crate::tgbot::transfer::command::menu::build_menu_home_callback_data;

use super::super::text::{build_menu_status_text, build_step_prompt_text};
use super::state::{
    AdminInputAction, MenuInputDraft, MenuJobAction, admin_input_prompt_meta, cancel_menu_input,
    put_draft,
};

/// 管理输入入口的步骤标签。
///
/// 新增别名是两步输入，其他管理动作仍是一条回复完成。
fn admin_input_step_label(action: AdminInputAction) -> &'static str {
    match action {
        AdminInputAction::TargetsAliasName => "1/2",
        _ => "1/1",
    }
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
    let meta = admin_input_prompt_meta(action, context_text.as_deref(), context_i64);
    put_draft(
        (chat_id, sender_user_id),
        MenuInputDraft::admin_input(action, context_text, context_i64),
    )
    .await?;
    let prompt_title = prompt_title.unwrap_or(meta.title);
    let prompt_detail = prompt_detail.unwrap_or(meta.detail);
    let prompt_placeholder = prompt_placeholder.unwrap_or(meta.placeholder);
    send::answer_callback_query(callback_query_id, Some("请输入参数"), client_id).await?;
    super::callbacks_target::edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        admin_input_step_label(action),
        &prompt_title,
        &prompt_detail,
    )
    .await;
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text(
            admin_input_step_label(action),
            &prompt_title,
            &prompt_detail,
        ),
        chat_id,
        &prompt_placeholder,
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
    let removed = cancel_menu_input(chat_id, sender_user_id).await?;
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
    Ok(())
}
