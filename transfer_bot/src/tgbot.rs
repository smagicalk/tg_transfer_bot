//! Telegram Bot 消息分发与核心事件网关模块。
//!
//! # 核心职责
//! 1. **TDLib Update 统一监听与路由**：
//!    - 接收来自各 TDLib Client（Bot 交互端、User 下载端）的异步 Update 数据流。
//!    - 路由登录授权状态机更新（`Update::AuthorizationState`）至 `login` 模块。
//!    - 路由文件下载/上传进度更新（`Update::File`）至全局进度存储。
//!    - 拦截并校正异步消息发送结果（`MessageSendSucceeded` / `MessageSendFailed`）。
//! 2. **交互命令与菜单分发**：
//!    - 拦截新消息（`Update::NewMessage`）与内联按钮回调（`Update::NewCallbackQuery`）。
//!    - 实施权限校验（Owner/Admin 静态白名单及数据库动态白名单）。
//!    - 支持纯文本消息中的直接转存链接识别与转发媒体直接转存。
//!    - 统一将 `/transfer`, `/lookup`, `/config`, `/targets`, `/health`, `/cache`, `/downloads`, `/job`, `/auth`, `/menu`, `/help` 等命令路由至 transfer 子系统。

mod error;
pub(crate) mod executor;
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

/// 记录进程启动时的 Unix 时间戳（秒）。
///
/// 用于在 Update 监听流中过滤掉程序启动前已产生的历史未读消息，避免服务重启时产生重复转存或误响应。
static START_TS: std::sync::LazyLock<i32> = std::sync::LazyLock::new(|| {
    let secs = match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(err) => {
            // 系统时间异常时不要让机器人启动即 panic；回退到 0 只会少过滤历史消息
            tracing::error!(error = %err, "system time is before unix epoch, fallback start ts");
            0
        }
    };
    match i32::try_from(secs) {
        Ok(ts) => ts,
        Err(err) => {
            // TDLib message date 仍是 i32；超过可表示范围时使用最大值并记录日志
            tracing::error!(error = %err, secs, "system time overflowed tdlib date range");
            i32::MAX
        }
    }
});

/// 创建一个新的 TDLib 客户端实例并返回其 `client_id`。
pub async fn create_client() -> anyhow::Result<i32> {
    Ok(tdlib_rs::create_client())
}

/// 读取指定 TDLib 客户端的底层版本字符串（供诊断排查）。
///
/// # 参数
/// * `client_id` - TDLib 客户端标识
pub async fn get_version(client_id: i32) -> anyhow::Result<()> {
    let version = tdlib_rs::functions::get_option("version".to_string(), client_id).await;
    match version {
        Ok(version) => {
            tracing::info!(version = ?version, "tdlib version loaded");
            Ok(())
        }
        Err(err) => anyhow::bail!("get_version failed, error={err:?}"),
    }
}

/// 设置指定 TDLib 客户端的日志冗余详细级别。
///
/// # 参数
/// * `client_id` - TDLib 客户端标识
/// * `verbosity_level` - 日志详细级别（0-10）
pub async fn set_log(client_id: i32, verbosity_level: i32) {
    match tdlib_rs::functions::set_log_verbosity_level(verbosity_level, client_id).await {
        Ok(_) => tracing::debug!(client_id, verbosity_level, "tdlib log level configured"),
        Err(err) => {
            tracing::warn!(client_id, error = ?err, "configure tdlib log level failed");
        }
    }
}

/// 机器人主事件循环接收入口。
///
/// 启动 Bot 交互端客户端的监听协程并等待其持续运行。
///
/// # 参数
/// * `app_context` - 全局应用上下文
/// * `config` - 机器人全局配置
/// * `bot_client` - 已初始化的 Bot TDLib 客户端
pub async fn receive(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    config: std::sync::Arc<crate::config::BotConfig>,
    bot_client: tdlib_rs::Client,
) -> anyhow::Result<()> {
    let ready_roles = std::sync::Arc::new(tokio::sync::Mutex::new(BTreeSet::new()));
    let bot_task = spawn_client_listener(
        bot_client,
        crate::config::ClientRole::Bot,
        app_context,
        config,
        ready_roles,
    );
    bot_task.await?;
    Ok(())
}

