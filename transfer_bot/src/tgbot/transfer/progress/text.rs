// 转存进度面板的文案渲染。
// 本模块只负责把任务快照和执行结果转换成 card 标记文本，不触碰 TDLib 调用。

use crate::tgbot::transfer::{card, store, types};

/// 构造任务尚未入库时的等待文本。
pub(super) fn format_transfer_waiting_text(plan: &types::TransferPlan) -> String {
    let mut lines = vec![
        "转存进度".to_owned(),
        card::status_target("waiting", plan.target_chat_id),
        card::DIVIDER.to_owned(),
        "说明：正在排队或抓取源消息。".to_owned(),
    ];
    lines.extend(card::source_link_block(&plan.source_link));
    lines.join("\n")
}

/// 构造单任务进度文本。
pub(super) fn format_transfer_progress_text(
    snapshot: &store::JobProgressSnapshot,
    source_link: &str,
) -> String {
    let total = snapshot.job.total_items.max(0);
    let finished = snapshot.success_count + snapshot.failed_count + snapshot.cancelled_count;
    let progress = if total <= 0 {
        0
    } else {
        finished.saturating_mul(100) / total
    };
    let mut lines = vec![
        format!("转存进度 {}", card::job_ref(snapshot.job.id)),
        format!(
            "{}",
            card::status_target(&snapshot.job.status, snapshot.job.target_chat_id)
        ),
        card::DIVIDER.to_owned(),
        format!(
            "{}：{}",
            card::section("进度"),
            card::code(format!("{}/{} ({}%)", finished, total, progress))
        ),
        format!(
            "阶段：等待 {} | 下载 {} | 就绪 {} | 上传 {}",
            card::code(snapshot.pending_count),
            card::code(snapshot.preparing_count),
            card::code(snapshot.prepared_count),
            card::code(snapshot.uploading_count)
        ),
        format!(
            "结果：成功 {} | 失败 {} | 已停 {}",
            card::code(snapshot.success_count),
            card::code(snapshot.failed_count),
            card::code(snapshot.cancelled_count)
        ),
    ];

    if snapshot.active_download_files > 0 {
        lines.push(format!(
            "真实下载：{}",
            card::code(format_progress_live_download(snapshot))
        ));
    }

    lines.push(format!(
        "更新：{}",
        card::code(snapshot.job.updated_at.format("%Y-%m-%d %H:%M:%S"))
    ));
    lines.extend(card::source_link_block(source_link));
    lines.join("\n")
}

/// 构造完成或复用历史结果的最终文本。
pub(super) fn format_transfer_final_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    result_link: &str,
) -> String {
    let mut lines = vec![
        title.to_owned(),
        card::status_target("success", target_chat_id),
        card::DIVIDER.to_owned(),
        card::result_block(result_link),
        String::new(),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

/// 构造暂停、停止、运行中这类控制态文本。
pub(super) fn format_transfer_control_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    detail: &str,
) -> String {
    let mut lines = vec![
        title.to_owned(),
        format!(
            "job：{}  目标：{}",
            card::job_ref(job_id),
            card::code(target_chat_id)
        ),
        card::DIVIDER.to_owned(),
        format!("说明：{}", detail),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

/// 构造失败结果文本。
pub(super) fn format_transfer_error_text(
    source_link: &str,
    target_chat_id: i64,
    error: &str,
) -> String {
    let mut lines = vec![
        "转存失败".to_owned(),
        card::status_target("failed", target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("错误"),
        card::code(error),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

/// 渲染真实下载进度。
fn format_progress_live_download(snapshot: &store::JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_download_files);
    if snapshot.active_download_total_bytes > 0 && !snapshot.has_unknown_download_total {
        let progress = snapshot.active_downloaded_bytes.saturating_mul(100)
            / snapshot.active_download_total_bytes.max(1);
        return format!(
            "{} {}/{} ({}%)",
            prefix,
            format_progress_bytes(snapshot.active_downloaded_bytes),
            format_progress_bytes(snapshot.active_download_total_bytes),
            progress
        );
    }

    if snapshot.active_download_total_bytes > 0 {
        return format!(
            "{} 已下 {} / 已知总量 {}+",
            prefix,
            format_progress_bytes(snapshot.active_downloaded_bytes),
            format_progress_bytes(snapshot.active_download_total_bytes)
        );
    }

    format!(
        "{} 已下 {}",
        prefix,
        format_progress_bytes(snapshot.active_downloaded_bytes)
    )
}

/// 以人类可读形式展示字节数。
pub(super) fn format_progress_bytes(bytes: i64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes.max(0) as f64;
    let mut unit_idx = 0usize;
    while value >= 1024.0 && unit_idx < units.len() - 1 {
        value /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", value as i64, units[unit_idx])
    } else {
        format!("{:.1} {}", value, units[unit_idx])
    }
}
