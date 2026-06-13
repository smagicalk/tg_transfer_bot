// `/job` 命令入口。
// 参数解析和具体控制动作拆到子模块，入口只负责分发。

mod actions;
mod args;
mod callback;
mod keyboard;
mod render;
mod status_meta;

use crate::tgbot::send;
use crate::tgbot::send::send_interaction_error_card;
use actions::{pause_job, resume_job, show_job_status, stop_job};
use args::{
    JobAction, JobCallbackAction, is_job_callback_data, parse_job_args, parse_job_callback_data,
};
use callback::{JobCallbackResult, handle_job_callback};

/// 判断 callback payload 是否属于 `/job`。
pub(super) fn is_job_callback_payload(data: &str) -> bool {
    is_job_callback_data(data)
}

/// 生成单任务详情按钮所需的 `/job status` callback 数据。
///
/// 这个包装函数给 `/downloads` 复用，避免它直接依赖 `/job` 的内部参数枚举。
pub(super) fn build_job_status_callback_data(job_id: i64) -> String {
    args::build_job_callback_data(args::JobCallbackAction::Status, job_id)
}

/// 生成单任务暂停按钮所需的 callback 数据。
pub(super) fn build_job_pause_callback_data(job_id: i64) -> String {
    args::build_job_callback_data(args::JobCallbackAction::Pause, job_id)
}

/// 生成单任务恢复按钮所需的 callback 数据。
pub(super) fn build_job_resume_callback_data(job_id: i64) -> String {
    args::build_job_callback_data(args::JobCallbackAction::Resume, job_id)
}

/// 生成单任务停止按钮所需的 callback 数据。
pub(super) fn build_job_stop_callback_data(job_id: i64) -> String {
    args::build_job_callback_data(args::JobCallbackAction::Stop, job_id)
}

/// 给非 `/job` 模块使用的任务列表入口信息。
///
/// 外部卡片只需要知道“跳到哪个列表”和“按钮显示什么”，不需要依赖
/// `/job` 内部的完整状态元信息结构。
pub(super) fn job_list_button_meta(status: &str) -> (&'static str, &'static str) {
    let meta = status_meta::job_status_meta(status);
    (meta.list_filter, meta.list_button_label)
}

/// `/job` 命令入口。
/// 命令格式：`/job <pause|resume|stop|status> <job_id>`
pub async fn job_command(
    text: Vec<&str>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let args = parse_job_args(&text)?;

    match args.action {
        JobAction::Pause => pause_job(args.job_id, actor, client_id).await,
        JobAction::Resume => resume_job(args.job_id, actor, client_id).await,
        JobAction::Stop => stop_job(args.job_id, actor, client_id).await,
        JobAction::Status => show_job_status(args.job_id, actor, client_id).await,
    }
}

/// `/job` inline keyboard 回调入口。
///
/// 点击任务详情里的按钮后，直接编辑当前卡片，而不是要求用户复制命令再发送。
pub async fn job_callback_query(
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            crate::tgbot::send::answer_callback_query(
                update.id,
                Some("暂不支持这种按钮类型"),
                client_id,
            )
            .await?;
            return Ok(());
        }
    };

    let Some(args) = parse_job_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("任务按钮参数无效"), client_id).await?;
        return Ok(());
    };

    send::answer_callback_query(
        update.id,
        Some(job_callback_started_tip(args.action)),
        client_id,
    )
    .await?;

    match handle_job_callback(
        args.action,
        args.job_id,
        actor,
        update.message_id,
        client_id,
    )
    .await
    {
        Ok(JobCallbackResult::Updated) => Ok(()),
        Ok(JobCallbackResult::RefreshFailed(err)) => {
            send_job_refresh_error(update.chat_id, client_id, args.job_id, &err).await?;
            Err(err)
        }
        Err(err) => {
            send_job_callback_error(update.chat_id, client_id, args.job_id, &err).await?;
            Err(err)
        }
    }
}

/// 任务按钮点击后的即时提示。
fn job_callback_started_tip(action: JobCallbackAction) -> &'static str {
    match action {
        JobCallbackAction::Pause => "正在暂停",
        JobCallbackAction::Resume => "正在恢复",
        JobCallbackAction::Stop => "正在停止",
        JobCallbackAction::Status => "正在刷新",
    }
}

/// 任务按钮失败提示。
///
/// callback 已经先 ACK，失败时不能再 answer 同一个 callback，因此发送一条短卡片说明错误。
async fn send_job_callback_error(
    request_chat_id: i64,
    client_id: i32,
    job_id: i64,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    let title = format!("任务操作失败 #{}", job_id);
    send_interaction_error_card(
        request_chat_id,
        client_id,
        &title,
        "任务状态未更新，请检查日志或复制错误信息。",
        err,
    )
    .await
}

/// 任务已处理但详情卡片编辑失败时的提示。
async fn send_job_refresh_error(
    request_chat_id: i64,
    client_id: i32,
    job_id: i64,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    let title = format!("任务详情刷新失败 #{}", job_id);
    send_interaction_error_card(
        request_chat_id,
        client_id,
        &title,
        "任务操作已处理，但原详情卡片未刷新；可重新打开任务详情确认状态。",
        err,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    // callback 应先给即时提示，避免 Telegram 客户端按钮持续转圈。
    #[test]
    fn test_job_callback_started_tip() {
        assert_eq!(
            job_callback_started_tip(JobCallbackAction::Pause),
            "正在暂停"
        );
        assert_eq!(
            job_callback_started_tip(JobCallbackAction::Resume),
            "正在恢复"
        );
        assert_eq!(
            job_callback_started_tip(JobCallbackAction::Stop),
            "正在停止"
        );
        assert_eq!(
            job_callback_started_tip(JobCallbackAction::Status),
            "正在刷新"
        );
    }
}
