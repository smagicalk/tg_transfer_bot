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
            tracing::info!("get version success, version={:#?}", version);
            Ok(())
        }
        Err(err) => anyhow::bail!("get_version failed, error={:?}", err),
    }
}

// 设置 TDLib 日志级别。
pub async fn set_log(client_id: i32) {
    let _ = tdlib_rs::functions::set_log_verbosity_level(1, client_id).await;
}

// 主循环：持续接收 TDLib update 并异步处理。
pub async fn receive(config: std::sync::Arc<crate::config::BotConfig>) -> anyhow::Result<()> {
    loop {
        let receive = tokio::task::spawn_blocking(tdlib_rs::receive).await?;
        match receive {
            None => {}
            Some((msg_update, _client_id)) => {
                let config = config.clone();
                tokio::spawn(async move {
                    let res = handle_update(msg_update, config.clone()).await;
                    if let Err(err) = res {
                        tracing::error!("Received error: {}", err);
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
// - File => 调试日志
pub async fn handle_update(
    update: Update,
    config: std::sync::Arc<crate::config::BotConfig>,
) -> anyhow::Result<()> {
    // 授权状态更新：交给登录处理逻辑。
    if let Update::AuthorizationState(update) = update {
        handle_authorization(update.authorization_state, config).await?;
        return Ok(());
    }

    // 新消息更新：执行命令分发。
    if let Update::NewMessage(update_new_message) = update {
        let message = update_new_message.message;
        let chat_id = message.chat_id;

        // 忽略进程启动前消息，避免重复处理。
        if message.date < *START_TS {
            return Ok(());
        }

        // 解析发送者 ID，用于管理员白名单校验。
        let sender_id = match &message.sender_id {
            tdlib_rs::enums::MessageSender::User(user) => user.user_id,
            tdlib_rs::enums::MessageSender::Chat(chat_id) => chat_id.chat_id,
        };

        // 仅允许管理员 chat 且发送者也在管理员列表中。
        if !(config.admin_ids.contains(&chat_id) && config.admin_ids.contains(&sender_id)) {
            return Ok(());
        }

        let message_content = message.content;
        let client_id = config.client_id;

        // 当前仅处理文本消息。
        if let tdlib_rs::enums::MessageContent::MessageText(message_text) = message_content {
            let text = message_text
                .text
                .text
                .split_whitespace()
                .collect::<Vec<&str>>();
            if text.is_empty() {
                if let Some(client_id) = client_id {
                    crate::tgbot::send::send_text_message(
                        "not input".to_owned(),
                        chat_id,
                        client_id,
                    )
                    .await?;
                }
                return Ok(());
            }

            if text[0].starts_with("/") {
                let client_id = client_id.ok_or_else(|| anyhow::anyhow!("not found client_id"))?;

                match text[0] {
                    // /help 命令入口。
                    // 返回机器人当前支持的命令说明。
                    "/help" | "/h" => {
                        tgbot::transfer::help_command(text, chat_id, client_id).await?;
                    }
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
                        .await?;
                    }
                    // /lookup 命令入口。
                    // 按源链接查找历史转存结果。
                    "/lookup" | "/lk" => {
                        tgbot::transfer::lookup_command(text, config.clone(), chat_id, client_id)
                            .await?;
                    }
                    // /config 命令入口。
                    // 仅开放运行时安全可调的配置项。
                    "/config" | "/cfg" => {
                        tgbot::transfer::config_command(text, chat_id, client_id).await?;
                    }
                    // /downloads 命令入口。
                    // 展示当前聊天最近的转存任务进度列表。
                    "/downloads" | "/d" => {
                        tgbot::transfer::downloads_command(text, chat_id, client_id).await?;
                    }
                    // /job 命令入口。
                    // 手动暂停、恢复、停止指定转存任务。
                    "/job" | "/j" => {
                        tgbot::transfer::job_command(text, chat_id, client_id).await?;
                    }
                    _ => {}
                }
            }
        }
        return Ok(());
    }

    // inline keyboard 回调：目前用于 `/downloads` 分页。
    if let Update::NewCallbackQuery(update_callback_query) = update {
        // 只接受管理员在管理员聊天里点击的按钮。
        if !(config.admin_ids.contains(&update_callback_query.chat_id)
            && config
                .admin_ids
                .contains(&update_callback_query.sender_user_id))
        {
            return Ok(());
        }

        let client_id = config
            .client_id
            .ok_or_else(|| anyhow::anyhow!("not found client_id"))?;
        tgbot::transfer::downloads_callback_query(update_callback_query, client_id).await?;
        return Ok(());
    }

    // 文件更新目前仅记录调试日志。
    if let Update::File(update_file) = update {
        // 将 TDLib 实时文件进度写入内存快照，供 `/downloads` 查询。
        queue::update_download_progress(&update_file.file);
        tracing::debug!("\n{:?}", update_file);
    }

    Ok(())
}
