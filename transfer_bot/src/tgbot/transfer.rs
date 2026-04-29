// 转存模块入口：
// - 对外仅暴露 `/transfer` 命令处理函数
// - 内部按职责拆分到子模块，降低单文件复杂度

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
    config_command, downloads_callback_query, downloads_command, help_command, job_command,
    lookup_command, transfer_command,
};
pub(in crate::tgbot::transfer) use runtime::runtime_config;
pub use runtime::{init_runtime_config, update_runtime_config};
pub(in crate::tgbot::transfer) use spawn::{spawn_recovery_job, spawn_transfer_job};

// 启动任务仅初始化一次：
// - 后台文件删除队列消费者
// - 未完成任务恢复
static TRANSFER_STARTUP_ONCE: std::sync::OnceLock<()> = std::sync::OnceLock::new();

/// TDLib Ready 后触发转存子系统启动逻辑。
pub fn on_client_ready(client_id: i32) {
    if TRANSFER_STARTUP_ONCE.set(()).is_err() {
        tracing::debug!(client_id, "transfer background services already started");
        return;
    }

    tracing::info!(client_id, "starting transfer background services");
    tokio::spawn(async move {
        if let Err(err) = workflow::recover_unfinished_jobs(client_id).await {
            tracing::error!("recover_unfinished_jobs failed: {:#}", err);
        }
    });

    tokio::spawn(async move {
        workflow::run_file_gc_loop(client_id).await;
    });
}
