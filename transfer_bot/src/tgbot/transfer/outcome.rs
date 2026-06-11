// 转存结果回复入口。
// 成功、状态和失败卡片分别拆到子模块，后台派发层只依赖这些统一函数。

mod failure;
mod status;
mod success;

pub(super) use failure::send_failure_message;
pub(in crate::tgbot) use failure::{TransferErrorKind, classify_transfer_error_text};
pub(super) use status::{
    send_cancelled_message, send_cancelling_message, send_paused_message, send_running_message,
};
pub(super) use success::{
    format_result_card_text, normalize_result_messages, send_history_hit_message,
};
