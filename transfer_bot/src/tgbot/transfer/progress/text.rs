// 转存进度面板的文案渲染。
// 本模块只负责把任务快照和执行结果转换成 Markdown 文本，不触碰 TDLib 调用。

use crate::tgbot::transfer::{store, types};

/// 构造任务尚未入库时的等待文本。
pub(super) fn format_transfer_waiting_text(plan: &types::TransferPlan) -> String {
    format!(
        "*转存进度*\n源链接：`{}`\n目标 chat：`{}`\n状态：`waiting`\n正在排队或抓取源消息。",
        plan.source_link, plan.target_chat_id
    )
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
        format!("*转存进度* `#{}`", snapshot.job.id),
        format!("源链接：`{}`", source_link),
        format!("目标 chat：`{}`", snapshot.job.target_chat_id),
        format!(
            "状态：`{}`  进度：`{}/{} ({}%)`",
            snapshot.job.status, finished, total, progress
        ),
        format!(
            "等待 `{}` | 下载中 `{}` | 已就绪 `{}` | 上传中 `{}`",
            snapshot.pending_count,
            snapshot.preparing_count,
            snapshot.prepared_count,
            snapshot.uploading_count
        ),
        format!(
            "成功 `{}` | 失败 `{}` | 已停 `{}`",
            snapshot.success_count, snapshot.failed_count, snapshot.cancelled_count
        ),
    ];

    if snapshot.active_download_files > 0 {
        lines.push(format!(
            "真实下载：{}",
            format_progress_live_download(snapshot)
        ));
    }

    lines.push(format!(
        "更新：`{}`",
        snapshot.job.updated_at.format("%Y-%m-%d %H:%M:%S")
    ));
    lines.join("\n")
}

/// 构造完成或复用历史结果的最终文本。
pub(super) fn format_transfer_final_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    result_link: &str,
) -> String {
    format!(
        "*{}*\n源链接：`{}`\n目标 chat：`{}`\n结果消息：[打开转存消息]({})",
        title, source_link, target_chat_id, result_link
    )
}

/// 构造暂停、停止、运行中这类控制态文本。
pub(super) fn format_transfer_control_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    detail: &str,
) -> String {
    format!(
        "*{}*\n源链接：`{}`\n目标 chat：`{}`\njob_id：`{}`\n{}",
        title, source_link, target_chat_id, job_id, detail
    )
}

/// 构造失败结果文本。
pub(super) fn format_transfer_error_text(
    source_link: &str,
    target_chat_id: i64,
    error: &str,
) -> String {
    format!(
        "*转存失败*\n源链接：`{}`\n目标 chat：`{}`\n错误：`{}`",
        source_link,
        target_chat_id,
        markdown_inline_code(error)
    )
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

/// 转义 Markdown 行内代码里的反引号，避免错误文本破坏面板格式。
fn markdown_inline_code(text: &str) -> String {
    text.replace('`', "'")
}
