// `/menu` 输入流程的 inline callback 操作。
// 普通文本输入仍留在父模块；这里集中处理按钮点击后的状态推进和消息编辑。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::text::{
    build_menu_status_text, build_step_prompt_text, build_transfer_prompt_text,
};
use super::state::{
    DraftKey, DraftTakeResult, MenuInputDraft, MenuInputKind, MenuInputStep, MenuJobAction,
    cancel_menu_input_with_state, put_confirm_draft, put_draft, put_target_choice_draft,
    remember_last_target, take_current_draft, target_context_from_step,
};
use super::target::{
    edit_confirm_prompt, edit_target_choice_prompt, resolve_default_target, resolve_target_by_id,
};

/// 处理“使用默认目标”按钮。
pub(in crate::tgbot::transfer::command::menu) async fn target_default_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    config: Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<()> {
    let key = (chat_id, sender_user_id);
    let Some((kind, source_link)) = take_target_context_for_callback(
        key,
        callback_query_id,
        chat_id,
        "没有等待选择目标的输入",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };

    let Some(target_chat_id) = resolve_default_target(&config, chat_id) else {
        put_target_choice_draft(key, kind, source_link).await?;
        send::answer_callback_query(callback_query_id, Some("当前没有默认目标"), client_id).await?;
        return Ok(());
    };

    put_confirm_draft(key, kind, source_link.clone(), target_chat_id).await?;
    send::answer_callback_query(callback_query_id, Some("已选择默认目标"), client_id).await?;
    edit_confirm_prompt(
        kind,
        &source_link,
        target_chat_id,
        chat_id,
        message_id,
        client_id,
    )
    .await
}

/// 处理“常用目标”按钮。
pub(in crate::tgbot::transfer::command::menu) async fn target_alias_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    target_chat_id: i64,
    config: Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<()> {
    let key = (chat_id, sender_user_id);
    let Some((kind, source_link)) = take_target_context_for_callback(
        key,
        callback_query_id,
        chat_id,
        "没有等待选择目标的输入",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };

    if let Err(err) = resolve_target_by_id(target_chat_id, &config, chat_id) {
        put_target_choice_draft(key, kind, source_link).await?;
        send::answer_callback_query(callback_query_id, Some("目标不在允许列表"), client_id).await?;
        tracing::warn!(
            chat_id,
            sender_user_id,
            target_chat_id,
            error = %err,
            "target alias callback rejected"
        );
        return Ok(());
    }

    put_confirm_draft(key, kind, source_link.clone(), target_chat_id).await?;
    send::answer_callback_query(callback_query_id, Some("已选择目标"), client_id).await?;
    edit_confirm_prompt(
        kind,
        &source_link,
        target_chat_id,
        chat_id,
        message_id,
        client_id,
    )
    .await
}

/// 处理“手动输入目标”按钮。
pub(in crate::tgbot::transfer::command::menu) async fn target_manual_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let key = (chat_id, sender_user_id);
    let Some((kind, source_link)) = take_target_context_for_callback(
        key,
        callback_query_id,
        chat_id,
        "没有等待选择目标的输入",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };

    put_draft(key, MenuInputDraft::target_chat(kind, source_link)).await?;
    send::answer_callback_query(callback_query_id, Some("请输入目标"), client_id).await?;
    edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        "2/3",
        "等待手动输入",
        "请回复目标 chat_id、配置别名，或回复 default。",
    )
    .await;
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text(
            "2/3",
            "输入目标",
            "请回复数字 chat_id、配置里的目标别名，或回复 default 使用配置默认目标。",
        ),
        chat_id,
        "输入目标 chat_id、别名或 default",
        client_id,
    )
    .await?;
    Ok(())
}

