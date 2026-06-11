// `/job` 控制动作实现。
// 每个动作都先更新数据库状态，再返回可复制的下一步命令按钮。

use crate::tgbot::send;
use crate::tgbot::transfer::store;
use crate::tgbot::transfer::workflow;

use super::super::common::{CommandStyle, downloads_command, job_command as build_job_command};
use super::keyboard::build_job_status_buttons;
use super::render::{format_job_action_text, format_job_status_text};

/// 暂停任务。
///
/// 当前正在执行的 TDLib 单次下载/上传不会被强制中断；工作流会在下一个安全点停止。
pub(super) async fn pause_job(
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let job = store::pause_job_with_owner_scope(job_id, actor.request_chat_id, actor.owner_scope())
        .await?;
    tracing::info!(
        job_id = job.id,
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        actor_role = actor.role.as_str(),
        status = %job.status,
        "transfer job paused by command"
    );
    send::ReplyPanel::card(format_job_action_text(
        "任务已暂停",
        job.id,
        &job.status,
        "恢复后会从已有子项状态继续处理。",
    ))
    .row(vec![
        send::build_copy_button(
            "复制恢复",
            &build_job_command("r", job.id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_copy_button(
            "复制停止",
            &build_job_command("s", job.id, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_copy_button(
            "复制暂停列表",
            &downloads_command(Some("pause"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ])
    .send(actor.request_chat_id, client_id)
    .await
}

/// 唤醒未完成任务。
///
/// paused 会先改回 pending；pending/running 若当前进程没有执行器，也会重新派发后台任务。
pub(super) async fn resume_job(
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let job = store::wake_job_with_owner_scope(job_id, actor.request_chat_id, actor.owner_scope())
        .await?;
    let is_running = workflow::is_job_running_in_process(job.id).await;
    if !is_running {
        super::super::super::spawn_recovery_job(
            crate::app_context::app_context(),
            job.clone(),
            super::super::super::transfer_client_ids()?,
        );
    }
    tracing::info!(
        job_id = job.id,
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        actor_role = actor.role.as_str(),
        status = %job.status,
        is_running,
        "transfer job resumed by command"
    );

    let title = if is_running {
        "任务已在执行中"
    } else {
        "任务已唤醒"
    };
    let detail = if is_running {
        "当前进程已有后台执行器，不会重复派发。"
    } else {
        "后台会继续下载/上传剩余内容。"
    };

    send::ReplyPanel::card(format_job_action_text(title, job.id, &job.status, detail))
        .row(vec![
            send::build_copy_button(
                "复制暂停",
                &build_job_command("p", job.id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制停止",
                &build_job_command("s", job.id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制运行列表",
                &downloads_command(Some("run"), None, None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
        ])
        .send(actor.request_chat_id, client_id)
        .await
}

/// 停止任务。
///
/// 先把数据库状态改成 cancelling，再判断是否存在执行器：
/// - 有执行器：由工作流在安全点释放引用；
/// - 无执行器：当前命令立即收敛为 cancelled 并释放引用。
pub(super) async fn stop_job(
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let requested = store::request_cancel_job_with_owner_scope(
        job_id,
        actor.request_chat_id,
        actor.owner_scope(),
    )
    .await?;
    let is_running = workflow::is_job_running_in_process(job_id).await;
    let job = if is_running {
        requested
    } else {
        store::cancel_job_now(
            job_id,
            "cancelled by user",
            super::super::super::runtime_config()
                .file_delete_delay_minutes
                .max(0),
        )
        .await?
    };
    tracing::info!(
        job_id = job.id,
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        actor_role = actor.role.as_str(),
        status = %job.status,
        is_running,
        "transfer job stopped by command"
    );

    let title = if is_running {
        "任务已请求停止"
    } else {
        "任务已停止"
    };
    let detail = if is_running {
        "当前下载/上传调用会在安全点收尾，随后释放文件引用。"
    } else {
        "文件引用已释放，后续由删除队列按配置清理。"
    };

    send::ReplyPanel::card(format_job_action_text(title, job.id, &job.status, detail))
        .row(vec![
            send::build_copy_button(
                "复制 job_id",
                &job.id.to_string(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制停止列表",
                &downloads_command(Some("cancel"), None, None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .send(actor.request_chat_id, client_id)
        .await
}

/// 查看单个任务详情。
///
/// 只读取轻量进度快照，不会影响后台任务状态。
pub(super) async fn show_job_status(
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let Some(snapshot) = store::get_job_progress_snapshot_for_actor(job_id, actor).await? else {
        anyhow::bail!("job not found: {}", job_id);
    };
    tracing::info!(
        job_id,
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        actor_role = actor.role.as_str(),
        status = %snapshot.job.status,
        "transfer job status requested"
    );

    send::ReplyPanel::card(format_job_status_text(&snapshot))
        .rows(build_job_status_buttons(&snapshot))
        .send(actor.request_chat_id, client_id)
        .await
}
