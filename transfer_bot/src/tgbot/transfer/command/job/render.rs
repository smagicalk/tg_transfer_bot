// `/job` 回复文本渲染。
// 只负责把任务快照转换成卡片文本，不执行数据库写入或 TDLib 调用。

use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store::{self, JobProgressSnapshot};

use super::super::common::{build_ready_page_header, format_bytes};

/// 构造 `/job` 控制动作操作结果卡片（如暂停、恢复、停止时的即时回复）。
///
/// # 参数
/// - `title`: 卡片主标题
/// - `job_id`: 目标任务主键 ID
/// - `status`: 当前最新状态
/// - `detail`: 补充说明文案
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

/// 构造单任务完整详情卡片富文本。
///
/// 包含状态摘要、子任务各阶段计数、百分比进度条、目标消息链接、真实下载/上传动态、错误信息与时间戳。
///
/// # 参数
/// - `snapshot`: 任务进度完整快照
pub(super) fn format_job_status_text(snapshot: &JobProgressSnapshot) -> String {
    let total = snapshot.job.total_items.max(0);
    let finished = snapshot.success_count + snapshot.failed_count + snapshot.cancelled_count;
    let mut lines = build_ready_page_header("任务详情");
    lines.extend([
        // 渲染单行任务状态、任务 ID 与目标群组 ID
        card::summary_line(
            &snapshot.job.status,
            Some(snapshot.job.id),
            snapshot.job.target_chat_id,
        ),
        card::section("进度"),
        // 总体完成数量/总量
        card::field("总进度", format!("{finished}/{total}")),
        // 总体百分比字符进度条
        card::field("完成率", card::progress_bar(finished.into(), total.into())),
        // 等待排队数 / 正在准备下载数 与 就绪数 / 正在上传数
        card::field_pair(
            "等待/下载",
            format!("{}/{}", snapshot.pending_count, snapshot.preparing_count),
            "就绪/上传",
            format!("{}/{}", snapshot.prepared_count, snapshot.uploading_count),
        ),
        // 成功数 / 失败数 与 已停止取消数
        card::field_pair(
            "成功/失败",
            format!("{}/{}", snapshot.success_count, snapshot.failed_count),
            "已停",
            snapshot.cancelled_count,
        ),
    ]);

    // 渲染转存结果在 Telegram 中的目标位置
    lines.push(card::section("目标消息"));
    match snapshot.job.result_message_link.as_deref() {
        // 如果是有效且可打开的 URL，则渲染超链接与明文地址
        Some(link) if crate::tgbot::send::is_openable_url(link) => {
            lines.push(format!("跳转：{}", card::link("打开转存消息", link)));
            lines.push(card::field("地址", link));
        }
        // 普通群聊或私聊无法直接生成公网跳转链接，呈现消息定位符并附带说明
        Some(locator) => {
            lines.push(card::field("定位", locator));
            lines.push(card::note(
                "Telegram 普通群、私聊等目标不提供可点击的消息链接；超级群或频道才可直接跳转。",
            ));
        }
        // 尚未开始或无定位信息
        None => lines.push(card::field("地址", "任务尚未完成或暂无结果地址")),
    }

    // 若当前活跃下载数 > 0，展示底层文件传输实时下载进度
    if snapshot.active_download_files > 0 {
        lines.push(format!("真实下载：{}", format_job_live_download(snapshot)));
    }
    // 若当前活跃上传数 > 0，展示文件推送到 Telegram 的实时上传进度
    if snapshot.active_upload_files > 0 {
        lines.push(format!("真实上传：{}", format_job_live_upload(snapshot)));
    }

    // 错误追踪：展示最近一次记录的报错原因
    if let Some(last_error) = snapshot.job.last_error.as_deref() {
        // 失败详情可能来自 TDLib 或 anyhow 链，保留原文方便复制到日志中定位。
        lines.push(card::section("最后错误"));
        lines.push(card::pre_code(last_error));
    }

    // 时间节点记录
    lines.push(card::section("时间"));
    lines.push(card::field(
        "创建",
        snapshot.job.created_at.format("%Y-%m-%d %H:%M:%S"),
    ));
    lines.push(card::field(
        "更新",
        snapshot.job.updated_at.format("%Y-%m-%d %H:%M:%S"),
    ));

    // 根据任务当前所处状态生成针对性的操作引导提示
    let interaction_note = match snapshot.job.status.as_str() {
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING => {
            "可直接点击下方按钮暂停或停止任务；需要命令时点击“查看命令”。"
        }
        store::JOB_STATUS_PAUSED => "可直接点击下方按钮恢复或停止任务；需要命令时点击“查看命令”。",
        store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING => {
            "任务正在停止并安全收尾，当前无需重复操作。"
        }
        _ => "任务已结束，不能再暂停或停止；可点击下方按钮刷新详情。",
    };
    lines.push(card::note(interaction_note));
    lines.join("\n")
}

/// 渲染单任务详情里的真实下载进度（已下载字节、总字节、活跃文件数与百分比条）。
///
/// # 参数
/// - `snapshot`: 任务进度快照
pub(super) fn format_job_live_download(snapshot: &JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_download_files);
    // 当总大小确定时渲染精确进度条
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

    // 总大小部分确定时的估算展示
    if snapshot.active_download_total_bytes > 0 {
        return format!(
            "{} 已下 {} / 已知总量 {}+",
            prefix,
            format_bytes(snapshot.active_downloaded_bytes),
            format_bytes(snapshot.active_download_total_bytes)
        );
    }

    // 仅显示当前已完成的字节数
    format!(
        "{} 已下 {}",
        prefix,
        format_bytes(snapshot.active_downloaded_bytes)
    )
}

