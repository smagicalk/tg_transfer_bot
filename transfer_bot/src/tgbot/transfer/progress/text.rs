// 转存进度面板的文案渲染。
// 本模块只负责把任务快照和执行结果转换成 Markdown 文本，不触碰 TDLib 调用。

use crate::tgbot::transfer::{store, types};

const CARD_DIVIDER: &str = "━━━━━━━━━━━━";

/// 构造任务尚未入库时的等待文本。
pub(super) fn format_transfer_waiting_text(plan: &types::TransferPlan) -> String {
    format!(
        "*转存进度*\n状态：`waiting`\n目标：`{}`\n{}\n说明：正在排队或抓取源消息。\n源：`{}`",
        plan.target_chat_id,
        CARD_DIVIDER,
        markdown_inline_code(&plan.source_link)
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
        format!("状态：`{}`", snapshot.job.status),
        CARD_DIVIDER.to_owned(),
        format!("进度：`{}/{} ({}%)`", finished, total, progress),
        format!("目标：`{}`", snapshot.job.target_chat_id),
        format!(
            "阶段：等待 `{}` | 下载 `{}` | 就绪 `{}` | 上传 `{}`",
            snapshot.pending_count,
            snapshot.preparing_count,
            snapshot.prepared_count,
            snapshot.uploading_count
        ),
        format!(
            "结果：成功 `{}` | 失败 `{}` | 已停 `{}`",
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
    lines.push(format!("源：`{}`", markdown_inline_code(source_link)));
    lines.join("\n")
}

/// 构造完成或复用历史结果的最终文本。
pub(super) fn format_transfer_final_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    result_link: &str,
) -> String {
    let result_line = if crate::tgbot::send::is_openable_url(result_link) {
        "结果：`可打开，见下方按钮`".to_owned()
    } else {
        format!(
            "结果：`已上传，但当前 chat 无可跳转公开链接`\n定位：`{}`",
            markdown_inline_code(result_link)
        )
    };

    format!(
        "*{}*\n状态：`success`\n目标：`{}`\n{}\n{}\n源：`{}`",
        title,
        target_chat_id,
        CARD_DIVIDER,
        result_line,
        markdown_inline_code(source_link)
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
        "*{}*\njob：`#{}`\n目标：`{}`\n{}\n说明：{}\n源：`{}`",
        title,
        job_id,
        target_chat_id,
        CARD_DIVIDER,
        detail,
        markdown_inline_code(source_link)
    )
}

/// 构造失败结果文本。
pub(super) fn format_transfer_error_text(
    source_link: &str,
    target_chat_id: i64,
    error: &str,
) -> String {
    format!(
        "*转存失败*\n状态：`failed`\n目标：`{}`\n{}\n错误：`{}`\n源：`{}`",
        target_chat_id,
        CARD_DIVIDER,
        markdown_inline_code(error),
        markdown_inline_code(source_link)
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
