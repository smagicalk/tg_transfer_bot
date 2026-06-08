// `/transfer` 命令实现。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;

use super::build_downloads_status_button_data;
use super::common::{CommandStyle, downloads_command, lookup_command, resolve_target_chat_id};
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
    // 日志只记录请求定位和目标 chat；源链接会回显给用户，但不写入日志文件。
    tracing::info!(
        request_chat_id,
        request_message_id,
        target_chat_id,
        "transfer command accepted"
    );

    // 先给用户一个即时反馈，避免长时间下载/上传期间命令看起来像“卡住了”。
    let lookup_command =
        lookup_command(&plan.source_link, plan.target_chat_id, CommandStyle::Short);
    let progress_message = send::send_card_message_with_buttons_returning(
        format_transfer_accepted_text(&plan),
        request_chat_id,
        vec![vec![
            send::build_callback_button(
                "查看运行列表",
                &build_downloads_status_button_data("running", 8),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制源链接",
                &plan.source_link,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制查询命令",
                &lookup_command,
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

/// 构造 `/transfer` 首次回执卡片。
///
/// 后台任务启动后会持续编辑同一条消息，因此初始卡片也使用 card 格式，避免样式闪变。
fn format_transfer_accepted_text(plan: &TransferPlan) -> String {
    [
        "已接收转存请求".to_owned(),
        card::status_target("queued", plan.target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("进度"),
        "后台会自动下载并上传，本消息会持续刷新。".to_owned(),
        card::section("命令"),
        card::command_line(
            "查询",
            lookup_command(&plan.source_link, plan.target_chat_id, CommandStyle::Short),
        ),
        card::command_line(
            "列表",
            downloads_command(Some("run"), None, None, CommandStyle::Short),
        ),
        String::new(),
    ]
    .into_iter()
    .chain(card::source_block(&plan.source_link))
    .collect::<Vec<_>>()
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::format_transfer_accepted_text;
    use crate::tgbot::transfer::types::TransferPlan;

    // 首次回执应直接使用卡片标记，后续编辑不会从 Markdown 样式跳到 card 样式。
    #[test]
    fn test_format_transfer_accepted_text() {
        let text = format_transfer_accepted_text(&TransferPlan {
            source_link: "https://t.me/c/1/2".to_owned(),
            target_chat_id: -100,
            request_chat_id: 1,
            request_message_id: 2,
        });

        assert!(text.contains("状态：‹queued›"));
        assert!(text.contains("目标：‹-100›"));
        assert!(text.contains("‹https://t.me/c/1/2›"));
    }
}
