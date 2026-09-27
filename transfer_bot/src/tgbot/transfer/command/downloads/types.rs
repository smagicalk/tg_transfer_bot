// `/downloads` 的参数和筛选条件。
// 该模块只负责把命令参数解释为结构化条件，并判断任务快照是否命中筛选。

use crate::tgbot::transfer::store;

/// 下载列表筛选器。
///
/// 定义用户在查看 `/downloads` 任务列表时支持的各种状态筛选维度。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DownloadsFilter {
    /// 全部任务（不进行任何状态过滤）
    All,
    /// 等待中（处于等待队列或预处理准备就绪）
    Waiting,
    /// 下载中（正在拉取元数据或传输文件数据）
    Downloading,
    /// 上传中（文件正推送到 Telegram 目标会话）
    Uploading,
    /// 已完成（所有终态集合：成功、失败或取消）
    Finished,
    /// 成功（所有子项均顺利转存完成）
    Success,
    /// 失败（包含部分失败或完全失败的任务）
    Failed,
    /// 运行中（包含待处理与执行中的动态任务）
    Running,
    /// 已就绪（已完成本地拉取等待上传就绪）
    Ready,
    /// 已暂停（由用户或管理员主动暂停）
    Paused,
    /// 停止中（正在响应取消请求并清理现场）
    Cancelling,
    /// 已停止（已彻底取消或终止）
    Cancelled,
}

/// `/downloads` 参数解析结果。
///
/// 包含解析后的筛选模式、单页条目容量以及当前请求的目标页码。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DownloadsArgs {
    /// 列表筛选条件
    pub(super) filter: DownloadsFilter,
    /// 每页展示条数（受 1~20 范围钳制）
    pub(super) limit: u64,
    /// 当前请求页码（从 1 开始计）
    pub(super) page: u64,
}

impl DownloadsFilter {
    /// 将用户输入的字符串参数映射为对应的筛选条件枚举。
    ///
    /// 针对常见缩写（如 dl, up, ok, done）提供友好别名解析。
    pub(super) fn parse(input: &str) -> Option<Self> {
        match input {
            // "all" -> 全部任务
            "all" => Some(Self::All),
            // "wait" 或 "waiting" -> 等待中任务
            "wait" | "waiting" => Some(Self::Waiting),
            // "dl", "download", "downloading" -> 下载中任务
            "dl" | "download" | "downloading" => Some(Self::Downloading),
            // "up", "upload", "uploading" -> 上传中任务
            "up" | "upload" | "uploading" => Some(Self::Uploading),
            // "done", "finished" -> 已结束任务
            "done" | "finished" => Some(Self::Finished),
            // "ok", "success" -> 成功任务
            "ok" | "success" => Some(Self::Success),
            // "failed", "fail" -> 失败任务
            "failed" | "fail" => Some(Self::Failed),
            // "run", "running" -> 执行中任务
            "run" | "running" => Some(Self::Running),
            // "ready" -> 已就绪任务
            "ready" => Some(Self::Ready),
            // "pause", "paused" -> 已暂停任务
            "pause" | "paused" => Some(Self::Paused),
            // "cancelling", "stopping" -> 正在取消中
            "cancelling" | "stopping" => Some(Self::Cancelling),
            // "cancel", "cancelled", "stop", "stopped" -> 已取消停止
            "cancel" | "cancelled" | "stop" | "stopped" => Some(Self::Cancelled),
            // 无法识别的筛选词返回 None
            _ => None,
        }
    }

