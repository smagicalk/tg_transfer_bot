// 程序入口：
// 1. 初始化日志
// 2. 执行数据库迁移
// 3. 读取 CLI 与配置
// 4. 创建 TDLib client 并进入 update 接收循环
use clap::Parser;
use std::process::exit;
use std::sync::Arc;

use crate::config::BotConfig;
use crate::db::get_db;
use migration::MigratorTrait;

mod cli;
mod config;
pub mod crypto;
pub mod db;
pub mod logs;
pub mod tgbot;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化 tracing（控制台 + 文件）输出。
    crate::logs::init_tracing();
    tracing::info!("transfer bot starting");

    // 程序启动时执行一次迁移，保证表结构可用。
    tracing::info!("running database migrations");
    migration::Migrator::up(get_db().await?, None).await?;
    tracing::info!("database migrations ready");

    // 解析命令行参数。
    let cli = crate::cli::TransferBotCli::parse();
    let config_path = cli.config.clone();
    let config_mode = match &cli.mode {
        None | Some(crate::cli::Mode::None) => "plain",
        Some(crate::cli::Mode::Encrypt { .. }) => "encrypt",
        Some(crate::cli::Mode::Decrypt { .. }) => "decrypt",
    };
    // CLI 里可能包含解密密码，只记录模式和路径，不打印完整结构。
    tracing::info!(
        config_path = %config_path,
        config_mode,
        "loading runtime config"
    );
    crate::config::init_runtime_config_path(cli.config.clone());

    // 读取配置（可能是明文文件，也可能是解密后的内容）。
    let config_str = match cli.get_config().await {
        Ok(config_str) => config_str,
        Err(err) => {
            tracing::error!(error = ?err, "load runtime config failed");
            exit(-1)
        }
    };

    // 将 JSON 配置反序列化为结构体。
    let mut config = serde_json::from_str::<BotConfig>(&config_str)?;
    let login_mode = match &config.login_info {
        crate::config::LoginInfo::Phone(_) => "phone",
        crate::config::LoginInfo::Token(_) => "token",
        crate::config::LoginInfo::Ocr => "ocr",
    };
    // 配置里含 api_hash、数据库密钥、手机号/token，日志只保留安全摘要。
    tracing::info!(
        login_mode,
        admin_count = config.admin_ids.len(),
        target_count = config.target_map.len(),
        job_concurrency = config.transfer_config.job_concurrency,
        file_delete_delay_minutes = config.transfer_config.file_delete_delay_minutes,
        file_gc_interval_seconds = config.transfer_config.file_gc_interval_seconds,
        "runtime config loaded"
    );

    // 初始化转存运行配置。
    // 运行时可调项来自 transfer_config；TDLib 文件目录只用于 GC 安全边界，不开放动态修改。
    tgbot::transfer::init_runtime_config(
        config.transfer_config.clone(),
        config.tdlib_config.files_directory.clone(),
    );

    // 创建 TDLib client，并写回到运行时配置。
    let client_id = tgbot::create_client().await?;
    config.client_id = Some(client_id);
    tracing::info!(client_id, "tdlib client created");

    // 异步设置 TDLib 日志级别（不阻塞主流程）。
    let log_client_id = client_id;
    tokio::spawn(async move {
        tgbot::set_log(log_client_id).await;
    });

    // 异步读取 TDLib 版本信息，用于诊断日志。
    let version_client_id = client_id;
    tokio::spawn(async move {
        let _ = tgbot::get_version(version_client_id).await;
    });

    // 主循环：持续接收并处理 TDLib update。
    tracing::info!("entering tdlib receive loop");
    tgbot::receive(Arc::from(config)).await?;

    Ok(())
}