/// 渲染单任务详情里的 TDLib 实时上传进度。
///
/// # 参数
/// - `snapshot`: 任务进度快照
pub(super) fn format_job_live_upload(snapshot: &JobProgressSnapshot) -> String {
    let prefix = format!("{} 个文件", snapshot.active_upload_files);
    // 当总大小确定时渲染精确上传进度条
    if snapshot.active_upload_total_bytes > 0 && !snapshot.has_unknown_upload_total {
        let progress = snapshot.active_uploaded_bytes.saturating_mul(100)
            / snapshot.active_upload_total_bytes.max(1);
        return format!(
            "{} {}/{}\n{}",
            prefix,
            format_bytes(snapshot.active_uploaded_bytes),
            format_bytes(snapshot.active_upload_total_bytes),
            card::progress_bar_percent(progress)
        );
    }

    // 总大小部分确定时的估算展示
    if snapshot.active_upload_total_bytes > 0 {
        return format!(
            "{} 已传 {} / 已知总量 {}+",
            prefix,
            format_bytes(snapshot.active_uploaded_bytes),
            format_bytes(snapshot.active_upload_total_bytes)
        );
    }

    // 仅显示已上传字节
    format!(
        "{} 已传 {}",
        prefix,
        format_bytes(snapshot.active_uploaded_bytes)
    )
}

#[cfg(test)]
mod tests {
    use super::{
        format_job_action_text, format_job_live_download, format_job_live_upload,
        format_job_status_text,
    };
    use crate::tgbot::transfer::store;

    /// 验证 job 控制回复使用 card 样式高亮展示 job_id 和状态。
    #[test]
    fn test_format_job_action_text() {
        let text = format_job_action_text("任务已暂停", 42, "paused", "等待恢复。");

        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("状态：‹paused›"));
        assert!(text.contains("说明：等待恢复。"));
    }

    /// 验证单任务详情卡片包含状态、目标、进度和时间字段。
    #[test]
    fn test_format_job_status_text() {
        let snapshot = snapshot_with_status(store::JOB_STATUS_RUNNING);
        let text = format_job_status_text(&snapshot);

        assert!(text.contains("任务详情"));
        assert!(text.contains("状态：‹ready›"));
        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("状态：‹running›"));
        assert!(text.contains("总进度：‹1/3›"));
        assert!(text.contains("完成率：‹|||||||------------- 33%›"));
        assert!(text.contains("■ 时间"));
        assert!(!text.contains("■ 命令"));
        assert!(text.contains("可直接点击下方按钮暂停或停止任务"));
    }

    /// 验证失败任务详情展示 transfer_job.last_error，方便定位。
    #[test]
    fn test_format_job_status_text_shows_last_error() {
        let mut snapshot = snapshot_with_status(store::JOB_STATUS_FAILED);
        snapshot.job.last_error = Some("code=400, message=Message not found".to_owned());

        let text = format_job_status_text(&snapshot);

        assert!(text.contains("■ 最后错误"));
        assert!(text.contains("«code=400, message=Message not found»"));
    }

    /// 验证存在有效目标链接时渲染富文本跳转。
    #[test]
    fn test_format_job_status_text_shows_result_link() {
        let mut snapshot = snapshot_with_status(store::JOB_STATUS_SUCCESS);
        snapshot.job.result_message_link = Some("https://t.me/c/123/456".to_owned());

        let text = format_job_status_text(&snapshot);

        assert!(text.contains("■ 目标消息"));
        assert!(text.contains("跳转：【打开转存消息】(https://t.me/c/123/456)"));
        assert!(text.contains("地址：‹https://t.me/c/123/456›"));
        assert!(text.contains("任务已结束，不能再暂停或停止"));
    }

    /// 验证不可点击定位符（普通群）给出对应说明。
    #[test]
    fn test_format_job_status_text_explains_non_openable_result_locator() {
        let mut snapshot = snapshot_with_status(store::JOB_STATUS_SUCCESS);
        snapshot.job.result_message_link =
            Some("chat_id=-5221439438 message_id=318767104".to_owned());

        let text = format_job_status_text(&snapshot);

        assert!(text.contains("定位：‹chat_id=-5221439438 message_id=318767104›"));
        assert!(text.contains("Telegram 普通群、私聊等目标不提供可点击的消息链接"));
        assert!(text.contains("任务已结束，不能再暂停或停止"));
    }

    /// 真实下载摘要应和下载列表保持同一风格。
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

    /// 验证实时上传进度的文字与进度条渲染。
    #[test]
    fn test_format_job_status_text_shows_live_upload_progress() {
        let mut snapshot = snapshot_with_status(store::JOB_STATUS_RUNNING);
        snapshot.active_upload_files = 2;
        snapshot.active_uploaded_bytes = 1024;
        snapshot.active_upload_total_bytes = 4096;

        assert_eq!(
            format_job_live_upload(&snapshot),
            "2 个文件 1.0 KB/4.0 KB\n|||||--------------- 25%"
        );
        let text = format_job_status_text(&snapshot);
        assert!(text.contains("真实上传：2 个文件 1.0 KB/4.0 KB"));
        assert!(text.contains("25%"));
    }

    /// 测试辅助工具：构造指定状态的快照。
    fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
        let now = store::now_utc8();
        store::JobProgressSnapshot {
            job: store::JobProgressJob {
                id: 42,
                target_chat_id: -100,
                result_message_link: None,
                status: status.to_owned(),
                total_items: 3,
                created_at: now,
                updated_at: now,
                last_error: None,
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
            active_upload_files: 0,
            active_uploaded_bytes: 0,
            active_upload_total_bytes: 0,
            has_unknown_upload_total: false,
        }
    }
}
