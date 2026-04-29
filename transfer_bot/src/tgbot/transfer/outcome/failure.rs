// 转存失败回复卡片。
// 失败详情使用可复制等宽文本，便于直接复制错误堆栈继续排查。

use super::super::command::common::{
    CommandStyle, downloads_command as build_downloads_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};

/// 发送失败信息。
pub(in crate::tgbot::transfer) async fn send_failure_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    err: anyhow::Error,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let retry_command = build_transfer_command(source_link, target_chat_id, CommandStyle::Short);
    let lookup_command = build_lookup_command(source_link, target_chat_id, CommandStyle::Short);
    crate::tgbot::send::ReplyPanel::copyable(format!(
        "{}\nsource_link={}\ntarget_chat_id={}\nretry_command={}\n\n{:#}",
        title, source_link, target_chat_id, retry_command, err
    ))
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制重转命令",
            &retry_command,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制查询命令",
            &lookup_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制 /d fail",
            &build_downloads_command(Some("fail"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(notify_chat_id, client_id)
    .await
}
