// `/menu` 交互式菜单入口。
// 菜单页只做导航和轻量输入引导，真正转存、查询、分页仍复用现有命令模块。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::store;
use super::config_cmd;
use crate::tgbot::send::send_interaction_error_card;

mod callback;
mod input;
mod keyboard;
mod text;

use callback::{MenuPage, MenuRequestAction, parse_menu_callback_data};
use input::MenuInputKind;
use keyboard::build_menu_buttons;
use text::{
    MenuHomeSummary, build_menu_home_text, build_menu_status_text, build_menu_text,
    build_step_prompt_text, build_user_account_menu_text,
};

/// 判断 callback payload 是否属于 `/menu`。
pub(super) fn is_menu_callback_data(data: &str) -> bool {
    callback::is_menu_callback_data(data)
}

/// 生成菜单首页 callback 数据。
///
/// 供进度卡片、结果卡片等非菜单模块放置“返回菜单”按钮；
/// 外部不直接依赖 `MenuPage`，避免把菜单内部页面枚举扩散出去。
pub(super) fn build_menu_home_callback_data() -> String {
    callback::menu_page_callback_data(MenuPage::Home)
}

/// `/menu` 命令入口。
///
/// 默认返回首页；菜单本身不要求用户记命令参数，复杂输入交给 ForceReply 引导。
pub async fn menu_command(
    _text: Vec<&str>,
    actor: crate::config::RequestActor,
    client_id: i32,
    supports_reply_markup: bool,
) -> anyhow::Result<()> {
    if !supports_reply_markup {
        tracing::info!(
            request_chat_id = actor.request_chat_id,
            "menu command uses text fallback because current login mode cannot show reply markup"
        );
        return send::ReplyPanel::card(build_user_account_menu_text())
            .send(actor.request_chat_id, client_id)
            .await;
    }

    send_menu_page(MenuPage::Home, actor, client_id).await
}

/// `/menu` inline keyboard 回调入口。
///
/// 菜单 callback 只处理自身页面切换和“开始输入”；下载/任务/帮助按钮会走各自模块的 callback。
pub async fn menu_callback_query(
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: Arc<BotConfig>,
    actor: crate::config::RequestActor,
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
            send::answer_callback_query(update.id, Some(page.title()), client_id).await?;
            let (text, rows) = match build_menu_page(page, actor).await {
                Ok(page) => page,
                Err(err) => {
                    send_menu_callback_error(update.chat_id, client_id, &err).await?;
                    return Err(err);
                }
            };
            let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
            if let Err(err) = send::edit_card_message_with_inline_keyboard(
                text,
                update.chat_id,
                update.message_id,
                keyboard,
                client_id,
            )
            .await
            {
                send_menu_callback_error(update.chat_id, client_id, &err).await?;
                return Err(err);
            }
            Ok(())
        }
        MenuRequestAction::NewTransfer => {
            start_input_prompt(
                update.id,
                update.chat_id,
                update.message_id,
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
                update.message_id,
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
                update.message_id,
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
                update.message_id,
                update.sender_user_id,
                client_id,
                MenuInputKind::LookupDefault,
            )
            .await
        }
        MenuRequestAction::TargetDefault => {
            input::target_default_callback_query(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                config,
                client_id,
            )
            .await
        }
        MenuRequestAction::TargetManual => {
            input::target_manual_callback_query(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                client_id,
            )
            .await
        }
        MenuRequestAction::TargetRequestChat => {
            input::target_request_chat_callback_query(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                client_id,
            )
            .await
        }
        MenuRequestAction::TargetAlias(target_chat_id) => {
            input::target_alias_callback_query(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                target_chat_id,
                config,
                client_id,
            )
            .await
        }
        MenuRequestAction::TargetConfirm => {
            input::target_confirm_callback_query(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                config,
                actor,
                client_id,
            )
            .await
        }
        MenuRequestAction::TargetBack => {
            input::target_back_callback_query(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                config,
                client_id,
            )
            .await
        }
        MenuRequestAction::JobIdInput(action) => {
            input::job_id_input_callback_query(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                action,
                client_id,
            )
            .await
        }
        MenuRequestAction::PointLedgerUserInput => {
            input::point_ledger_user_input_callback_query(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                actor,
                client_id,
            )
            .await
        }
        MenuRequestAction::ContinueInput => {
            send::answer_callback_query(update.id, Some("继续输入"), client_id).await?;
            let continued = input::continue_current_input(
                update.chat_id,
                update.sender_user_id,
                config,
                client_id,
            )
            .await?;
            if !continued {
                send::ReplyPanel::card(build_menu_status_text(
                    "没有未完成输入",
                    "empty",
                    "当前没有可继续的菜单输入，可重新开始转存或查询。",
                ))
                .row(vec![send::build_callback_button(
                    "首页",
                    &build_menu_home_callback_data(),
                    tdlib_rs::enums::ButtonStyle::Primary,
                )])
                .send(update.chat_id, client_id)
                .await?;
            }
            Ok(())
        }
        MenuRequestAction::CancelInput => {
            input::cancel_input_callback_query(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                client_id,
            )
            .await
        }
    }
}

