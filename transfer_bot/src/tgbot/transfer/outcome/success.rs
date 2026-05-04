// 转存成功或命中历史结果时的回复卡片。
// 成功卡片需要同时提供打开链接、复制结果和继续查询/重转的快捷按钮。

use super::super::card;
use super::super::command::common::{
    CommandStyle, downloads_command as build_downloads_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};
use super::super::command::{build_downloads_filter_button_data, build_job_status_button_data};

/// 发送“命中历史结果 / 已完成”的结果卡片。
pub(in crate::tgbot::transfer) async fn send_history_hit_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    result_link: &str,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let lookup_command = build_lookup_command(source_link, target_chat_id, CommandStyle::Short);
    let transfer_command = build_transfer_command(source_link, target_chat_id, CommandStyle::Short);
    let mut result_row = Vec::new();
    if crate::tgbot::send::is_openable_url(result_link) {
        result_row.push(crate::tgbot::send::build_url_button(
            "打开转存消息",
            result_link,
            tdlib_rs::enums::ButtonStyle::Primary,
        ));
    }
    result_row.push(crate::tgbot::send::build_copy_button(
        if crate::tgbot::send::is_openable_url(result_link) {
            "复制结果链接"
        } else {
            "复制结果定位"
        },
        result_link,
        if crate::tgbot::send::is_openable_url(result_link) {
            tdlib_rs::enums::ButtonStyle::Default
        } else {
            tdlib_rs::enums::ButtonStyle::Primary
        },
    ));
    result_row.push(crate::tgbot::send::build_callback_button(
        "查看任务详情",
        &build_job_status_button_data(job_id),
        tdlib_rs::enums::ButtonStyle::Default,
    ));
    result_row.push(crate::tgbot::send::build_copy_button(
        "复制查询命令",
        &lookup_command,
        tdlib_rs::enums::ButtonStyle::Default,
    ));

    crate::tgbot::send::ReplyPanel::card(format_result_card_text(
        title,
        source_link,
        target_chat_id,
        Some(job_id),
        result_link,
    ))
    .row(result_row)
    .row(build_result_list_row(&transfer_command))
    .send(notify_chat_id, client_id)
    .await
}

/// 构造结果卡片第二行：重转、进入完成列表、复制列表命令。
fn build_result_list_row(transfer_command: &str) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    let mut row = vec![crate::tgbot::send::build_copy_button(
        "复制重新转存",
        transfer_command,
        tdlib_rs::enums::ButtonStyle::Default,
    )];
    if let Some(callback_data) = build_downloads_filter_button_data("done", 8) {
        row.push(crate::tgbot::send::build_callback_button(
            "查看完成列表",
            &callback_data,
            tdlib_rs::enums::ButtonStyle::Primary,
        ));
    }
    row.push(crate::tgbot::send::build_copy_button(
        "复制列表命令",
        &build_downloads_command(Some("done"), None, None, CommandStyle::Short),
        tdlib_rs::enums::ButtonStyle::Default,
    ));
    row
}

/// 构造结果卡片正文。
///
/// 正文只对 HTTP(S) 结果使用 TDLib 原生文本链接；旧的 `tg://openmessage`
/// 或纯定位字符串只作为代码字段展示，避免客户端显示成不可用链接。
pub(in crate::tgbot::transfer) fn format_result_card_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: Option<i64>,
    result_link: &str,
) -> String {
    let mut lines = vec![
        title.to_owned(),
        card::summary_line("success", job_id, target_chat_id),
        card::DIVIDER.to_owned(),
        card::result_block(result_link),
        String::new(),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::format_result_card_text;

    // HTTP(S) 结果应在正文中渲染为 Telegram 原生文本链接，按钮之外也能点击。
    #[test]
    fn test_format_result_card_text_uses_card_link_for_openable_result() {
        let text = format_result_card_text(
            "转存完成",
            "https://t.me/c/1/2",
            -5106953357,
            Some(42),
            "https://t.me/c/5106953357/734",
        );

        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("【打开转存消息】(https://t.me/c/5106953357/734)"));
        assert!(text.contains("链接：‹https://t.me/c/5106953357/734›"));
    }

    // 不可打开的定位信息只能作为代码展示，不能伪装成可点击链接。
    #[test]
    fn test_format_result_card_text_keeps_locator_as_code() {
        let text = format_result_card_text(
            "转存完成",
            "https://t.me/c/1/2",
            -5106953357,
            Some(42),
            "chat_id=-5106953357 message_id=769654784",
        );

        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("无可跳转消息链接"));
        assert!(text.contains("定位：‹chat_id=-5106953357 message_id=769654784›"));
        assert!(!text.contains("【打开转存消息】("));
    }
}