/// 启动指定角色 TDLib 客户端的 Update 流异步监听任务。
///
/// 循环接收 client 的 Update 事件，并为每个 Update 产生独立异步协程调用 `handle_update` 进行非阻塞处理。
///
/// # 参数
/// * `client` - TDLib 客户端实例
/// * `role` - 客户端角色（`Bot` 或 `User`）
/// * `app_context` - 全局应用上下文
/// * `config` - 机器人全局配置
/// * `ready_roles` - 就绪客户端角色集合互斥锁
///
/// # 返回
/// 监听协程的 `JoinHandle`
pub fn spawn_client_listener(
    mut client: tdlib_rs::Client,
    role: crate::config::ClientRole,
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    config: std::sync::Arc<crate::config::BotConfig>,
    ready_roles: std::sync::Arc<tokio::sync::Mutex<BTreeSet<crate::config::ClientRole>>>,
) -> tokio::task::JoinHandle<()> {
    let client_id = client.id();
    tokio::spawn(async move {
        tracing::info!(
            client_id,
            role = role.as_str(),
            "client update stream listener started"
        );
        while let Some(msg_update) = client.receive().await {
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
                    tracing::error!(error = %err, client_id, "handle tdlib update failed");
                }
            });
        }
        tracing::info!(
            client_id,
            role = role.as_str(),
            "client update stream listener exited"
        );
    })
}

