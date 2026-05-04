// `/lookup` 命令实现。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::refresh_stored_result_link;
use crate::tgbot::transfer::store;

use super::common::{
    CommandStyle, job_command as build_job_command, lookup_command as build_lookup_command,
    resolve_target_chat_id, transfer_command as build_transfer_command,
};
use super::{build_downloads_status_button_data, build_job_status_button_data};

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
        result_row.push(send::build_callback_button(
            "查看任务详情",
            &build_job_status_button_data(job.id),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
        result_row.push(send::build_copy_button(
            "复制查询命令",
            &lookup_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ));

        return send::ReplyPanel::card(crate::tgbot::transfer::outcome::format_result_card_text(
            "已找到历史转存结果",
            &source_link,
            target_chat_id,
            Some(job.id),
            &link,
        ))
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
        return send::ReplyPanel::card(format_lookup_active_text(
            &source_link,
            target_chat_id,
            job.id,
            &job.status,
        ))
        .row(vec![
            send::build_callback_button(
                "查看任务详情",
                &build_job_status_button_data(job.id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "返回列表",
                &build_downloads_status_button_data(&job.status, 8),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .row(vec![
            send::build_copy_button(
                "复制暂停",
                &build_job_command("p", job.id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制停止",
                &build_job_command("s", job.id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制 job_id",
                &job.id.to_string(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .row(vec![
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
            send::build_copy_button(
                "复制重新转存",
                &transfer_command,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .send(request_chat_id, client_id)
        .await;
    }

    tracing::info!(request_chat_id, target_chat_id, "lookup command missed");
    send::ReplyPanel::card(format_lookup_miss_text(&source_link, target_chat_id))
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

/// 构造命中进行中任务时的查询卡片。
fn format_lookup_active_text(
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    status: &str,
) -> String {
    let mut lines = vec![
        "找到进行中的转存任务".to_owned(),
        card::status_job_target(status, job_id, target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("下一步"),
        format!(
            "可直接复制暂停/停止命令，或使用 {} 查看运行列表。",
            card::code("/d run")
        ),
        String::new(),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

/// 构造未命中历史结果时的查询卡片。
fn format_lookup_miss_text(source_link: &str, target_chat_id: i64) -> String {
    let mut lines = vec![
        "未找到转存结果".to_owned(),
        card::status_target("miss", target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("下一步"),
        "复制转存命令后重新发起任务。".to_owned(),
        String::new(),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{format_lookup_active_text, format_lookup_miss_text};

    // lookup 命中运行中任务时应使用 card 标记，避免 Markdown 原文泄露到消息里。
    #[test]
    fn test_format_lookup_active_text() {
        let text = format_lookup_active_text("https://t.me/c/1/2", -100, 42, "running");

        assert!(text.contains("状态：‹running›"));
        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("‹/d run›"));
        assert!(text.contains("暂停/停止命令"));
    }

    // lookup 未命中时应保留源链接并给出 miss 状态。
    #[test]
    fn test_format_lookup_miss_text() {
        let text = format_lookup_miss_text("https://t.me/c/1/2", -100);

        assert!(text.contains("状态：‹miss›"));
        assert!(text.contains("‹https://t.me/c/1/2›"));
    }
}