/// 处理“选择群组”按钮，发送 Telegram 原生选群键盘。
pub(in crate::tgbot::transfer::command::menu) async fn target_request_chat_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let key = (chat_id, sender_user_id);
    let Some((kind, source_link)) = take_target_context_for_callback(
        key,
        callback_query_id,
        chat_id,
        "没有等待选择目标的输入",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };

    put_draft(key, MenuInputDraft::chat_picker(kind, source_link)).await?;
    send::answer_callback_query(callback_query_id, Some("请选择群组"), client_id).await?;
    edit_input_waiting_card(
        chat_id,
        message_id,
        client_id,
        "2/3",
        "等待选择群组",
        "请使用输入框下方的 Telegram 原生选群按钮。",
    )
    .await;
    send::send_card_message_with_chat_request_keyboard_returning(
        build_step_prompt_text(
            "2/3",
            "选择目标群组",
            "点击输入框下方的“选择群组”，Telegram 会打开原生群组选择器；不想继续就点“取消”。",
        ),
        chat_id,
        super::TARGET_CHAT_REQUEST_BUTTON_ID,
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
    edit_input_waiting_card(
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
    edit_input_waiting_card(
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

/// 处理确认页“执行”按钮。
pub(in crate::tgbot::transfer::command::menu) async fn target_confirm_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    config: Arc<BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let key = (chat_id, sender_user_id);
    let draft = match take_current_draft(key).await? {
        DraftTakeResult::Active(draft) => draft,
        DraftTakeResult::Expired => {
            send::answer_callback_query(callback_query_id, Some("输入已过期"), client_id).await?;
            return Ok(());
        }
        DraftTakeResult::None => {
            send::answer_callback_query(callback_query_id, Some("没有待执行的输入"), client_id)
                .await?;
            return Ok(());
        }
    };

    let MenuInputStep::Confirm {
        kind,
        source_link,
        target_chat_id,
    } = draft.step
    else {
        put_draft(key, draft).await?;
        send::answer_callback_query(callback_query_id, Some("请先选择目标"), client_id).await?;
        return Ok(());
    };

    remember_last_target(chat_id, sender_user_id, target_chat_id);
    send::answer_callback_query(callback_query_id, Some("开始执行"), client_id).await?;
    super::run_existing_command(
        kind,
        vec![
            kind.command_name().to_owned(),
            source_link,
            target_chat_id.to_string(),
        ],
        config,
        chat_id,
        message_id,
        actor,
        client_id,
    )
    .await
}

/// 处理确认页“返回选择目标”按钮。
pub(in crate::tgbot::transfer::command::menu) async fn target_back_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    config: Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<()> {
    let key = (chat_id, sender_user_id);
    let Some((kind, source_link)) = take_target_context_for_callback(
        key,
        callback_query_id,
        chat_id,
        "没有可返回的目标选择",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };
    put_target_choice_draft(key, kind, source_link.clone()).await?;
    send::answer_callback_query(callback_query_id, Some("已返回目标选择"), client_id).await?;
    edit_target_choice_prompt(
        &config,
        chat_id,
        sender_user_id,
        message_id,
        client_id,
        kind,
        &source_link,
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
    .row(vec![send::build_copy_button(
        "复制 /menu",
        "/menu",
        tdlib_rs::enums::ButtonStyle::Primary,
    )])
    .into_card_parts()?;
    send::edit_card_message_with_inline_keyboard(text, chat_id, message_id, keyboard, client_id)
        .await?;
    if cancelled
        .map(|cancelled| cancelled.needs_reply_keyboard_cleanup)
        .unwrap_or(false)
    {
        super::send_keyboard_cleanup_notice(
            chat_id,
            client_id,
            "键盘已收起",
            "已移除输入框下方的选群键盘，可重新打开 /menu。",
        )
        .await?;
    }
    Ok(())
}

/// 把旧目标选择卡片改成等待状态。
///
/// ForceReply / reply keyboard 需要单独消息承载；旧 inline 卡片如果继续保留所有目标按钮，
/// 用户容易重复点击造成流程跳转。因此这里原地收敛为“等待 + 取消”。
async fn edit_input_waiting_card(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    step: &str,
    title: &str,
    detail: &str,
) {
    let Ok((text, keyboard)) = send::ReplyPanel::card(build_step_prompt_text(step, title, detail))
        .row(vec![send::build_callback_button(
            "取消",
            &super::super::callback::cancel_input_callback_data(),
            tdlib_rs::enums::ButtonStyle::Danger,
        )])
        .into_card_parts()
    else {
        tracing::warn!(chat_id, message_id, "build waiting input card failed");
        return;
    };

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

/// 从 callback 取出当前目标选择上下文。
async fn take_target_context_for_callback(
    key: DraftKey,
    callback_query_id: i64,
    chat_id: i64,
    missing_tip: &str,
    client_id: i32,
) -> anyhow::Result<Option<(MenuInputKind, String)>> {
    let draft = match take_current_draft(key).await? {
        DraftTakeResult::Active(draft) => draft,
        DraftTakeResult::Expired => {
            send::answer_callback_query(callback_query_id, Some("输入已过期"), client_id).await?;
            send::ReplyPanel::card(build_transfer_prompt_text(
                "输入已过期",
                "上一次菜单输入已超过有效时间，请重新打开 /menu。",
            ))
            .send(chat_id, client_id)
            .await?;
            return Ok(None);
        }
        DraftTakeResult::None => {
            send::answer_callback_query(callback_query_id, Some(missing_tip), client_id).await?;
            return Ok(None);
        }
    };

    let Some(context) = target_context_from_step(&draft.step) else {
        put_draft(key, draft).await?;
        send::answer_callback_query(callback_query_id, Some("请先发送源链接"), client_id).await?;
        return Ok(None);
    };
    Ok(Some(context))
}