/// TDLib Update 核心分发路由总入口。
///
/// 针对不同类型的 TDLib Update 事件执行分流与路由处理：
/// - `Update::AuthorizationState` => 委托至 `login::handle_authorization` 推进登录状态机。
/// - `Update::MessageSendSucceeded` / `MessageSendFailed` => 同步校准异步发送消息的真实 Message ID。
/// - `Update::NewMessage` => 解析命令文本（如 `/transfer`, `/menu`）、直接链接识别、草稿输入消费与自动转存。
/// - `Update::NewCallbackQuery` => 解析并路由内联按钮交互（如菜单跳转、分页浏览、任务控制）。
/// - `Update::File` => 更新全局内存中的实时文件下载与上传进度快照。
///
/// # 参数
/// * `app_context` - 全局应用上下文引用
/// * `update` - 底层 TDLib Update 对象
/// * `client_id` - 产生该 Update 的 TDLib 客户端标识
/// * `config` - 机器人全局配置
/// * `ready_roles` - 就绪客户端角色集合互斥锁
///
/// # 返回
/// 成功分发处理返回 `Ok(())`，遇到内部处理错误返回 Err
pub async fn handle_update(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    update: Update,
    client_id: i32,
    config: std::sync::Arc<crate::config::BotConfig>,
    ready_roles: std::sync::Arc<tokio::sync::Mutex<BTreeSet<crate::config::ClientRole>>>,
) -> anyhow::Result<()> {
    // 根据 client_id 识别当前客户端所属的角色（Bot 或 User）
    let Some(role) = app_context
        .executor_runtime
        .role_for_client_id(client_id)
        .or_else(|| config.client_ids.role_for_client_id(client_id))
    else {
        tracing::warn!(client_id, "ignored update from unknown tdlib client");
        return Ok(());
    };

    // 分支 1：授权状态更新 => 交给登录状态机处理
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

    // 判断当前客户端是否允许处理交互式命令与 Callback 回调（仅 Bot 允许）
    let is_interaction_client =
        should_process_interactive_update(role, crate::config::ClientRole::Bot);

    // 分支 2：消息发送成功更新 => 校准 sendMessage 返回的临时 message_id 到最终 message_id
    if let Update::MessageSendSucceeded(update_send_succeeded) = update {
        tracing::debug!(
            chat_id = update_send_succeeded.message.chat_id,
            old_message_id = update_send_succeeded.old_message_id,
            final_message_id = update_send_succeeded.message.id,
            "tdlib message send succeeded"
        );
        crate::tgbot::send::observe_message_send_succeeded_for_client(
            *update_send_succeeded,
            client_id,
        );
        return Ok(());
    }
    // 分支 3：消息发送失败更新
    if let Update::MessageSendFailed(update_send_failed) = update {
        tracing::warn!(
            chat_id = update_send_failed.message.chat_id,
            old_message_id = update_send_failed.old_message_id,
            failed_message_id = update_send_failed.message.id,
            error_code = update_send_failed.error.code,
            error_message = %update_send_failed.error.message,
            "tdlib message send failed"
        );
        crate::tgbot::send::observe_message_send_failed_for_client(*update_send_failed, client_id);
        return Ok(());
    }

    // 分支 4：新消息更新 => 仅交互端（Bot）允许处理命令、菜单输入和直接转发媒体
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

        // 私聊消息的发送者固定为 User；群聊中的匿名管理员可能以 Chat 身份发送。
        let sender_id = match &message.sender_id {
            tdlib_rs::enums::MessageSender::User(user) => user.user_id,
            tdlib_rs::enums::MessageSender::Chat(chat_id) => chat_id.chat_id,
        };

        // 群聊仅开放“回复某个普通用户 + /auth”这一条 owner 管理捷径。
        // 权限仍由 auth_command_on 统一校验，非 owner 无法借此扩散授权。
        if !is_private_interaction_chat(chat_id, sender_id)
            && is_reply_auth_command(&message.content, message.reply_to.is_some())
        {
            let actor = match reply_auth_request_actor(chat_id, &message.sender_id) {
                Ok(actor) => actor,
                Err(error) => {
                    tracing::debug!(
                        chat_id,
                        sender_id,
                        message_id = message.id,
                        error = %error,
                        "reply authorization actor could not be verified"
                    );
                    send_group_auth_error_message(&error, chat_id, client_id).await?;
                    return Ok(());
                }
            };
            let result = tgbot::transfer::auth_command_on(
                app_context.as_ref(),
                vec!["/auth"],
                config.as_ref(),
                &message,
                actor,
                client_id,
            )
            .await;
            if let Err(error) = result {
                tracing::warn!(
                    chat_id,
                    sender_id,
                    message_id = message.id,
                    error = %error,
                    "reply authorization command failed"
                );
                send_group_auth_error_message(&error, chat_id, client_id).await?;
            }
            return Ok(());
        }

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

        let Some(actor) =
            resolve_request_actor(app_context.as_ref(), config.as_ref(), chat_id, sender_id)
        else {
            tracing::debug!(
                chat_id,
                sender_id,
                message_id = message.id,
                "rejected unauthorized interactive message"
            );
            send_unauthorized_interaction_message(chat_id, client_id).await?;
            return Ok(());
        };
        let request_message = message.clone();
        let message_content = message.content;
        // 这里的 update 已经确认来自 interaction client，后续所有回复都必须继续用这个 client。
        // 交互端固定为 bot，当前 update 的 client_id 就是回复使用的 client。
        let interaction_client_id = client_id;

        // 当前仅处理文本消息。
        if let tdlib_rs::enums::MessageContent::MessageText(message_text) = message_content {
            tracing::debug!(
                chat_id,
                sender_id,
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

            // 二次验证密码只在 owner 私聊且执行器明确等待密码时消费；先删除用户
            // 的密码消息，再发送给 TDLib，避免密码留在聊天记录中。
            if !text[0].starts_with("/")
                && app_context.executor_runtime.phase()
                    == crate::app_context::ExecutorPhase::WaitingPassword
                && app_context.executor_runtime.owner_chat_id() == Some(chat_id)
            {
                let reply_message_id = match message.reply_to.as_ref() {
                    Some(tdlib_rs::enums::MessageReplyTo::Message(reply))
                        if reply.chat_id == 0 || reply.chat_id == chat_id =>
                    {
                        Some(reply.message_id)
                    }
                    _ => None,
                };
                if app_context.executor_runtime.password_prompt_message_id() != reply_message_id {
                    return Ok(());
                }
                let _ =
                    crate::tgbot::send::delete_message(chat_id, message.id, interaction_client_id)
                        .await;
                if tgbot::executor::submit_two_factor_password(
                    app_context.as_ref(),
                    sender_id,
                    reply_message_id,
                    raw_text.clone(),
                )
                .await?
                {
                    crate::tgbot::send::send_card_message(
                        "执行器登录\n\n二次验证密码已提交。".to_owned(),
                        chat_id,
                        interaction_client_id,
                    )
                    .await?;
                    return Ok(());
                }
            }

            let first_token_command = if text[0].starts_with("/") {
                Some(normalize_bot_command(text[0]))
            } else {
                None
            };

            if first_token_command == Some("/cancel") {
                let auth_cancelled =
                    tgbot::transfer::cancel_auth_input(chat_id, sender_id, interaction_client_id)
                        .await?;
                let menu_cancelled =
                    tgbot::transfer::cancel_menu_input(chat_id, sender_id, interaction_client_id)
                        .await?;
                if auth_cancelled || menu_cancelled {
                    return Ok(());
                }
            }

            if text[0].starts_with("/") {
                let raw_command = text[0];
                let command = normalize_bot_command(raw_command);
                if command != "/cancel"
                    && tgbot::transfer::discard_auth_input_for_command(
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
                        "discarded pending auth input because command has priority"
                    );
                }
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
                    message_id = message.id,
                    "bot command received"
                );

                let command_result = match command {
                    // /help 命令入口。
                    // 返回机器人当前支持的命令说明。
                    "/help" => {
                        tgbot::transfer::help_command(text, actor, interaction_client_id).await
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
                    "/config" => {
                        tgbot::transfer::config_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    "/targets" => {
                        tgbot::transfer::targets_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    // /health 命令入口。
                    // 只读展示运行状态、任务规模和缓存状态，方便排障。
                    "/health" => {
                        tgbot::transfer::health_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
                    }
                    // /cache 命令入口。
                    // 只读展示 file_cache 汇总和最近记录，不执行清理。
                    "/cache" => {
                        tgbot::transfer::cache_command_on(
                            app_context.as_ref(),
                            text,
                            chat_id,
                            interaction_client_id,
                        )
                        .await
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
                    // /auth 命令入口。
                    // 仅 owner 可查看和修改数据库动态授权名单。
                    "/auth" => {
                        tgbot::transfer::auth_command_on(
                            app_context.as_ref(),
                            text,
                            config.as_ref(),
                            &request_message,
                            actor,
                            interaction_client_id,
                        )
                        .await
                    }
                    // /menu 命令入口。
                    "/menu" => {
                        tgbot::transfer::menu_command_on(
                            app_context.as_ref(),
                            text,
                            config.as_ref(),
                            actor,
                            interaction_client_id,
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
            } else if tgbot::transfer::handle_auth_text_input_on(
                app_context.as_ref(),
                raw_text.as_str(),
                config.clone(),
                actor,
                interaction_client_id,
            )
            .await?
            {
                tracing::debug!(
                    chat_id,
                    sender_id,
                    message_id = message.id,
                    "admin text message consumed by auth input"
                );
                return Ok(());
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
            if let tdlib_rs::enums::MessageContent::MessageUsersShared(shared) =
                &request_message.content
                && tgbot::transfer::handle_auth_shared_user_input(
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
                    user_count = shared.users.len(),
                    "shared users consumed by auth input"
                );
                return Ok(());
            }
            if let tdlib_rs::enums::MessageContent::MessageChatShared(shared) =
                &request_message.content
                && tgbot::transfer::handle_menu_shared_chat_input(
                    shared,
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
                    target_chat_id = shared.chat.chat_id,
                    "shared target chat consumed by menu input"
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

        let Some(actor) = resolve_request_actor(
            app_context.as_ref(),
            config.as_ref(),
            update_callback_query.chat_id,
            update_callback_query.sender_user_id,
        ) else {
            tracing::debug!(
                chat_id = update_callback_query.chat_id,
                sender_user_id = update_callback_query.sender_user_id,
                message_id = update_callback_query.message_id,
                "rejected unauthorized callback query"
            );
            crate::tgbot::send::answer_callback_query(
                update_callback_query.id,
                Some(unauthorized_interaction_message()),
                client_id,
            )
            .await?;
            return Ok(());
        };
        tracing::debug!(
            chat_id = update_callback_query.chat_id,
            sender_user_id = update_callback_query.sender_user_id,
            "authorized callback query received"
        );
        // callback update 已经确认来自 interaction client，直接使用当前 client_id 回答并编辑消息。
        // 这能避免双 client 运行时误把 callback 交给 download/upload client。
        tgbot::transfer::transfer_callback_query_on(
            app_context.as_ref(),
            *update_callback_query,
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
    // 因此这里同时监听 bot 和 user，实际下载端跟随每个任务的 source_client_role。
    if let Update::File(update_file) = update {
        // 将 TDLib 实时文件进度写入内存快照，供 `/downloads` 查询。
        app_context
            .download_progress
            .update_download_progress(client_id, &update_file.file);
        app_context
            .upload_progress
            .update_upload_progress(client_id, &update_file.file);
        tracing::trace!(
            role = role.as_str(),
            file_id = update_file.file.id,
            downloaded_size = update_file.file.local.downloaded_size,
            size = update_file.file.size,
            expected_size = update_file.file.expected_size,
            is_downloading_active = update_file.file.local.is_downloading_active,
            is_downloading_completed = update_file.file.local.is_downloading_completed,
            uploaded_size = update_file.file.remote.uploaded_size,
            is_uploading_active = update_file.file.remote.is_uploading_active,
            is_uploading_completed = update_file.file.remote.is_uploading_completed,
            "tdlib file progress updated"
        );
    }

    Ok(())
}

/// 归一化 Telegram Bot 命令名称。
///
/// Telegram 群组中由于 @机器人的习惯，命令常见格式为 `/transfer@TransferBot`；
/// 业务路由层只需要纯粹的 `/transfer`，此处去除 `@` 及之后的后缀。
///
/// # 参数
/// * `command` - 用户输入的原始命令字符串
fn normalize_bot_command(command: &str) -> &str {
    command.split_once('@').map_or(command, |(name, _)| name)
}

/// 从文本消息中提取“可直接进入转存流程”的 Telegram 源链接。
///
/// 支持三种来源形式：
/// 1. 整条纯文本仅包含单个 `t.me/...` 链接。
/// 2. 富文本实体中的隐藏文字超链接（`TextUrl`）。
/// 3. TDLib 解析出的消息链接预览（`LinkPreview`）。
///
/// 此处仅进行初步提取与格式探测，深度的链接有效性与消息定位交由 Spider 模块负责。
///
/// # 参数
/// * `message_text` - TDLib 文本消息内容对象
///
/// # 返回
/// 提取到合法外观的链接则返回 `Some(url)`，否则返回 `None`
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

/// 轻量判断给定的字符串是否符合 Telegram 消息链接的前缀特征。
///
/// # 参数
/// * `input` - 待检查的字符串
fn looks_like_transfer_link_text(input: &str) -> bool {
    input.starts_with("https://t.me/")
        || input.starts_with("http://t.me/")
        || input.starts_with("t.me/")
}

/// 按照 TDLib 的 UTF-16 offset/length 规范切出对应的实体文本切片。
///
/// # 参数
/// * `text` - 原始完整文本
/// * `entity` - TDLib 文本实体对象
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

/// 将 TDLib 使用的 UTF-16 偏移量映射为 Rust `&str` 的字节索引（Byte Index）。
///
/// 避免在包含 Emoji、双字节字符等情况下发生字符串切片越界 panic。
///
/// # 参数
/// * `text` - 字符串引用
/// * `target_utf16_offset` - 目标 UTF-16 代码单元偏移量
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

/// 判断给定的会话是否为用户与 Bot 的私聊会话。
///
/// 本项目规定核心管理与交互仅允许在 Bot 私聊中进行，群聊仅作为转存目的地。
///
/// # 参数
/// * `chat_id` - 会话 ID
/// * `sender_user_id` - 发送者用户 ID
fn is_private_interaction_chat(chat_id: i64, sender_user_id: i64) -> bool {
    chat_id == sender_user_id
}

/// 判断某条群聊消息是否属于群聊授权捷径（即回复某条普通用户消息并发送 `/auth`）。
///
/// # 参数
/// * `content` - 消息内容枚举
/// * `has_reply` - 是否包含回复引用
fn is_reply_auth_command(content: &tdlib_rs::enums::MessageContent, has_reply: bool) -> bool {
    if !has_reply {
        return false;
    }
    let tdlib_rs::enums::MessageContent::MessageText(message) = content else {
        return false;
    };
    let mut tokens = message.text.text.split_whitespace();
    let Some(command) = tokens.next() else {
        return false;
    };
    tokens.next().is_none() && normalize_bot_command(command) == "/auth"
}

/// 解析群聊中“回复消息 + /auth”命令的操作者身份。
///
/// 严格拒绝匿名管理员（`MessageSender::Chat`）与无效用户 ID，确保 Owner 校验万无一失。
///
/// # 参数
/// * `request_chat_id` - 所在群聊 ID
/// * `sender` - 消息发送者对象
fn reply_auth_request_actor(
    request_chat_id: i64,
    sender: &tdlib_rs::enums::MessageSender,
) -> anyhow::Result<crate::config::RequestActor> {
    let tdlib_rs::enums::MessageSender::User(sender) = sender else {
        anyhow::bail!(
            "匿名管理员或群组身份无法验证真实用户 ID；请切换为个人用户身份后重新发送 /auth"
        );
    };
    if sender.user_id <= 0 {
        anyhow::bail!("发送者没有有效的用户 ID，无法验证 owner 权限");
    }
    Ok(crate::config::RequestActor {
        request_chat_id,
        user_id: sender.user_id,
    })
}

/// 格式化群聊回复授权失败时的提示文案。
///
/// # 参数
/// * `error` - 错误信息
fn format_group_auth_error(error: &anyhow::Error) -> String {
    format!(
        "群聊授权失败：{error:#}\n请使用个人账号身份发送命令，并确认 config.json 中的 owner_user_id 是你的 Telegram 用户 ID。"
    )
}

/// 向群聊发送授权失败错误提示。
async fn send_group_auth_error_message(
    error: &anyhow::Error,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    crate::tgbot::send::send_text_message(format_group_auth_error(error), chat_id, client_id).await
}

/// 统一解析私聊请求操作者身份（`RequestActor`）。
///
/// 融合静态配置中的 Owner/Admin 白名单以及数据库中的动态授权白名单。
///
/// # 参数
/// * `app` - 全局应用上下文
/// * `config` - 机器人全局配置
/// * `request_chat_id` - 请求来源会话 ID
/// * `sender_user_id` - 发送者用户 ID
fn resolve_request_actor(
    app: &crate::app_context::AppContext,
    config: &crate::config::BotConfig,
    request_chat_id: i64,
    sender_user_id: i64,
) -> Option<crate::config::RequestActor> {
    config
        .request_actor(request_chat_id, sender_user_id)
        .or_else(|| {
            (sender_user_id > 0
                && is_private_interaction_chat(request_chat_id, sender_user_id)
                && app.access_control.is_authorized(sender_user_id))
            .then_some(crate::config::RequestActor {
                request_chat_id,
                user_id: sender_user_id,
            })
        })
}

/// 判断非私聊消息是否需要发送“请在私聊中使用”的提示。
///
/// 仅当消息为斜杠命令时才提醒，普通文字和媒体静默忽略，避免机器人被拉入群后刷屏。
fn should_send_private_only_notice(content: &tdlib_rs::enums::MessageContent) -> bool {
    match content {
        tdlib_rs::enums::MessageContent::MessageText(text) => {
            text.text.text.trim_start().starts_with('/')
        }
        _ => false,
    }
}

/// 发送“仅支持私聊使用”的统一提示消息。
async fn send_private_chat_only_message(chat_id: i64, client_id: i32) -> anyhow::Result<()> {
    crate::tgbot::send::send_text_message(
        "当前只支持私聊 bot 使用；目标群请在私聊菜单中选择。".to_owned(),
        chat_id,
        client_id,
    )
    .await
}

/// 未经授权的交互操作提示文本。
fn unauthorized_interaction_message() -> &'static str {
    "无权限，请联系管理员。"
}

/// 发送无权限提示消息。
async fn send_unauthorized_interaction_message(chat_id: i64, client_id: i32) -> anyhow::Result<()> {
    crate::tgbot::send::send_text_message(
        unauthorized_interaction_message().to_owned(),
        chat_id,
        client_id,
    )
    .await
}

/// 解码 TDLib Callback Query 中的 `payload` 数据。
///
/// TDLib 协议中内联按钮的 `data` 为二进制 bytes，在 JSON 中表示为 Base64；
/// 此处统一将其解码回业务短字符串（如 `m:home`、`d:r:all:8:1`）。
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
                // 兼容历史测试或明文 payload：若非 base64 则保留原字符串
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

/// 返回 TDLib Update 的类型名称简述（供 Trace 日志记录）。
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

/// 判断指定角色是否允许处理交互式 Update。
fn should_process_interactive_update(
    role: crate::config::ClientRole,
    interaction_role: crate::config::ClientRole,
) -> bool {
    role == interaction_role
}

/// 返回消息内容的类型名称简述。
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
        tdlib_rs::enums::MessageContent::MessageUsersShared(_) => "users_shared",
        tdlib_rs::enums::MessageContent::MessageChatShared(_) => "chat_shared",
        _ => "other",
    }
}

/// 构造用于单元测试的模拟 `Message` 实例。
#[cfg(test)]
pub(crate) fn mock_message() -> tdlib_rs::types::Message {
    tdlib_rs::types::Message {
        id: 0,
        sender_id: tdlib_rs::enums::MessageSender::User(Box::new(
            tdlib_rs::types::MessageSenderUser { user_id: 1 },
        )),
        receiver_id: None,
        chat_id: 0,
        sending_state: None,
        scheduling_state: None,
        is_outgoing: false,
        is_pinned: false,
        is_from_offline: false,
        can_be_saved: true,
        has_timestamped_media: false,
        is_channel_post: false,
        is_paid_star_suggested_post: false,
        is_paid_gram_suggested_post: false,
        contains_unread_mention: false,
        contains_unread_poll_votes: false,
        date: 0,
        edit_date: 0,
        forward_info: None,
        import_info: None,
        interaction_info: None,
        unread_reactions: Vec::new(),
        fact_check: None,
        suggested_post_info: None,
        reply_to: None,
        topic_id: None,
        self_destruct_type: None,
        self_destruct_in: 0.0,
        auto_delete_in: 0.0,
        via_bot_user_id: 0,
        guest_bot_caller_id: None,
        sender_business_bot_user_id: 0,
        sender_boost_count: 0,
        sender_tag: String::new(),
        paid_message_star_count: 0,
        author_signature: String::new(),
        media_album_id: 0,
        effect_id: 0,
        restriction_info: None,
        summary_language_code: String::new(),
        content: tdlib_rs::enums::MessageContent::MessageText(Box::new(
            tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: String::new(),
                    entities: Vec::new(),
                },
                link_preview: None,
                link_preview_options: None,
            },
        )),
        ephemeral_content: None,
        reply_markup: None,
        ephemeral_message_id: 0,
        chat_instance: 0,
    }
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose};

    use super::{
        command_error_hint, decode_callback_query_payload, extract_direct_transfer_link,
        format_group_auth_error, is_private_interaction_chat, is_reply_auth_command,
        normalize_bot_command, reply_auth_request_actor, resolve_request_actor,
        should_process_interactive_update, should_send_private_only_notice,
        unauthorized_interaction_message,
    };
    use crate::app_context::AppContext;
    use crate::config::{BotConfig, ClientRole, RequestActor};

    /// 动态授权必须复用私聊权限入口，不能让文本消息和 callback 各自判断一套名单。
    #[test]
    fn test_runtime_authorized_user_uses_same_private_chat_gate() {
        let app = AppContext::default();
        let config = BotConfig {
            owner_user_id: 1,
            ..BotConfig::default()
        };
        app.access_control.replace_authorized_user_ids([2]);

        assert_eq!(
            resolve_request_actor(&app, &config, 2, 2),
            Some(RequestActor {
                request_chat_id: 2,
                user_id: 2,
            })
        );
        assert!(resolve_request_actor(&app, &config, -1002, 2).is_none());
        assert!(resolve_request_actor(&app, &config, 1, 1).is_some());
        assert!(resolve_request_actor(&app, &config, 3, 3).is_none());
    }

    /// TDLib JSON 协议会用 base64 表示 callback bytes；入口应解回业务短 payload。
    #[test]
    fn test_decode_callback_query_payload() {
        let mut update = tdlib_rs::types::UpdateNewCallbackQuery {
            id: 1,
            sender_user_id: 2,
            chat_id: 3,
            message_id: 4,
            chat_instance: 5,
            payload: tdlib_rs::enums::CallbackQueryPayload::Data(Box::new(
                tdlib_rs::types::CallbackQueryPayloadData {
                    data: general_purpose::STANDARD.encode("m:home"),
                },
            )),
        };

        decode_callback_query_payload(&mut update);

        let tdlib_rs::enums::CallbackQueryPayload::Data(data) = update.payload else {
            panic!("payload must be data");
        };
        assert_eq!(data.data, "m:home");
    }

    /// 兼容已有测试构造的明文 payload，避免单元测试和未来绑定差异直接崩掉。
    #[test]
    fn test_decode_callback_query_payload_keeps_plain_text() {
        let mut update = tdlib_rs::types::UpdateNewCallbackQuery {
            id: 1,
            sender_user_id: 2,
            chat_id: 3,
            message_id: 4,
            chat_instance: 5,
            payload: tdlib_rs::enums::CallbackQueryPayload::Data(Box::new(
                tdlib_rs::types::CallbackQueryPayloadData {
                    data: "m:home".to_owned(),
                },
            )),
        };

        decode_callback_query_payload(&mut update);

        let tdlib_rs::enums::CallbackQueryPayload::Data(data) = update.payload else {
            panic!("payload must be data");
        };
        assert_eq!(data.data, "m:home");
    }

    /// bot 在群里收到的命令可能带 username 后缀；路由前必须归一成基础命令。
    #[test]
    fn test_normalize_bot_command() {
        assert_eq!(normalize_bot_command("/t"), "/t");
        assert_eq!(normalize_bot_command("/t@TransferBot"), "/t");
        assert_eq!(normalize_bot_command("/help@TransferBot"), "/help");
        assert_eq!(normalize_bot_command("/cancel@TransferBot"), "/cancel");
    }

    /// user client 只用于链接读取/下载 fallback，不应处理普通消息或 callback。
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

    /// 项目只支持 bot 私聊交互；群聊里 chat_id 与 sender_user_id 不同，必须拒绝。
    #[test]
    fn test_private_interaction_chat_only() {
        assert!(is_private_interaction_chat(100, 100));
        assert!(!is_private_interaction_chat(-100123, 100));
        assert!(!is_private_interaction_chat(200, 100));
    }

    /// 群聊回复授权必须按发送者 user_id 校验 owner，不能把负数群 ID 当成用户身份。
    #[test]
    fn test_reply_auth_request_actor_uses_sender_user_id() {
        let sender =
            tdlib_rs::enums::MessageSender::User(Box::new(tdlib_rs::types::MessageSenderUser {
                user_id: 123456,
            }));

        assert_eq!(
            reply_auth_request_actor(-100987654, &sender).unwrap(),
            RequestActor {
                request_chat_id: -100987654,
                user_id: 123456,
            }
        );
    }

    /// 匿名管理员 update 只有 chat_id，没有可验证的真实用户 ID，必须给出明确操作提示。
    #[test]
    fn test_reply_auth_request_actor_rejects_anonymous_admin() {
        let sender =
            tdlib_rs::enums::MessageSender::Chat(Box::new(tdlib_rs::types::MessageSenderChat {
                chat_id: -100987654,
            }));

        let error = reply_auth_request_actor(-100987654, &sender).unwrap_err();
        assert!(error.to_string().contains("匿名管理员"));
        assert!(error.to_string().contains("用户身份"));
    }

    /// 群聊授权错误不能附带只能在私聊使用的菜单按钮，应直接给出可执行的身份检查步骤。
    #[test]
    fn test_group_auth_error_is_actionable_without_private_menu() {
        let text = format_group_auth_error(&anyhow::anyhow!("仅 owner 可管理授权"));

        assert!(text.contains("仅 owner 可管理授权"));
        assert!(text.contains("owner_user_id"));
        assert!(text.contains("个人账号身份"));
        assert!(!text.contains("打开菜单"));
    }

    /// 验证无权限提示包含操作性指引。
    #[test]
    fn test_unauthorized_interaction_notice_is_actionable() {
        let message = unauthorized_interaction_message();

        assert!(message.contains("无权限"));
        assert!(message.contains("管理员"));
    }

    /// 群聊里只对命令回复“请私聊”，普通文本和媒体应静默忽略，避免刷屏。
    #[test]
    fn test_private_only_notice_only_for_commands() {
        let command =
            tdlib_rs::enums::MessageContent::MessageText(Box::new(tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: " /menu".to_owned(),
                    entities: vec![],
                },
                link_preview: None,
                link_preview_options: None,
            }));
        let text =
            tdlib_rs::enums::MessageContent::MessageText(Box::new(tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: "hello".to_owned(),
                    entities: vec![],
                },
                link_preview: None,
                link_preview_options: None,
            }));
        let non_text = tdlib_rs::enums::MessageContent::MessageBasicGroupChatCreate(Box::default());

        assert!(should_send_private_only_notice(&command));
        assert!(!should_send_private_only_notice(&text));
        assert!(!should_send_private_only_notice(&non_text));
    }

    /// 群聊只为“回复某人 + /auth”开放窄入口，其他命令仍要求私聊。
    #[test]
    fn test_reply_auth_command_is_narrow_group_exception() {
        let text_content = |text: &str| {
            tdlib_rs::enums::MessageContent::MessageText(Box::new(tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: text.to_owned(),
                    entities: vec![],
                },
                link_preview: None,
                link_preview_options: None,
            }))
        };

        assert!(is_reply_auth_command(&text_content("/auth"), true));
        assert!(is_reply_auth_command(
            &text_content("/auth@transfer_bot"),
            true
        ));
        assert!(!is_reply_auth_command(&text_content("/auth"), false));
        assert!(!is_reply_auth_command(&text_content("/auth list"), true));
        assert!(!is_reply_auth_command(&text_content("hello"), true));
    }

    /// 单独一条 Telegram 链接文本应直接进入目标选择，不需要先手输 /transfer。
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

    /// 隐藏链接文本也应能提取出真实 Telegram URL。
    #[test]
    fn test_extract_direct_transfer_link_from_text_url_entity() {
        let message_text = tdlib_rs::types::MessageText {
            text: tdlib_rs::types::FormattedText {
                text: "点我打开".to_owned(),
                entities: vec![tdlib_rs::types::TextEntity {
                    offset: 0,
                    length: 4,
                    r#type: tdlib_rs::enums::TextEntityType::TextUrl(Box::new(
                        tdlib_rs::types::TextEntityTypeTextUrl {
                            url: "https://t.me/c/123/456".to_owned(),
                        },
                    )),
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

    /// Telegram 链接预览消息也应能作为直接入口。
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
                r#type: tdlib_rs::enums::LinkPreviewType::Article(Box::new(
                    tdlib_rs::types::LinkPreviewTypeArticle { photo: None },
                )),
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

    /// UTF-16 实体切片必须正确处理 emoji 等双单元字符，避免 URL 实体定位错位。
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

    /// 私有源不可读时，应提示 bot 与备用 user 的访问前提。
    #[test]
    fn test_command_error_hint_for_source_access() {
        let hint = command_error_hint("code=400, message=Message not found");

        assert_eq!(hint.title, "源不可访问");
        assert!(hint.advice.contains("备用 user"));
        assert!(hint.advice.contains("备用 user"));
    }

    /// 缺少目标时应直接进入交互式转存，不再要求复制和补全命令模板。
    #[test]
    fn test_command_error_hint_for_missing_target_starts_interactive_transfer() {
        let hint = command_error_hint("not found transfer target");

        assert_eq!(hint.primary_label, "选择目标转存");
        assert_eq!(
            hint.primary_action,
            crate::tgbot::error::CommandErrorPrimaryAction::StartTransfer
        );
    }

    /// 未分类错误仍保留通用排查建议。
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
