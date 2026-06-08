// tgbot 模块入口：
// - 接收 TDLib update
// - 分发授权状态 / 命令消息
// - 委托 transfer 命令处理逻辑

mod error;
mod login;
mod queue;
pub mod send;
pub mod transfer;

use crate::tgbot;
use crate::tgbot::transfer::card;
use base64::{Engine as _, engine::general_purpose};
pub use error::*;
pub use login::*;
use std::time::SystemTime;
use tdlib_rs::enums::Update;

// 记录进程启动时间戳。
// 用于过滤掉程序启动前的历史消息，避免重复处理。
static START_TS: std::sync::LazyLock<i32> = std::sync::LazyLock::new(|| {
    let secs = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();
    i32::try_from(secs).expect("i32 overflow (Year 2038 problem)")
});

// 创建 TDLib client id。
pub async fn create_client() -> anyhow::Result<i32> {
    Ok(tdlib_rs::create_client())
}

// 读取 TDLib 运行时版本（诊断信息）。
pub async fn get_version(client_id: i32) -> anyhow::Result<()> {
    let version = tdlib_rs::functions::get_option("version".to_string(), client_id).await;
    match version {
        Ok(version) => {
            tracing::info!(version = ?version, "tdlib version loaded");
            Ok(())
        }
        Err(err) => anyhow::bail!("get_version failed, error={:?}", err),
    }
}

// 设置 TDLib 日志级别。
pub async fn set_log(client_id: i32) {
    match tdlib_rs::functions::set_log_verbosity_level(1, client_id).await {
        Ok(_) => tracing::debug!(client_id, verbosity_level = 1, "tdlib log level configured"),
        Err(err) => {
            tracing::warn!(client_id, error = ?err, "configure tdlib log level failed");
        }
    }
}

// 主循环：持续接收 TDLib update 并异步处理。
pub async fn receive(config: std::sync::Arc<crate::config::BotConfig>) -> anyhow::Result<()> {
    loop {
        let receive = tokio::task::spawn_blocking(tdlib_rs::receive).await?;
        match receive {
            None => {
                // TDLib receive 超时返回 None 是正常空轮询；放在 trace，避免默认日志刷屏。
                tracing::trace!("tdlib receive returned no update");
            }
            Some((msg_update, _client_id)) => {
                tracing::trace!(
                    update_kind = update_kind(&msg_update),
                    "tdlib update received"
                );
                let config = config.clone();
                tokio::spawn(async move {
                    let res = handle_update(msg_update, config.clone()).await;
                    if let Err(err) = res {
                        tracing::error!(error = %err, "handle tdlib update failed");
                    }
                });
            }
        }
    }
}

