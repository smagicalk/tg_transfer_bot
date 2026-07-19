// 转存结果回复入口。
// 成功、状态和失败卡片分别拆到子模块，后台派发层只依赖这些统一函数。

mod failure;
mod status;
mod success;

pub(in crate::tgbot::transfer) use failure::format_failure_card_text;
pub(super) use failure::send_failure_message;
pub(in crate::tgbot) use failure::{TransferErrorKind, classify_transfer_error_text};
pub(in crate::tgbot::transfer) use status::build_job_action_row;
pub(in crate::tgbot::transfer) use status::format_status_card_text;
pub(super) use status::{
    send_cancelled_message, send_cancelling_message, send_paused_message, send_running_message,
};
pub(in crate::tgbot::transfer) use success::{
    build_list_menu_row, build_result_message_rows, build_result_navigation_rows,
    format_result_card_text, normalize_result_messages, send_history_hit_message,
};
