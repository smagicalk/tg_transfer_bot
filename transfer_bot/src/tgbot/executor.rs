//! 用户执行器的 owner 专属控制面板。
//!
//! Bot 是默认执行端；本模块只负责按需创建 user TDLib client，并把登录状态
//! 显示在 Bot 私聊中。登录凭据与二维码链接均不落盘、不写日志。

use std::sync::Arc;

use crate::app_context::{ExecutorIdentity, ExecutorPhase, ExecutorRuntimeState};
use crate::config::{BotConfig, ClientRole, RequestActor};
use crate::tgbot::send;

const EXECUTOR_CALLBACK_PREFIX: &str = "ex:";

pub(crate) fn is_executor_callback_data(data: &str) -> bool {
    data.starts_with(EXECUTOR_CALLBACK_PREFIX)
}

pub(crate) fn build_executor_panel_callback_data() -> String {
    format!("{EXECUTOR_CALLBACK_PREFIX}open")
}

fn build_executor_callback_data(action: &str) -> String {
    format!("{EXECUTOR_CALLBACK_PREFIX}{action}")
}

/// 从 user TDLib 收到二维码链接后生成图片并发送给 owner。
pub(crate) async fn send_qr_code_to_owner(
    app: &crate::app_context::AppContext,
    user_client_id: i32,
    qr_link: String,
    bot_client_id: i32,
) -> anyhow::Result<()> {
    let Some(owner_chat_id) = app.executor_runtime.owner_chat_id() else {
        anyhow::bail!("executor login owner chat is not available");
    };
    if app.executor_runtime.user_client_id() != Some(user_client_id) {
        anyhow::bail!("executor QR belongs to an inactive client");
    }

    let code = qrcode::QrCode::with_error_correction_level(qr_link.as_bytes(), qrcode::EcLevel::Q)?;
    let image = code.render::<image::Luma<u8>>().quiet_zone(true).build();
    let path =
        std::env::temp_dir().join(format!("tg-transfer-bot-executor-qr-{user_client_id}.png"));
    image.save(&path)?;
    let caption = "请在 Telegram 已登录设备中扫描此二维码登录执行器。二维码会自动失效。";
    if let Some(message_id) = app.executor_runtime.qr_message_id() {
        send::edit_local_photo(
            &path.to_string_lossy(),
            caption,
            owner_chat_id,
            message_id,
            bot_client_id,
        )
        .await?;
    } else {
        let receipt = send::send_local_photo_returning(
            &path.to_string_lossy(),
            caption,
            owner_chat_id,
            bot_client_id,
        )
        .await?;
        app.executor_runtime.replace_qr_message_id(receipt.id);
    }
    if let Some(old_path) = app.executor_runtime.replace_qr_image_path(path.clone())
        && old_path != path
    {
        let _ = std::fs::remove_file(old_path);
    }
    Ok(())
}

