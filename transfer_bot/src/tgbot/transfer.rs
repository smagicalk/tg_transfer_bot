// 转存模块入口：
// - 对外仅暴露 `/transfer` 命令处理函数
// - 内部按职责拆分到子模块，降低单文件复杂度

pub(in crate::tgbot) mod card;
mod command;
mod file;
mod outcome;
mod progress;
mod runtime;
mod spawn;
mod spider;
mod store;
pub(crate) mod types;
mod workflow;

pub(in crate::tgbot) use command::{
    auth_command_on, cache_command_on, config_command_on, downloads_command_on,
    handle_auth_shared_user_input, handle_auth_text_input_on, handle_menu_shared_chat_input,
    handle_menu_text_input_on, health_command_on, job_command_on, lookup_command_on,
    menu_command_on, start_transfer_target_choice_from_bot_message,
    start_transfer_target_choice_from_link_message, targets_command_on, transfer_callback_query_on,
    transfer_command_on, transferable_message_source_location,
};
pub(in crate::tgbot) use command::{
    build_help_message_button_data, build_menu_home_button_data_for_outer,
    build_menu_new_transfer_button_data_for_outer, build_view_commands_button,
};
pub(in crate::tgbot) use command::{cancel_auth_input, discard_auth_input_for_command};
pub use command::{
    cancel_menu_input, discard_menu_input, discard_menu_input_for_command, help_command,
    job_command, transfer_callback_query,
};
pub(in crate::tgbot) use outcome::{TransferErrorKind, classify_transfer_error_text};
pub use runtime::{
    RuntimeInitBundle, init_runtime_config_on, update_runtime_config_on,
    update_targets_runtime_config_on,
};
pub(in crate::tgbot::transfer) use runtime::{
    runtime_config_on, runtime_default_config_on, targets_runtime_config_on,
    targets_runtime_default_config_on,
};
pub(in crate::tgbot::transfer) use spawn::{spawn_recovery_job, spawn_transfer_job};
pub(crate) use store::{ensure_targets_runtime_config_on, ensure_transfer_runtime_config_on};
#[cfg(test)]
pub(crate) use store::{load_transfer_runtime_config, load_transfer_runtime_config_on};
pub(crate) use store::{save_targets_runtime_config, save_transfer_runtime_config};
pub(in crate::tgbot::transfer) use workflow::{
    refresh_stored_result_link, refresh_stored_result_messages,
};

/// 判断一条消息是否能被转存流程处理（包含可下载媒体如文档、音视频、照片等）。
///
/// tgbot 入口收到所有者的非文本消息时用它决定是否自动转存或给出引导。
///
/// # 参数
/// - `message`: 待检查的 TDLib Message 引用。
///
/// # 返回值
/// - `true`: 该消息包含支持转存的文件类型。
/// - `false`: 纯文本或其他不支持的特殊消息类型。
pub(in crate::tgbot) fn is_transferable_message(message: &tdlib_rs::types::Message) -> bool {
    file::is_transferable_message(message)
}

/// TDLib 客户端就绪（Ready）后触发转存子系统启动逻辑。
///
/// 1. 保存当前运行分配的交互、下载与上传客户端 ID；
/// 2. 标记后台服务已启动（防止重复触发）；
/// 3. 后台生成异步任务执行未完成任务恢复（`recover_unfinished_jobs`）；
/// 4. 后台生成异步任务启动磁盘文件垃圾回收循环（`run_file_gc_loop`）。
///
/// # 参数
/// - `app_context`: 全局应用上下文 Arc 指针。
/// - `client_ids`: 转存相关的客户端 ID 分配集合。
pub fn on_clients_ready(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    client_ids: crate::config::TransferClientIds,
) {
    app_context
        .transfer_runtime
        .set_transfer_client_ids(client_ids);

    if !app_context
        .transfer_runtime
        .mark_background_services_started()
    {
        tracing::debug!(
            interaction_client_id = client_ids.interaction,
            bot_download_client_id = client_ids.download,
            upload_client_id = client_ids.upload,
            "transfer background services already started"
        );
        return;
    }

    tracing::info!(
        interaction_client_id = client_ids.interaction,
        bot_download_client_id = client_ids.download,
        upload_client_id = client_ids.upload,
        "starting transfer background services"
    );
    let recovery_context = app_context.clone();
    tokio::spawn(async move {
        if let Err(err) = workflow::recover_unfinished_jobs(recovery_context, client_ids).await {
            tracing::error!("recover_unfinished_jobs failed: {:#}", err);
        }
    });

    let gc_context = app_context.clone();
    tokio::spawn(async move {
        workflow::run_file_gc_loop(gc_context, client_ids).await;
    });
}

/// 获取当前转存执行 client 组合（包含交互、下载和上传对应的 TDLib client_id）。
///
/// # 返回值
/// - `Ok(TransferClientIds)`: 已就绪的客户端 ID 结构体。
/// - `Err(anyhow::Error)`: 客户端尚未完全就绪时报错。
pub(in crate::tgbot::transfer) fn transfer_client_ids()
-> anyhow::Result<crate::config::TransferClientIds> {
    crate::app_context::app_context()
        .transfer_runtime
        .transfer_client_ids()
        .ok_or_else(|| anyhow::anyhow!("transfer clients are not ready"))
}