    /// 判断任务进度快照是否命中当前筛选条件。
    ///
    /// # 参数
    /// - `snapshot`: 任务进度完整快照
    pub(super) fn matches(self, snapshot: &store::JobProgressSnapshot) -> bool {
        match self {
            // 全部条件无条件命中
            Self::All => true,
            // 等待中：存在排队待处理项或已准备项
            Self::Waiting => snapshot.pending_count > 0 || snapshot.prepared_count > 0,
            // 下载中：正在准备下载或活跃下载文件数大于 0
            Self::Downloading => snapshot.preparing_count > 0 || snapshot.active_download_files > 0,
            // 上传中：正在上传的文件数大于 0
            Self::Uploading => snapshot.uploading_count > 0,
            // 已完成：判断任务状态是否为任何终态（成功、失败、取消等）
            Self::Finished => store::is_finished_job_status(&snapshot.job.status),
            // 成功：任务主状态完全为成功
            Self::Success => snapshot.job.status == store::JOB_STATUS_SUCCESS,
            // 失败：存在失败子项或主状态为失败/部分失败
            Self::Failed => {
                snapshot.failed_count > 0
                    || matches!(
                        snapshot.job.status.as_str(),
                        store::JOB_STATUS_FAILED | store::JOB_STATUS_PARTIAL
                    )
            }
            // 运行中：任务处于挂起待跑或正在运行
            Self::Running => matches!(
                snapshot.job.status.as_str(),
                store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
            ),
            // 就绪：具备已预备完毕的文件待转存
            Self::Ready => snapshot.prepared_count > 0,
            // 暂停：任务主状态被标记为已暂停
            Self::Paused => snapshot.job.status == store::JOB_STATUS_PAUSED,
            // 停止中：任务正处于取消中或取消收尾确认中
            Self::Cancelling => matches!(
                snapshot.job.status.as_str(),
                store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING
            ),
            // 已停止：任务已被彻底取消
            Self::Cancelled => snapshot.job.status == store::JOB_STATUS_CANCELLED,
        }
    }

    /// 获取面向用户的人类可读中文标签文本。
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

    /// 获取标准命令参数值（用于反向生成翻页或跳转的命令行参数）。
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

/// 解析 `/downloads` 参数（测试快捷入口）。
///
/// 规则：
/// - 默认：`全部 + transfer_config.downloads_default_page_size`
/// - 若第一个参数是数字，则视为 limit
/// - 否则第一个参数视为 filter，后两个参数依次是 limit / page
#[cfg(test)]
pub(super) fn parse_downloads_args(text: &[&str]) -> anyhow::Result<DownloadsArgs> {
    parse_downloads_args_on(crate::app_context::app_context().as_ref(), text)
}

/// 在指定应用上下文上解析 `/downloads` 命令的参数切片。
///
/// # 参数
/// - `app`: 全局应用上下文实例引用
/// - `text`: 命令参数切片，如 `["/downloads", "dl", "10", "2"]`
pub(super) fn parse_downloads_args_on(
    app: &crate::app_context::AppContext,
    text: &[&str],
) -> anyhow::Result<DownloadsArgs> {
    // 默认筛选为全部
    let mut filter = DownloadsFilter::All;
    // 默认每页条数取自运行时配置并钳制在 1~20 范围内
    let mut limit = crate::tgbot::transfer::runtime_config_on(app)
        .downloads_default_page_size
        .clamp(1, 20);
    // 默认起始页码为第 1 页
    let mut page = 1u64;
    // 存储解析出的数字类型参数容器
    let mut numeric_args = Vec::new();

    // 检查第 1 个参数（即 text[1]）
    if let Some(arg1) = text.get(1) {
        if let Some(parsed_filter) = DownloadsFilter::parse(arg1) {
            // 成功匹配为状态筛选词
            filter = parsed_filter;
        } else if let Ok(num) = arg1.parse::<u64>() {
            // 属于纯数字，归入数字参数（表示 limit）
            numeric_args.push(num);
        } else {
            // 既非筛选词也非有效数字，返回未知筛选错误
            anyhow::bail!("unknown downloads filter: {arg1}");
        }
    }

    // 检查第 2 个参数（即 text[2]）
    if let Some(arg2) = text.get(2) {
        if let Ok(num) = arg2.parse::<u64>() {
            numeric_args.push(num);
        } else {
            anyhow::bail!("downloads limit/page must be number: {arg2}");
        }
    }

    // 检查第 3 个参数（即 text[3]）
    if let Some(arg3) = text.get(3) {
        if let Ok(num) = arg3.parse::<u64>() {
            numeric_args.push(num);
        } else {
            anyhow::bail!("downloads page must be number: {arg3}");
        }
    }

    // 如果收集到了数字参数，首个数字作为 limit（钳制在 1~20）
    if !numeric_args.is_empty() {
        limit = numeric_args[0].clamp(1, 20);
    }
    // 若收集到了两个及以上数字，次个数字作为 page（至少从 1 起）
    if numeric_args.len() >= 2 {
        page = numeric_args[1].max(1);
    }

    // 组装最终结果
    Ok(DownloadsArgs {
        filter,
        limit,
        page,
    })
}