/// 菜单按钮失败提示。
///
/// callback 已经先 ACK，失败时不能再 answer 同一个 callback，因此发送独立卡片帮助排查。
async fn send_menu_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_interaction_error_card(
        request_chat_id,
        client_id,
        "菜单刷新失败",
        "菜单未更新，请检查日志或复制错误信息。",
        err,
    )
    .await
}

/// 发送 ForceReply 输入提示，并记录对应输入流程。
async fn start_input_prompt(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
    kind: MenuInputKind,
) -> anyhow::Result<()> {
    input::start_menu_input(chat_id, sender_user_id, kind).await?;
    send::answer_callback_query(callback_query_id, Some("请输入源链接"), client_id).await?;
    edit_menu_waiting_card(
        chat_id,
        message_id,
        client_id,
        "1/3",
        "等待源链接",
        "请直接回复源链接，或点击取消结束当前向导。",
    )
    .await;
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text("1/3", kind.source_title(), kind.source_detail()),
        chat_id,
        "输入源链接，或发送 /cancel",
        client_id,
    )
    .await?;
    Ok(())
}

/// 把原菜单页收敛为等待态，避免用户继续点击旧入口造成多条并行草稿。
async fn edit_menu_waiting_card(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    step: &str,
    title: &str,
    detail: &str,
) {
    let Ok((text, keyboard)) = send::ReplyPanel::card(build_step_prompt_text(step, title, detail))
        .row(vec![
            send::build_callback_button(
                "取消",
                &callback::cancel_input_callback_data(),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
            send::build_callback_button(
                "首页",
                &build_menu_home_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .into_card_parts()
    else {
        tracing::warn!(chat_id, message_id, "build menu waiting card failed");
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
            "edit menu waiting card failed"
        );
    }
}

/// 从显式命令启动转存输入向导。
///
/// `/transfer` 不带参数时进入这里；如果用户是回复媒体消息，上层会优先走回复消息转存。
pub(super) async fn start_transfer_input_from_command(
    chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    input::start_menu_input(chat_id, sender_user_id, MenuInputKind::Transfer).await?;
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text(
            "1/3",
            MenuInputKind::Transfer.source_title(),
            MenuInputKind::Transfer.source_detail(),
        ),
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
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<bool> {
    input::handle_menu_input(
        text,
        config,
        request_chat_id,
        request_message_id,
        sender_user_id,
        actor,
        client_id,
    )
    .await
}

/// 处理 Telegram 原生选群结果。
pub async fn handle_menu_shared_chat_input(
    shared: &tdlib_rs::types::MessageChatShared,
    config: Arc<BotConfig>,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    input::handle_shared_chat_input(shared, config, request_chat_id, sender_user_id, client_id)
        .await
}

/// 丢弃当前聊天里的菜单输入草稿。
pub async fn discard_menu_input(request_chat_id: i64, sender_user_id: i64) -> anyhow::Result<bool> {
    input::cancel_menu_input(request_chat_id, sender_user_id).await
}

/// 当前用户发送新命令时丢弃菜单输入草稿，并在必要时清理 reply keyboard。
///
/// 命令优先级高于输入向导；如果旧向导停在原生选群阶段，只丢草稿会让客户端继续显示
/// “选择群组”键盘，所以这里把清理动作放在命令真正执行前完成。
pub async fn discard_menu_input_for_command(
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    let Some(cancelled) =
        input::cancel_menu_input_with_state(request_chat_id, sender_user_id).await?
    else {
        return Ok(false);
    };

    if cancelled.needs_reply_keyboard_cleanup {
        send::send_card_message_with_remove_keyboard(
            build_menu_status_text(
                "已切换命令",
                "keyboard-cleared",
                "已收起之前的选群键盘，继续执行当前命令。",
            ),
            request_chat_id,
            client_id,
        )
        .await?;
    }

    Ok(true)
}

/// 取消当前聊天里的菜单输入草稿，并给用户明确反馈。
pub async fn cancel_menu_input(
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    let Some(cancelled) =
        input::cancel_menu_input_with_state(request_chat_id, sender_user_id).await?
    else {
        return Ok(false);
    };

    let text = build_menu_status_text(
        "已取消",
        "cancelled",
        "当前菜单输入已取消，可重新打开 /menu。",
    );

    if cancelled.needs_reply_keyboard_cleanup {
        send::send_card_message_with_remove_keyboard(text, request_chat_id, client_id).await?;
        return Ok(true);
    }

    send::ReplyPanel::card(text)
        .row(vec![send::build_copy_button(
            "复制 /menu",
            "/menu",
            tdlib_rs::enums::ButtonStyle::Primary,
        )])
        .send(request_chat_id, client_id)
        .await?;
    Ok(true)
}

/// 发送一个菜单页。
async fn send_menu_page(
    page: MenuPage,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, rows) = build_menu_page(page, actor).await?;
    send::ReplyPanel::card(text)
        .rows(rows)
        .send(actor.request_chat_id, client_id)
        .await
}

/// 构造一个菜单页的正文和按钮。
///
/// 首页需要读取最近任务作为快捷入口；配置页直接复用 `/cfg` 的实时配置卡片，
/// 这样 `/m -> 配置` 和 `/cfg` 展示的是同一套运行参数。
async fn build_menu_page(
    page: MenuPage,
    actor: crate::config::RequestActor,
) -> anyhow::Result<(String, Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>)> {
    let (recent_jobs, health, draft_summary) = if page == MenuPage::Home {
        let recent_jobs = store::list_recent_job_snapshots_for_actor(
            crate::app_context::app_context().as_ref(),
            actor,
            5,
        )
        .await?;
        let health = if actor.is_admin() {
            Some(
                store::list_transfer_health_snapshot(crate::app_context::app_context().as_ref())
                    .await?,
            )
        } else {
            None
        };
        let draft_summary =
            input::current_draft_summary(actor.request_chat_id, actor.user_id).await?;
        (recent_jobs, health, draft_summary)
    } else {
        (Vec::new(), None, None)
    };
    let text = if page == MenuPage::Config {
        config_cmd::format_current_transfer_config_text("当前可调配置")
    } else if page == MenuPage::Home {
        build_menu_home_text(&MenuHomeSummary {
            active_jobs: health.as_ref().map_or(0, |health| health.active_jobs),
            failed_jobs: health.as_ref().map_or(0, |health| health.failed_jobs),
            recoverable_jobs: health.as_ref().map_or(0, |health| health.recoverable_jobs),
            due_cache_files: health
                .as_ref()
                .map_or(0, |health| health.file_cache_due_rows),
            failed_cache_files: health
                .as_ref()
                .map_or(0, |health| health.file_cache_failed_rows),
            recent_jobs: recent_jobs.len(),
            pending_input: draft_summary.as_ref().map(|draft| draft.title),
            is_admin: actor.is_admin(),
        })
    } else {
        build_menu_text(page)
    };
    Ok((
        text,
        build_menu_buttons(page, &recent_jobs, actor.is_admin(), draft_summary.as_ref()),
    ))
}
