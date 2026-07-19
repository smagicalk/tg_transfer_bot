use crate::tgbot::transfer::store;

pub(super) struct JobStatusMeta {
    pub list_filter: &'static str,
    pub list_button_label: &'static str,
    pub show_pause: bool,
    pub show_resume: bool,
    pub show_stop: bool,
}

pub(super) fn job_status_meta(status: &str) -> JobStatusMeta {
    match status {
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING => JobStatusMeta {
            list_filter: "run",
            list_button_label: "查看运行列表",
            show_pause: true,
            show_resume: false,
            show_stop: true,
        },
        store::JOB_STATUS_PAUSED => JobStatusMeta {
            list_filter: "pause",
            list_button_label: "查看暂停列表",
            show_pause: false,
            show_resume: true,
            show_stop: true,
        },
        store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING => JobStatusMeta {
            list_filter: "cancelling",
            list_button_label: "查看停止列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
        store::JOB_STATUS_CANCELLED => JobStatusMeta {
            list_filter: "cancel",
            list_button_label: "查看已停列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
        store::JOB_STATUS_SUCCESS => JobStatusMeta {
            list_filter: "done",
            list_button_label: "查看完成列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
        store::JOB_STATUS_FAILED | store::JOB_STATUS_PARTIAL => JobStatusMeta {
            list_filter: "fail",
            list_button_label: "查看失败列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
        _ => JobStatusMeta {
            list_filter: "all",
            list_button_label: "查看全部列表",
            show_pause: false,
            show_resume: false,
            show_stop: false,
        },
    }
}
