// 转存进度面板：
// - 周期性编辑 `/transfer` 初始回复
// - 将最终结果写回同一条消息
// - 构造进度/结果按钮

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::{store, types, workflow};
use keyboard::{build_transfer_progress_keyboard, build_transfer_result_keyboard};
use text::{
    format_transfer_control_text, format_transfer_error_text,
    format_transfer_final_text_with_results, format_transfer_progress_text,
    format_transfer_waiting_text,
};

mod keyboard;
#[cfg(test)]
mod tests;
mod text;

/// 恢复任务进度卡片的定位与生命周期状态结构体。
pub(super) struct RecoveryProgressUpdate {
    /// 恢复的任务唯一标识 ID
    pub(super) job_id: i64,
    /// 任务原始源链接
    pub(super) source_link: String,
    /// 目标聊天会话 ID
    pub(super) target_chat_id: i64,
    /// 接收进度更新通知的会话 ID
    pub(super) notify_chat_id: i64,
    /// 进度消息的 Telegram 消息 ID
    pub(super) message_id: i64,
    /// TDLib 客户端实例 ID
    pub(super) client_id: i32,
    /// 任务完成或终止的原子退出信号
    pub(super) done: Arc<AtomicBool>,
}

/// 周期性刷新 `/transfer` 的进度面板后台轮询协程。
///
/// 这里不直接参与下载/上传，只读取数据库快照；即使编辑失败，也不能影响后台转存任务。
///
/// # 参数
/// - `app_context`: 全局应用上下文智能指针
/// - `plan`: 转存规划详情
/// - `notify_chat_id`: 目标通知会话 ID
/// - `message_id`: 原进度卡片消息 ID
/// - `client_id`: 客户端实例 ID
/// - `done`: 结束信号原子标志
pub(super) async fn update_transfer_progress_message(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    plan: types::TransferPlan,
    notify_chat_id: i64,
    message_id: i64,
    client_id: i32,
    done: Arc<AtomicBool>,
) {
    // 缓存上一次渲染的文本，实现有变化才编辑（防 Telegram 限流）
    let mut last_text = String::new();
    loop {
        // 检查退出信号
        if done.load(Ordering::SeqCst) {
            return;
        }

        // 根据请求会话与消息 ID 查找对应的任务快照
        let snapshot =
            match store::find_job_by_request(plan.request_chat_id, plan.request_message_id).await {
                // 每条请求消息绑定自己的 job；同源同目标的并发请求不能串看最新任务。
                Ok(Some(job)) => {
                    store::get_job_progress_snapshot_with_context(app_context.as_ref(), job.id)
                        .await
                        .ok()
                        .flatten()
                }
                Ok(None) => None,
                Err(err) => {
                    tracing::warn!("load transfer progress failed: {:#}", err);
                    None
                }
            };

        // 渲染进度或等待提示卡片文本
        let text = match &snapshot {
            Some(snapshot) => format_transfer_progress_text(snapshot, &plan.source_link),
            None => format_transfer_waiting_text(&plan),
        };

        // 文本不变时不编辑，减少无效请求和 Telegram 限流风险。
        if text != last_text {
            // 构造进度内联键盘
            let keyboard = build_transfer_progress_keyboard(
                snapshot.as_ref().map(|snapshot| snapshot.job.id),
                snapshot
                    .as_ref()
                    .map(|snapshot| snapshot.job.status.as_str()),
                &plan.source_link,
                plan.target_chat_id,
            );
            // 原地编辑 Telegram 消息
            if let Err(err) = crate::tgbot::send::edit_card_message_with_inline_keyboard(
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

        // 进度编辑间隔从运行时配置读取，避免频繁 editMessageText 触发 Telegram 限流。
        let interval = crate::tgbot::transfer::runtime_config_on(app_context.as_ref())
            .progress_edit_interval_seconds
            .max(1);
        tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
    }
}

/// 周期性刷新手动恢复任务的原详情卡片。
///
/// 手动恢复没有新的 `/transfer` 请求，因此不能依赖 request_message_id 查找任务；
/// 直接按 job_id 读取快照，确保暂停后恢复仍持续更新同一条消息。
///
/// # 参数
/// - `app_context`: 全局应用上下文智能指针
/// - `update`: 恢复进度卡片状态
pub(super) async fn update_recovery_progress_message(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    update: RecoveryProgressUpdate,
) {
    let mut last_text = String::new();
    loop {
        // 校验退出标志
        if update.done.load(Ordering::SeqCst) {
            return;
        }

        // 直接根据 job_id 获取任务进度快照
        let snapshot = match store::get_job_progress_snapshot_with_context(
            app_context.as_ref(),
            update.job_id,
        )
        .await
        {
            Ok(snapshot) => snapshot,
            Err(err) => {
                tracing::warn!(job_id = update.job_id, error = %err, "load recovery transfer progress failed");
                None
            }
        };
        // 格式化文本
        let text = match &snapshot {
            Some(snapshot) => format_transfer_progress_text(snapshot, &update.source_link),
            None => format_transfer_control_text(
                "恢复任务等待中",
                "waiting",
                &update.source_link,
                update.target_chat_id,
                update.job_id,
                "正在重新获取源消息并恢复任务。",
            ),
        };

        // 差异化更新
        if text != last_text {
            let keyboard = build_transfer_progress_keyboard(
                Some(update.job_id),
                snapshot
                    .as_ref()
                    .map(|snapshot| snapshot.job.status.as_str()),
                &update.source_link,
                update.target_chat_id,
            );
            if let Err(err) = crate::tgbot::send::edit_card_message_with_inline_keyboard(
                text.clone(),
                update.notify_chat_id,
                update.message_id,
                keyboard,
                update.client_id,
            )
            .await
            {
                tracing::warn!(job_id = update.job_id, error = %err, "edit recovery transfer progress message failed");
            }
            last_text = text;
        }

        // 读取动态配置刷新休眠间隔
        let interval = crate::tgbot::transfer::runtime_config_on(app_context.as_ref())
            .progress_edit_interval_seconds
            .max(1);
        tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
    }
}

/// 将最终执行结果写回同一条进度面板，替换进度状态为终态卡片。
///
/// # 参数
/// - `source_link`: 转存源链接
/// - `target_chat_id`: 目标聊天会话 ID
/// - `result`: 执行结果引用
/// - `notify_chat_id`: 通知目标会话 ID
/// - `message_id`: 待编辑的原卡片消息 ID
/// - `client_id`: 客户端实例 ID
pub(super) async fn edit_transfer_progress_for_outcome(
    source_link: &str,
    target_chat_id: i64,
    result: &anyhow::Result<workflow::TransferOutcome>,
    notify_chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    // 若转存成功或命中历史复用，读取已发送的结果消息列表供结果展示
    let outcome_result_messages = match result {
        Ok(workflow::TransferOutcome::Reused { job_id, link })
        | Ok(workflow::TransferOutcome::Completed { job_id, link }) => {
            let records = store::list_result_messages_by_job(*job_id)
                .await
                .unwrap_or_else(|err| {
                    tracing::warn!(
                        job_id,
                        error = %err,
                        "load result messages for progress final text failed"
                    );
                    Vec::new()
                });
            Some(crate::tgbot::transfer::outcome::normalize_result_messages(
                records,
                link,
                target_chat_id,
            ))
        }
        _ => None,
    };

    // 匹配结果类型，构造最终文本与内联键盘
    let (text, keyboard) = match result {
        Ok(workflow::TransferOutcome::Reused { job_id, link }) => (
            format_transfer_final_text_with_results(
                "已存在历史转存结果",
                source_link,
                target_chat_id,
                Some(*job_id),
                outcome_result_messages.as_deref().unwrap_or(&[]),
            ),
            build_transfer_result_keyboard(source_link, target_chat_id, Some(*job_id), Some(link)),
        ),
        Ok(workflow::TransferOutcome::Running { job_id }) => (
            format_transfer_control_text(
                "相同链接正在转存中",
                "running",
                source_link,
                target_chat_id,
                *job_id,
                "可以继续观察当前进度，或使用停止命令取消。",
            ),
            build_transfer_progress_keyboard(
                Some(*job_id),
                Some(store::JOB_STATUS_RUNNING),
                source_link,
                target_chat_id,
            ),
        ),
        Ok(workflow::TransferOutcome::Paused { job_id }) => (
            format_transfer_control_text(
                "转存任务已暂停",
                "paused",
                source_link,
                target_chat_id,
                *job_id,
                "恢复后会从已有子项状态继续处理。",
            ),
            build_transfer_progress_keyboard(
                Some(*job_id),
                Some(store::JOB_STATUS_PAUSED),
                source_link,
                target_chat_id,
            ),
        ),
        Ok(workflow::TransferOutcome::Cancelling { job_id }) => (
            format_transfer_control_text(
                "转存任务正在停止",
                "cancelling",
                source_link,
                target_chat_id,
                *job_id,
                "当前下载/上传调用会在安全点收尾。",
            ),
            build_transfer_progress_keyboard(
                Some(*job_id),
                Some(store::JOB_STATUS_CANCELLING),
                source_link,
                target_chat_id,
            ),
        ),
        Ok(workflow::TransferOutcome::Cancelled { job_id }) => (
            format_transfer_control_text(
                "转存任务已停止",
                "cancelled",
                source_link,
                target_chat_id,
                *job_id,
                "文件引用已释放，后续由删除队列清理。",
            ),
            build_transfer_progress_keyboard(
                Some(*job_id),
                Some(store::JOB_STATUS_CANCELLED),
                source_link,
                target_chat_id,
            ),
        ),
        Ok(workflow::TransferOutcome::Completed { job_id, link }) => (
            format_transfer_final_text_with_results(
                "转存完成",
                source_link,
                target_chat_id,
                Some(*job_id),
                outcome_result_messages.as_deref().unwrap_or(&[]),
            ),
            build_transfer_result_keyboard(source_link, target_chat_id, Some(*job_id), Some(link)),
        ),
        Err(err) => (
            format_transfer_error_text("转存失败", source_link, target_chat_id, &err.to_string()),
            build_transfer_result_keyboard(source_link, target_chat_id, None, None),
        ),
    };

    // 编辑消息
    crate::tgbot::send::edit_card_message_with_inline_keyboard(
        text,
        notify_chat_id,
        message_id,
        keyboard,
        client_id,
    )
    .await
}
