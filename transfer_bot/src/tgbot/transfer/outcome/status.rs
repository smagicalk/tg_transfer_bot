// 转存任务中间状态回复卡片。
// 这些状态不会携带错误堆栈，只提供 job_id 和下一步控制命令。

use super::super::command::common::{
    CommandStyle, downloads_command as build_downloads_command, job_command as build_job_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
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
    crate::tgbot::send::ReplyPanel::markdown(format!(
        "*{}*\n状态：`paused`\njob：`#{}`\n目标：`{}`\n━━━━━━━━━━━━\n说明：可手动恢复或停止该任务。\n源：`{}`",
        title,
        job_id,
        target_chat_id,
        markdown_inline_code(source_link)
    ))
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制恢复",
            &build_job_command("r", job_id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制停止",
            &build_job_command("s", job_id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制暂停列表",
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
    crate::tgbot::send::ReplyPanel::markdown(format!(
        "*{}*\n状态：`cancelling`\njob：`#{}`\n目标：`{}`\n━━━━━━━━━━━━\n说明：当前调用会在安全点收尾。\n源：`{}`",
        title,
        job_id,
        target_chat_id,
        markdown_inline_code(source_link)
    ))
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制 job_id",
            &job_id.to_string(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制停止列表",
            &build_downloads_command(Some("cancel"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
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
    crate::tgbot::send::ReplyPanel::markdown(format!(
        "*{}*\n状态：`cancelled`\njob：`#{}`\n目标：`{}`\n━━━━━━━━━━━━\n说明：文件引用已释放，后续由删除队列清理。\n源：`{}`",
        title,
        job_id,
        target_chat_id,
        markdown_inline_code(source_link)
    ))
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制 job_id",
            &job_id.to_string(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制停止列表",
            &build_downloads_command(Some("cancel"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
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
    crate::tgbot::send::ReplyPanel::markdown(format!(
        "*{}*\n状态：`running`\njob：`#{}`\n目标：`{}`\n━━━━━━━━━━━━\n建议：使用 `/d run` 查看后台进度。\n源：`{}`",
        title,
        job_id,
        target_chat_id,
        markdown_inline_code(source_link)
    ))
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制 job_id",
            &job_id.to_string(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制运行列表",
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

/// 转义 Markdown 行内代码里的反引号，避免用户输入链接破坏卡片格式。
fn markdown_inline_code(text: &str) -> String {
    text.replace('`', "'")
}
