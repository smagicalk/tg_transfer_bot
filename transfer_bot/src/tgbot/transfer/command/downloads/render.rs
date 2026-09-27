// `/downloads` 的文本渲染。
// 该模块只把已经查询好的任务快照渲染为 card 标记文本。

use super::super::common::{build_page_empty_note, build_ready_page_header, format_bytes};
use super::types::DownloadsArgs;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store;

/// 将任务快照渲染成便于 Telegram 阅读的富文本卡片格式。
///
/// # 参数
/// - `snapshots`: 当前页所包含的任务进度快照切片
/// - `args`: 当前下载列表请求的筛选条件及分页参数
/// - `total`: 命中当前筛选条件的历史任务总条目数
pub(super) fn format_downloads_text(
    snapshots: &[store::JobProgressSnapshot],
    args: &DownloadsArgs,
    total: usize,
) -> String {
    // 根据总条数和单页容量计算总页码
    let total_pages = compute_total_pages(total, args.limit);
    // 标注当前查询的数据权限范围（全量任务）
    let scope_label = "范围：所有任务";

    // 针对空列表场景做特殊空态排版展示
    if snapshots.is_empty() {
        let page_label = format!("{}/{total_pages}", args.page);
        // 生成统一样式的空页面头部提示
        let mut lines = build_ready_page_header("下载列表为空");
        // 追加当前筛选条件、当前页码与每页容量
        lines.push(format!(
            "筛选：{}  页码：{}  每页：{}",
            card::code(args.filter.label()),
            card::code(page_label),
            card::code(args.limit),
        ));
        lines.push(scope_label.to_owned());
        // 增加友好提示，引导用户切换筛选或稍后刷新
        lines.push(build_page_empty_note("可切换筛选或稍后刷新。"));
        return lines.join("\n");
    }

    let mut lines = Vec::new();
    // 渲染列表总体标题及状态分页摘要
    lines.push(format!(
        "下载列表 · {}\n页码：{}  每页：{}  总数：{}",
        card::code(args.filter.label()),
        card::code(format!("{}/{total_pages}", args.page)),
        card::code(args.limit),
        card::code(total)
    ));
    // 插入视觉分割线
    lines.push(card::DIVIDER.to_owned());
    lines.push(scope_label.to_owned());

    // 遍历当前页的任务快照，逐个构建详情卡片
    for snapshot in snapshots {
        lines.push(card::DIVIDER.to_owned());
        // 任务总项数（兜底最小为 0）
        let total = snapshot.job.total_items.max(0);
        // 已完成子项数（成功 + 失败 + 取消）
        let finished = snapshot.success_count + snapshot.failed_count + snapshot.cancelled_count;

        // 渲染任务 ID 区块标题与状态摘要行
        lines.push(format!(
            "{}\n{}",
            card::section(&format!("任务 #{}", snapshot.job.id)),
            card::summary_line(
                &snapshot.job.status,
                Some(snapshot.job.id),
                snapshot.job.target_chat_id
            )
        ));
        // 渲染总项数进度（如 5/10）
        lines.push(card::field("总进度", format!("{finished}/{total}")));
        // 渲染图形化百分比进度条
        lines.push(card::field(
            "完成率",
            card::progress_bar(finished.into(), total.into()),
        ));
        // 成对展示排队与运行子项数
        lines.push(card::field_pair(
            "等待/下载",
            format!("{}/{}", snapshot.pending_count, snapshot.preparing_count),
            "就绪/上传",
            format!("{}/{}", snapshot.prepared_count, snapshot.uploading_count),
        ));
        // 成对展示终态子项数
        lines.push(card::field_pair(
            "成功/失败",
            format!("{}/{}", snapshot.success_count, snapshot.failed_count),
            "已停",
            snapshot.cancelled_count,
        ));

        // 若当前有正在进行的底层文件下载，则渲染实时下载进度条与速度数据
        if snapshot.active_download_files > 0 {
            lines.push(format!("真实下载：{}", format_live_download(snapshot)));
        }

        // 记录最近更新时间（转换为 UTC+8 友好格式）
        lines.push(card::field(
            "更新",
            snapshot.job.updated_at.format("%Y-%m-%d %H:%M:%S"),
        ));
    }

    // 合并为最终的富文本字符串
    lines.join("\n")
}

/// 计算总页数，即使无任何数据也至少按 1 页展示。
///
/// # 参数
/// - `total`: 条目总数
/// - `limit`: 单页容量
pub(super) fn compute_total_pages(total: usize, limit: u64) -> u64 {
    if total == 0 {
        1
    } else {
        ((total as u64 - 1) / limit.max(1)) + 1
    }
}

/// 计算 `/downloads` 向底层数据库拉取数据的查询窗口。
/// 当前阶段筛选和分页仍在命令层统一计算，因此这里适当放大拉取范围以保证翻页连续性。
///
/// # 参数
/// - `limit`: 单页条目数
/// - `page`: 当前请求的页码
pub(super) fn compute_downloads_query_limit(limit: u64, page: u64) -> u64 {
    limit
        .saturating_mul(page.max(1))
        .saturating_mul(10)
        .clamp(50, 500)
}

/// 渲染某个任务的真实物理下载进度摘要。
///
/// 包含活跃下载文件数、已下载字节数、总字节数以及进度条。
fn format_live_download(snapshot: &store::JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_download_files);
    // 当总大小明确且大于 0 时，显示精确百分比与进度条
    if snapshot.active_download_total_bytes > 0 && !snapshot.has_unknown_download_total {
        let progress = snapshot.active_downloaded_bytes.saturating_mul(100)
            / snapshot.active_download_total_bytes.max(1);
        return format!(
            "{} {}/{}\n{}",
            prefix,
            format_bytes(snapshot.active_downloaded_bytes),
            format_bytes(snapshot.active_download_total_bytes),
            card::progress_bar_percent(progress)
        );
    }

    // 若部分文件总大小未知，但存在已知已下载字节
    if snapshot.active_download_total_bytes > 0 {
        return format!(
            "{} 已下 {} / 已知总量 {}+",
            prefix,
            format_bytes(snapshot.active_downloaded_bytes),
            format_bytes(snapshot.active_download_total_bytes)
        );
    }

    // 无法获取总大小时，只展示已下载的字节数
    format!(
        "{} 已下 {}",
        prefix,
        format_bytes(snapshot.active_downloaded_bytes)
    )
}
