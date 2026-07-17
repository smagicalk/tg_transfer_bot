pub mod app_context;
pub mod cli;
pub mod config;
pub mod crypto;
pub mod db;
pub mod logs;
pub mod tgbot;

use clap::Parser;
use std::process::exit;
use std::sync::Arc;

use crate::config::{BotConfig, ClientRole, TargetsConfig};
use crate::db::{ensure_runtime_schema, get_db};

pub const TOKIO_WORKER_STACK_SIZE: usize = 8 * 1024 * 1024;

/// 启动数据库初始化后的运行态快照。
///
/// 这层把“文件默认值”和“数据库当前值”一起返回，方便：
/// - 正常运行回填 `BotConfig`
/// - 测试直接验证 PostgreSQL/SQLite 启动链是否真的完成了 migration + seed
#[derive(Debug, Clone)]
pub(crate) struct SeededRuntimeState {
    pub(crate) transfer_config: crate::config::TransferConfig,
    pub(crate) targets_config: TargetsConfig,
}

/// 启动期真实使用的数据库初始化链。
///
/// 顺序固定为：
/// 1. 初始化业务数据库 URL
/// 2. 执行 migration
/// 3. 读取或 seed 四类运行态配置
///
/// 抽成 helper 后，PostgreSQL 验证可以复用和 `run()` 完全一致的链路，
/// 避免“测试只验证表结构，运行时却漏 seed 或漏回填”的分叉。
pub(crate) async fn bootstrap_runtime_database_state_on(
    db: &sea_orm::DatabaseConnection,
    database_url_for_log: &str,
    transfer_config_default: &crate::config::TransferConfig,
    targets_config_default: &TargetsConfig,
) -> anyhow::Result<SeededRuntimeState> {
    let dialect = match db.get_database_backend() {
        sea_orm::DatabaseBackend::Sqlite => "sqlite",
        sea_orm::DatabaseBackend::Postgres => "postgres",
        other => {
            tracing::warn!(backend = ?other, "runtime database backend is not explicitly profiled");
            "other"
        }
    };
    tracing::info!(
        database_url = %database_url_for_log,
        database_backend = dialect,
        "ensuring runtime database schema"
    );
    ensure_runtime_schema(db).await?;
    tracing::info!(database_backend = dialect, "runtime database schema ready");

    let transfer_config =
        crate::tgbot::transfer::ensure_transfer_runtime_config_on(db, transfer_config_default)
            .await?;
    let targets_config =
        crate::tgbot::transfer::ensure_targets_runtime_config_on(db, targets_config_default)
            .await?;

    tracing::info!(
        database_backend = dialect,
        runtime_job_concurrency = transfer_config.job_concurrency,
        runtime_target_default_chat_id = targets_config.default_chat_id,
        "runtime database state loaded"
    );

    Ok(SeededRuntimeState {
        transfer_config,
        targets_config,
    })
}

/// 启动期真实使用的数据库初始化链。
///
/// 先初始化全局数据库连接池，再在该连接上执行 migration 与运行态 seed。
pub(crate) async fn bootstrap_runtime_database_state(
    config: &BotConfig,
) -> anyhow::Result<SeededRuntimeState> {
    let transfer_config_default = config.transfer_config.clone();
    let targets_config_default = config.targets.clone();
    crate::db::init_database_url(config.storage.database_url.clone()).await?;
    let db = get_db().await?;
    bootstrap_runtime_database_state_on(
        db,
        &config.storage.database_url,
        &transfer_config_default,
        &targets_config_default,
    )
    .await
}

