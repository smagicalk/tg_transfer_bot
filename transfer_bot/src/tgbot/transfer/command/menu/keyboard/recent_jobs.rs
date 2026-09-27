//! 任务中心（TasksHub）最近任务快捷操作按钮构建模块。
//!
//! 在任务中心界面中，展示最近 5 个任务的实时状态入口与直接控制按钮（暂停、恢复、停止）。
//! 本模块生成的按钮全部复用既有的 `/job` 模块回调数据格式，不重复开发任务控制的底层逻辑。

use crate::tgbot::send;

use super::super::super::super::store;
use super::super::super::job::{
    build_job_pause_callback_data, build_job_resume_callback_data, build_job_status_callback_data,
    build_job_stop_callback_data,
};

/// 构建最近任务列表的快捷按钮行。
///
/// 针对传入的最近任务快照列表（最多取前 5 条），每个任务生成独立的一整行按钮：
/// - 左侧：展示任务编号与当前状态（点击可直接查看该任务详情 `/job <id>`）；
/// - 右侧：根据当前任务状态追加快捷操作按钮（如“暂停”、“恢复”、“停止”）。
///
/// # 参数说明
/// - `recent_jobs`: 最近任务的进度快照列表切片
///
/// # 返回值
/// 返回按钮行列表；若传入的任务列表为空，则返回空向量。
pub(super) fn recent_job_buttons(
    recent_jobs: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    // 若无任何任务记录，直接返回空列表
    if recent_jobs.is_empty() {
        return Vec::new();
    }

    // 仅截取最新的前 5 个任务，避免按钮过长撑满聊天界面
    recent_jobs
        .iter()
        .take(5)
        .map(|snapshot| {
            let status = snapshot.job.status.as_str();
            // 活跃态（等待中、运行中、已暂停）使用高亮主色，其余终态（已完成、已失败、已取消）使用默认样式
            let style = if matches!(
                status,
                store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING | store::JOB_STATUS_PAUSED
            ) {
                tdlib_rs::enums::ButtonStyle::Primary
            } else {
                tdlib_rs::enums::ButtonStyle::Default
            };
            let job_id = snapshot.job.id;
            // 任务详情入口按钮
            let mut row = vec![send::build_callback_button(
                &format!("#{} {}", snapshot.job.id, snapshot.job.status),
                &build_job_status_callback_data(snapshot.job.id),
                style,
            )];
            // 根据任务当前运行状态动态追加操作按钮
            row.extend(recent_job_control_buttons(job_id, status));
            row
        })
        .collect::<Vec<_>>()
}

/// 根据任务当前状态构建快捷控制按钮集合。
///
/// 控制按钮映射规则：
/// - `PENDING` 或 `RUNNING` 状态：
///   - “暂停”按钮（Default 样式）：触发任务暂停
///   - “停止”按钮（Danger 警示样式）：触发任务终止（带二次确认）
/// - `PAUSED` 状态：
///   - “恢复”按钮（Primary 样式）：触发任务恢复执行
///   - “停止”按钮（Danger 警示样式）：触发任务终止
/// - 其他非活跃终态（如 COMPLETED、FAILED、CANCELLED）：
///   - 不提供直接行内控制按钮，仅保留详情入口
///
/// # 参数说明
/// - `job_id`: 目标任务的唯一数据库自增 ID
/// - `status`: 任务当前状态字符串切片
///
/// # 返回值
/// 返回该任务所附属的控制按钮向量。
fn recent_job_control_buttons(
    job_id: i64,
    status: &str,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    // 待处理或运行中：可暂停，可停止
    if matches!(
        status,
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
    ) {
        return vec![
            send::build_callback_button(
                "暂停",
                &build_job_pause_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ];
    }

    // 已暂停：可恢复，可停止
    if status == store::JOB_STATUS_PAUSED {
        return vec![
            send::build_callback_button(
                "恢复",
                &build_job_resume_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ];
    }

    // 终态或其他状态不展示行内控制按钮
    Vec::new()
}
