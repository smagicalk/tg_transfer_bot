// `/lookup` 命令实现。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::refresh_stored_result_link;
use crate::tgbot::transfer::store;

use super::common::{
    CommandStyle, lookup_command as build_lookup_command, resolve_target_chat_id,
    transfer_command as build_transfer_command,
};

/// `/lookup` 命令入口。
/// 命令格式：`/lookup <link> [target_chat_id]`
/// 用于按源链接查询历史转存结果。
pub async fn lookup_command(
    text: Vec<&str>,
    config: Arc<BotConfig>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    if text.len() < 2 {
        anyhow::bail!("usage: /lookup <link> [target_chat_id]");
    }

    let source_link = text[1].to_string();
    let target_chat_id = resolve_target_chat_id(&text, &config, request_chat_id)?;
    // 源链接可能来自私有聊天，日志只记录请求 chat 与目标 chat。
    tracing::info!(request_chat_id, target_chat_id, "lookup command started");
    let lookup_command = build_lookup_command(&source_link, target_chat_id, CommandStyle::Short);
    let transfer_command =
        build_transfer_command(&source_link, target_chat_id, CommandStyle::Short);

    if let Some(job) =
        store::find_success_job_by_source_target(&source_link, target_chat_id).await?
    {
        let link = refresh_stored_result_link(
            job.id,
            job.target_chat_id,
            job.result_message_id,
            &job.result_message_link,
            client_id,
        )
        .await?;
        tracing::info!(
            request_chat_id,
            target_chat_id,
            job_id = job.id,
            "lookup command hit success job"
        );
        let mut result_row = Vec::new();
        if send::is_openable_url(&link) {
            result_row.push(send::build_url_button(
                "打开转存消息",
                &link,
                tdlib_rs::enums::ButtonStyle::Primary,
            ));
        }
        result_row.push(send::build_copy_button(
            if send::is_openable_url(&link) {
                "复制结果链接"
            } else {
                "复制结果定位"
            },
            &link,
            if send::is_openable_url(&link) {
                tdlib_rs::enums::ButtonStyle::Default
            } else {
                tdlib_rs::enums::ButtonStyle::Primary
            },
        ));
        result_row.push(send::build_copy_button(
            "复制查询命令",
            &lookup_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ));

        return send::ReplyPanel::markdown(
            crate::tgbot::transfer::outcome::format_result_card_text(
                "已找到历史转存结果",
                &source_link,
                target_chat_id,
                &link,
            ),
        )
        .row(result_row)
        .row(vec![send::build_copy_button(
            "复制重新转存",
            &transfer_command,
            tdlib_rs::enums::ButtonStyle::Default,
        )])
        .send(request_chat_id, client_id)
        .await;
    }

    if let Some(job) = store::find_active_job_by_source_target(&source_link, target_chat_id).await?
    {
        tracing::info!(
            request_chat_id,
            target_chat_id,
            job_id = job.id,
            status = %job.status,
            "lookup command hit active job"
        );
        return send::ReplyPanel::markdown(format!(
            "*找到进行中的转存任务*\n状态：`{}`  job：`#{}`\n目标：`{}`\n━━━━━━━━━━━━\n*下一步*\n使用 `/d run` 查看运行列表，或复制 job_id 手动暂停/停止。\n\n*来源*\n`{}`",
            job.status,
            job.id,
            target_chat_id,
            markdown_inline_code(&source_link)
        ))
        .row(vec![
            send::build_copy_button(
                "复制 job_id",
                &job.id.to_string(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制运行列表",
                "/d run",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制查询命令",
                &lookup_command,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .row(vec![send::build_copy_button(
            "复制重新转存",
            &transfer_command,
            tdlib_rs::enums::ButtonStyle::Default,
        )])
        .send(request_chat_id, client_id)
        .await;
    }

    tracing::info!(request_chat_id, target_chat_id, "lookup command missed");
    send::ReplyPanel::markdown(format!(
        "*未找到转存结果*\n状态：`miss`  目标：`{}`\n━━━━━━━━━━━━\n*下一步*\n复制转存命令后重新发起任务。\n\n*来源*\n`{}`",
        target_chat_id,
        markdown_inline_code(&source_link)
    ))
    .row(vec![
        send::build_copy_button(
            "复制转存命令",
            &transfer_command,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_copy_button(
            "复制查询命令",
            &lookup_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_copy_button(
            "复制源链接",
            &source_link,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(request_chat_id, client_id)
    .await
}

/// 转义 Markdown 行内代码里的反引号，避免用户输入链接破坏卡片格式。
fn markdown_inline_code(text: &str) -> String {
    text.replace('`', "'")
}
