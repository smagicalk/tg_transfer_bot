// `/menu` 交互式菜单入口。
// 菜单页只做导航和轻量输入引导，真正转存、查询、分页仍复用现有命令模块。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::store;
use super::config_cmd;

mod input;
mod keyboard;
mod text;

use input::MenuInputKind;
use keyboard::{MenuPage, MenuRequestAction, build_menu_buttons, parse_menu_callback_data};
use text::{build_menu_text, build_transfer_prompt_text, build_user_account_menu_text};

/// 判断 callback payload 是否属于 `/menu`。
pub(super) fn is_menu_callback_data(data: &str) -> bool {
    keyboard::is_menu_callback_data(data)
}

/// `/menu` 命令入口。
///
/// 默认返回首页；菜单本身不要求用户记命令参数，复杂输入交给 ForceReply 引导。
pub async fn menu_command(
    _text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
    supports_reply_markup: bool,
) -> anyhow::Result<()> {
    if !supports_reply_markup {
        tracing::info!(
            request_chat_id,
            "menu command uses text fallback because current login mode cannot show reply markup"
        );
        return send::ReplyPanel::card(build_user_account_menu_text())
            .send(request_chat_id, client_id)
            .await;
    }

    send_menu_page(MenuPage::Home, request_chat_id, client_id).await
}

/// `/menu` inline keyboard 回调入口。
///
/// 菜单 callback 只处理自身页面切换和“开始输入”；下载/任务/帮助按钮会走各自模块的 callback。
pub async fn menu_callback_query(
    update: tdlib_rs::enums::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    let Some(action) = parse_menu_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("菜单按钮参数无效"), client_id).await?;
        return Ok(());
    };

    match action {
        MenuRequestAction::Page(page) => {
            let (text, rows) = build_menu_page(page, update.chat_id).await?;
            let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
            send::answer_callback_query(update.id, Some(page.title()), client_id).await?;
            send::edit_card_message_with_inline_keyboard(
                text,
                update.chat_id,
                update.message_id,
                keyboard,
                client_id,
            )
            .await
        }
        MenuRequestAction::NewTransfer => {
            start_input_prompt(
                update.id,
                update.chat_id,
                update.sender_user_id,
                client_id,
                MenuInputKind::Transfer,
            )
            .await
        }
        MenuRequestAction::QuickTransferDefault => {
            start_input_prompt(
                update.id,
                update.chat_id,
                update.sender_user_id,
                client_id,
                MenuInputKind::TransferDefault,
            )
            .await
        }
        MenuRequestAction::NewLookup => {
            start_input_prompt(
                update.id,
                update.chat_id,
                update.sender_user_id,
                client_id,
                MenuInputKind::Lookup,
            )
            .await
        }
        MenuRequestAction::QuickLookupDefault => {
            start_input_prompt(
                update.id,
                update.chat_id,
                update.sender_user_id,
                client_id,
                MenuInputKind::LookupDefault,
            )
            .await
        }
    }
}

/// 发送 ForceReply 输入提示，并记录对应输入流程。
async fn start_input_prompt(
    callback_query_id: i64,
    chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
    kind: MenuInputKind,
) -> anyhow::Result<()> {
    input::start_menu_input(chat_id, sender_user_id, kind);
    send::answer_callback_query(callback_query_id, Some("请输入源链接"), client_id).await?;
    send::send_card_message_with_force_reply_returning(
        build_transfer_prompt_text(kind.source_title(), kind.source_detail()),
        chat_id,
        "输入源链接，或发送 /cancel",
        client_id,
    )
    .await?;
    Ok(())
}

/// 处理菜单 ForceReply 输入。
///
/// 返回 `true` 表示本条普通文本已被菜单输入流消费，上层不应继续按未知文本处理。
pub async fn handle_menu_text_input(
    text: &str,
    config: Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    input::handle_menu_input(
        text,
        config,
        request_chat_id,
        request_message_id,
        sender_user_id,
        client_id,
    )
    .await
}

/// 丢弃当前聊天里的菜单输入草稿。
pub fn discard_menu_input(request_chat_id: i64, sender_user_id: i64) -> bool {
    input::cancel_menu_input(request_chat_id, sender_user_id)
}

/// 取消当前聊天里的菜单输入草稿，并给用户明确反馈。
pub async fn cancel_menu_input(
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    if !input::cancel_menu_input(request_chat_id, sender_user_id) {
        return Ok(false);
    }

    send::ReplyPanel::card(build_transfer_prompt_text(
        "已取消",
        "当前菜单输入已取消，可重新打开 /m。",
    ))
    .row(vec![send::build_copy_button(
        "复制 /m",
        "/m",
        tdlib_rs::enums::ButtonStyle::Primary,
    )])
    .send(request_chat_id, client_id)
    .await?;
    Ok(true)
}

/// 发送一个菜单页。
async fn send_menu_page(
    page: MenuPage,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, rows) = build_menu_page(page, request_chat_id).await?;
    send::ReplyPanel::card(text)
        .rows(rows)
        .send(request_chat_id, client_id)
        .await
}

/// 构造一个菜单页的正文和按钮。
///
/// 首页需要读取最近任务作为快捷入口；配置页直接复用 `/cfg` 的实时配置卡片，
/// 这样 `/m -> 配置` 和 `/cfg` 展示的是同一套运行参数。
async fn build_menu_page(
    page: MenuPage,
    request_chat_id: i64,
) -> anyhow::Result<(String, Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>)> {
    let recent_jobs = if page == MenuPage::Home {
        store::list_recent_job_snapshots(request_chat_id, 5).await?
    } else {
        Vec::new()
    };
    let text = if page == MenuPage::Config {
        config_cmd::format_current_transfer_config_text("当前可调配置")
    } else {
        build_menu_text(page)
    };
    Ok((text, build_menu_buttons(page, &recent_jobs)))
}
