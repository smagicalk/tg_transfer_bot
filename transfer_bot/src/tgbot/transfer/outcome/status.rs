// 转存任务中间状态回复卡片。
// 这些状态不会携带错误堆栈，只提供 job_id 和下一步控制命令。

use super::super::card;
use super::super::command::common::{
    CommandStyle, downloads_command as build_downloads_command, job_command as build_job_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};
use super::super::command::{
    build_downloads_status_button_data, build_job_pause_button_data, build_job_resume_button_data,
    build_job_status_button_data, build_job_stop_button_data, build_menu_home_button_data,
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
    .rows(build_status_button_rows("paused", job_id))
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
    .rows(build_status_button_rows("cancelling", job_id))
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
    .rows(build_status_button_rows("cancelled", job_id))
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
    crate::tgbot::send::ReplyPanel::card(format_status_card_text(
        title,
        "running",
        source_link,
        target_chat_id,
        job_id,
        &format!("建议：使用 {} 查看后台进度。", card::code("/downloads run")),
    ))
    .rows(build_status_button_rows("running", job_id))
    .send(notify_chat_id, client_id)
    .await
}

/// 构造中间状态卡片按钮。
///
/// 所有状态统一为：第一行任务主操作，第二行列表/菜单导航。
/// 任务详情和正文命令已经能覆盖后续动作，这里不再重复堆叠 `job_id` 复制按钮。
pub(in crate::tgbot::transfer) fn build_status_button_rows(
    status: &str,
    job_id: i64,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        build_job_action_row(status, job_id),
        vec![
            crate::tgbot::send::build_callback_button(
                status_list_label(status),
                &build_downloads_status_button_data(status, 8),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            crate::tgbot::send::build_callback_button(
                "菜单",
                &build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ]
}

/// 构造任务操作行。
///
/// 这行只承载和单个任务直接相关的操作；列表与菜单导航统一由上层单独拼接。
pub(in crate::tgbot::transfer) fn build_job_action_row(
    status: &str,
    job_id: i64,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    let mut action_row = vec![crate::tgbot::send::build_callback_button(
        "查看任务详情",
        &build_job_status_button_data(job_id),
        tdlib_rs::enums::ButtonStyle::Primary,
    )];

    match status {
        "paused" => {
            action_row.push(crate::tgbot::send::build_callback_button(
                "恢复",
                &build_job_resume_button_data(job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ));
            action_row.push(crate::tgbot::send::build_callback_button(
                "停止",
                &build_job_stop_button_data(job_id),
                tdlib_rs::enums::ButtonStyle::Danger,
            ));
        }
        "running" => {
            action_row.push(crate::tgbot::send::build_callback_button(
                "暂停",
                &build_job_pause_button_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ));
            action_row.push(crate::tgbot::send::build_callback_button(
                "停止",
                &build_job_stop_button_data(job_id),
                tdlib_rs::enums::ButtonStyle::Danger,
            ));
        }
        _ => {}
    }

    action_row
}

/// 中间状态卡片的列表入口文案。
fn status_list_label(status: &str) -> &'static str {
    match status {
        "paused" => "查看暂停列表",
        "cancelling" | "cancel_finalizing" => "查看停止列表",
        "cancelled" => "查看已停列表",
        "running" => "查看运行列表",
        _ => "查看列表",
    }
}

/// 构造任务中间状态卡片。
pub(in crate::tgbot::transfer) fn format_status_card_text(
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
        card::command_line(
            "详情",
            build_job_command("status", job_id, CommandStyle::Long),
        ),
        card::command_line(
            "查询",
            build_lookup_command(source_link, target_chat_id, CommandStyle::Long),
        ),
    ];

    match status {
        "paused" => {
            lines.push(card::command_line(
                "恢复",
                build_job_command("resume", job_id, CommandStyle::Long),
            ));
            lines.push(card::command_line(
                "停止",
                build_job_command("stop", job_id, CommandStyle::Long),
            ));
            lines.push(card::command_line(
                "列表",
                build_downloads_command(Some("pause"), None, None, CommandStyle::Long),
            ));
        }
        "cancelling" | "cancel_finalizing" | "cancelled" => {
            lines.push(card::command_line(
                "列表",
                build_downloads_command(Some("cancel"), None, None, CommandStyle::Long),
            ));
        }
        _ => {
            lines.push(card::command_line(
                "暂停",
                build_job_command("pause", job_id, CommandStyle::Long),
            ));
            lines.push(card::command_line(
                "停止",
                build_job_command("stop", job_id, CommandStyle::Long),
            ));
            lines.push(card::command_line(
                "列表",
                build_downloads_command(Some("run"), None, None, CommandStyle::Long),
            ));
        }
    }

    lines.push(card::command_line(
        "重转",
        build_transfer_command(source_link, target_chat_id, CommandStyle::Long),
    ));
    lines
}

#[cfg(test)]
mod tests {
    use super::{
        build_job_action_row, build_status_button_rows, format_status_card_text,
        status_command_lines,
    };
    use base64::{Engine as _, engine::general_purpose};

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
        assert!(text.contains("详情：‹/job status 42›"));
        assert!(text.contains("列表：‹/downloads run›"));
    }

    // paused/cancelled 状态应生成对应列表和控制命令，不能继续展示无效暂停命令。
    #[test]
    fn test_status_command_lines_by_status() {
        let paused = status_command_lines("paused", "https://t.me/c/1/2", -100, 42).join("\n");
        let cancelled =
            status_command_lines("cancelled", "https://t.me/c/1/2", -100, 42).join("\n");

        assert!(paused.contains("恢复：‹/job resume 42›"));
        assert!(paused.contains("列表：‹/downloads pause›"));
        assert!(cancelled.contains("列表：‹/downloads cancel›"));
        assert!(!cancelled.contains("暂停：‹/job pause 42›"));
    }

    // 中间状态卡片按钮应统一为主操作、导航两层，不再混排命令或 `job_id` 复制按钮。
    #[test]
    fn test_build_status_button_rows_layout() {
        let rows = build_status_button_rows("running", 42);
        let paused_rows = build_status_button_rows("paused", 42);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(rows[0][0].text, "查看任务详情");
        assert_eq!(rows[0][1].text, "暂停");
        assert_eq!(rows[0][2].text, "停止");
        assert_eq!(rows[0][2].style, tdlib_rs::enums::ButtonStyle::Danger);
        assert_eq!(decoded_callback_data(&rows[0][2]), "j:sc:42");
        assert_eq!(paused_rows[0][1].text, "恢复");
        assert_eq!(paused_rows[0][2].text, "停止");
        assert_eq!(
            paused_rows[0][2].style,
            tdlib_rs::enums::ButtonStyle::Danger
        );
        assert_eq!(decoded_callback_data(&paused_rows[0][2]), "j:sc:42");
        assert_eq!(rows[1][0].text, "查看运行列表");
        assert_eq!(rows[1][1].text, "菜单");
        assert_eq!(rows.len(), 2);
        assert!(!labels.contains(&"复制查询命令"));
        assert!(!labels.contains(&"复制重新转存"));
        assert!(!labels.contains(&"复制 job_id"));
    }

    // 任务操作行是 progress/status 共用入口，状态变化时按钮集合必须稳定。
    #[test]
    fn test_build_job_action_row_by_status() {
        let running = build_job_action_row("running", 42);
        let paused = build_job_action_row("paused", 42);
        let cancelled = build_job_action_row("cancelled", 42);

        assert_eq!(running[0].text, "查看任务详情");
        assert_eq!(running[1].text, "暂停");
        assert_eq!(running[2].text, "停止");
        assert_eq!(paused[1].text, "恢复");
        assert_eq!(paused[2].text, "停止");
        assert_eq!(cancelled.len(), 1);
        assert_eq!(cancelled[0].text, "查看任务详情");
    }

    fn decoded_callback_data(button: &tdlib_rs::types::InlineKeyboardButton) -> String {
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &button.r#type else {
            panic!("button must be callback");
        };
        String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap()
    }
}