/// 读取已登录执行器的账号摘要。面板仅展示 ID、名称和用户名，不保存手机号。
pub(crate) async fn refresh_executor_identity(
    app: &crate::app_context::AppContext,
    user_client_id: i32,
) -> anyhow::Result<()> {
    let tdlib_rs::enums::User::User(user) = tdlib_rs::functions::get_me(user_client_id)
        .await
        .map_err(|error| anyhow::Error::new(crate::tgbot::TdError(error)))?;
    let display_name = [user.first_name.trim(), user.last_name.trim()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let username = user
        .usernames
        .as_ref()
        .and_then(|names| names.active_usernames.first())
        .cloned()
        .filter(|name| !name.trim().is_empty());
    let display_name = if display_name.is_empty() {
        username.clone().unwrap_or_else(|| "未设置名称".to_owned())
    } else {
        display_name
    };
    let identity = ExecutorIdentity {
        user_id: user.id,
        display_name,
        username,
    };
    if app
        .executor_runtime
        .set_identity_if_ready(user_client_id, identity)
    {
        tracing::info!(user_client_id, "executor account identity refreshed");
    }
    Ok(())
}

/// user TDLib 要求二次验证密码时，使用 Bot 私聊 ForceReply 接收密码。
pub(crate) async fn request_two_factor_password(
    app: &crate::app_context::AppContext,
    user_client_id: i32,
    password_hint: &str,
    bot_client_id: i32,
) -> anyhow::Result<()> {
    if !app.executor_runtime.set_waiting_password(user_client_id) {
        return Ok(());
    }
    let Some(owner_chat_id) = app.executor_runtime.owner_chat_id() else {
        anyhow::bail!("executor login owner chat is not available");
    };
    let hint = if password_hint.trim().is_empty() {
        "无".to_owned()
    } else {
        password_hint.to_owned()
    };
    let prompt = send::send_card_message_with_force_reply_returning(
        format!(
            "执行器登录\n\n需要二次验证密码。\n提示：{hint}\n\n回复本消息输入密码；密码消息会在提交后删除。"
        ),
        owner_chat_id,
        "输入二次验证密码",
        bot_client_id,
    )
    .await?;
    if let Some(previous_prompt_id) = app
        .executor_runtime
        .replace_password_prompt_message_id(prompt.id)
    {
        let _ = send::delete_message(owner_chat_id, previous_prompt_id, bot_client_id).await;
    }
    Ok(())
}

/// 提交二次验证密码。调用方负责在成功或失败后删除 owner 发出的密码消息。
pub(crate) async fn submit_two_factor_password(
    app: &crate::app_context::AppContext,
    sender_user_id: i64,
    reply_message_id: Option<i64>,
    password: String,
) -> anyhow::Result<bool> {
    if app.executor_runtime.phase() != ExecutorPhase::WaitingPassword {
        return Ok(false);
    }
    let Some(client_id) = app.executor_runtime.user_client_id() else {
        return Ok(false);
    };
    if app.executor_runtime.owner_chat_id() != Some(sender_user_id) {
        return Ok(false);
    }
    if app.executor_runtime.password_prompt_message_id() != reply_message_id {
        return Ok(false);
    }
    tdlib_rs::functions::check_authentication_password(password, client_id)
        .await
        .map_err(|error| {
            anyhow::anyhow!("executor password verification failed: {}", error.message)
        })?;
    Ok(true)
}

pub(crate) async fn executor_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: Arc<BotConfig>,
    actor: RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let tdlib_rs::enums::CallbackQueryPayload::Data(data) = update.payload else {
        send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
        return Ok(());
    };
    if actor.user_id != config.owner_user_id {
        send::answer_callback_query(update.id, Some("仅 owner 可管理执行器"), client_id).await?;
        return Ok(());
    }

    match data.data.as_str() {
        "ex:open" => {
            send::answer_callback_query(update.id, Some("执行器"), client_id).await?;
            edit_executor_panel(app, update.chat_id, update.message_id, client_id).await
        }
        "ex:login" => {
            if app.executor_runtime.phase() != ExecutorPhase::Offline {
                send::answer_callback_query(update.id, Some("执行器已在登录或在线"), client_id)
                    .await?;
                return Ok(());
            }
            let user_client_id = create_user_client(config.as_ref()).await?;
            app.executor_runtime
                .begin_login(user_client_id, update.chat_id);
            send::answer_callback_query(update.id, Some("正在申请二维码"), client_id).await?;
            edit_executor_panel(app, update.chat_id, update.message_id, client_id).await
        }
        "ex:logout" => {
            let Some(user_client_id) = app.executor_runtime.user_client_id() else {
                send::answer_callback_query(update.id, Some("执行器未登录"), client_id).await?;
                return Ok(());
            };
            if !app.executor_runtime.begin_draining(user_client_id) {
                send::answer_callback_query(update.id, Some("执行器当前不能退出"), client_id)
                    .await?;
                return Ok(());
            }
            app.transfer_runtime.begin_transfer_drain();
            spawn_executor_logout_after_drain(
                app.transfer_runtime.clone(),
                app.executor_runtime.clone(),
                user_client_id,
            );
            send::answer_callback_query(update.id, Some("等待现有任务结束"), client_id).await?;
            edit_executor_panel(app, update.chat_id, update.message_id, client_id).await
        }
        "ex:cancel" => {
            let Some(user_client_id) = app.executor_runtime.user_client_id() else {
                send::answer_callback_query(update.id, Some("执行器未登录"), client_id).await?;
                return Ok(());
            };
            if !app.executor_runtime.cancel_draining(user_client_id) {
                send::answer_callback_query(update.id, Some("当前没有可取消的退出操作"), client_id)
                    .await?;
                return Ok(());
            }
            app.transfer_runtime.cancel_transfer_drain();
            send::answer_callback_query(update.id, Some("已继续接收新任务"), client_id).await?;
            edit_executor_panel(app, update.chat_id, update.message_id, client_id).await
        }
        _ => {
            send::answer_callback_query(update.id, Some("执行器按钮参数无效"), client_id).await?;
            Ok(())
        }
    }
}

fn spawn_executor_logout_after_drain(
    transfer_runtime: Arc<crate::app_context::TransferRuntimeState>,
    executor_runtime: Arc<ExecutorRuntimeState>,
    user_client_id: i32,
) {
    tokio::spawn(async move {
        transfer_runtime.wait_for_transfer_drain().await;
        if !executor_runtime.mark_logging_out(user_client_id) {
            return;
        }
        if let Err(error) = tdlib_rs::functions::log_out(user_client_id).await {
            tracing::error!(
                user_client_id,
                error_code = error.code,
                error_message = %error.message,
                "executor logout failed after transfer drain"
            );
            if executor_runtime.restore_ready_after_logout_failure(user_client_id) {
                transfer_runtime.cancel_transfer_drain();
            }
        }
    });
}

