pub mod app_context;
pub mod cli;
pub mod config;
pub mod crypto;
pub mod db;
pub mod logs;
pub mod tgbot;
pub mod utils;

use clap::Parser;
use std::process::exit;
use std::sync::Arc;

use crate::config::{AccessControlConfig, BotConfig, ClientRole, TargetsConfig};
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
    pub(crate) billing_config: crate::config::BillingConfig,
    pub(crate) targets_config: TargetsConfig,
    pub(crate) access_control_config: AccessControlConfig,
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
    billing_config_default: &crate::config::BillingConfig,
    targets_config_default: &TargetsConfig,
    access_control_default: &AccessControlConfig,
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
    let billing_config =
        crate::tgbot::transfer::ensure_billing_runtime_config_on(db, billing_config_default)
            .await?;
    let targets_config =
        crate::tgbot::transfer::ensure_targets_runtime_config_on(db, targets_config_default)
            .await?;
    let access_control_config =
        crate::tgbot::transfer::ensure_access_control_runtime_config_on(db, access_control_default)
            .await?;

    tracing::info!(
        database_backend = dialect,
        runtime_job_concurrency = transfer_config.job_concurrency,
        runtime_billing_enabled = billing_config.enabled,
        runtime_target_default_chat_id = targets_config.default_chat_id,
        runtime_admin_user_count = access_control_config.admin_user_ids.len(),
        runtime_allowed_user_count = access_control_config.allowed_user_ids.len(),
        runtime_allowed_target_chat_count = access_control_config.allowed_target_chat_ids.len(),
        "runtime database state loaded"
    );

    Ok(SeededRuntimeState {
        transfer_config,
        billing_config,
        targets_config,
        access_control_config,
    })
}

