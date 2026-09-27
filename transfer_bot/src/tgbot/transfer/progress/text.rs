// 转存进度面板的文案渲染。
// 本模块只负责把任务快照和执行结果转换成 card 标记文本，不触碰 TDLib 调用。

use crate::tgbot::transfer::{card, store, types};

/// 构造任务尚未入库时的等待文本卡片内容。
///
/// 当用户发送转存链接后，后台尚在抓取源信息或进入排队队列时，通过此卡片向用户展示等待状态。
///
/// # 参数
/// - `plan`: 转存规划详情引用
///
/// # 返回
/// 格式化后的 Markdown 卡片正文
pub(super) fn format_transfer_waiting_text(plan: &types::TransferPlan) -> String {
    let mut lines = vec![
        "转存进度 · 等待".to_owned(),
        card::summary_line("waiting", None, plan.target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("当前阶段"),
        card::note("正在排队或抓取源消息，任务创建后会自动刷新为实时进度。"),
        String::new(),
    ];
    // 附带源链接卡片块
    lines.extend(card::source_link_block(&plan.source_link));
    lines.join("\n")
}

/// 构造单任务实时进度文本卡片。
///
/// 包含任务状态、进度条、计数统计、实时下载/上传速率与字节数等。
///
/// # 参数
/// - `snapshot`: 任务进度快照数据引用
/// - `source_link`: 原始源链接
///
/// # 返回
/// 渲染完成的进度卡片正文字符串
pub(super) fn format_transfer_progress_text(
    snapshot: &store::JobProgressSnapshot,
    source_link: &str,
) -> String {
    // 规整总项数与已完成项数（成功 + 失败 + 已取消）
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

    // 若有活跃下载任务，展示实时下载文件进度和字节统计
    if snapshot.active_download_files > 0 {
        lines.push(format!(
            "真实下载：{}",
            format_progress_live_download(snapshot)
        ));
    }
    // 若有活跃上传任务，展示实时上传文件进度和字节统计
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
///
/// # 参数
/// - `title`: 卡片主标题
/// - `source_link`: 源链接
/// - `target_chat_id`: 目标聊天 ID
/// - `job_id`: 可选的任务 ID
/// - `result_messages`: 成功转存出的消息记录切片
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
///
/// # 参数
/// - `title`: 卡片主标题
/// - `status`: 状态标识
/// - `source_link`: 源链接
/// - `target_chat_id`: 目标聊天 ID
/// - `job_id`: 关联任务 ID
/// - `detail`: 详细说明文本
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
///
/// # 参数
/// - `title`: 标题
/// - `source_link`: 源链接
/// - `target_chat_id`: 目标聊天 ID
/// - `error`: 错误信息文本
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
///
/// # 参数
/// - `snapshot`: 任务进度快照
fn format_progress_live_download(snapshot: &store::JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_download_files);
    // 当总大小明确已知时，计算百分比并绘制进度条
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

    // 若总大小包含未知部分
    if snapshot.active_download_total_bytes > 0 {
        return format!(
            "{} 已下 {} / 已知总量 {}+",
            prefix,
            format_progress_bytes(snapshot.active_downloaded_bytes),
            format_progress_bytes(snapshot.active_download_total_bytes)
        );
    }

    // 仅知已下载字节数
    format!(
        "{} 已下 {}",
        prefix,
        format_progress_bytes(snapshot.active_downloaded_bytes)
    )
}

/// 渲染目标发送客户端报告的真实上传进度。
///
/// # 参数
/// - `snapshot`: 任务进度快照
fn format_progress_live_upload(snapshot: &store::JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_upload_files);
    // 当总大小明确已知时，计算百分比并绘制进度条
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

    // 若总大小包含未知部分
    if snapshot.active_upload_total_bytes > 0 {
        return format!(
            "{} 已传 {} / 已知总量 {}+",
            prefix,
            format_progress_bytes(snapshot.active_uploaded_bytes),
            format_progress_bytes(snapshot.active_upload_total_bytes)
        );
    }

    // 仅知已上传字节数
    format!(
        "{} 已传 {}",
        prefix,
        format_progress_bytes(snapshot.active_uploaded_bytes)
    )
}

/// 以人类可读形式展示字节数（自适应选择 B, KB, MB, GB, TB）。
///
/// # 参数
/// - `bytes`: 字节数
///
/// # 返回
/// 格式化后的字符串（例如 "1.5 MB"）
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
