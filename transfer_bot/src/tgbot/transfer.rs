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
mod types;
mod workflow;

pub(in crate::tgbot) use command::{
    acl_command_on, billing_command_on, cache_command_on, config_command_on, downloads_command_on,
    handle_menu_shared_chat_input_on, handle_menu_text_input_on, health_command_on, job_command_on,
    lookup_command_on, menu_command_on, start_transfer_target_choice_from_bot_message,
    targets_command_on, transfer_callback_query_on, transfer_command_on,
    transferable_message_source_location,
};
pub use command::{
    balance_command, cancel_menu_input, discard_menu_input, discard_menu_input_for_command,
    help_command, job_command, points_command, transfer_callback_query,
};
pub(in crate::tgbot) use command::{
    build_balance_button_data, build_help_button_data, build_menu_home_button_data_for_outer,
};
pub(in crate::tgbot) use outcome::{TransferErrorKind, classify_transfer_error_text};
pub(in crate::tgbot) use runtime::billing_runtime_config_on;
pub use runtime::{
    RuntimeInitBundle, init_runtime_config_on, update_access_control_runtime_config_on,
    update_billing_runtime_config_on, update_runtime_config_on, update_targets_runtime_config_on,
};
pub(in crate::tgbot::transfer) use runtime::{
    access_control_runtime_config_on, access_control_runtime_default_config_on,
    billing_runtime_default_config_on, runtime_config_on, runtime_default_config_on,
    targets_runtime_config_on, targets_runtime_default_config_on,
};
pub(in crate::tgbot::transfer) use spawn::{spawn_recovery_job, spawn_transfer_job};
pub(crate) use store::{
    ensure_access_control_runtime_config_on, ensure_billing_runtime_config_on,
    ensure_targets_runtime_config_on, ensure_transfer_runtime_config_on,
};
#[cfg(test)]
pub(crate) use store::{load_transfer_runtime_config, load_transfer_runtime_config_on};
pub(crate) use store::{
    save_access_control_runtime_config, save_billing_runtime_config, save_targets_runtime_config,
    save_transfer_runtime_config,
};
pub(in crate::tgbot::transfer) use workflow::{
    refresh_stored_result_link, refresh_stored_result_messages,
};

/// 判断一条消息是否能被转存流程处理。
///
/// tgbot 入口收到非文本管理员消息时用它决定是否自动转存或给出引导。
pub(in crate::tgbot) fn is_transferable_message(message: &tdlib_rs::types::Message) -> bool {
    file::is_transferable_message(message)
}

/// 确保交互用户在 user_account 中存在。
///
/// 入口层在处理消息和 callback 前调用；后续 `/balance`、扣费和 admin 调分都可以依赖账号存在。
pub(in crate::tgbot) async fn ensure_user_account_for_actor(
    actor: crate::config::RequestActor,
    initial_points: i64,
) -> anyhow::Result<()> {
    store::ensure_user_account(actor.user_id, actor.role, initial_points).await?;
    Ok(())
}

/// TDLib Ready 后触发转存子系统启动逻辑。
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
            configured_download_client_id = client_ids.download,
            upload_client_id = client_ids.upload,
            "transfer background services already started"
        );
        return;
    }

    tracing::info!(
        interaction_client_id = client_ids.interaction,
        configured_download_client_id = client_ids.download,
        upload_client_id = client_ids.upload,
        "starting transfer background services"
    );
    let startup_context = app_context.clone();
    tokio::spawn(async move {
        if let Err(err) = workflow::maybe_send_startup_setup_guide_on(
            startup_context.as_ref(),
            client_ids.interaction,
        )
        .await
        {
            tracing::error!("send startup setup guide failed: {:#}", err);
        }
    });

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

/// 获取当前转存执行 client 组合。
pub(in crate::tgbot::transfer) fn transfer_client_ids()
-> anyhow::Result<crate::config::TransferClientIds> {
    crate::app_context::app_context()
        .transfer_runtime
        .transfer_client_ids()
        .ok_or_else(|| anyhow::anyhow!("transfer clients are not ready"))
}
