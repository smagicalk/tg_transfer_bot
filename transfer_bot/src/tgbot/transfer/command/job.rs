// `/job` 命令入口。
// 参数解析和具体控制动作拆到子模块，入口只负责分发。

mod actions;
mod args;

use actions::{handle_job_callback, pause_job, resume_job, show_job_status, stop_job};
use args::{JobAction, is_job_callback_data, parse_job_args, parse_job_callback_data};

/// 判断 callback payload 是否属于 `/job`。
pub(super) fn is_job_callback_payload(data: &str) -> bool {
    is_job_callback_data(data)
}

/// `/job` 命令入口。
/// 命令格式：`/job <pause|resume|stop|status> <job_id>`
pub async fn job_command(
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let args = parse_job_args(&text)?;

    match args.action {
        JobAction::Pause => pause_job(args.job_id, request_chat_id, client_id).await,
        JobAction::Resume => resume_job(args.job_id, request_chat_id, client_id).await,
        JobAction::Stop => stop_job(args.job_id, request_chat_id, client_id).await,
        JobAction::Status => show_job_status(args.job_id, request_chat_id, client_id).await,
    }
}

/// `/job` inline keyboard 回调入口。
///
/// 点击任务详情里的按钮后，直接编辑当前卡片，而不是要求用户复制命令再发送。
pub async fn job_callback_query(
    update: tdlib_rs::enums::UpdateNewCallbackQuery,
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
        crate::tgbot::send::answer_callback_query(update.id, Some("任务按钮参数无效"), client_id)
            .await?;
        return Ok(());
    };

    let callback_tip = match handle_job_callback(
        args.action,
        args.job_id,
        update.chat_id,
        update.message_id,
        client_id,
    )
    .await
    {
        Ok(callback_tip) => callback_tip,
        Err(err) => {
            crate::tgbot::send::answer_callback_query(update.id, Some("任务操作失败"), client_id)
                .await?;
            return Err(err);
        }
    };

    crate::tgbot::send::answer_callback_query(update.id, Some(callback_tip), client_id).await?;

    Ok(())
}
