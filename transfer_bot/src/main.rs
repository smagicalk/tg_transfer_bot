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

    // 程序启动时执行一次迁移，保证表结构可用。
    migration::Migrator::up(get_db().await?, None).await?;

    // 解析命令行参数。
    let cli = crate::cli::TransferBotCli::parse();
    tracing::debug!("{:?}", cli);
    crate::config::init_runtime_config_path(cli.config.clone());

    // 读取配置（可能是明文文件，也可能是解密后的内容）。
    let config_str = match cli.get_config().await {
        Ok(config_str) => config_str,
        Err(err) => {
            tracing::error!("{:?}", err);
            exit(-1)
        }
    };
    tracing::debug!("{:#?}", config_str);

    // 将 JSON 配置反序列化为结构体。
    let mut config = serde_json::from_str::<BotConfig>(&config_str)?;
    tracing::debug!("{:#?}", config);

    // 初始化转存运行配置。
    // 这类参数原先使用环境变量读取，现在统一从 config.json 注入。
    tgbot::transfer::init_runtime_config(config.transfer_config.clone());

    // 创建 TDLib client，并写回到运行时配置。
    let client_id = tgbot::create_client().await?;
    config.client_id = Some(client_id);
    tracing::debug!("create client success, client_id {:#?}", client_id);

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
    tgbot::receive(Arc::from(config)).await?;

    Ok(())
}
