//! 转存机器人核心库根模块。
//!
//! 负责整个机器人服务的生命周期管理，包括：
//! - 数据库迁移与运行态配置（TransferConfig、TargetsConfig、动态授权用户）载入；
//! - 全局应用上下文（AppContext）与日志跟踪初始化；
//! - TDLib 客户端生命周期管理与事件主循环驱动。

/// 动态用户访问控制模块（权限列表、白名单过滤）。
pub(crate) mod access;
/// 全局应用共享上下文（数据库连接池、内存状态管理）。
pub mod app_context;
/// 命令行参数解析与配置加密/解密模块。
pub mod cli;
/// 配置模型定义（JSON 配置文件映射、运行态配置项）。
pub mod config;
/// 密码学加解密工具模块（AES-GCM 配置加解密）。
pub mod crypto;
/// 数据库连接与 SeaORM 数据持久化模块。
pub mod db;
/// 日志与 Tracing 跟踪子系统初始化。
pub mod logs;
/// Telegram 业务逻辑主模块（TDLib 事件处理、命令分发、转存工作流）。
pub mod tgbot;

use clap::Parser;
use std::process::exit;
use std::sync::Arc;

use crate::config::{BotConfig, ClientRole, TargetsConfig};
use crate::db::{ensure_runtime_schema, get_db};

/// Tokio 工作线程的调用栈深度限制（8MB）。
///
/// 由于 TDLib 返回的消息结构和嵌套更新深层递归反序列化可能占用较深栈空间，
/// 设置为 8MB 可有效避免深层调用时的线程栈溢出崩溃。
pub const TOKIO_WORKER_STACK_SIZE: usize = 8 * 1024 * 1024;

/// 启动数据库初始化后的运行态配置快照。
///
/// 该结构体同时包含了从磁盘文件加载的初始默认值以及数据库中已持久化的当前值，
/// 主要用于：
/// 1. 服务启动时将数据库中保存的最新配置回填覆盖到内存中的 `BotConfig`；
/// 2. 单元测试中验证数据库初始化链路是否正确完成了 schema 迁移和数据种子播种。
#[derive(Debug, Clone)]
pub(crate) struct SeededRuntimeState {
    /// 从数据库恢复或初始播种的转存运行时配置（并发数、延迟删除时间、GC 间隔等）
    pub(crate) transfer_config: crate::config::TransferConfig,
    /// 从数据库恢复或初始播种的目标频道/群组配置（默认目标、快捷别名映射）
    pub(crate) targets_config: TargetsConfig,
    /// 从数据库权限表加载的已授权用户 Telegram ID 集合
    pub(crate) authorized_user_ids: std::collections::BTreeSet<i64>,
}

/// 在指定的数据库连接上执行完整的运行态初始化流程。
///
/// 初始化顺序固定为：
/// 1. 检查并记录数据库类型（SQLite 或 PostgreSQL）；
/// 2. 执行数据库 schema 迁移（`ensure_runtime_schema`）；
/// 3. 读取数据库中的运行态配置；若数据库为空，则使用默认值进行种子播种（seed）；
/// 4. 查询并返回当前生效的已授权用户列表。
///
/// 抽离成独立 helper 函数后，测试套件可与主生产流程完全复用同一套初始化链，
/// 确保开发测试环境与生产环境数据库行为严格一致。
///
/// # 参数
/// - `db`: 活跃的 SeaORM 数据库连接引用
/// - `database_url_for_log`: 用于日志脱敏记录的数据库连接 URL 描述
/// - `transfer_config_default`: 文件中配置的默认转存配置，在数据库无记录时用作初始值
/// - `targets_config_default`: 文件中配置的默认目标配置，在数据库无记录时用作初始值
///
/// # 返回值
/// 返回包含已生效转存配置、目标配置和授权用户集合的 `SeededRuntimeState`。
pub(crate) async fn bootstrap_runtime_database_state_on(
    db: &sea_orm::DatabaseConnection,
    database_url_for_log: &str,
    transfer_config_default: &crate::config::TransferConfig,
    targets_config_default: &TargetsConfig,
) -> anyhow::Result<SeededRuntimeState> {
    // 识别当前数据库方言类型（用于日志与特定方言行为适配）
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
    // 执行数据表创建与版本迁移
    ensure_runtime_schema(db).await?;
    tracing::info!(database_backend = dialect, "runtime database schema ready");

    // 确保转存运行时配置在数据库中存在（不存在则自动写入初始值）
    let transfer_config =
        crate::tgbot::transfer::ensure_transfer_runtime_config_on(db, transfer_config_default)
            .await?;
    // 确保目标频道配置在数据库中存在（不存在则自动写入初始值）
    let targets_config =
        crate::tgbot::transfer::ensure_targets_runtime_config_on(db, targets_config_default)
            .await?;
    // 从数据库中查询已授权的用户 ID 清单
    let authorized_user_ids = crate::access::list_authorized_user_ids_on(db).await?;

    tracing::info!(
        database_backend = dialect,
        runtime_job_concurrency = transfer_config.job_concurrency,
        runtime_target_default_chat_id = targets_config.default_chat_id,
        authorized_user_count = authorized_user_ids.len(),
        "runtime database state loaded"
    );

    Ok(SeededRuntimeState {
        transfer_config,
        targets_config,
        authorized_user_ids,
    })
}

