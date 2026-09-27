use crate::tgbot::transfer::store;

/// 任务状态元信息结构体。
///
/// 定义某种特定任务状态所对应的列表筛选词、返回列表按钮标签，以及允许在当前状态下展示的交互按钮掩码。
pub(super) struct JobStatusMeta {
    /// 对应的下载列表筛选标识（如 "run", "pause", "done" 等）
    pub list_filter: &'static str,
    /// 返回列表按钮所呈现的文本标签
    pub list_button_label: &'static str,
    /// 是否允许展示“暂停”操作按钮
    pub show_pause: bool,
    /// 是否允许展示“恢复”操作按钮
    pub show_resume: bool,
    /// 是否允许展示“停止”操作按钮
    pub show_stop: bool,
}

/// 根据任务状态字符串计算并返回对应的元配置信息。
///
/// # 参数
/// - `status`: 数据库存储的任务状态字符串（如 pending, running, paused 等）
pub(super) fn job_status_meta(status: &str) -> JobStatusMeta {
    match status {
        // 排队待处理或正在运行中的任务：允许暂停与停止
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING => JobStatusMeta {
            list_filter: "run",
            list_button_label: "查看运行列表",
            show_pause: true,
            show_resume: false,
            show_stop: true,
        },
        // 已暂停任务：允许恢复与彻底停止
        store::JOB_STATUS_PAUSED => JobStatusMeta {
            list_filter: "pause",
            list_button_label: "查看暂停列表",
            show_pause: false,
            show_resume: true,
            show_stop: true,
        },
        // 正在取消或收尾确认中的任务：处于过渡状态，不提供破坏性重复按钮
        store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING => JobStatusMeta {
            list_filter: "cancelling",
            list_button_label: "查看停止列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
        // 已彻底取消停止的任务：只提供返回列表导航
        store::JOB_STATUS_CANCELLED => JobStatusMeta {
            list_filter: "cancel",
            list_button_label: "查看已停列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
        // 成功转存的任务：终态，不提供控制按钮
        store::JOB_STATUS_SUCCESS => JobStatusMeta {
            list_filter: "done",
            list_button_label: "查看完成列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
        // 失败或部分失败的任务：终态，不提供控制按钮
        store::JOB_STATUS_FAILED | store::JOB_STATUS_PARTIAL => JobStatusMeta {
            list_filter: "fail",
            list_button_label: "查看失败列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
        // 其他未知或兜底状态：映射至全部列表
        _ => JobStatusMeta {
            list_filter: "all",
            list_button_label: "查看全部列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
    }
}
