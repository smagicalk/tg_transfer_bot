// 转存进度面板的文案渲染。
// 本模块只负责把任务快照和执行结果转换成 card 标记文本，不触碰 TDLib 调用。

use crate::tgbot::transfer::{card, store, types};

/// 构造任务尚未入库时的等待文本。
pub(super) fn format_transfer_waiting_text(plan: &types::TransferPlan) -> String {
    let mut lines = vec![
        "转存进度 · 等待".to_owned(),
        card::summary_line("waiting", None, plan.target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("当前阶段"),
        card::note("正在排队或抓取源消息，任务创建后会自动刷新为实时进度。"),
        String::new(),
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
    let mut lines = vec![
        format!("转存进度 {}", card::job_ref(snapshot.job.id)),
        card::summary_line(
            &snapshot.job.status,
            Some(snapshot.job.id),
            snapshot.job.target_chat_id,
        ),
        card::DIVIDER.to_owned(),
        card::section("进度"),
        card::field("总进度", format!("{finished}/{total}")),
        card::field("完成率", card::progress_bar(finished.into(), total.into())),
        card::field("更新", snapshot.job.updated_at.format("%Y-%m-%d %H:%M:%S")),
        card::field_pair(
            "等待/下载",
            format!("{}/{}", snapshot.pending_count, snapshot.preparing_count),
            "就绪/上传",
            format!("{}/{}", snapshot.prepared_count, snapshot.uploading_count),
        ),
        card::field_pair(
            "成功/失败",
            format!("{}/{}", snapshot.success_count, snapshot.failed_count),
            "已停",
            snapshot.cancelled_count,
        ),
    ];

    if snapshot.active_download_files > 0 {
        lines.push(format!(
            "真实下载：{}",
            format_progress_live_download(snapshot)
        ));
    }
    if snapshot.active_upload_files > 0 {
        lines.push(format!(
            "真实上传：{}",
            format_progress_live_upload(snapshot)
        ));
    }

    lines.push(card::note(
        "可直接点击下方按钮查看详情或控制任务；需要命令时点击“查看命令”。",
    ));
    lines.push(String::new());
    lines.extend(card::source_link_block(source_link));
    lines.join("\n")
}

/// 构造完成或复用历史结果的最终文本，并支持多个结果入口。
pub(super) fn format_transfer_final_text_with_results(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: Option<i64>,
    result_messages: &[store::ResultMessageRecord],
) -> String {
    crate::tgbot::transfer::outcome::format_result_card_text(
        title,
        source_link,
        target_chat_id,
        job_id,
        result_messages,
    )
}

/// 构造暂停、停止、运行中这类控制态文本。
pub(super) fn format_transfer_control_text(
    title: &str,
    status: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    detail: &str,
) -> String {
    crate::tgbot::transfer::outcome::format_status_card_text(
        title,
        status,
        source_link,
        target_chat_id,
        job_id,
        detail,
    )
}

/// 构造失败结果文本。
pub(super) fn format_transfer_error_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    error: &str,
) -> String {
    crate::tgbot::transfer::outcome::format_failure_card_text(
        title,
        source_link,
        target_chat_id,
        None,
        &anyhow::anyhow!(error.to_owned()),
    )
}

/// 渲染真实下载进度。
fn format_progress_live_download(snapshot: &store::JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_download_files);
    if snapshot.active_download_total_bytes > 0 && !snapshot.has_unknown_download_total {
        let progress = snapshot.active_downloaded_bytes.saturating_mul(100)
            / snapshot.active_download_total_bytes.max(1);
        return format!(
            "{} {}/{}\n{}",
            prefix,
            format_progress_bytes(snapshot.active_downloaded_bytes),
            format_progress_bytes(snapshot.active_download_total_bytes),
            card::progress_bar_percent(progress)
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

/// 渲染目标发送客户端报告的真实上传进度。
fn format_progress_live_upload(snapshot: &store::JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_upload_files);
    if snapshot.active_upload_total_bytes > 0 && !snapshot.has_unknown_upload_total {
        let progress = snapshot.active_uploaded_bytes.saturating_mul(100)
            / snapshot.active_upload_total_bytes.max(1);
        return format!(
            "{} {}/{}\n{}",
            prefix,
            format_progress_bytes(snapshot.active_uploaded_bytes),
            format_progress_bytes(snapshot.active_upload_total_bytes),
            card::progress_bar_percent(progress)
        );
    }

    if snapshot.active_upload_total_bytes > 0 {
        return format!(
            "{} 已传 {} / 已知总量 {}+",
            prefix,
            format_progress_bytes(snapshot.active_uploaded_bytes),
            format_progress_bytes(snapshot.active_upload_total_bytes)
        );
    }

    format!(
        "{} 已传 {}",
        prefix,
        format_progress_bytes(snapshot.active_uploaded_bytes)
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
