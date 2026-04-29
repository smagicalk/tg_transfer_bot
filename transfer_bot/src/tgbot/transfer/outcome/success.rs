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
    crate::tgbot::send::ReplyPanel::markdown(format!(
        "*{}*\n源链接：`{}`\n目标 chat：`{}`\n结果消息：[打开转存消息]({})",
        title, source_link, target_chat_id, result_link
    ))
    .row(vec![
        crate::tgbot::send::build_url_button(
            "打开结果",
            result_link,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制结果链接",
            result_link,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制查询命令",
            &lookup_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制重转命令",
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
