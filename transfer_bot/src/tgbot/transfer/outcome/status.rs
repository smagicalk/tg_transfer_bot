// 转存任务中间状态回复卡片。
// 这些状态不会携带错误堆栈，只提供 job_id 和下一步控制命令。

use super::super::card;
use super::super::command::common::{
    CommandStyle, downloads_command as build_downloads_command, job_command as build_job_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};
use super::super::command::{
    build_downloads_status_button_data, build_job_pause_button_data, build_job_resume_button_data,
    build_job_status_button_data, build_job_stop_button_data,
};

/// 发送“任务已暂停”的状态卡片。
pub(in crate::tgbot::transfer) async fn send_paused_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    crate::tgbot::send::ReplyPanel::card(format_status_card_text(
        title,
        "paused",
        source_link,
        target_chat_id,
        job_id,
        "可手动恢复或停止该任务。",
    ))
    .row(vec![
        crate::tgbot::send::build_callback_button(
            "查看任务详情",
            &build_job_status_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_callback_button(
            "恢复",
            &build_job_resume_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_callback_button(
            "停止",
            &build_job_stop_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制恢复",
            &build_job_command("r", job_id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制停止",
            &build_job_command("s", job_id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .row(vec![
        crate::tgbot::send::build_callback_button(
            "查看暂停列表",
            &build_downloads_status_button_data("paused", 8),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制列表命令",
            &build_downloads_command(Some("pause"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(notify_chat_id, client_id)
    .await
}

/// 发送“任务正在停止”的状态卡片。
pub(in crate::tgbot::transfer) async fn send_cancelling_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    crate::tgbot::send::ReplyPanel::card(format_status_card_text(
        title,
        "cancelling",
        source_link,
        target_chat_id,
        job_id,
        "当前调用会在安全点收尾。",
    ))
    .row(vec![
        crate::tgbot::send::build_callback_button(
            "查看任务详情",
            &build_job_status_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_callback_button(
            "查看停止列表",
            &build_downloads_status_button_data("cancelling", 8),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制 job_id",
            &job_id.to_string(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .row(vec![crate::tgbot::send::build_copy_button(
        "复制列表命令",
        &build_downloads_command(Some("cancel"), None, None, CommandStyle::Short),
        tdlib_rs::enums::ButtonStyle::Default,
    )])
    .send(notify_chat_id, client_id)
    .await
}

/// 发送“任务已停止”的状态卡片。
pub(in crate::tgbot::transfer) async fn send_cancelled_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    crate::tgbot::send::ReplyPanel::card(format_status_card_text(
        title,
        "cancelled",
        source_link,
        target_chat_id,
        job_id,
        "文件引用已释放，后续由删除队列清理。",
    ))
    .row(vec![
        crate::tgbot::send::build_callback_button(
            "查看任务详情",
            &build_job_status_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_callback_button(
            "查看停止列表",
            &build_downloads_status_button_data("cancelled", 8),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制 job_id",
            &job_id.to_string(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .row(vec![crate::tgbot::send::build_copy_button(
        "复制列表命令",
        &build_downloads_command(Some("cancel"), None, None, CommandStyle::Short),
        tdlib_rs::enums::ButtonStyle::Default,
    )])
    .send(notify_chat_id, client_id)
    .await
}

/// 发送“任务仍在运行中”的状态卡片。
pub(in crate::tgbot::transfer) async fn send_running_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let lookup_command = build_lookup_command(source_link, target_chat_id, CommandStyle::Short);
    let transfer_command = build_transfer_command(source_link, target_chat_id, CommandStyle::Short);
    crate::tgbot::send::ReplyPanel::card(format_status_card_text(
        title,
        "running",
        source_link,
        target_chat_id,
        job_id,
        &format!("建议：使用 {} 查看后台进度。", card::code("/d run")),
    ))
    .row(vec![
        crate::tgbot::send::build_callback_button(
            "查看任务详情",
            &build_job_status_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_callback_button(
            "暂停",
            &build_job_pause_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_callback_button(
            "停止",
            &build_job_stop_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .row(vec![
        crate::tgbot::send::build_callback_button(
            "查看运行列表",
            &build_downloads_status_button_data("running", 8),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制 job_id",
            &job_id.to_string(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制暂停",
            &build_job_command("p", job_id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制运行列表命令",
            &build_downloads_command(Some("run"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制查询命令",
            &lookup_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .row(vec![crate::tgbot::send::build_copy_button(
        "复制重新转存",
        &transfer_command,
        tdlib_rs::enums::ButtonStyle::Default,
    )])
    .send(notify_chat_id, client_id)
    .await
}

/// 构造任务中间状态卡片。
fn format_status_card_text(
    title: &str,
    status: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    detail: &str,
) -> String {
    let mut lines = vec![
        title.to_owned(),
        card::summary_line(status, Some(job_id), target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("说明"),
        card::note(detail),
        card::section("命令"),
    ];
    lines.extend(status_command_lines(
        status,
        source_link,
        target_chat_id,
        job_id,
    ));
    lines.push(String::new());
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

/// 根据任务状态生成正文命令。
///
/// 按钮只在 bot token 模式稳定可用；正文命令是用户号模式和日志截图排查时的兜底入口。
fn status_command_lines(
    status: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
) -> Vec<String> {
    let mut lines = vec![
        card::command_line("详情", build_job_command("st", job_id, CommandStyle::Short)),
        card::command_line(
            "查询",
            build_lookup_command(source_link, target_chat_id, CommandStyle::Short),
        ),
    ];

    match status {
        "paused" => {
            lines.push(card::command_line(
                "恢复",
                build_job_command("r", job_id, CommandStyle::Short),
            ));
            lines.push(card::command_line(
                "停止",
                build_job_command("s", job_id, CommandStyle::Short),
            ));
            lines.push(card::command_line(
                "列表",
                build_downloads_command(Some("pause"), None, None, CommandStyle::Short),
            ));
        }
        "cancelling" | "cancel_finalizing" | "cancelled" => {
            lines.push(card::command_line(
                "列表",
                build_downloads_command(Some("cancel"), None, None, CommandStyle::Short),
            ));
        }
        _ => {
            lines.push(card::command_line(
                "暂停",
                build_job_command("p", job_id, CommandStyle::Short),
            ));
            lines.push(card::command_line(
                "停止",
                build_job_command("s", job_id, CommandStyle::Short),
            ));
            lines.push(card::command_line(
                "列表",
                build_downloads_command(Some("run"), None, None, CommandStyle::Short),
            ));
        }
    }

    lines.push(card::command_line(
        "重转",
        build_transfer_command(source_link, target_chat_id, CommandStyle::Short),
    ));
    lines
}

#[cfg(test)]
mod tests {
    use super::{format_status_card_text, status_command_lines};

    // 后台状态卡片应使用 card 标记展示状态、job 和来源。
    #[test]
    fn test_format_status_card_text() {
        let text = format_status_card_text(
            "任务仍在运行中",
            "running",
            "https://t.me/c/1/2",
            -100,
            42,
            "建议查看运行列表。",
        );

        assert!(text.contains("状态：‹running›"));
        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("‹https://t.me/c/1/2›"));
        assert!(text.contains("详情：‹/j st 42›"));
        assert!(text.contains("列表：‹/d run›"));
    }

    // paused/cancelled 状态应生成对应列表和控制命令，不能继续展示无效暂停命令。
    #[test]
    fn test_status_command_lines_by_status() {
        let paused = status_command_lines("paused", "https://t.me/c/1/2", -100, 42).join("\n");
        let cancelled =
            status_command_lines("cancelled", "https://t.me/c/1/2", -100, 42).join("\n");

        assert!(paused.contains("恢复：‹/j r 42›"));
        assert!(paused.contains("列表：‹/d pause›"));
        assert!(cancelled.contains("列表：‹/d cancel›"));
        assert!(!cancelled.contains("暂停：‹/j p 42›"));
    }
}
