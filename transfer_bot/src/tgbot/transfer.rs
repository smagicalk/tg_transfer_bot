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

pub use command::{
    cancel_menu_input, config_command, discard_menu_input, downloads_command,
    handle_menu_text_input, help_command, job_command, lookup_command, menu_command,
    transfer_bot_message_auto_command, transfer_callback_query, transfer_command,
};
pub use runtime::{init_runtime_config, update_runtime_config};
pub(in crate::tgbot::transfer) use runtime::{runtime_config, tdlib_files_directory_for};
pub(in crate::tgbot::transfer) use spawn::{spawn_recovery_job, spawn_transfer_job};
pub(in crate::tgbot::transfer) use workflow::{
    refresh_stored_result_link, refresh_stored_result_messages,
};

/// 判断一条消息是否能被转存流程处理。
///
/// tgbot 入口收到非文本管理员消息时用它决定是否自动转存或给出引导。
pub(in crate::tgbot) fn is_transferable_message(message: &tdlib_rs::types::Message) -> bool {
    file::is_transferable_message(message)
}

// 启动任务仅初始化一次：
// - 后台文件删除队列消费者
// - 未完成任务恢复
static TRANSFER_STARTUP_ONCE: std::sync::OnceLock<()> = std::sync::OnceLock::new();
static TRANSFER_CLIENT_IDS: std::sync::OnceLock<crate::config::TransferClientIds> =
    std::sync::OnceLock::new();

/// TDLib Ready 后触发转存子系统启动逻辑。
pub fn on_clients_ready(client_ids: crate::config::TransferClientIds) {
    // 先发布 client id，再发布“后台服务已启动”标记。
    // 双 client 模式下 user/bot 可能几乎同时 Ready；如果顺序反过来，
    // 某些按钮或命令可能在极短窗口内看到“服务已启动”但读不到 TransferClientIds。
    let _ = TRANSFER_CLIENT_IDS.set(client_ids);

    if TRANSFER_STARTUP_ONCE.set(()).is_err() {
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
    tokio::spawn(async move {
        if let Err(err) = workflow::recover_unfinished_jobs(client_ids).await {
            tracing::error!("recover_unfinished_jobs failed: {:#}", err);
        }
    });

    tokio::spawn(async move {
        workflow::run_file_gc_loop(client_ids).await;
    });
}

/// 获取当前转存执行 client 组合。
pub(in crate::tgbot::transfer) fn transfer_client_ids()
-> anyhow::Result<crate::config::TransferClientIds> {
    TRANSFER_CLIENT_IDS
        .get()
        .copied()
        .ok_or_else(|| anyhow::anyhow!("transfer clients are not ready"))
}