/// 生产启动期调用的数据库运行态初始化总入口。
///
/// 先基于配置文件中的连接串初始化全局数据库连接池，
/// 随后调用 `bootstrap_runtime_database_state_on` 完成迁移与播种。
///
/// # 参数
/// - `config`: 解析后的全局机器人配置对象引用
///
/// # 返回值
/// 成功返回初始化完成的 `SeededRuntimeState` 快照。
pub(crate) async fn bootstrap_runtime_database_state(
    config: &BotConfig,
) -> anyhow::Result<SeededRuntimeState> {
    let transfer_config_default = config.transfer_config.clone();
    let targets_config_default = config.targets.clone();
    // 初始化全局数据库连接池配置
    crate::db::init_database_url(config.storage.database_url.clone()).await?;
    let db = get_db().await?;
    // 执行建表、迁移与数据播种
    bootstrap_runtime_database_state_on(
        db,
        &config.storage.database_url,
        &transfer_config_default,
        &targets_config_default,
    )
    .await
}

/// 机器人核心服务运行总入口。
///
/// 串联完整的启动与事件监听工作流：
/// 1. 初始化 Tracing 结构化日志；
/// 2. 解析命令行参数并读取/解密配置文件；
/// 3. 执行数据库迁移并将持久化状态注入运行态；
/// 4. 实例化 `AppContext` 并注册初始权限与 TDLib 回调按钮能力；
/// 5. 创建 Bot 与可选的 User TDLib 客户端实例；
/// 6. 进入非阻塞异步事件分发主循环，开始监听和处理 Telegram 消息与任务。
///
/// # 返回值
/// 当事件循环因不可恢复的错误异常终止时返回错误，正常退出返回 `Ok(())`。
pub async fn run() -> anyhow::Result<()> {
    // 1. 初始化控制台与日志追踪
    crate::logs::init_tracing();
    tracing::info!("transfer bot starting");

    // 2. 解析命令行启动参数
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
    // 记录运行时配置文件所在路径，方便后续动态回写或查看
    crate::config::init_runtime_config_path(cli.config.clone());

    // 3. 读取配置文件内容（支持密文解密）
    let config_str = match cli.get_config().await {
        Ok(config_str) => config_str,
        Err(err) => {
            tracing::error!(error = ?err, "load runtime config failed");
            exit(-1)
        }
    };

    // 4. 解析 JSON 配置模型
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
        admin_user_count = config.admin_user_ids.len(),
        target_default_chat_id = config.targets.default_chat_id,
        target_alias_count = config.targets.aliases.len(),
        upload_client = config.workflow.upload_client.as_str(),
        job_concurrency = config.transfer_config.job_concurrency,
        file_delete_delay_minutes = config.transfer_config.file_delete_delay_minutes,
        file_gc_interval_seconds = config.transfer_config.file_gc_interval_seconds,
        "runtime config loaded"
    );
    let transfer_config_default = config.transfer_config.clone();

    // 5. 数据库初始化：建表、迁移以及动态配置加载
    let seeded_runtime = bootstrap_runtime_database_state(&config).await?;
    config.transfer_config = seeded_runtime.transfer_config.clone();
    let targets_config = seeded_runtime.targets_config.clone();
    config.targets = targets_config.clone();
    let authorized_user_ids = seeded_runtime.authorized_user_ids;
    tracing::info!(
        job_concurrency = config.transfer_config.job_concurrency,
        file_delete_delay_minutes = config.transfer_config.file_delete_delay_minutes,
        file_gc_interval_seconds = config.transfer_config.file_gc_interval_seconds,
        progress_edit_interval_seconds = config.transfer_config.progress_edit_interval_seconds,
        downloads_default_page_size = config.transfer_config.downloads_default_page_size,
        menu_input_timeout_seconds = config.transfer_config.menu_input_timeout_seconds,
        target_default_chat_id = targets_config.default_chat_id,
        target_alias_count = targets_config.aliases.len(),
        authorized_user_count = authorized_user_ids.len(),
        "runtime transfer config loaded from database"
    );

    // 6. 初始化全局应用共享上下文
    let app_context = crate::app_context::app_context();
    // 注入数据库加载的已授权用户白名单
    app_context
        .access_control
        .replace_authorized_user_ids(authorized_user_ids);
    // 启用内联按钮及自定义回复交互能力
    app_context.send_capabilities.set_reply_markup_enabled(true);
    tracing::info!(
        enabled = app_context.send_capabilities.reply_markup_enabled(),
        "tdlib reply markup capability configured"
    );

    // 收集各客户端角色的 TDLib 下载与缓存保存目录映射
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
    // 初始化转存引擎运行时状态（默认配置与路径关联）
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

    // 7. 创建并初始化配置要求的 TDLib 客户端实例
    let mut bot_client = None;
    for role in config.required_client_roles() {
        let client = create_and_register_client(role, &mut config).await?;
        if role == ClientRole::Bot {
            bot_client = Some(client);
        }
    }
    let bot_client = bot_client.ok_or_else(|| anyhow::anyhow!("bot client is required"))?;

    // 8. 进入 TDLib 事件监听与分发循环
    tracing::info!("entering tdlib receive loop");
    if let Err(err) = tgbot::receive(app_context.clone(), Arc::from(config), bot_client).await {
        tracing::error!(error = %err, "tdlib receive loop exited with error");
        return Err(err);
    }

    tracing::warn!("tdlib receive loop exited without error");
    Ok(())
}

