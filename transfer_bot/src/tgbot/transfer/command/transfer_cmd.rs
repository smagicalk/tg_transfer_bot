// `/transfer` 命令实现。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::common::{CommandStyle, lookup_command, resolve_target_chat_id};
use crate::tgbot::transfer::types::TransferPlan;

/// `/transfer` 命令入口。
/// 命令格式：`/transfer <link> [target_chat_id]`
pub async fn transfer_command(
    text: Vec<&str>,
    config: Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    if text.len() < 2 {
        anyhow::bail!("usage: /transfer <link> [target_chat_id]");
    }

    let source_link = text[1].to_string();
    let target_chat_id = resolve_target_chat_id(&text, &config, request_chat_id)?;

    let plan = TransferPlan {
        source_link,
        target_chat_id,
        request_chat_id,
        request_message_id,
    };

    // 先给用户一个即时反馈，避免长时间下载/上传期间命令看起来像“卡住了”。
    let lookup_command =
        lookup_command(&plan.source_link, plan.target_chat_id, CommandStyle::Short);
    let progress_message = send::send_markdown_message_with_buttons_returning(
        format!(
            "*已接收转存请求*\n源链接：`{}`\n目标 chat：`{}`\n状态：后台处理中，可稍后查看运行列表。",
            plan.source_link, plan.target_chat_id
        ),
        request_chat_id,
        vec![vec![
            send::build_copy_button(
                "复制源链接",
                &plan.source_link,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制查询命令",
                &lookup_command,
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制 /d run",
                "/d run",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ]],
        client_id,
    )
    .await?;
    // 后台任务会持续编辑这条消息，把它变成转存进度面板。
    super::super::spawn_transfer_job(plan, request_chat_id, Some(progress_message.id), client_id);
    Ok(())
}
