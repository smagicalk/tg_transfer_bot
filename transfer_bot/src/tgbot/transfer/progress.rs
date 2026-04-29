// 转存进度面板：
// - 周期性编辑 `/transfer` 初始回复
// - 将最终结果写回同一条消息
// - 构造进度/结果按钮

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::{store, types, workflow};
use keyboard::{build_transfer_progress_keyboard, build_transfer_result_keyboard};
use text::{
    format_transfer_control_text, format_transfer_error_text, format_transfer_final_text,
    format_transfer_progress_text, format_transfer_waiting_text,
};

mod keyboard;
#[cfg(test)]
mod tests;
mod text;

// 进度面板编辑间隔，避免频繁调用 editMessageText 触发 Telegram 限流。
const PROGRESS_EDIT_INTERVAL_SECONDS: u64 = 2;

/// 周期性刷新 `/transfer` 的进度面板。
///
/// 这里不直接参与下载/上传，只读取数据库快照；即使编辑失败，也不能影响后台转存任务。
pub(super) async fn update_transfer_progress_message(
    plan: types::TransferPlan,
    notify_chat_id: i64,
    message_id: i64,
    client_id: i32,
    done: Arc<AtomicBool>,
) {
    let mut last_text = String::new();
    loop {
        if done.load(Ordering::SeqCst) {
            return;
        }

        let snapshot =
            match store::find_active_job_by_source_target(&plan.source_link, plan.target_chat_id)
                .await
            {
                Ok(Some(job)) => store::get_job_progress_snapshot(job.id)
                    .await
                    .ok()
                    .flatten(),
                Ok(None) => None,
                Err(err) => {
                    tracing::warn!("load transfer progress failed: {:#}", err);
                    None
                }
            };

        let text = match &snapshot {
            Some(snapshot) => format_transfer_progress_text(snapshot, &plan.source_link),
            None => format_transfer_waiting_text(&plan),
        };

        // 文本不变时不编辑，减少无效请求和 Telegram 限流风险。
        if text != last_text {
            let keyboard = build_transfer_progress_keyboard(
                snapshot.as_ref().map(|snapshot| snapshot.job.id),
                &plan.source_link,
                plan.target_chat_id,
            );
            if let Err(err) = crate::tgbot::send::edit_markdown_message_with_inline_keyboard(
                text.clone(),
                notify_chat_id,
                message_id,
                keyboard,
                client_id,
            )
            .await
            {
                tracing::warn!("edit transfer progress message failed: {:#}", err);
            }
            last_text = text;
        }

        tokio::time::sleep(std::time::Duration::from_secs(
            PROGRESS_EDIT_INTERVAL_SECONDS,
        ))
        .await;
    }
}

/// 将最终执行结果写回同一条进度面板。
pub(super) async fn edit_transfer_progress_for_outcome(
    source_link: &str,
    target_chat_id: i64,
    result: &anyhow::Result<workflow::TransferOutcome>,
    notify_chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, keyboard) = match result {
        Ok(workflow::TransferOutcome::Reused { link }) => (
            format_transfer_final_text("已存在历史转存结果", source_link, target_chat_id, link),
            build_transfer_result_keyboard(source_link, target_chat_id, Some(link)),
        ),
        Ok(workflow::TransferOutcome::Running { job_id }) => (
            format_transfer_control_text(
                "相同链接正在转存中",
                source_link,
                target_chat_id,
                *job_id,
                "可以继续观察当前进度，或使用停止命令取消。",
            ),
            build_transfer_progress_keyboard(Some(*job_id), source_link, target_chat_id),
        ),
        Ok(workflow::TransferOutcome::Paused { job_id }) => (
            format_transfer_control_text(
                "转存任务已暂停",
                source_link,
                target_chat_id,
                *job_id,
                "恢复后会从已有子项状态继续处理。",
            ),
            build_transfer_progress_keyboard(Some(*job_id), source_link, target_chat_id),
        ),
        Ok(workflow::TransferOutcome::Cancelling { job_id }) => (
            format_transfer_control_text(
                "转存任务正在停止",
                source_link,
                target_chat_id,
                *job_id,
                "当前下载/上传调用会在安全点收尾。",
            ),
            build_transfer_progress_keyboard(Some(*job_id), source_link, target_chat_id),
        ),
        Ok(workflow::TransferOutcome::Cancelled { job_id }) => (
            format_transfer_control_text(
                "转存任务已停止",
                source_link,
                target_chat_id,
                *job_id,
                "文件引用已释放，后续由删除队列清理。",
            ),
            build_transfer_progress_keyboard(Some(*job_id), source_link, target_chat_id),
        ),
        Ok(workflow::TransferOutcome::Completed { link }) => (
            format_transfer_final_text("转存完成", source_link, target_chat_id, link),
            build_transfer_result_keyboard(source_link, target_chat_id, Some(link)),
        ),
        Err(err) => (
            format_transfer_error_text(source_link, target_chat_id, &err.to_string()),
            build_transfer_result_keyboard(source_link, target_chat_id, None),
        ),
    };

    crate::tgbot::send::edit_markdown_message_with_inline_keyboard(
        text,
        notify_chat_id,
        message_id,
        keyboard,
        client_id,
    )
    .await
}
