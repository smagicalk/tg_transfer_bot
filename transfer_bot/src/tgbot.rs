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
use std::collections::BTreeSet;
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
pub async fn set_log(client_id: i32, verbosity_level: i32) {
    match tdlib_rs::functions::set_log_verbosity_level(verbosity_level, client_id).await {
        Ok(_) => tracing::debug!(client_id, verbosity_level, "tdlib log level configured"),
        Err(err) => {
            tracing::warn!(client_id, error = ?err, "configure tdlib log level failed");
        }
    }
}

// 主循环：持续接收 TDLib update 并异步处理。
pub async fn receive(config: std::sync::Arc<crate::config::BotConfig>) -> anyhow::Result<()> {
    let ready_roles = std::sync::Arc::new(tokio::sync::Mutex::new(BTreeSet::new()));
    loop {
        let receive = tokio::task::spawn_blocking(tdlib_rs::receive).await?;
        match receive {
            None => {
                // TDLib receive 超时返回 None 是正常空轮询；放在 trace，避免默认日志刷屏。
                tracing::trace!("tdlib receive returned no update");
            }
            Some((msg_update, client_id)) => {
                tracing::trace!(
                    client_id,
                    update_kind = update_kind(&msg_update),
                    "tdlib update received"
                );
                let config = config.clone();
                let ready_roles = ready_roles.clone();
                tokio::spawn(async move {
                    let res =
                        handle_update(msg_update, client_id, config.clone(), ready_roles).await;
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
    client_id: i32,
    config: std::sync::Arc<crate::config::BotConfig>,
    ready_roles: std::sync::Arc<tokio::sync::Mutex<BTreeSet<crate::config::ClientRole>>>,
) -> anyhow::Result<()> {
    let Some(role) = config.client_ids.role_for_client_id(client_id) else {
        tracing::warn!(client_id, "ignored update from unknown tdlib client");
        return Ok(());
    };

    // 授权状态更新：交给登录处理逻辑。
    if let Update::AuthorizationState(update) = update {
        handle_authorization(
            update.authorization_state,
            role,
            client_id,
            config,
            ready_roles,
        )
        .await?;
        return Ok(());
    }

    // 非交互 client 的 update 只用于登录和文件进度，不能处理命令或 callback。
    let is_interaction_client = role == config.workflow.interaction_client;

    // 发送成功/失败更新：用于把 sendMessage 返回的临时 message_id 对齐到最终 message_id。
    if let Update::MessageSendSucceeded(update_send_succeeded) = update {
        tracing::debug!(
            chat_id = update_send_succeeded.message.chat_id,
            old_message_id = update_send_succeeded.old_message_id,
            final_message_id = update_send_succeeded.message.id,
            "tdlib message send succeeded"
        );
        crate::tgbot::send::observe_message_send_succeeded_for_client(
            update_send_succeeded,
            client_id,
        );
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
        crate::tgbot::send::observe_message_send_failed_for_client(update_send_failed, client_id);
        return Ok(());
    }

    // 新消息更新：执行命令分发。
    if is_interaction_client && let Update::NewMessage(update_new_message) = update {
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

        let request_message = message.clone();
        let message_content = message.content;
        // 这里的 update 已经确认来自 interaction client，后续所有回复都必须继续用这个 client。
        // 不再依赖兼容字段 `config.client_id`，避免双 client 后续扩展时误用其他角色。
        let interaction_client_id = client_id;

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
                crate::tgbot::send::send_text_message(
                    "未收到文本内容。".to_owned(),
                    chat_id,
                    interaction_client_id,
                )
                .await?;
                return Ok(());
            }

            let first_token_command = if text[0].starts_with("/") {
                Some(normalize_bot_command(text[0]))
            } else {
                None
            };

            if first_token_command == Some("/cancel")
                && tgbot::transfer::cancel_menu_input(chat_id, sender_id, interaction_client_id)
                    .await?
            {
                return Ok(());
            }

            if text[0].starts_with("/") {
                let raw_command = text[0];
                let command = normalize_bot_command(raw_command);
                if command != "/cancel" && tgbot::transfer::discard_menu_input(chat_id, sender_id) {
                    tracing::debug!(
                        command = raw_command,
                        normalized_command = command,
                        chat_id,
                        sender_id,
                        message_id = message.id,
                        "discarded pending menu input because command has priority"
                    );
                }
                // 只记录命令名和消息定位信息，不记录参数中的链接，避免日志暴露私有消息入口。
                tracing::info!(
                    command = raw_command,
                    normalized_command = command,
                    chat_id,
                    sender_id,
                    message_id = message.id,
                    "admin command received"
                );

                let command_result = match command {
                    // /help 命令入口。
                    // 返回机器人当前支持的命令说明。
                    "/help" | "/h" => {
                        tgbot::transfer::help_command(text, chat_id, interaction_client_id).await
                    }
                    // /transfer 命令入口。
                    "/transfer" | "/t" => {
                        // request_message_id 用于请求级幂等（防止同一条指令重复建任务）。
                        tgbot::transfer::transfer_command(
                            text,
                            config.clone(),
                            &request_message,
                            interaction_client_id,
                        )
                        .await
                    }
                    // /lookup 命令入口。
                    // 按源链接查找历史转存结果。
                    "/lookup" | "/lk" => {
                        tgbot::transfer::lookup_command(
                            text,
                            config.clone(),
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    // /config 命令入口。
                    // 仅开放运行时安全可调的配置项。
                    "/config" | "/cfg" => {
                        tgbot::transfer::config_command(text, chat_id, interaction_client_id).await
                    }
                    // /downloads 命令入口。
                    // 展示当前聊天最近的转存任务进度列表。
                    "/downloads" | "/d" => {
                        tgbot::transfer::downloads_command(text, chat_id, interaction_client_id)
                            .await
                    }
                    // /job 命令入口。
                    // 手动暂停、恢复、停止指定转存任务。
                    "/job" | "/j" => {
                        tgbot::transfer::job_command(text, chat_id, interaction_client_id).await
                    }
                    // /menu 命令入口。
                    // 正常配置下交互端固定为 bot；supports_reply_markup 只作为异常配置/测试场景的兜底开关。
                    "/menu" | "/m" => {
                        let supports_reply_markup = config.supports_reply_markup();
                        tgbot::transfer::menu_command(
                            text,
                            chat_id,
                            interaction_client_id,
                            supports_reply_markup,
                        )
                        .await
                    }
                    _ => {
                        tracing::warn!(
                            command = raw_command,
                            normalized_command = command,
                            chat_id,
                            sender_id,
                            message_id = message.id,
                            "unknown admin command"
                        );
                        send_unknown_command_message(raw_command, chat_id, interaction_client_id)
                            .await
                    }
                };

                if let Err(err) = command_result {
                    tracing::warn!(
                        command = raw_command,
                        normalized_command = command,
                        chat_id,
                        sender_id,
                        message_id = message.id,
                        error = %err,
                        "admin command failed"
                    );
                    send_command_error_message(raw_command, &err, chat_id, interaction_client_id)
                        .await?;
                } else {
                    tracing::debug!(
                        command = raw_command,
                        normalized_command = command,
                        chat_id,
                        sender_id,
                        message_id = message.id,
                        "admin command completed"
                    );
                }
            } else if tgbot::transfer::handle_menu_text_input(
                raw_text.as_str(),
                config.clone(),
                chat_id,
                message.id,
                sender_id,
                interaction_client_id,
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
            if tgbot::transfer::is_transferable_message(&request_message) {
                tracing::info!(
                    chat_id,
                    sender_id,
                    message_id = request_message.id,
                    content_kind = message_content_kind(&request_message.content),
                    "admin media message received, dispatching auto transfer"
                );
                match tgbot::transfer::transfer_bot_message_auto_command(
                    config.clone(),
                    request_message.clone(),
                    interaction_client_id,
                )
                .await
                {
                    Ok(()) => {
                        tracing::debug!(
                            chat_id,
                            sender_id,
                            message_id = request_message.id,
                            "admin media message auto transfer dispatched"
                        );
                    }
                    Err(err) => {
                        tracing::warn!(
                            chat_id,
                            sender_id,
                            message_id = request_message.id,
                            error = %err,
                            "admin media auto transfer failed"
                        );
                        send_auto_transfer_hint_message(&err, chat_id, interaction_client_id)
                            .await?;
                    }
                }
                return Ok(());
            }
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
    if is_interaction_client && let Update::NewCallbackQuery(mut update_callback_query) = update {
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

        tracing::debug!(
            chat_id = update_callback_query.chat_id,
            sender_user_id = update_callback_query.sender_user_id,
            "admin callback query received"
        );
        // callback update 已经确认来自 interaction client，直接使用当前 client_id 回答并编辑消息。
        // 这能避免双 client 运行时误把 callback 交给 download/upload client。
        tgbot::transfer::transfer_callback_query(update_callback_query, client_id).await?;
        return Ok(());
    }

    // 文件更新只写入进度快照；完整 File 结构很大且包含本地路径，不直接打日志。
    //
    // 当前源策略是 bot-first + user fallback，两个 client 都可能实际下载文件。
    // 因此这里不能只监听 workflow.download_client；该字段只是兼容配置，不代表实际下载端。
    if let Update::File(update_file) = update {
        // 将 TDLib 实时文件进度写入内存快照，供 `/downloads` 查询。
        queue::update_download_progress(client_id, &update_file.file);
        tracing::trace!(
            role = role.as_str(),
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

/// 归一化 bot 命令名。
///
/// Telegram 群里常见命令格式是 `/t@BotName`；业务路由只需要 `/t`。
/// 这里不校验 bot username，原因是 TDLib update 入口已经做了管理员白名单过滤。
fn normalize_bot_command(command: &str) -> &str {
    command.split_once('@').map_or(command, |(name, _)| name)
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

/// 自动转存媒体失败时给出可执行提示。
///
/// 最常见原因是当前请求 chat 没配置默认 target；用户可以直接回复这条媒体发送 `/t <target>`。
async fn send_auto_transfer_hint_message(
    err: &anyhow::Error,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    crate::tgbot::send::ReplyPanel::card(
        [
            "自动转存未启动".to_owned(),
            format!("状态：{}", card::code("need-target")),
            card::DIVIDER.to_owned(),
            card::section("原因"),
            card::pre_code(format!("{:#}", err)),
            card::section("下一步"),
            "请回复要转存的媒体消息，并发送下面命令。".to_owned(),
            card::command_line("指定目标", "/t <target_chat_id_or_alias>"),
        ]
        .join("\n"),
    )
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制 /t",
            "/t ",
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制帮助",
            "/h transfer",
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(chat_id, client_id)
    .await
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose};

    use super::{decode_callback_query_payload, normalize_bot_command};

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

    // bot 在群里收到的命令可能带 username 后缀；路由前必须归一成基础命令。
    #[test]
    fn test_normalize_bot_command() {
        assert_eq!(normalize_bot_command("/t"), "/t");
        assert_eq!(normalize_bot_command("/t@TransferBot"), "/t");
        assert_eq!(normalize_bot_command("/help@TransferBot"), "/help");
        assert_eq!(normalize_bot_command("/cancel@TransferBot"), "/cancel");
    }
}