async fn create_user_client(config: &BotConfig) -> anyhow::Result<i32> {
    let runtime = config.runtime_client(ClientRole::User)?.clone();
    let client_id = crate::tgbot::create_client().await?;
    let log_level = runtime.log_verbosity_level;
    tokio::spawn(async move {
        crate::tgbot::set_log(client_id, log_level).await;
    });
    tokio::spawn(async move {
        if let Err(error) = crate::tgbot::get_version(client_id).await {
            tracing::warn!(client_id, error = %error, "load executor tdlib version failed");
        }
    });
    tracing::info!(client_id, "executor user tdlib client created");
    Ok(client_id)
}

async fn edit_executor_panel(
    app: &crate::app_context::AppContext,
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, rows) = build_executor_panel(app.executor_runtime.as_ref());
    let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        "执行器页面更新失败",
        "执行器状态已变更；请返回管理菜单重新打开。",
    )
    .await
}

fn build_executor_panel(
    state: &ExecutorRuntimeState,
) -> (String, Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>) {
    let (status, detail, action) = match state.phase() {
        ExecutorPhase::Offline => (
            "未登录",
            "Bot 正常可用；需要回退时再登录执行器。",
            "登录执行器",
        ),
        ExecutorPhase::Starting => ("正在启动", "正在初始化执行器并申请二维码。", "刷新"),
        ExecutorPhase::WaitingQr => (
            "等待扫码",
            "二维码已发送到本私聊，请使用已登录设备扫描。",
            "刷新",
        ),
        ExecutorPhase::WaitingPassword => ("等待二次验证", "请回复密码输入提示完成登录。", "刷新"),
        ExecutorPhase::Ready => (
            "已登录",
            "Bot 默认执行；执行器仅在需要时自动回退使用。",
            "退出执行器",
        ),
        ExecutorPhase::Draining => (
            "等待任务结束",
            "已停止接收新任务；现有任务仍可暂停、恢复或停止。",
            "取消退出",
        ),
        ExecutorPhase::LoggingOut => ("正在退出", "等待 TDLib 清理本地执行器会话。", "刷新"),
    };
    let mut rows = Vec::new();
    match state.phase() {
        ExecutorPhase::Offline => rows.push(vec![send::build_callback_button(
            action,
            &build_executor_callback_data("login"),
            tdlib_rs::enums::ButtonStyle::Primary,
        )]),
        ExecutorPhase::Ready => rows.push(vec![send::build_callback_button(
            action,
            &build_executor_callback_data("logout"),
            tdlib_rs::enums::ButtonStyle::Danger,
        )]),
        ExecutorPhase::Draining => rows.push(vec![send::build_callback_button(
            action,
            &build_executor_callback_data("cancel"),
            tdlib_rs::enums::ButtonStyle::Default,
        )]),
        _ => rows.push(vec![send::build_callback_button(
            action,
            &build_executor_panel_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        )]),
    }
    rows.push(vec![send::build_callback_button(
        "返回管理",
        "m:mh",
        tdlib_rs::enums::ButtonStyle::Default,
    )]);
    let identity = state.identity().map(|identity| {
        let username = identity
            .username
            .as_deref()
            .map(|username| format!("\n用户名：@{username}"))
            .unwrap_or_default();
        format!(
            "\n\n账号\nID：{}\n名称：{}{}",
            identity.user_id, identity.display_name, username
        )
    });
    (
        format!(
            "执行器\n\n状态：{status}\n说明：{detail}{}",
            identity.unwrap_or_default()
        ),
        rows,
    )
}

/// 把菜单中的 owner 专属入口转换为执行器面板按钮。
pub(crate) fn build_executor_panel_button() -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        "执行器",
        &build_executor_panel_callback_data(),
        tdlib_rs::enums::ButtonStyle::Primary,
    )
}

#[cfg(test)]
mod tests {
    use super::build_executor_panel;
    use crate::app_context::{ExecutorIdentity, ExecutorRuntimeState};

    #[test]
    fn executor_panel_shows_non_sensitive_account_identity_after_login() {
        let state = ExecutorRuntimeState::default();
        state.begin_login(71, 1001);
        assert!(state.mark_ready(71));
        assert!(state.set_identity_if_ready(
            71,
            ExecutorIdentity {
                user_id: 2002,
                display_name: "测试账号".to_owned(),
                username: Some("tester".to_owned()),
            },
        ));

        let (text, _) = build_executor_panel(&state);

        assert!(text.contains("状态：已登录"));
        assert!(text.contains("ID：2002"));
        assert!(text.contains("名称：测试账号"));
        assert!(text.contains("用户名：@tester"));
        assert!(!text.contains("手机号"));
    }
}
