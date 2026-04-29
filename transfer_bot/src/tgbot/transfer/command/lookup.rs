// `/lookup` 命令实现。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;
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
        let link = job.result_message_link;
        tracing::info!(
            request_chat_id,
            target_chat_id,
            job_id = job.id,
            "lookup command hit success job"
        );
        return send::ReplyPanel::markdown(format!(
            "*已找到历史转存结果*\n源链接：`{}`\n目标 chat：`{}`\n结果消息：[打开转存消息]({})",
            source_link, target_chat_id, link
        ))
        .row(vec![
            send::build_url_button("打开结果", &link, tdlib_rs::enums::ButtonStyle::Primary),
            send::build_copy_button("复制结果链接", &link, tdlib_rs::enums::ButtonStyle::Default),
            send::build_copy_button(
                "复制查询命令",
                &lookup_command,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .row(vec![send::build_copy_button(
            "复制重转命令",
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
            "*找到进行中的任务*\n源链接：`{}`\n目标 chat：`{}`\njob_id：`{}`\n建议：使用 `/d run` 或 `/d all` 查看进度。",
            source_link, target_chat_id, job.id
        ))
        .row(vec![
            send::build_copy_button(
                "复制 job_id",
                &job.id.to_string(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制 /d run",
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
            "复制重转命令",
            &transfer_command,
            tdlib_rs::enums::ButtonStyle::Default,
        )])
        .send(request_chat_id, client_id)
        .await;
    }

    tracing::info!(request_chat_id, target_chat_id, "lookup command missed");
    send::ReplyPanel::markdown(format!(
        "*未找到历史转存结果*\n源链接：`{}`\n目标 chat：`{}`\n可直接执行下面的转存命令重新发起任务。",
        source_link, target_chat_id
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
