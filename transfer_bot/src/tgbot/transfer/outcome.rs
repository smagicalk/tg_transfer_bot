// 转存结果回复入口。
// 成功、状态和失败卡片分别拆到子模块，后台派发层只依赖这些统一函数。

mod failure;
mod status;
mod success;

pub(super) use failure::send_failure_message;
pub(super) use status::{
    send_cancelled_message, send_cancelling_message, send_paused_message, send_running_message,
};
pub(super) use success::send_history_hit_message;
