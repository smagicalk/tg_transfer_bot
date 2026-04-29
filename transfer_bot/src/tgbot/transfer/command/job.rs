// `/job` 命令入口。
// 参数解析和具体控制动作拆到子模块，入口只负责分发。

mod actions;
mod args;

use actions::{pause_job, resume_job, stop_job};
use args::{JobAction, parse_job_args};

/// `/job` 命令入口。
/// 命令格式：`/job <pause|resume|stop> <job_id>`
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
    }
}
