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
            "打开结果",
            result_link,
            tdlib_rs::enums::ButtonStyle::Primary,
        ));
    }
    result_row.push(crate::tgbot::send::build_copy_button(
        if crate::tgbot::send::is_openable_url(result_link) {
            "复制链接"
        } else {
            "复制定位"
        },
        result_link,
        if crate::tgbot::send::is_openable_url(result_link) {
            tdlib_rs::enums::ButtonStyle::Default
        } else {
            tdlib_rs::enums::ButtonStyle::Primary
        },
    ));
    result_row.push(crate::tgbot::send::build_copy_button(
        "复制查询",
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
            "复制重转",
            &transfer_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制 /d done",
            &build_downloads_command(Some("done"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(notify_chat_id, client_id)
    .await
}

/// 构造结果卡片正文。
///
/// 正文不再使用 Markdown 链接，避免 `tg://openmessage` 在客户端里显示成
/// “标题 (url)” 且点击无效；真正可打开的 HTTP(S) 链接交给按钮处理。
pub(in crate::tgbot::transfer) fn format_result_card_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    result_link: &str,
) -> String {
    let result_line = if crate::tgbot::send::is_openable_url(result_link) {
        "结果：`可打开，见下方按钮`".to_owned()
    } else {
        format!(
            "结果：`已上传，但当前 chat 无可跳转公开链接`\n定位：`{}`",
            markdown_inline_code(result_link)
        )
    };

    format!(
        "*{}*\n状态：`success`\n目标：`{}`\n━━━━━━━━━━━━\n{}\n源：`{}`",
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
