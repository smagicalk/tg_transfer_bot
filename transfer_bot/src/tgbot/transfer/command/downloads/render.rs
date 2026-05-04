// `/downloads` 的文本渲染。
// 该模块只把已经查询好的任务快照渲染为 card 标记文本。

use super::super::common::{CommandStyle, format_bytes, short_and_long};
use super::keyboard::build_downloads_page_command;
use super::types::DownloadsArgs;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store;

/// 将任务快照渲染成便于 Telegram 阅读的文本。
pub(super) fn format_downloads_text(
    snapshots: &[store::JobProgressSnapshot],
    args: &DownloadsArgs,
    total: usize,
) -> String {
    let total_pages = compute_total_pages(total, args.limit);
    if snapshots.is_empty() {
        return format!(
            "下载列表为空\n筛选：{}\n页码：{}  每页：{}\n{}\n命令：{}\n说明：可切换筛选或稍后刷新。",
            card::code(args.filter.label()),
            card::code(format!("{}/{}", args.page, total_pages)),
            card::code(args.limit),
            card::DIVIDER,
            short_and_long(
                build_downloads_page_command(
                    args.filter,
                    args.limit,
                    args.page,
                    CommandStyle::Short
                ),
                build_downloads_page_command(
                    args.filter,
                    args.limit,
                    args.page,
                    CommandStyle::Long
                ),
            )
        );
    }

    let mut lines = Vec::new();
    lines.push(format!(
        "下载列表\n筛选：{}\n页码：{}  每页：{}  总数：{}",
        card::code(args.filter.label()),
        card::code(format!("{}/{}", args.page, total_pages)),
        card::code(args.limit),
        card::code(total)
    ));
    lines.push(card::DIVIDER.to_owned());
    lines.push(format!(
        "命令：{}",
        short_and_long(
            build_downloads_page_command(args.filter, args.limit, args.page, CommandStyle::Short),
            build_downloads_page_command(args.filter, args.limit, args.page, CommandStyle::Long),
        )
    ));

    for snapshot in snapshots {
        lines.push(card::DIVIDER.to_owned());
        let total = snapshot.job.total_items.max(0);
        let finished = snapshot.success_count + snapshot.failed_count + snapshot.cancelled_count;
        let progress = if total <= 0 {
            0
        } else {
            finished.saturating_mul(100) / total
        };

        lines.push(format!(
            "{}\n{}",
            card::section(&format!("任务 #{}", snapshot.job.id)),
            card::summary_line(
                &snapshot.job.status,
                Some(snapshot.job.id),
                snapshot.job.target_chat_id
            )
        ));
        lines.push(card::field(
            "进度",
            format!("{}/{} ({}%)", finished, total, progress),
        ));
        lines.push(card::field_pair(
            "等待/下载",
            format!("{}/{}", snapshot.pending_count, snapshot.preparing_count),
            "就绪/上传",
            format!("{}/{}", snapshot.prepared_count, snapshot.uploading_count),
        ));
        lines.push(card::field_pair(
            "成功/失败",
            format!("{}/{}", snapshot.success_count, snapshot.failed_count),
            "已停",
            snapshot.cancelled_count,
        ));

        if snapshot.active_download_files > 0 {
            lines.push(format!(
                "真实下载：{}",
                card::code(format_live_download(snapshot))
            ));
        }

        lines.push(card::field(
            "更新",
            snapshot.job.updated_at.format("%Y-%m-%d %H:%M:%S"),
        ));
    }

    lines.join("\n")
}

/// 计算总页数，空结果也至少按 1 页展示。
pub(super) fn compute_total_pages(total: usize, limit: u64) -> u64 {
    if total == 0 {
        1
    } else {
        ((total as u64 - 1) / limit.max(1)) + 1
    }
}

/// 计算 `/downloads` 的查询窗口。
/// 当前阶段分页仍在命令层完成，所以这里适当放大查询范围。
pub(super) fn compute_downloads_query_limit(limit: u64, page: u64) -> u64 {
    limit
        .saturating_mul(page.max(1))
        .saturating_mul(10)
        .clamp(50, 500)
}

/// 渲染某个任务的真实下载进度摘要。
fn format_live_download(snapshot: &store::JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_download_files);
    if snapshot.active_download_total_bytes > 0 && !snapshot.has_unknown_download_total {
        let progress = snapshot.active_downloaded_bytes.saturating_mul(100)
            / snapshot.active_download_total_bytes.max(1);
        return format!(
            "{} {}/{} ({}%)",
            prefix,
            format_bytes(snapshot.active_downloaded_bytes),
            format_bytes(snapshot.active_download_total_bytes),
            progress
        );
    }

    if snapshot.active_download_total_bytes > 0 {
        return format!(
            "{} 已下 {} / 已知总量 {}+",
            prefix,
            format_bytes(snapshot.active_downloaded_bytes),
            format_bytes(snapshot.active_download_total_bytes)
        );
    }

    format!(
        "{} 已下 {}",
        prefix,
        format_bytes(snapshot.active_downloaded_bytes)
    )
}