// update 分发器：
// - AuthorizationState => 登录状态机
// - NewMessage(text command) => 命令路由
// - NewCallbackQuery => inline keyboard 回调
// - File => 进度快照
pub async fn handle_update(
    update: Update,
    config: std::sync::Arc<crate::config::BotConfig>,
) -> anyhow::Result<()> {
    // 授权状态更新：交给登录处理逻辑。
    if let Update::AuthorizationState(update) = update {
        handle_authorization(update.authorization_state, config).await?;
        return Ok(());
    }

    // 发送成功/失败更新：用于把 sendMessage 返回的临时 message_id 对齐到最终 message_id。
    if let Update::MessageSendSucceeded(update_send_succeeded) = update {
        tracing::debug!(
            chat_id = update_send_succeeded.message.chat_id,
            old_message_id = update_send_succeeded.old_message_id,
            final_message_id = update_send_succeeded.message.id,
            "tdlib message send succeeded"
        );
        crate::tgbot::send::observe_message_send_succeeded(update_send_succeeded);
        return Ok(());
    }
    if let Update::MessageSendFailed(update_send_failed) = update {
        tracing::warn!(
            chat_id = update_send_failed.message.chat_id,
            old_message_id = update_send_failed.old_message_id,
            failed_message_id = update_send_failed.message.id,
            error_code = update_send_failed.error.code,
            error_message = %update_send_failed.error.message,
            "tdlib message send failed"
        );
        crate::tgbot::send::observe_message_send_failed(update_send_failed);
        return Ok(());
    }

    // 新消息更新：执行命令分发。
    if let Update::NewMessage(update_new_message) = update {
        let message = update_new_message.message;
        let chat_id = message.chat_id;

        // 忽略进程启动前消息，避免重复处理。
        if message.date < *START_TS {
            tracing::debug!(
                chat_id,
                message_id = message.id,
                message_date = message.date,
                start_ts = *START_TS,
                "ignored historical message"
            );
            return Ok(());
        }

        // 解析发送者 ID，用于管理员白名单校验。
        let sender_id = match &message.sender_id {
            tdlib_rs::enums::MessageSender::User(user) => user.user_id,
            tdlib_rs::enums::MessageSender::Chat(chat_id) => chat_id.chat_id,
        };

        // 仅允许管理员 chat 且发送者也在管理员列表中。
        if !(config.admin_ids.contains(&chat_id) && config.admin_ids.contains(&sender_id)) {
            tracing::debug!(
                chat_id,
                sender_id,
                message_id = message.id,
                chat_allowed = config.admin_ids.contains(&chat_id),
                sender_allowed = config.admin_ids.contains(&sender_id),
                "ignored non-admin message"
            );
            return Ok(());
        }

        let message_content = message.content;
        let client_id = config.client_id;

        // 当前仅处理文本消息。
        if let tdlib_rs::enums::MessageContent::MessageText(message_text) = message_content {
            tracing::debug!(
                chat_id,
                sender_id,
                message_id = message.id,
                "admin text message received"
            );
            let raw_text = message_text.text.text;
            let text = raw_text.split_whitespace().collect::<Vec<&str>>();
            if text.is_empty() {
                tracing::debug!(
                    chat_id,
                    sender_id,
                    message_id = message.id,
                    "ignored empty admin text message"
                );
                if let Some(client_id) = client_id {
                    crate::tgbot::send::send_text_message(
                        "未收到文本内容。".to_owned(),
                        chat_id,
                        client_id,
                    )
                    .await?;
                }
                return Ok(());
            }

            if text[0] == "/cancel" {
                let client_id = client_id.ok_or_else(|| anyhow::anyhow!("not found client_id"))?;
                if tgbot::transfer::cancel_menu_input(chat_id, sender_id, client_id).await? {
                    return Ok(());
                }
            }

            if text[0].starts_with("/") {
                let client_id = client_id.ok_or_else(|| anyhow::anyhow!("not found client_id"))?;
                let command = text[0];
                if command != "/cancel" && tgbot::transfer::discard_menu_input(chat_id, sender_id) {
                    tracing::debug!(
                        command,
                        chat_id,
                        sender_id,
                        message_id = message.id,
                        "discarded pending menu input because command has priority"
                    );
                }
                // 只记录命令名和消息定位信息，不记录参数中的链接，避免日志暴露私有消息入口。
                tracing::info!(
                    command,
                    chat_id,
                    sender_id,
                    message_id = message.id,
                    "admin command received"
                );

                let command_result = match command {
                    // /help 命令入口。
                    // 返回机器人当前支持的命令说明。
                    "/help" | "/h" => tgbot::transfer::help_command(text, chat_id, client_id).await,
                    // /transfer 命令入口。
                    "/transfer" | "/t" => {
                        // request_message_id 用于请求级幂等（防止同一条指令重复建任务）。
                        tgbot::transfer::transfer_command(
                            text,
                            config.clone(),
                            chat_id,
                            message.id,
                            client_id,
                        )
                        .await
                    }
                    // /lookup 命令入口。
                    // 按源链接查找历史转存结果。
                    "/lookup" | "/lk" => {
                        tgbot::transfer::lookup_command(text, config.clone(), chat_id, client_id)
                            .await
                    }
                    // /config 命令入口。
                    // 仅开放运行时安全可调的配置项。
                    "/config" | "/cfg" => {
                        tgbot::transfer::config_command(text, chat_id, client_id).await
                    }
                    // /downloads 命令入口。
                    // 展示当前聊天最近的转存任务进度列表。
                    "/downloads" | "/d" => {
                        tgbot::transfer::downloads_command(text, chat_id, client_id).await
                    }
                    // /job 命令入口。
                    // 手动暂停、恢复、停止指定转存任务。
                    "/job" | "/j" => tgbot::transfer::job_command(text, chat_id, client_id).await,
                    // /menu 命令入口。
                    // TDLib 的 reply_markup 只支持 bot 账号；用户号登录时改走纯文本菜单，避免提示“点按钮”但实际没有按钮。
                    "/menu" | "/m" => {
                        let supports_reply_markup =
                            matches!(config.login_info, crate::config::LoginInfo::Token(_));
                        tgbot::transfer::menu_command(
                            text,
                            chat_id,
                            client_id,
                            supports_reply_markup,
                        )
                        .await
                    }
                    _ => {
                        tracing::warn!(
                            command,
                            chat_id,
                            sender_id,
                            message_id = message.id,
                            "unknown admin command"
                        );
                        send_unknown_command_message(command, chat_id, client_id).await
                    }
                };

                if let Err(err) = command_result {
                    tracing::warn!(
                        command,
                        chat_id,
                        sender_id,
                        message_id = message.id,
                        error = %err,
                        "admin command failed"
                    );
                    send_command_error_message(command, &err, chat_id, client_id).await?;
                } else {
                    tracing::debug!(
                        command,
                        chat_id,
                        sender_id,
                        message_id = message.id,
                        "admin command completed"
                    );
                }
            } else if let Some(client_id) = client_id
                && tgbot::transfer::handle_menu_text_input(
                    raw_text.as_str(),
                    config.clone(),
                    chat_id,
                    message.id,
                    sender_id,
                    client_id,
                )
                .await?
            {
                tracing::debug!(
                    chat_id,
                    sender_id,
                    message_id = message.id,
                    "admin text message consumed by menu input"
                );
                return Ok(());
            } else {
                tracing::debug!(
                    chat_id,
                    sender_id,
                    message_id = message.id,
                    "admin text message ignored because it is not a command and no menu input is active"
                );
            }
        } else {
            tracing::debug!(
                chat_id,
                sender_id,
                message_id = message.id,
                content_kind = message_content_kind(&message_content),
                "ignored non-text admin message"
            );
        }
        return Ok(());
    }

    // inline keyboard 回调：用于 `/downloads` 分页和 `/job` 原地控制。
    if let Update::NewCallbackQuery(mut update_callback_query) = update {
        decode_callback_query_payload(&mut update_callback_query);

        // 只接受管理员在管理员聊天里点击的按钮。
        if !(config.admin_ids.contains(&update_callback_query.chat_id)
            && config
                .admin_ids
                .contains(&update_callback_query.sender_user_id))
        {
            tracing::debug!(
                chat_id = update_callback_query.chat_id,
                sender_user_id = update_callback_query.sender_user_id,
                message_id = update_callback_query.message_id,
                chat_allowed = config.admin_ids.contains(&update_callback_query.chat_id),
                sender_allowed = config
                    .admin_ids
                    .contains(&update_callback_query.sender_user_id),
                "ignored non-admin callback query"
            );
            return Ok(());
        }

        let client_id = config
            .client_id
            .ok_or_else(|| anyhow::anyhow!("not found client_id"))?;
        tracing::debug!(
            chat_id = update_callback_query.chat_id,
            sender_user_id = update_callback_query.sender_user_id,
            "admin callback query received"
        );
        tgbot::transfer::transfer_callback_query(update_callback_query, client_id).await?;
        return Ok(());
    }

    // 文件更新只写入进度快照；完整 File 结构很大且包含本地路径，不直接打日志。
    if let Update::File(update_file) = update {
        // 将 TDLib 实时文件进度写入内存快照，供 `/downloads` 查询。
        queue::update_download_progress(&update_file.file);
        tracing::trace!(
            file_id = update_file.file.id,
            downloaded_size = update_file.file.local.downloaded_size,
            size = update_file.file.size,
            expected_size = update_file.file.expected_size,
            is_downloading_active = update_file.file.local.is_downloading_active,
            is_downloading_completed = update_file.file.local.is_downloading_completed,
            "tdlib file progress updated"
        );
    }

    Ok(())
}

