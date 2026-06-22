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
use base64::{Engine as _, engine::general_purpose};
pub use error::*;
pub use login::*;
use std::collections::BTreeSet;
use std::time::SystemTime;
use tdlib_rs::enums::Update;

// 记录进程启动时间戳。
// 用于过滤掉程序启动前的历史消息，避免重复处理。
static START_TS: std::sync::LazyLock<i32> = std::sync::LazyLock::new(|| {
    let secs = match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(err) => {
            // 系统时间异常时不要让机器人启动即 panic；回退到 0 只会少过滤历史消息。
            tracing::error!(error = %err, "system time is before unix epoch, fallback start ts");
            0
        }
    };
    match i32::try_from(secs) {
        Ok(ts) => ts,
        Err(err) => {
            // TDLib message date 仍是 i32；超过可表示范围时使用最大值并记录日志。
            tracing::error!(error = %err, secs, "system time overflowed tdlib date range");
            i32::MAX
        }
    }
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
pub async fn receive(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    config: std::sync::Arc<crate::config::BotConfig>,
) -> anyhow::Result<()> {
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
                let app_context = app_context.clone();
                let config = config.clone();
                let ready_roles = ready_roles.clone();
                tokio::spawn(async move {
                    let res = handle_update(
                        app_context,
                        msg_update,
                        client_id,
                        config.clone(),
                        ready_roles,
                    )
                    .await;
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
    app_context: std::sync::Arc<crate::app_context::AppContext>,
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
            app_context,
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
    let is_interaction_client =
        should_process_interactive_update(role, config.workflow.interaction_client);

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

    // 新消息更新：只有交互端允许处理命令、菜单输入和直接转发来的媒体。
    //
    // user client 只作为链接读取/下载 fallback 使用，不能消费自己收到的普通消息，
    // 否则用户号所在聊天里的杂散消息可能被误当成转存输入。
    if let Update::NewMessage(update_new_message) = update {
        if !is_interaction_client {
            tracing::debug!(
                role = role.as_str(),
                client_id,
                chat_id = update_new_message.message.chat_id,
                message_id = update_new_message.message.id,
                "ignored new message from non-interaction client"
            );
            return Ok(());
        }

        let message = update_new_message.message;
        if message.is_outgoing {
            tracing::trace!(
                chat_id = message.chat_id,
                message_id = message.id,
                "ignored outgoing message update"
            );
            return Ok(());
        }

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

        if !is_private_interaction_chat(chat_id, sender_id) {
            tracing::debug!(
                chat_id,
                sender_id,
                message_id = message.id,
                "ignored non-private interactive message"
            );
            if should_send_private_only_notice(&message.content) {
                send_private_chat_only_message(chat_id, client_id).await?;
            }
            return Ok(());
        }

        let Some(actor) = app_context
            .access_control_runtime
            .request_actor(chat_id, sender_id)
        else {
            tracing::debug!(
                chat_id,
                sender_id,
                message_id = message.id,
                "ignored unauthorized interactive message"
            );
            return Ok(());
        };
        tgbot::transfer::ensure_user_account_for_actor(
            actor,
            tgbot::transfer::billing_runtime_config_on(app_context.as_ref()).initial_user_points,
        )
        .await?;

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
                actor_role = actor.role.as_str(),
                message_id = message.id,
                "authorized text message received"
            );
            let direct_transfer_link = extract_direct_transfer_link(&message_text);
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
                if command != "/cancel"
                    && tgbot::transfer::discard_menu_input_for_command(
                        chat_id,
                        sender_id,
                        interaction_client_id,
                    )
                    .await?
                {
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
                    actor_role = actor.role.as_str(),
                    message_id = message.id,
                    "bot command received"
                );

                let command_result = match command {
                    // /help 命令入口。
                    // 返回机器人当前支持的命令说明。
                    "/help" => {
                        tgbot::transfer::help_command(text, actor, interaction_client_id).await
                    }
                    "/balance" => {
                        tgbot::transfer::balance_command(text, actor, interaction_client_id).await
                    }
                    "/points" if actor.is_admin() => {
                        tgbot::transfer::points_command(text, actor, interaction_client_id).await
                    }
                    "/points" => {
                        send_permission_denied_message(chat_id, interaction_client_id).await
                    }
                    // /transfer 命令入口。
                    "/transfer" => {
                        // request_message_id 用于请求级幂等（防止同一条指令重复建任务）。
                        tgbot::transfer::transfer_command_on(
                            app_context.clone(),
                            text,
                            config.clone(),
                            &request_message,
                            actor,
                            interaction_client_id,
                        )
                        .await
                    }
                    // /lookup 命令入口。
                    // 按源链接查找历史转存结果。
                    "/lookup" => {
                        tgbot::transfer::lookup_command_on(
                            app_context.as_ref(),
                            text,
                            config.clone(),
                            actor,
                            interaction_client_id,
                        )
                        .await
                    }
                    // /config 命令入口。
                    // 仅开放运行时安全可调的配置项。
                    "/config" if actor.is_admin() => {
                        tgbot::transfer::config_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    "/config" => {
                        send_permission_denied_message(chat_id, interaction_client_id).await
                    }
                    "/targets" if actor.is_admin() => {
                        tgbot::transfer::targets_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    "/targets" => {
                        send_permission_denied_message(chat_id, interaction_client_id).await
                    }
                    "/acl" if actor.is_admin() => {
                        tgbot::transfer::acl_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    "/acl" => send_permission_denied_message(chat_id, interaction_client_id).await,
                    "/billing" if actor.is_admin() => {
                        tgbot::transfer::billing_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    "/billing" => {
                        send_permission_denied_message(chat_id, interaction_client_id).await
                    }
                    // /health 命令入口。
                    // 只读展示运行状态、任务规模和缓存状态，方便排障。
                    "/health" if actor.is_admin() => {
                        tgbot::transfer::health_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    "/health" => {
                        send_permission_denied_message(chat_id, interaction_client_id).await
                    }
                    // /cache 命令入口。
                    // 只读展示 file_cache 汇总和最近记录，不执行清理。
                    "/cache" if actor.is_admin() => {
                        tgbot::transfer::cache_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    "/cache" => {
                        send_permission_denied_message(chat_id, interaction_client_id).await
                    }
                    // /downloads 命令入口。
                    // 展示当前聊天最近的转存任务进度列表。
                    "/downloads" => {
                        tgbot::transfer::downloads_command_on(
                            app_context.as_ref(),
                            text,
                            actor,
                            interaction_client_id,
                        )
                        .await
                    }
                    // /job 命令入口。
                    // 手动暂停、恢复、停止指定转存任务。
                    "/job" => {
                        tgbot::transfer::job_command_on(
                            app_context.as_ref(),
                            text,
                            actor,
                            interaction_client_id,
                        )
                        .await
                    }
                    // /menu 命令入口。
                    // 正常配置下交互端固定为 bot；supports_reply_markup 只作为异常配置/测试场景的兜底开关。
                    "/menu" => {
                        let supports_reply_markup = config.supports_reply_markup();
                        tgbot::transfer::menu_command_on(
                            app_context.as_ref(),
                            text,
                            actor,
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
            } else if tgbot::transfer::handle_menu_text_input_on(
                app_context.as_ref(),
                raw_text.as_str(),
                config.clone(),
                (chat_id, sender_id),
                message.id,
                actor,
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
            } else if let Some(source_link) = direct_transfer_link {
                tracing::info!(
                    chat_id,
                    sender_id,
                    message_id = message.id,
                    actor_role = actor.role.as_str(),
                    "direct transfer link received, entering transfer target selection"
                );
                if let Err(err) = tgbot::transfer::start_transfer_target_choice_from_link_message(
                    app_context.as_ref(),
                    config.clone(),
                    chat_id,
                    sender_id,
                    source_link,
                    interaction_client_id,
                )
                .await
                {
                    tracing::warn!(
                        chat_id,
                        sender_id,
                        message_id = message.id,
                        error = %err,
                        "link text target selection failed"
                    );
                    send_command_error_message("/transfer", &err, chat_id, interaction_client_id)
                        .await?;
                }
                return Ok(());
            } else if request_message.forward_info.is_some()
                && tgbot::transfer::is_transferable_message(&request_message)
            {
                let Some((source_chat_id, source_message_id)) =
                    tgbot::transfer::transferable_message_source_location(&request_message)
                else {
                    tracing::warn!(
                        chat_id,
                        sender_id,
                        message_id = request_message.id,
                        "forwarded text message has no resolvable source location"
                    );
                    send_auto_transfer_hint_message(
                        &anyhow::anyhow!(
                            "无法定位原始消息，请改用消息链接或回复 bot 可见媒体后再试"
                        ),
                        chat_id,
                        interaction_client_id,
                    )
                    .await?;
                    return Ok(());
                };
                tracing::info!(
                    chat_id,
                    sender_id,
                    message_id = message.id,
                    actor_role = actor.role.as_str(),
                    "forwarded text message received, entering transfer target selection"
                );
                if let Err(err) = tgbot::transfer::start_transfer_target_choice_from_bot_message(
                    app_context.as_ref(),
                    config.clone(),
                    chat_id,
                    sender_id,
                    source_chat_id,
                    source_message_id,
                    interaction_client_id,
                )
                .await
                {
                    tracing::warn!(
                        chat_id,
                        sender_id,
                        message_id = message.id,
                        error = %err,
                        "forwarded text target selection failed"
                    );
                    send_auto_transfer_hint_message(&err, chat_id, interaction_client_id).await?;
                }
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
            if let tdlib_rs::enums::MessageContent::MessageChatShared(shared) =
                &request_message.content
                && tgbot::transfer::handle_menu_shared_chat_input_on(
                    app_context.as_ref(),
                    shared,
                    config.clone(),
                    chat_id,
                    sender_id,
                    interaction_client_id,
                )
                .await?
            {
                tracing::debug!(
                    chat_id,
                    sender_id,
                    message_id = request_message.id,
                    "admin shared chat message consumed by menu input"
                );
                return Ok(());
            }
            if tgbot::transfer::is_transferable_message(&request_message) {
                let Some((source_chat_id, source_message_id)) =
                    tgbot::transfer::transferable_message_source_location(&request_message)
                else {
                    tracing::warn!(
                        chat_id,
                        sender_id,
                        message_id = request_message.id,
                        "transferable media message has no resolvable source location"
                    );
                    send_auto_transfer_hint_message(
                        &anyhow::anyhow!(
                            "无法定位原始消息，请改用消息链接或回复 bot 可见媒体后再试"
                        ),
                        chat_id,
                        interaction_client_id,
                    )
                    .await?;
                    return Ok(());
                };
                tracing::info!(
                    chat_id,
                    sender_id,
                    message_id = request_message.id,
                    content_kind = message_content_kind(&request_message.content),
                    "media message received, entering transfer target selection"
                );
                match tgbot::transfer::start_transfer_target_choice_from_bot_message(
                    app_context.as_ref(),
                    config.clone(),
                    chat_id,
                    sender_id,
                    source_chat_id,
                    source_message_id,
                    interaction_client_id,
                )
                .await
                {
                    Ok(()) => {
                        tracing::debug!(
                            chat_id,
                            sender_id,
                            message_id = request_message.id,
                            "media message transfer target selection started"
                        );
                    }
                    Err(err) => {
                        tracing::warn!(
                            chat_id,
                            sender_id,
                            message_id = request_message.id,
                            error = %err,
                            "media transfer target selection failed"
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

    // inline keyboard 回调：只允许交互端处理，用于 `/downloads` 分页和 `/job` 原地控制。
    if let Update::NewCallbackQuery(mut update_callback_query) = update {
        if !is_interaction_client {
            tracing::debug!(
                role = role.as_str(),
                client_id,
                chat_id = update_callback_query.chat_id,
                message_id = update_callback_query.message_id,
                sender_user_id = update_callback_query.sender_user_id,
                "ignored callback query from non-interaction client"
            );
            return Ok(());
        }

        decode_callback_query_payload(&mut update_callback_query);

        if !is_private_interaction_chat(
            update_callback_query.chat_id,
            update_callback_query.sender_user_id,
        ) {
            tracing::debug!(
                chat_id = update_callback_query.chat_id,
                sender_user_id = update_callback_query.sender_user_id,
                message_id = update_callback_query.message_id,
                "ignored non-private callback query"
            );
            crate::tgbot::send::answer_callback_query(
                update_callback_query.id,
                Some("请私聊 bot 使用"),
                client_id,
            )
            .await?;
            return Ok(());
        }

        let Some(actor) = app_context.access_control_runtime.request_actor(
            update_callback_query.chat_id,
            update_callback_query.sender_user_id,
        ) else {
            tracing::debug!(
                chat_id = update_callback_query.chat_id,
                sender_user_id = update_callback_query.sender_user_id,
                message_id = update_callback_query.message_id,
                "ignored unauthorized callback query"
            );
            return Ok(());
        };
        tgbot::transfer::ensure_user_account_for_actor(
            actor,
            tgbot::transfer::billing_runtime_config_on(app_context.as_ref()).initial_user_points,
        )
        .await?;

        tracing::debug!(
            chat_id = update_callback_query.chat_id,
            sender_user_id = update_callback_query.sender_user_id,
            actor_role = actor.role.as_str(),
            "authorized callback query received"
        );
        // callback update 已经确认来自 interaction client，直接使用当前 client_id 回答并编辑消息。
        // 这能避免双 client 运行时误把 callback 交给 download/upload client。
        tgbot::transfer::transfer_callback_query_on(
            app_context.as_ref(),
            update_callback_query,
            config.clone(),
            actor,
            client_id,
        )
        .await?;
        return Ok(());
    }

    // 文件更新只写入进度快照；完整 File 结构很大且包含本地路径，不直接打日志。
    //
    // 当前源策略是 bot-first + user fallback，两个 client 都可能实际下载文件。
    // 因此这里不能只监听 workflow.download_client；该字段只是兼容配置，不代表实际下载端。
    if let Update::File(update_file) = update {
        // 将 TDLib 实时文件进度写入内存快照，供 `/downloads` 查询。
        app_context
            .download_progress
            .update_download_progress(client_id, &update_file.file);
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

/// 从文本消息中提取“可直接进入转存流程”的 Telegram 源链接。
///
/// 支持三种入口：
/// - 整条纯文本就是 `t.me/...`
/// - 文本实体里是隐藏链接 `TextUrl`
/// - TDLib 已经生成 Telegram 链接预览
///
/// 这里只做轻量提取；真正链接是否合法仍交给 spider 层。
fn extract_direct_transfer_link(message_text: &tdlib_rs::types::MessageText) -> Option<String> {
    let trimmed = message_text.text.text.trim();
    if !trimmed.is_empty()
        && trimmed.split_whitespace().count() == 1
        && looks_like_transfer_link_text(trimmed)
    {
        return Some(trimmed.to_owned());
    }

    for entity in &message_text.text.entities {
        match &entity.r#type {
            tdlib_rs::enums::TextEntityType::TextUrl(url)
                if looks_like_transfer_link_text(&url.url) =>
            {
                return Some(url.url.clone());
            }
            tdlib_rs::enums::TextEntityType::Url => {
                if let Some(value) = extract_entity_text_slice(&message_text.text.text, entity)
                    && looks_like_transfer_link_text(value.trim())
                {
                    return Some(value.trim().to_owned());
                }
            }
            _ => {}
        }
    }

    if let Some(link_preview) = &message_text.link_preview
        && looks_like_transfer_link_text(link_preview.url.trim())
    {
        return Some(link_preview.url.trim().to_owned());
    }

    None
}

/// 轻量判断一条文本是否像 Telegram 消息链接。
fn looks_like_transfer_link_text(input: &str) -> bool {
    input.starts_with("https://t.me/")
        || input.starts_with("http://t.me/")
        || input.starts_with("t.me/")
}

/// 按 TDLib 的 UTF-16 offset/length 规则切出实体文本。
///
/// 这里只用于 URL 实体的轻量识别，因此失败时直接返回 `None`。
fn extract_entity_text_slice<'a>(
    text: &'a str,
    entity: &tdlib_rs::types::TextEntity,
) -> Option<&'a str> {
    let start = usize::try_from(entity.offset).ok()?;
    let len = usize::try_from(entity.length).ok()?;
    let end = start.checked_add(len)?;
    let start_byte = utf16_offset_to_byte_index(text, start)?;
    let end_byte = utf16_offset_to_byte_index(text, end)?;
    text.get(start_byte..end_byte)
}

/// 把 TDLib 的 UTF-16 offset 映射到 Rust `str` 的 byte index。
fn utf16_offset_to_byte_index(text: &str, target_utf16_offset: usize) -> Option<usize> {
    let mut current_utf16_offset = 0usize;
    for (byte_index, ch) in text.char_indices() {
        if current_utf16_offset == target_utf16_offset {
            return Some(byte_index);
        }
        current_utf16_offset += ch.len_utf16();
    }
    if current_utf16_offset == target_utf16_offset {
        Some(text.len())
    } else {
        None
    }
}

/// 判断交互消息是否来自 bot 私聊。
///
/// 本项目不支持群聊命令交互；目标群只作为转存目的地出现。
fn is_private_interaction_chat(chat_id: i64, sender_user_id: i64) -> bool {
    chat_id == sender_user_id
}

/// 群聊里只有明显发给 bot 的命令才回复私聊提示，避免 bot 被误加群后刷屏。
fn should_send_private_only_notice(content: &tdlib_rs::enums::MessageContent) -> bool {
    match content {
        tdlib_rs::enums::MessageContent::MessageText(text) => {
            text.text.text.trim_start().starts_with('/')
        }
        _ => false,
    }
}

/// 群聊/频道中触发命令时的统一提示。
async fn send_private_chat_only_message(chat_id: i64, client_id: i32) -> anyhow::Result<()> {
    crate::tgbot::send::send_text_message(
        "当前只支持私聊 bot 使用；目标群请在私聊菜单中选择。".to_owned(),
        chat_id,
        client_id,
    )
    .await
}

/// 解码 TDLib callback payload。
///
/// TDLib schema 里的 callback `data` 是 bytes：
/// - 发送 JSON 请求时必须写 base64。
/// - 收到 update 时 TDLib 也会把 bytes 表示为 base64。
///
/// 业务路由只认识 `m:home`、`d:r:all:8:1` 这类短字符串，因此入口统一解码。
fn decode_callback_query_payload(update: &mut tdlib_rs::types::UpdateNewCallbackQuery) {
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

/// 判断指定 client 角色是否允许处理交互 update。
///
/// 当前配置校验强制交互端为 bot；这里仍保留显式判断，避免未来新增角色或配置迁移时
/// user client 误消费收到的普通消息、菜单输入或 callback。
fn should_process_interactive_update(
    role: crate::config::ClientRole,
    interaction_role: crate::config::ClientRole,
) -> bool {
    role == interaction_role
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

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose};

    use super::{
        command_error_hint, decode_callback_query_payload, extract_direct_transfer_link,
        is_private_interaction_chat, normalize_bot_command, should_process_interactive_update,
        should_send_private_only_notice,
    };
    use crate::config::ClientRole;

    // TDLib JSON 协议会用 base64 表示 callback bytes；入口应解回业务短 payload。
    #[test]
    fn test_decode_callback_query_payload() {
        let mut update = tdlib_rs::types::UpdateNewCallbackQuery {
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
        let mut update = tdlib_rs::types::UpdateNewCallbackQuery {
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

    // user client 只用于链接读取/下载 fallback，不应处理普通消息或 callback。
    #[test]
    fn test_user_client_is_not_interaction_client() {
        assert!(should_process_interactive_update(
            ClientRole::Bot,
            ClientRole::Bot
        ));
        assert!(!should_process_interactive_update(
            ClientRole::User,
            ClientRole::Bot
        ));
    }

    // 项目只支持 bot 私聊交互；群聊里 chat_id 与 sender_user_id 不同，必须拒绝。
    #[test]
    fn test_private_interaction_chat_only() {
        assert!(is_private_interaction_chat(100, 100));
        assert!(!is_private_interaction_chat(-100123, 100));
        assert!(!is_private_interaction_chat(200, 100));
    }

    // 群聊里只对命令回复“请私聊”，普通文本和媒体应静默忽略，避免刷屏。
    #[test]
    fn test_private_only_notice_only_for_commands() {
        let command = tdlib_rs::enums::MessageContent::MessageText(tdlib_rs::types::MessageText {
            text: tdlib_rs::types::FormattedText {
                text: " /menu".to_owned(),
                entities: vec![],
            },
            link_preview: None,
            link_preview_options: None,
        });
        let text = tdlib_rs::enums::MessageContent::MessageText(tdlib_rs::types::MessageText {
            text: tdlib_rs::types::FormattedText {
                text: "hello".to_owned(),
                entities: vec![],
            },
            link_preview: None,
            link_preview_options: None,
        });
        let non_text = tdlib_rs::enums::MessageContent::MessageBasicGroupChatCreate(
            tdlib_rs::types::MessageBasicGroupChatCreate::default(),
        );

        assert!(should_send_private_only_notice(&command));
        assert!(!should_send_private_only_notice(&text));
        assert!(!should_send_private_only_notice(&non_text));
    }

    // 单独一条 Telegram 链接文本应直接进入目标选择，不需要先手输 /transfer。
    #[test]
    fn test_extract_direct_transfer_link_from_plain_text() {
        let message_text = tdlib_rs::types::MessageText {
            text: tdlib_rs::types::FormattedText {
                text: "https://t.me/c/123/456".to_owned(),
                entities: vec![],
            },
            link_preview: None,
            link_preview_options: None,
        };

        assert_eq!(
            extract_direct_transfer_link(&message_text),
            Some("https://t.me/c/123/456".to_owned())
        );
    }

    // 隐藏链接文本也应能提取出真实 Telegram URL。
    #[test]
    fn test_extract_direct_transfer_link_from_text_url_entity() {
        let message_text = tdlib_rs::types::MessageText {
            text: tdlib_rs::types::FormattedText {
                text: "点我打开".to_owned(),
                entities: vec![tdlib_rs::types::TextEntity {
                    offset: 0,
                    length: 4,
                    r#type: tdlib_rs::enums::TextEntityType::TextUrl(
                        tdlib_rs::types::TextEntityTypeTextUrl {
                            url: "https://t.me/c/123/456".to_owned(),
                        },
                    ),
                }],
            },
            link_preview: None,
            link_preview_options: None,
        };

        assert_eq!(
            extract_direct_transfer_link(&message_text),
            Some("https://t.me/c/123/456".to_owned())
        );
    }

    // Telegram 链接预览消息也应能作为直接入口。
    #[test]
    fn test_extract_direct_transfer_link_from_link_preview() {
        let message_text = tdlib_rs::types::MessageText {
            text: tdlib_rs::types::FormattedText {
                text: "转这个".to_owned(),
                entities: vec![],
            },
            link_preview: Some(tdlib_rs::types::LinkPreview {
                url: "https://t.me/c/123/456".to_owned(),
                display_url: "t.me/c/123/456".to_owned(),
                site_name: "Telegram".to_owned(),
                title: String::new(),
                description: tdlib_rs::types::FormattedText {
                    text: String::new(),
                    entities: vec![],
                },
                author: String::new(),
                r#type: tdlib_rs::enums::LinkPreviewType::Article(
                    tdlib_rs::types::LinkPreviewTypeArticle { photo: None },
                ),
                has_large_media: false,
                show_large_media: false,
                show_media_above_description: false,
                skip_confirmation: false,
                show_above_text: false,
                instant_view_version: 0,
            }),
            link_preview_options: None,
        };

        assert_eq!(
            extract_direct_transfer_link(&message_text),
            Some("https://t.me/c/123/456".to_owned())
        );
    }

    // UTF-16 实体切片必须正确处理 emoji 等双单元字符，避免 URL 实体定位错位。
    #[test]
    fn test_extract_direct_transfer_link_from_url_entity_with_utf16_offset() {
        let message_text = tdlib_rs::types::MessageText {
            text: tdlib_rs::types::FormattedText {
                text: "📦 https://t.me/c/123/456".to_owned(),
                entities: vec![tdlib_rs::types::TextEntity {
                    offset: 3,
                    length: 22,
                    r#type: tdlib_rs::enums::TextEntityType::Url,
                }],
            },
            link_preview: None,
            link_preview_options: None,
        };

        assert_eq!(
            extract_direct_transfer_link(&message_text),
            Some("https://t.me/c/123/456".to_owned())
        );
    }

    // 余额不足应提示用户查看余额和联系管理员，而不是只显示英文异常。
    #[test]
    fn test_command_error_hint_for_insufficient_points() {
        let hint = command_error_hint("insufficient points: user=1, balance=0, required=3");

        assert_eq!(hint.title, "积分不足");
        assert_eq!(hint.primary_command, "/balance");
        assert_eq!(
            hint.primary_action,
            crate::tgbot::error::CommandErrorPrimaryAction::OpenBalance
        );
        assert!(hint.advice.contains("联系管理员加分"));
    }

    // 目标白名单失败时，应指向目标配置，而不是提示源链接错误。
    #[test]
    fn test_command_error_hint_for_target_denied() {
        let hint = command_error_hint("target chat is not allowed: -100");

        assert_eq!(hint.title, "目标不可用");
        assert!(hint.advice.contains("allowed_target_chat_ids"));
        assert_eq!(
            hint.primary_action,
            crate::tgbot::error::CommandErrorPrimaryAction::OpenMenu
        );
        assert_eq!(hint.help_command, "/help transfer");
    }

    // 私有源不可读时，应明确区分普通用户处理方式和管理员 user fallback 前提。
    #[test]
    fn test_command_error_hint_for_source_access() {
        let hint = command_error_hint("code=400, message=Message not found");

        assert_eq!(hint.title, "源不可访问");
        assert!(hint.advice.contains("普通用户请转发源消息给 bot"));
        assert!(hint.advice.contains("备用 user"));
    }

    // 未分类错误仍保留通用排查建议。
    #[test]
    fn test_command_error_hint_fallback() {
        let hint = command_error_hint("network timeout");

        assert_eq!(hint.title, "命令执行失败");
        assert_eq!(hint.primary_command, "/help");
        assert_eq!(
            hint.primary_action,
            crate::tgbot::error::CommandErrorPrimaryAction::OpenHelp
        );
    }
}
