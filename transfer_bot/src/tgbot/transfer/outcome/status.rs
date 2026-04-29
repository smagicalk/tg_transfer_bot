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
        "*{}*\n源链接：`{}`\n目标 chat：`{}`\njob_id：`{}`\n可手动恢复或停止该任务。",
        title, source_link, target_chat_id, job_id
    ))
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制恢复命令",
            &build_job_command("r", job_id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制停止命令",
            &build_job_command("s", job_id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_copy_button(
            "复制 /d pause",
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
        "*{}*\n源链接：`{}`\n目标 chat：`{}`\njob_id：`{}`\n当前调用会在安全点收尾。",
        title, source_link, target_chat_id, job_id
    ))
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制 job_id",
            &job_id.to_string(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制 /d cancel",
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
        "*{}*\n源链接：`{}`\n目标 chat：`{}`\njob_id：`{}`\n文件引用已释放，后续由删除队列清理。",
        title, source_link, target_chat_id, job_id
    ))
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制 job_id",
            &job_id.to_string(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制 /d cancel",
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
        "*{}*\n源链接：`{}`\n目标 chat：`{}`\njob_id：`{}`\n建议：使用 `/d run` 查看后台进度。",
        title, source_link, target_chat_id, job_id
    ))
    .row(vec![
        crate::tgbot::send::build_copy_button(
            "复制 job_id",
            &job_id.to_string(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制 /d run",
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
        "复制重转命令",
        &transfer_command,
        tdlib_rs::enums::ButtonStyle::Default,
    )])
    .send(notify_chat_id, client_id)
    .await
}