/// 解码 TDLib callback payload。
///
/// TDLib schema 里的 callback `data` 是 bytes：
/// - 发送 JSON 请求时必须写 base64。
/// - 收到 update 时 TDLib 也会把 bytes 表示为 base64。
///
/// 业务路由只认识 `m:home`、`d:r:all:8:1` 这类短字符串，因此入口统一解码。
fn decode_callback_query_payload(update: &mut tdlib_rs::enums::UpdateNewCallbackQuery) {
    if let tdlib_rs::enums::CallbackQueryPayload::Data(data) = &mut update.payload {
        match general_purpose::STANDARD.decode(&data.data) {
            Ok(decoded) => match String::from_utf8(decoded) {
                Ok(decoded_text) => {
                    tracing::debug!(
                        chat_id = update.chat_id,
                        sender_user_id = update.sender_user_id,
                        message_id = update.message_id,
                        "callback payload decoded"
                    );
                    data.data = decoded_text;
                }
                Err(err) => {
                    tracing::warn!(
                        chat_id = update.chat_id,
                        sender_user_id = update.sender_user_id,
                        message_id = update.message_id,
                        error = %err,
                        "callback payload is not valid utf8"
                    );
                }
            },
            Err(err) => {
                // 兼容历史测试或未来 TDLib 绑定变更：如果拿到的已经是明文 payload，不强制失败。
                tracing::debug!(
                    chat_id = update.chat_id,
                    sender_user_id = update.sender_user_id,
                    message_id = update.message_id,
                    error = %err,
                    "callback payload is not base64, keep original"
                );
            }
        }
    }
}