/// 启动期真实使用的数据库初始化链。
///
/// 先初始化全局数据库连接池，再在该连接上执行 migration 与运行态 seed。
pub(crate) async fn bootstrap_runtime_database_state(
    config: &BotConfig,
) -> anyhow::Result<SeededRuntimeState> {
    let transfer_config_default = config.transfer_config.clone();
    let billing_config_default = config.billing.clone();
    let targets_config_default = crate::config::TargetsConfig::from_runtime_target_state(
        &config.target_map,
        &config.target_aliases,
    );
    let access_control_default = crate::config::AccessControlConfig {
        bootstrap_admin_user_ids: config.bootstrap_admin_user_ids.clone(),
        admin_user_ids: Vec::new(),
        allowed_user_ids: config.allowed_user_ids.clone(),
        allow_all_private_users: config.allow_all_private_users,
        banned_user_ids: config.banned_user_ids.clone(),
        allowed_request_chat_ids: config.allowed_request_chat_ids.clone(),
        allowed_target_chat_ids: config.allowed_target_chat_ids.clone(),
    };

    crate::db::init_database_url(config.storage.database_url.clone()).await?;
    let db = get_db().await?;
    bootstrap_runtime_database_state_on(
        db,
        &config.storage.database_url,
        &transfer_config_default,
        &billing_config_default,
        &targets_config_default,
        &access_control_default,
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
    let targets_config_default = crate::config::TargetsConfig::from_runtime_target_state(
        &config.target_map,
        &config.target_aliases,
    );
    let access_control_default = crate::config::AccessControlConfig {
        bootstrap_admin_user_ids: config.bootstrap_admin_user_ids.clone(),
        admin_user_ids: Vec::new(),
        allowed_user_ids: config.allowed_user_ids.clone(),
        allow_all_private_users: config.allow_all_private_users,
        banned_user_ids: config.banned_user_ids.clone(),
        allowed_request_chat_ids: config.allowed_request_chat_ids.clone(),
        allowed_target_chat_ids: config.allowed_target_chat_ids.clone(),
    };
    let login_mode = match &config.login_info {
        crate::config::LoginInfo::Phone(_) => "phone",
        crate::config::LoginInfo::Token(_) => "token",
        crate::config::LoginInfo::Ocr => "ocr",
    };
    tracing::info!(
        login_mode,
        admin_count = config.admin_ids.len(),
        target_count = config.target_map.len(),
        interaction_client = config.workflow.interaction_client.as_str(),
        configured_download_client = config.workflow.download_client.as_str(),
        upload_client = config.workflow.upload_client.as_str(),
        job_concurrency = config.transfer_config.job_concurrency,
        file_delete_delay_minutes = config.transfer_config.file_delete_delay_minutes,
        file_gc_interval_seconds = config.transfer_config.file_gc_interval_seconds,
        "runtime config loaded"
    );
    let transfer_config_default = config.transfer_config.clone();
    let billing_config_default = config.billing.clone();
    let seeded_runtime = bootstrap_runtime_database_state(&config).await?;
    config.transfer_config = seeded_runtime.transfer_config.clone();
    config.billing = seeded_runtime.billing_config.clone();
    let targets_config = seeded_runtime.targets_config.clone();
    let access_control = seeded_runtime.access_control_config.clone();
    let access_control_runtime = crate::config::AccessControlConfig {
        bootstrap_admin_user_ids: access_control_default.bootstrap_admin_user_ids.clone(),
        admin_user_ids: access_control.admin_user_ids.clone(),
        allowed_user_ids: access_control.allowed_user_ids.clone(),
        allow_all_private_users: access_control.allow_all_private_users,
        banned_user_ids: access_control.banned_user_ids.clone(),
        allowed_request_chat_ids: access_control.allowed_request_chat_ids.clone(),
        allowed_target_chat_ids: access_control.allowed_target_chat_ids.clone(),
    };
    config.target_map = targets_config.to_target_map();
    config.target_aliases = targets_config.aliases.clone();
    config.admin_user_ids = {
        let mut ids = std::collections::BTreeSet::new();
        for id in &access_control_default.bootstrap_admin_user_ids {
            ids.insert(*id);
        }
        for id in &access_control.admin_user_ids {
            ids.insert(*id);
        }
        ids.into_iter().collect()
    };
    config.bootstrap_admin_user_ids = access_control_default.bootstrap_admin_user_ids.clone();
    config.admin_ids = config.admin_user_ids.clone();
    config.allowed_user_ids = access_control.allowed_user_ids.clone();
    config.allow_all_private_users = access_control.allow_all_private_users;
    config.banned_user_ids = access_control.banned_user_ids.clone();
    config.allowed_request_chat_ids = access_control.allowed_request_chat_ids.clone();
    config.allowed_target_chat_ids = access_control.allowed_target_chat_ids.clone();
    tracing::info!(
        job_concurrency = config.transfer_config.job_concurrency,
        file_delete_delay_minutes = config.transfer_config.file_delete_delay_minutes,
        file_gc_interval_seconds = config.transfer_config.file_gc_interval_seconds,
        progress_edit_interval_seconds = config.transfer_config.progress_edit_interval_seconds,
        downloads_default_page_size = config.transfer_config.downloads_default_page_size,
        menu_input_timeout_seconds = config.transfer_config.menu_input_timeout_seconds,
        billing_enabled = config.billing.enabled,
        billing_base_cost_points = config.billing.base_cost_points,
        billing_item_cost_points = config.billing.item_cost_points,
        billing_initial_user_points = config.billing.initial_user_points,
        target_default_chat_id = targets_config.default_chat_id,
        target_route_count = targets_config.by_request_chat_id.len(),
        target_alias_count = targets_config.aliases.len(),
        admin_user_count = config.admin_user_ids.len(),
        allowed_user_count = config.allowed_user_ids.len(),
        allow_all_private_users = config.allow_all_private_users,
        banned_user_count = config.banned_user_ids.len(),
        allowed_request_chat_count = config.allowed_request_chat_ids.len(),
        allowed_target_chat_count = config.allowed_target_chat_ids.len(),
        "runtime transfer config loaded from database"
    );

    let app_context = crate::app_context::app_context();
    app_context
        .send_capabilities
        .set_reply_markup_enabled(config.supports_reply_markup());
    app_context
        .home_announcement
        .set_announcement_text(config.billing.announcement_text.clone());
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
    crate::tgbot::transfer::init_runtime_config(crate::tgbot::transfer::RuntimeInitBundle {
        transfer_config: config.transfer_config.clone(),
        transfer_default_config: transfer_config_default,
        billing_config: config.billing.clone(),
        billing_default_config: billing_config_default,
        targets_config: targets_config.clone(),
        targets_default_config: targets_config_default,
        access_control_config: access_control_runtime,
        access_control_default_config: access_control_default,
        tdlib_files_directories,
    });

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