pub async fn run() -> anyhow::Result<()> {
    crate::logs::init_tracing();
    tracing::info!("transfer bot starting");

    let cli = crate::cli::TransferBotCli::parse();
    let config_path = cli.config.clone();
    let config_mode = match &cli.mode {
        None | Some(crate::cli::Mode::None) => "plain",
        Some(crate::cli::Mode::Encrypt { .. }) => "encrypt",
        Some(crate::cli::Mode::Decrypt { .. }) => "decrypt",
    };
    tracing::info!(
        config_path = %config_path,
        config_mode,
        "loading runtime config"
    );
    crate::config::init_runtime_config_path(cli.config.clone());

    let config_str = match cli.get_config().await {
        Ok(config_str) => config_str,
        Err(err) => {
            tracing::error!(error = ?err, "load runtime config failed");
            exit(-1)
        }
    };

    let mut config = match BotConfig::from_json_str(&config_str) {
        Ok(config) => config,
        Err(err) => {
            tracing::error!(error = %err, "parse runtime config failed");
            return Err(err);
        }
    };
    let targets_config_default = config.targets.clone();
    let login_mode = match &config
        .runtime_client(crate::config::ClientRole::Bot)?
        .login_info
    {
        crate::config::LoginInfo::Phone(_) => "phone",
        crate::config::LoginInfo::Token(_) => "token",
        crate::config::LoginInfo::Ocr => "ocr",
    };
    tracing::info!(
        login_mode,
        owner_user_id = config.owner_user_id,
        target_default_chat_id = config.targets.default_chat_id,
        target_alias_count = config.targets.aliases.len(),
        upload_client = config.workflow.upload_client.as_str(),
        job_concurrency = config.transfer_config.job_concurrency,
        file_delete_delay_minutes = config.transfer_config.file_delete_delay_minutes,
        file_gc_interval_seconds = config.transfer_config.file_gc_interval_seconds,
        "runtime config loaded"
    );
    let transfer_config_default = config.transfer_config.clone();
    let seeded_runtime = bootstrap_runtime_database_state(&config).await?;
    config.transfer_config = seeded_runtime.transfer_config.clone();
    let targets_config = seeded_runtime.targets_config.clone();
    config.targets = targets_config.clone();
    tracing::info!(
        job_concurrency = config.transfer_config.job_concurrency,
        file_delete_delay_minutes = config.transfer_config.file_delete_delay_minutes,
        file_gc_interval_seconds = config.transfer_config.file_gc_interval_seconds,
        progress_edit_interval_seconds = config.transfer_config.progress_edit_interval_seconds,
        downloads_default_page_size = config.transfer_config.downloads_default_page_size,
        menu_input_timeout_seconds = config.transfer_config.menu_input_timeout_seconds,
        target_default_chat_id = targets_config.default_chat_id,
        target_alias_count = targets_config.aliases.len(),
        "runtime transfer config loaded from database"
    );

    let app_context = crate::app_context::app_context();
    app_context.send_capabilities.set_reply_markup_enabled(true);
    tracing::info!(
        enabled = app_context.send_capabilities.reply_markup_enabled(),
        "tdlib reply markup capability configured"
    );

    let tdlib_files_directories = config
        .runtime_clients
        .iter()
        .map(|(role, runtime)| {
            (
                *role,
                std::path::PathBuf::from(runtime.tdlib_config.files_directory.clone()),
            )
        })
        .collect::<std::collections::HashMap<_, _>>();
    crate::tgbot::transfer::init_runtime_config_on(
        app_context.as_ref(),
        crate::tgbot::transfer::RuntimeInitBundle {
            transfer_config: config.transfer_config.clone(),
            transfer_default_config: transfer_config_default,
            targets_config: targets_config.clone(),
            targets_default_config: targets_config_default,
            tdlib_files_directories,
        },
    );

    for role in config.required_client_roles() {
        create_and_register_client(role, &mut config).await?;
    }

    tracing::info!("entering tdlib receive loop");
    if let Err(err) = tgbot::receive(app_context.clone(), Arc::from(config)).await {
        tracing::error!(error = %err, "tdlib receive loop exited with error");
        return Err(err);
    }

    tracing::warn!("tdlib receive loop exited without error");
    Ok(())
}

async fn create_and_register_client(
    role: ClientRole,
    config: &mut BotConfig,
) -> anyhow::Result<()> {
    let runtime_client = config.runtime_client(role)?.clone();
    let client_id = tgbot::create_client().await?;
    config.set_client_id(role, client_id);
    tracing::info!(client_id, role = role.as_str(), "tdlib client created");

    let log_client_id = client_id;
    let log_verbosity_level = runtime_client.log_verbosity_level;
    tokio::spawn(async move {
        tgbot::set_log(log_client_id, log_verbosity_level).await;
    });

    let version_client_id = client_id;
    tokio::spawn(async move {
        if let Err(err) = tgbot::get_version(version_client_id).await {
            tracing::warn!(
                client_id = version_client_id,
                error = %err,
                "load tdlib version failed"
            );
        }
    });

    Ok(())
}
