// 转存失败回复卡片。
// 正文使用统一卡片风格，错误详情单独用等宽代码块，兼顾美观和排查便利。

use super::super::card;
use super::super::command::common::{
    CommandStyle, downloads_command as build_downloads_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};
use super::super::command::{build_downloads_filter_button_data, build_job_status_button_data};

/// 发送失败信息。
pub(in crate::tgbot::transfer) async fn send_failure_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: Option<i64>,
    err: anyhow::Error,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let retry_command = build_transfer_command(source_link, target_chat_id, CommandStyle::Short);
    let lookup_command = build_lookup_command(source_link, target_chat_id, CommandStyle::Short);
    crate::tgbot::send::ReplyPanel::card(format_failure_card_text(
        title,
        source_link,
        target_chat_id,
        job_id,
        &retry_command,
        &lookup_command,
        &err,
    ))
    .row(build_failure_buttons(
        job_id,
        &retry_command,
        &lookup_command,
    ))
    .send(notify_chat_id, client_id)
    .await
}

/// 构造失败卡片按钮。
///
/// 失败详情保留等宽可复制正文；按钮额外给出失败列表 callback，方便直接跳转排查。
fn build_failure_buttons(
    job_id: Option<i64>,
    retry_command: &str,
    lookup_command: &str,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    let mut row = vec![
        crate::tgbot::send::build_copy_button(
            "复制重新转存",
            retry_command,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制查询命令",
            lookup_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ];
    if let Some(job_id) = job_id {
        row.push(crate::tgbot::send::build_callback_button(
            "查看任务详情",
            &build_job_status_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    if let Some(callback_data) = build_downloads_filter_button_data("fail", 8) {
        row.push(crate::tgbot::send::build_callback_button(
            "查看失败列表",
            &callback_data,
            tdlib_rs::enums::ButtonStyle::Primary,
        ));
    }
    row.push(crate::tgbot::send::build_copy_button(
        "复制列表命令",
        &build_downloads_command(Some("fail"), None, None, CommandStyle::Short),
        tdlib_rs::enums::ButtonStyle::Default,
    ));
    row
}

/// 构造失败卡片正文。
///
/// 用户号模式下按钮会被发送层丢弃，因此重试、查询和失败列表命令必须出现在正文里。
fn format_failure_card_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: Option<i64>,
    retry_command: &str,
    lookup_command: &str,
    err: &anyhow::Error,
) -> String {
    let mut lines = vec![
        title.to_owned(),
        card::summary_line("failed", job_id, target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("错误"),
        card::pre_code(format!("{:#}", err)),
        card::section("命令"),
        card::command_line("重试", retry_command),
        card::command_line("查询", lookup_command),
        card::command_line(
            "列表",
            build_downloads_command(Some("fail"), None, None, CommandStyle::Short),
        ),
        String::new(),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{build_failure_buttons, format_failure_card_text};

    // 恢复失败已知 job_id 时，应能从失败卡片直接跳任务详情和失败列表。
    #[test]
    fn test_build_failure_buttons_with_job_id() {
        let buttons = build_failure_buttons(Some(42), "/t https://t.me/c/1/2 -100", "/lk x -100");

        assert!(buttons.iter().any(|button| button.text == "查看任务详情"));
        assert!(buttons.iter().any(|button| button.text == "查看失败列表"));
        assert!(buttons.iter().any(|button| matches!(
            button.r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        )));
    }

    // 失败正文应保留重试命令、查询命令和完整错误，用户号模式下也能继续操作。
    #[test]
    fn test_format_failure_card_text() {
        let err = anyhow::anyhow!("network failed");
        let text = format_failure_card_text(
            "转存失败",
            "https://t.me/c/1/2",
            -100,
            Some(42),
            "/t https://t.me/c/1/2 -100",
            "/lk https://t.me/c/1/2 -100",
            &err,
        );

        assert!(text.contains("状态：‹failed›"));
        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("«network failed»"));
        assert!(text.contains("重试：‹/t https://t.me/c/1/2 -100›"));
        assert!(text.contains("查询：‹/lk https://t.me/c/1/2 -100›"));
        assert!(text.contains("列表：‹/d fail›"));
    }
}
