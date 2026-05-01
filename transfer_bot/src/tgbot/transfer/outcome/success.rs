// 转存成功或命中历史结果时的回复卡片。
// 成功卡片需要同时提供打开链接、复制结果和继续查询/重转的快捷按钮。

use super::super::command::common::{
    CommandStyle, downloads_command as build_downloads_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};

/// 发送“命中历史结果 / 已完成”的结果卡片。
pub(in crate::tgbot::transfer) async fn send_history_hit_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
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
    result_row.push(crate::tgbot::send::build_copy_button(
        "复制查询命令",
        &lookup_command,
        tdlib_rs::enums::ButtonStyle::Default,
    ));

    crate::tgbot::send::ReplyPanel::markdown(format_result_card_text(
        title,
        source_link,
        target_chat_id,
        result_link,
    ))
    .row(result_row)
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制重新转存",
            &transfer_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制完成列表",
            &build_downloads_command(Some("done"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(notify_chat_id, client_id)
    .await
}

/// 构造结果卡片正文。
///
/// 正文只对 HTTP(S) 结果使用 Markdown 链接；旧的 `tg://openmessage`
/// 或纯定位字符串只作为行内代码展示，避免客户端显示成不可用链接。
pub(in crate::tgbot::transfer) fn format_result_card_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    result_link: &str,
) -> String {
    let result_line = if crate::tgbot::send::is_openable_url(result_link) {
        format!(
            "*结果*\n[打开转存消息]({})\n链接：`{}`",
            result_link,
            markdown_inline_code(result_link)
        )
    } else {
        format!(
            "*结果*\n状态：`已上传，但当前 chat 无可跳转消息链接`\n定位：`{}`",
            markdown_inline_code(result_link)
        )
    };

    format!(
        "*{}*\n状态：`success`  目标：`{}`\n━━━━━━━━━━━━\n{}\n\n*来源*\n`{}`",
        title,
        target_chat_id,
        result_line,
        markdown_inline_code(source_link)
    )
}

/// 转义 Markdown 行内代码里的反引号，避免链接或错误文本破坏卡片格式。
fn markdown_inline_code(text: &str) -> String {
    text.replace('`', "'")
}

#[cfg(test)]
mod tests {
    use super::format_result_card_text;

    // HTTP(S) 结果应在正文中渲染为 Telegram 原生文本链接，按钮之外也能点击。
    #[test]
    fn test_format_result_card_text_uses_markdown_link_for_openable_result() {
        let text = format_result_card_text(
            "转存完成",
            "https://t.me/c/1/2",
            -5106953357,
            "https://t.me/c/5106953357/734",
        );

        assert!(text.contains("[打开转存消息](https://t.me/c/5106953357/734)"));
        assert!(text.contains("链接：`https://t.me/c/5106953357/734`"));
    }

    // 不可打开的定位信息只能作为代码展示，不能伪装成可点击链接。
    #[test]
    fn test_format_result_card_text_keeps_locator_as_code() {
        let text = format_result_card_text(
            "转存完成",
            "https://t.me/c/1/2",
            -5106953357,
            "chat_id=-5106953357 message_id=769654784",
        );

        assert!(text.contains("无可跳转消息链接"));
        assert!(text.contains("定位：`chat_id=-5106953357 message_id=769654784`"));
        assert!(!text.contains("[打开转存消息]("));
    }
}
