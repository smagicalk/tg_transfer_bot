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

use crate::config::{BotConfig, ClientRole};
use crate::db::{ensure_runtime_schema, get_db};

pub const TOKIO_WORKER_STACK_SIZE: usize = 8 * 1024 * 1024;

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

    crate::db::init_database_url(config.storage.database_url.clone()).await?;

    tracing::info!("ensuring runtime database schema");
    ensure_runtime_schema(get_db().await?).await?;
    tracing::info!("runtime database schema ready");

    let app_context = crate::app_context::app_context();
    app_context
        .send_capabilities
        .set_reply_markup_enabled(config.supports_reply_markup());
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
    app_context
        .transfer_runtime
        .init_runtime_config(config.transfer_config.clone(), tdlib_files_directories);

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
