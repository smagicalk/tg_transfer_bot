// `/downloads` 的参数和筛选条件。
// 该模块只负责把命令参数解释为结构化条件，并判断任务快照是否命中筛选。

use crate::tgbot::transfer::store;

/// 下载列表筛选器。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DownloadsFilter {
    All,
    Waiting,
    Downloading,
    Uploading,
    Finished,
    Success,
    Failed,
    Running,
    Ready,
    Paused,
    Cancelling,
    Cancelled,
}

/// `/downloads` 参数解析结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DownloadsArgs {
    pub(super) filter: DownloadsFilter,
    pub(super) limit: u64,
    pub(super) page: u64,
}

impl DownloadsFilter {
    /// 将用户输入映射为筛选条件。
    pub(super) fn parse(input: &str) -> Option<Self> {
        match input {
            "all" => Some(Self::All),
            "wait" | "waiting" => Some(Self::Waiting),
            "dl" | "download" | "downloading" => Some(Self::Downloading),
            "up" | "upload" | "uploading" => Some(Self::Uploading),
            "done" | "finished" => Some(Self::Finished),
            "ok" | "success" => Some(Self::Success),
            "failed" | "fail" => Some(Self::Failed),
            "run" | "running" => Some(Self::Running),
            "ready" => Some(Self::Ready),
            "pause" | "paused" => Some(Self::Paused),
            "cancelling" | "stopping" => Some(Self::Cancelling),
            "cancel" | "cancelled" | "stop" | "stopped" => Some(Self::Cancelled),
            _ => None,
        }
    }

    /// 判断任务快照是否命中筛选条件。
    pub(super) fn matches(self, snapshot: &store::JobProgressSnapshot) -> bool {
        match self {
            Self::All => true,
            Self::Waiting => snapshot.pending_count > 0 || snapshot.prepared_count > 0,
            Self::Downloading => snapshot.preparing_count > 0 || snapshot.active_download_files > 0,
            Self::Uploading => snapshot.uploading_count > 0,
            Self::Finished => store::is_finished_job_status(&snapshot.job.status),
            Self::Success => snapshot.job.status == store::JOB_STATUS_SUCCESS,
            Self::Failed => {
                snapshot.failed_count > 0
                    || matches!(
                        snapshot.job.status.as_str(),
                        store::JOB_STATUS_FAILED | store::JOB_STATUS_PARTIAL
                    )
            }
            Self::Running => matches!(
                snapshot.job.status.as_str(),
                store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
            ),
            Self::Ready => snapshot.prepared_count > 0,
            Self::Paused => snapshot.job.status == store::JOB_STATUS_PAUSED,
            Self::Cancelling => matches!(
                snapshot.job.status.as_str(),
                store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING
            ),
            Self::Cancelled => snapshot.job.status == store::JOB_STATUS_CANCELLED,
        }
    }

    /// 人类可读标签。
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::All => "全部",
            Self::Waiting => "等待中",
            Self::Downloading => "下载中",
            Self::Uploading => "上传中",
            Self::Finished => "已完成",
            Self::Success => "成功",
            Self::Failed => "失败",
            Self::Running => "处理中",
            Self::Ready => "已就绪",
            Self::Paused => "已暂停",
            Self::Cancelling => "停止中",
            Self::Cancelled => "已停止",
        }
    }

    /// 命令参数值（用于生成翻页命令）。
    pub(super) fn command_value(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Waiting => "wait",
            Self::Downloading => "dl",
            Self::Uploading => "up",
            Self::Finished => "done",
            Self::Success => "ok",
            Self::Failed => "fail",
            Self::Running => "run",
            Self::Ready => "ready",
            Self::Paused => "pause",
            Self::Cancelling => "cancelling",
            Self::Cancelled => "cancel",
        }
    }
}

/// 解析 `/downloads` 参数。
///
/// 规则：
/// - 默认：`全部 + transfer_config.downloads_default_page_size`
/// - 若第一个参数是数字，则视为 limit
/// - 否则第一个参数视为 filter，后两个参数依次是 limit / page
#[cfg(test)]
pub(super) fn parse_downloads_args(text: &[&str]) -> anyhow::Result<DownloadsArgs> {
    parse_downloads_args_on(crate::app_context::app_context().as_ref(), text)
}

/// 在指定上下文上解析 `/downloads` 参数。
pub(super) fn parse_downloads_args_on(
    app: &crate::app_context::AppContext,
    text: &[&str],
) -> anyhow::Result<DownloadsArgs> {
    let mut filter = DownloadsFilter::All;
    let mut limit = crate::tgbot::transfer::runtime_config_on(app)
        .downloads_default_page_size
        .clamp(1, 20);
    let mut page = 1u64;
    let mut numeric_args = Vec::new();

    if let Some(arg1) = text.get(1) {
        if let Some(parsed_filter) = DownloadsFilter::parse(arg1) {
            filter = parsed_filter;
        } else if let Ok(num) = arg1.parse::<u64>() {
            numeric_args.push(num);
        } else {
            anyhow::bail!("unknown downloads filter: {}", arg1);
        }
    }

    if let Some(arg2) = text.get(2) {
        if let Ok(num) = arg2.parse::<u64>() {
            numeric_args.push(num);
        } else {
            anyhow::bail!("downloads limit/page must be number: {}", arg2);
        }
    }

    if let Some(arg3) = text.get(3) {
        if let Ok(num) = arg3.parse::<u64>() {
            numeric_args.push(num);
        } else {
            anyhow::bail!("downloads page must be number: {}", arg3);
        }
    }

    if !numeric_args.is_empty() {
        limit = numeric_args[0].clamp(1, 20);
    }
    if numeric_args.len() >= 2 {
        page = numeric_args[1].max(1);
    }

    Ok(DownloadsArgs {
        filter,
        limit,
        page,
    })
}