/// 为指定的客户端角色创建新版 `tdlib_rs::Client` 实例并完成基础参数注册。
///
/// 步骤：
/// 1. 调用 `tdlib_rs::Client::new()` 创建新客户端实例，自动向底层 `observer` 注册监听；
/// 2. 获取分配的 `client_id`，并记录在内存配置的对应角色映射中；
/// 3. 后台异步配置 TDLib 本地日志详细级别；
/// 4. 后台异步拉取并校验 TDLib 引擎版本。
///
/// # 参数
/// - `role`: 客户端角色（`ClientRole::Bot` 或 `ClientRole::User`）
/// - `config`: 全局配置对象的可变引用，用于回填生成的 `client_id`
///
/// # 返回值
/// 返回创建并绑定成功的 `tdlib_rs::Client` 客户端实例。
async fn create_and_register_client(
    role: ClientRole,
    config: &mut BotConfig,
) -> anyhow::Result<tdlib_rs::Client> {
    let runtime_client = config.runtime_client(role)?.clone();
    // 实例化新版 Client，其 drop 时会自动向 observer 取消注册
    let client = tdlib_rs::Client::new();
    let client_id = client.id();
    // 将生成的 ID 记录到配置映射中供后续查找
    config.set_client_id(role, client_id);
    tracing::info!(client_id, role = role.as_str(), "tdlib client created");

    // 异步配置日志级别
    let log_client_id = client_id;
    let log_verbosity_level = runtime_client.log_verbosity_level;
    tokio::spawn(async move {
        tgbot::set_log(log_client_id, log_verbosity_level).await;
    });

    // 异步检查 TDLib 版本号
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

    Ok(client)
}
