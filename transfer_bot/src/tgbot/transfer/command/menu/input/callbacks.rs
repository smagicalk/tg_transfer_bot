// `/menu` 输入流程 callback 统一导出入口。
// 具体实现按职责拆到目标流程和单步输入两个文件，外层仍通过本模块导入，避免调用方感知文件结构变化。

pub(in crate::tgbot::transfer::command::menu) use super::callbacks_simple::{
    cancel_input_callback_query, job_id_input_callback_query,
    point_ledger_user_input_callback_query,
};
pub(in crate::tgbot::transfer::command::menu) use super::callbacks_target::{
    target_alias_callback_query, target_back_callback_query, target_confirm_callback_query,
    target_default_callback_query, target_manual_callback_query,
    target_request_chat_callback_query,
};
