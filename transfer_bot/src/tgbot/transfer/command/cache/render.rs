// `/cache` 文案渲染。

use super::types::CacheArgs;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::command::common::{build_page_empty_note, build_ready_page_header};
use crate::tgbot::transfer::store;

/// 计算缓存页数。
pub(super) fn compute_cache_page_count(total: usize, limit: u64) -> u64 {
    if total == 0 {
        1
    } else {
        ((total as u64 - 1) / limit.max(1)) + 1
    }
}

/// 渲染缓存概览文本。
pub(super) fn format_cache_summary_text(
    health: &store::TransferHealthSnapshot,
    summary_rows: &[store::FileCacheStatusSummary],
) -> String {
    let mut lines = build_ready_page_header("文件缓存概览");
    lines.extend([
        card::section("总览"),
        card::field("记录数", health.file_cache_rows),
        card::field("活跃记录", health.file_cache_active_rows),
        card::field("待删除", health.file_cache_due_rows),
        card::field("删除失败", health.file_cache_failed_rows),
        card::field("总任务", health.total_jobs),
        card::field("活跃任务", health.active_jobs),
    ]);
    lines.push(card::section("状态分布"));
    if summary_rows.is_empty() {
        lines.push("暂无缓存记录".to_owned());
    } else {
        for row in summary_rows {
            lines.push(card::field_pair(
                &row.status,
                row.count,
                "refs",
                row.active_refs,
            ));
        }
    }
    lines.join("\n")
}

/// 渲染缓存分页文本。
pub(super) fn format_cache_page_text(
    health: &store::TransferHealthSnapshot,
    rows: &[store::FileCacheSnapshot],
    args: &CacheArgs,
    total_pages: u64,
) -> String {
    let mut lines = build_ready_page_header("文件缓存列表");
    lines.extend([
        card::section("分页"),
        card::field("页码", format!("{}/{}", args.page, total_pages)),
        card::field("每页", args.limit),
        card::field("总数", health.file_cache_rows),
    ]);
    if rows.is_empty() {
        lines.push(build_page_empty_note("当前页没有缓存记录。"));
        return lines.join("\n");
    }

    for row in rows {
        lines.push(card::DIVIDER.to_owned());
        lines.push(card::section(&format!(
            "{} · {}",
            row.owner_client_role, row.file_key
        )));
        lines.push(card::field("状态", &row.status));
        lines.push(card::field("引用", row.active_refs));
        if let Some(size) = row.size_bytes {
            lines.push(card::field("大小", size));
        }
        if let Some(td_file_id) = row.td_file_id {
            lines.push(card::field("td_file_id", td_file_id));
        }
        if let Some(delete_after) = row.delete_after {
            lines.push(card::field(
                "删除时间",
                delete_after.format("%Y-%m-%d %H:%M:%S"),
            ));
        }
        lines.push(card::field(
            "更新",
            row.updated_at.format("%Y-%m-%d %H:%M:%S"),
        ));
        lines.push(card::field(
            "最近使用",
            row.last_used_at.format("%Y-%m-%d %H:%M:%S"),
        ));
        if let Some(local_path) = row.local_path.as_deref() {
            lines.push(card::code(local_path));
        }
        if let Some(last_error) = row.last_error.as_deref() {
            lines.push(card::pre_code(last_error));
        }
    }

    lines.join("\n")
}