/// 返回 TDLib update 的粗粒度类型名。
///
/// trace 日志只需要知道 update 是否进入机器人，不打印完整 update，避免泄露消息内容。
fn update_kind(update: &Update) -> &'static str {
    match update {
        Update::AuthorizationState(_) => "authorization_state",
        Update::NewMessage(_) => "new_message",
        Update::NewCallbackQuery(_) => "new_callback_query",
        Update::MessageSendAcknowledged(_) => "message_send_acknowledged",
        Update::MessageSendSucceeded(_) => "message_send_succeeded",
        Update::MessageSendFailed(_) => "message_send_failed",
        Update::File(_) => "file",
        _ => "other",
    }
}

/// 返回消息内容类型名，用于 debug 排查“为什么消息没有被当成命令处理”。
fn message_content_kind(content: &tdlib_rs::enums::MessageContent) -> &'static str {
    match content {
        tdlib_rs::enums::MessageContent::MessageText(_) => "text",
        tdlib_rs::enums::MessageContent::MessageAnimation(_) => "animation",
        tdlib_rs::enums::MessageContent::MessageAudio(_) => "audio",
        tdlib_rs::enums::MessageContent::MessageDocument(_) => "document",
        tdlib_rs::enums::MessageContent::MessagePhoto(_) => "photo",
        tdlib_rs::enums::MessageContent::MessageVideo(_) => "video",
        tdlib_rs::enums::MessageContent::MessageVideoNote(_) => "video_note",
        tdlib_rs::enums::MessageContent::MessageVoiceNote(_) => "voice_note",
        _ => "other",
    }
}

/// 回复未知命令，避免用户输入错误时只在日志里可见。
async fn send_unknown_command_message(
    command: &str,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    crate::tgbot::send::ReplyPanel::card(
        [
            "未知命令".to_owned(),
            format!("状态：{}", card::code("invalid-command")),
            card::DIVIDER.to_owned(),
            card::section("输入"),
            card::command_line("命令", command),
            card::section("下一步"),
            card::command_line("帮助", "/h"),
        ]
        .join("\n"),
    )
    .row(vec![crate::tgbot::send::build_copy_button(
        "复制 /h",
        "/h",
        tdlib_rs::enums::ButtonStyle::Primary,
    )])
    .send(chat_id, client_id)
    .await
}

/// 回复命令执行错误。
///
/// 命令处理失败大多是参数错误或当前任务状态不允许操作；这里给用户明确反馈，
/// 同时保留可复制错误详情，避免问题只出现在日志中。
async fn send_command_error_message(
    command: &str,
    err: &anyhow::Error,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    crate::tgbot::send::ReplyPanel::card(
        [
            "命令执行失败".to_owned(),
            format!("状态：{}", card::code("failed")),
            card::DIVIDER.to_owned(),
            card::section("输入"),
            card::command_line("命令", command),
            card::section("错误"),
            card::pre_code(format!("{:#}", err)),
            card::section("下一步"),
            card::command_line("帮助", "/h"),
        ]
        .join("\n"),
    )
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制 /h",
            "/h",
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制错误",
            &format!("{:#}", err),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(chat_id, client_id)
    .await
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose};

    use super::decode_callback_query_payload;

    // TDLib JSON 协议会用 base64 表示 callback bytes；入口应解回业务短 payload。
    #[test]
    fn test_decode_callback_query_payload() {
        let mut update = tdlib_rs::enums::UpdateNewCallbackQuery {
            id: 1,
            sender_user_id: 2,
            chat_id: 3,
            message_id: 4,
            chat_instance: 5,
            payload: tdlib_rs::enums::CallbackQueryPayload::Data(
                tdlib_rs::types::CallbackQueryPayloadData {
                    data: general_purpose::STANDARD.encode("m:home"),
                },
            ),
        };

        decode_callback_query_payload(&mut update);

        let tdlib_rs::enums::CallbackQueryPayload::Data(data) = update.payload else {
            panic!("payload must be data");
        };
        assert_eq!(data.data, "m:home");
    }

    // 兼容已有测试构造的明文 payload，避免单元测试和未来绑定差异直接崩掉。
    #[test]
    fn test_decode_callback_query_payload_keeps_plain_text() {
        let mut update = tdlib_rs::enums::UpdateNewCallbackQuery {
            id: 1,
            sender_user_id: 2,
            chat_id: 3,
            message_id: 4,
            chat_instance: 5,
            payload: tdlib_rs::enums::CallbackQueryPayload::Data(
                tdlib_rs::types::CallbackQueryPayloadData {
                    data: "m:home".to_owned(),
                },
            ),
        };

        decode_callback_query_payload(&mut update);

        let tdlib_rs::enums::CallbackQueryPayload::Data(data) = update.payload else {
            panic!("payload must be data");
        };
        assert_eq!(data.data, "m:home");
    }
}
