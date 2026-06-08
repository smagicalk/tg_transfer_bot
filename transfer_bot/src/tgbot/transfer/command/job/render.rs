// `/job` 回复文本渲染。
// 只负责把任务快照转换成卡片文本，不执行数据库写入或 TDLib 调用。

use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store::{self, JobProgressSnapshot};

use super::super::common::{
    CommandStyle, downloads_command as build_downloads_command, format_bytes,
    job_command as build_job_command,
};

/// 构造 `/job` 动作结果卡片。
pub(super) fn format_job_action_text(
    title: &str,
    job_id: i64,
    status: &str,
    detail: &str,
) -> String {
    [
        title.to_owned(),
        format!(
            "job：{}  状态：{}",
            card::job_ref(job_id),
            card::code(status)
        ),
        card::DIVIDER.to_owned(),
        card::note(detail),
    ]
    .join("\n")
}

/// 构造单任务详情卡片。
pub(super) fn format_job_status_text(snapshot: &JobProgressSnapshot) -> String {
    let total = snapshot.job.total_items.max(0);
    let finished = snapshot.success_count + snapshot.failed_count + snapshot.cancelled_count;
    let mut lines = vec![
        "任务详情".to_owned(),
        card::summary_line(
            &snapshot.job.status,
            Some(snapshot.job.id),
            snapshot.job.target_chat_id,
        ),
        card::DIVIDER.to_owned(),
        card::section("进度"),
        card::field("总进度", format!("{}/{}", finished, total)),
        card::field("完成率", card::progress_bar(finished.into(), total.into())),
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
        lines.push(format!("真实下载：{}", format_job_live_download(snapshot)));
    }

    lines.push(card::section("时间"));
    lines.push(card::field(
        "创建",
        snapshot.job.created_at.format("%Y-%m-%d %H:%M:%S"),
    ));
    lines.push(card::field(
        "更新",
        snapshot.job.updated_at.format("%Y-%m-%d %H:%M:%S"),
    ));
    // 按钮在用户号登录模式下会被发送层统一禁用，因此正文也必须保留可复制命令。
    lines.push(card::section("命令"));
    lines.push(card::command_line(
        "详情",
        build_job_command("st", snapshot.job.id, CommandStyle::Short),
    ));
    match snapshot.job.status.as_str() {
        store::JOB_STATUS_PAUSED => {
            lines.push(card::command_line(
                "恢复",
                build_job_command("r", snapshot.job.id, CommandStyle::Short),
            ));
            lines.push(card::command_line(
                "停止",
                build_job_command("s", snapshot.job.id, CommandStyle::Short),
            ));
            lines.push(card::command_line(
                "列表",
                build_downloads_command(Some("pause"), None, None, CommandStyle::Short),
            ));
        }
        store::JOB_STATUS_CANCELLING
        | store::JOB_STATUS_CANCEL_FINALIZING
        | store::JOB_STATUS_CANCELLED => {
            lines.push(card::command_line(
                "列表",
                build_downloads_command(Some("cancel"), None, None, CommandStyle::Short),
            ));
        }
        _ => {
            lines.push(card::command_line(
                "暂停",
                build_job_command("p", snapshot.job.id, CommandStyle::Short),
            ));
            lines.push(card::command_line(
                "停止",
                build_job_command("s", snapshot.job.id, CommandStyle::Short),
            ));
            lines.push(card::command_line(
                "列表",
                build_downloads_command(Some("run"), None, None, CommandStyle::Short),
            ));
        }
    }
    lines.join("\n")
}

/// 渲染单任务详情里的真实下载进度。
pub(super) fn format_job_live_download(snapshot: &JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_download_files);
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

#[cfg(test)]
mod tests {
    use super::{format_job_action_text, format_job_live_download, format_job_status_text};
    use crate::tgbot::transfer::store;

    // job 控制回复应使用 card 代码字段展示 job_id 和状态。
    #[test]
    fn test_format_job_action_text() {
        let text = format_job_action_text("任务已暂停", 42, "paused", "等待恢复。");

        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("状态：‹paused›"));
        assert!(text.contains("说明：等待恢复。"));
    }

    // 单任务详情应展示状态、目标、进度和时间字段。
    #[test]
    fn test_format_job_status_text() {
        let snapshot = snapshot_with_status(store::JOB_STATUS_RUNNING);
        let text = format_job_status_text(&snapshot);

        assert!(text.contains("任务详情"));
        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("状态：‹running›"));
        assert!(text.contains("总进度：‹1/3›"));
        assert!(text.contains("完成率：‹|||||||------------- 33%›"));
        assert!(text.contains("■ 时间"));
        assert!(text.contains("■ 命令"));
        assert!(text.contains("暂停：‹/j p 42›"));
        assert!(text.contains("停止：‹/j s 42›"));
        assert!(text.contains("列表：‹/d run›"));
    }

    // 真实下载摘要应和下载列表保持同一风格。
    #[test]
    fn test_format_job_live_download() {
        let mut snapshot = snapshot_with_status(store::JOB_STATUS_RUNNING);
        snapshot.active_download_files = 1;
        snapshot.active_downloaded_bytes = 1024;
        snapshot.active_download_total_bytes = 2048;

        assert_eq!(
            format_job_live_download(&snapshot),
            "1 个文件 1.0 KB/2.0 KB\n||||||||||---------- 50%"
        );
    }

    fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
        let now = store::now_utc8();
        store::JobProgressSnapshot {
            job: store::JobProgressJob {
                id: 42,
                target_chat_id: -100,
                status: status.to_owned(),
                total_items: 3,
                created_at: now,
                updated_at: now,
            },
            pending_count: 1,
            preparing_count: 0,
            prepared_count: 1,
            uploading_count: 0,
            success_count: 1,
            failed_count: 0,
            cancelled_count: 0,
            active_download_files: 0,
            active_downloaded_bytes: 0,
            active_download_total_bytes: 0,
            has_unknown_download_total: false,
        }
    }
}
