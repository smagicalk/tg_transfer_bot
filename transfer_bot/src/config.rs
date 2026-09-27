//! 配置模型定义：
//! 负责 JSON <-> Rust 结构体映射，涵盖 TDLib 底层参数、转存引擎并发与 GC、
//! 存储连接串以及多客户端架构（Bot + User Executor）的角色分配。

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

/// 运行时配置文件路径单例容器：
/// - 主程序启动时通过 CLI 参数解析写入；
/// - 目前保留给需要追溯“当前配置文件来自何处”的流程使用。
static CONFIG_FILE_PATH: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

/// TDLib 客户端角色类型。
///
/// 本系统采用双客户端架构：
/// - `Bot`: 官方机器人客户端，固定负责处理命令、展示卡片与内联键盘、状态更新及默认转存上传；
/// - `User`: 用户账号客户端，作为按需登录的回退执行器，仅在需要读取私有频道或 Bot 权限不足时介入。
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ClientRole {
    /// 用户号执行器客户端角色。
    User,
    /// 官方机器人客户端角色。
    Bot,
}

impl ClientRole {
    /// 获取角色的字符串标识（"user" 或 "bot"）。
    ///
    /// 常用于日志记录与模块标签，不包含任何敏感信息。
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Bot => "bot",
        }
    }
}

/// TDLib 底层运行时参数配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct TdlibConfig {
    /// 是否连接 Telegram 官方测试数据中心（Test DC）。
    pub use_test_dc: bool,
    /// TDLib 数据库本地存储目录路径。
    pub database_directory: String,
    /// TDLib 下载与上传文件的本地缓存目录路径。
    pub files_directory: String,
    /// TDLib 本地数据库加密密钥（留空表示不启用加密）。
    pub database_encryption_key: String,
    /// 是否启用本地文件元数据数据库以加速文件定位。
    pub use_file_database: bool,
    /// 是否启用会话与聊天信息本地数据库。
    pub use_chat_info_database: bool,
    /// 是否启用历史消息本地数据库。
    pub use_message_database: bool,
    /// 是否支持端到端加密秘密聊天。
    pub use_secret_chats: bool,
    /// Telegram API Application ID。
    pub api_id: i32,
    /// Telegram API Application Hash。
    pub api_hash: String,
    /// 客户端系统语言代码（例如 "zh-hans"、"en"）。
    pub system_language_code: String,
    /// 运行设备型号标识（例如 "tg_transfer_bot"）。
    pub device_model: String,
    /// 操作系统或系统版本号。
    pub system_version: String,
    /// 应用程序发布版本号。
    pub application_version: String,
}

/// 转存运行时控制配置。
///
/// 涵盖后台并发控制、文件垃圾回收（GC）延迟、状态卡片编辑频率等核心调控参数。
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct TransferConfig {
    /// 后台转存重任务并发数上限。
    #[serde(default = "default_transfer_job_concurrency")]
    pub job_concurrency: usize,

    /// 本地文件引用归零后的延迟物理删除时间（分钟）。
    ///
    /// 预留延迟缓冲时间可允许用户在转存后立即再次转发而无需重新从 Telegram 云端下载。
    /// `alias` 兼容历史旧配置键 `file_delete_delay_hours`。
    #[serde(
        default = "default_transfer_file_delete_delay_minutes",
        alias = "file_delete_delay_hours"
    )]
    pub file_delete_delay_minutes: i64,

    /// 本地垃圾文件（GC）定期扫描回收的循环间隔时间（秒）。
    #[serde(default = "default_transfer_file_gc_interval_seconds")]
    pub file_gc_interval_seconds: u64,

    /// 向 Telegram 聊天会话中编辑刷新转存进度卡片的时间间隔（秒），避免触碰 Telegram Flood 限速。
    #[serde(default = "default_progress_edit_interval_seconds")]
    pub progress_edit_interval_seconds: u64,

    /// `/downloads` 命令展示下载列表时的默认单页条数。
    #[serde(default = "default_downloads_page_size")]
    pub downloads_default_page_size: u64,

    /// 交互式菜单等待用户输入文本或按钮操作的超时时间（秒）。
    #[serde(default = "default_menu_input_timeout_seconds")]
    pub menu_input_timeout_seconds: u64,
}

impl Default for TransferConfig {
    fn default() -> Self {
        Self {
            job_concurrency: default_transfer_job_concurrency(),
            file_delete_delay_minutes: default_transfer_file_delete_delay_minutes(),
            file_gc_interval_seconds: default_transfer_file_gc_interval_seconds(),
            progress_edit_interval_seconds: default_progress_edit_interval_seconds(),
            downloads_default_page_size: default_downloads_page_size(),
            menu_input_timeout_seconds: default_menu_input_timeout_seconds(),
        }
    }
}

impl TransferConfig {
    /// 将当前转存配置转换为数据库单行配置的 `ActiveModel`，用于持久化入库。
    pub fn to_db_row(
        &self,
        now: chrono::DateTime<chrono::FixedOffset>,
    ) -> crate::db::transfer_runtime_config::ActiveModel {
        crate::db::transfer_runtime_config::ActiveModel {
            id: sea_orm::ActiveValue::Set(1),
            job_concurrency: sea_orm::ActiveValue::Set(self.job_concurrency as i64),
            file_delete_delay_minutes: sea_orm::ActiveValue::Set(self.file_delete_delay_minutes),
            file_gc_interval_seconds: sea_orm::ActiveValue::Set(
                self.file_gc_interval_seconds as i64,
            ),
            progress_edit_interval_seconds: sea_orm::ActiveValue::Set(
                self.progress_edit_interval_seconds as i64,
            ),
            downloads_default_page_size: sea_orm::ActiveValue::Set(
                self.downloads_default_page_size as i64,
            ),
            menu_input_timeout_seconds: sea_orm::ActiveValue::Set(
                self.menu_input_timeout_seconds as i64,
            ),
            created_at: sea_orm::ActiveValue::Set(now),
            updated_at: sea_orm::ActiveValue::Set(now),
        }
    }

    /// 从数据库单行持久化模型中解析并恢复运行时配置。
    pub fn from_db_model(
        model: &crate::db::transfer_runtime_config::Model,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            job_concurrency: usize::try_from(model.job_concurrency)?,
            file_delete_delay_minutes: model.file_delete_delay_minutes,
            file_gc_interval_seconds: u64::try_from(model.file_gc_interval_seconds)?,
            progress_edit_interval_seconds: u64::try_from(model.progress_edit_interval_seconds)?,
            downloads_default_page_size: u64::try_from(model.downloads_default_page_size)?,
            menu_input_timeout_seconds: u64::try_from(model.menu_input_timeout_seconds)?,
        })
    }
}

/// 客户端登录鉴权方式。
///
/// 序列化为 JSON 格式形如：`{ "type": "PHONE", "data": "..." }` 或 `{ "type": "OCR" }`。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(
    rename = "login_info",
    rename_all = "UPPERCASE",
    tag = "type",
    content = "data"
)]
pub enum LoginInfo {
    /// 手机号登录，携带国际格式手机号字符串（例如 "+8613800000000"）。
    Phone(String),
    /// Bot Token 凭据登录，携带完整的 BotFather Token。
    Token(String),
    #[default]
    /// 交互式二维码扫码登录（按需用户执行器默认方式）。
    Ocr,
}

/// TDLib 跨客户端共享的公共默认参数。
///
/// v2 配置将“公共默认参数”与“客户端独占本地目录”拆分，避免 Bot 与 User 误用同一个 TDLib 存储目录。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct TdlibDefaults {
    /// 是否连接 Telegram 官方测试数据中心（Test DC）。
    pub use_test_dc: bool,
    /// Telegram API Application ID。
    pub api_id: i32,
    /// Telegram API Application Hash。
    pub api_hash: String,
    /// 客户端系统语言代码（例如 "zh-hans"、"en"）。
    pub system_language_code: String,
    /// 运行设备型号标识（例如 "tg_transfer_bot"）。
    pub device_model: String,
    /// 操作系统或系统版本号。
    pub system_version: String,
    /// 应用程序发布版本号。
    pub application_version: String,
    /// 是否支持端到端加密秘密聊天。
    pub use_secret_chats: bool,
    /// TDLib 底层 C++ 内核日志详细程度级别（默认 1，仅记录警告与错误）。
    #[serde(default = "default_tdlib_log_verbosity_level")]
    pub log_verbosity_level: i32,
}

/// 机器人业务持久化存储配置。
///
/// 注意：TDLib 的 `database_directory` 只属于 Telegram 内部引擎；
/// 转存任务队列、文件引用计数、断点恢复和授权名单等应用层数据均持久化在当前配置的 SQLite 中。
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StorageConfig {
    /// SeaORM / SQLx 连接使用的数据库连接串。
    ///
    /// 推荐使用本地 SQLite 文件路径：`sqlite://tg/app/transfer.sqlite?mode=rwc`。
    #[serde(default = "default_storage_database_url")]
    pub database_url: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            database_url: default_storage_database_url(),
        }
    }
}

/// 单个 TDLib 客户端实例独占的本地存储与目录配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct ClientTdlibConfig {
    /// 该客户端独占的 TDLib 数据库本地存储目录。
    pub database_directory: String,
    /// 该客户端独占的 TDLib 下载与上传文件本地缓存目录。
    pub files_directory: String,
    /// 该客户端数据库本地加密密钥（留空表示不加密）。
    pub database_encryption_key: String,
    /// 是否启用本地文件元数据数据库。
    pub use_file_database: bool,
    /// 是否启用会话与聊天信息本地数据库。
    pub use_chat_info_database: bool,
    /// 是否启用历史消息本地数据库。
    pub use_message_database: bool,
}

impl ClientTdlibConfig {
    /// 将公共默认参数与客户端独占本地目录合成为完整的 TDLib 启动参数。
    fn to_tdlib_config(&self, defaults: &TdlibDefaults) -> TdlibConfig {
        TdlibConfig {
            use_test_dc: defaults.use_test_dc,
            database_directory: self.database_directory.clone(),
            files_directory: self.files_directory.clone(),
            database_encryption_key: self.database_encryption_key.clone(),
            use_file_database: self.use_file_database,
            use_chat_info_database: self.use_chat_info_database,
            use_message_database: self.use_message_database,
            use_secret_chats: defaults.use_secret_chats,
            api_id: defaults.api_id,
            api_hash: defaults.api_hash.clone(),
            system_language_code: defaults.system_language_code.clone(),
            device_model: defaults.device_model.clone(),
            system_version: defaults.system_version.clone(),
            application_version: defaults.application_version.clone(),
        }
    }
}

/// 用户执行器客户端专属配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserClientConfig {
    /// 用户执行器的登录鉴权方式（默认二维码登录）。
    #[serde(default)]
    pub login_info: LoginInfo,
    /// 用户执行器独占的 TDLib 本地目录与数据库配置。
    pub tdlib: ClientTdlibConfig,
}

/// 官方 Bot 客户端专属配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct BotClientConfig {
    /// 官方 Bot 的 Telegram Bot Token（由 @BotFather 发行）。
    pub token: String,
    /// 官方 Bot 独占的 TDLib 本地目录与数据库配置。
    pub tdlib: ClientTdlibConfig,
}

/// v2 固定客户端组合配置（明确划分 User 执行器与 Bot 客户端）。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct ClientsConfig {
    /// 用户号执行器客户端配置。
    pub user: UserClientConfig,
    /// 官方 Bot 客户端配置。
    pub bot: BotClientConfig,
}

/// 业务流程客户端角色分配配置。
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowConfig {
    /// 负责向目标频道上传文件的客户端角色（兼容历史字段，当前固定为 Bot 优先执行）。
    #[serde(default = "default_client_role_bot")]
    pub upload_client: ClientRole,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            upload_client: ClientRole::Bot,
        }
    }
}

/// 所有者或管理员的一次 Bot 私聊交互上下文操作人凭据。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestActor {
    /// 发生私聊交互的 Telegram 会话 Chat ID。
    pub request_chat_id: i64,
    /// 发送请求的用户 Telegram User ID。
    pub user_id: i64,
}

/// 默认转存目标频道与快捷别名配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct TargetsConfig {
    /// 未指定目标频道时的全局默认目标频道或群组 Chat ID。
    pub default_chat_id: i64,
    /// 目标频道快捷别名映射表（例如 "anime" -> -100123456789）。
    #[serde(default)]
    pub aliases: HashMap<String, i64>,
}

impl TargetsConfig {
    /// 判断目标配置是否为空（未配置默认频道且无任何别名映射）。
    pub fn is_empty(&self) -> bool {
        self.default_chat_id == 0 && self.aliases.is_empty()
    }
}

/// v2 格式的配置文件顶层结构。
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct BotConfigV2 {
    /// 配置文件格式版本（必须为 2）。
    pub config_version: i32,
    /// 拥有者（Owner）的 Telegram User ID，具备最高管理权限。
    #[serde(default)]
    pub owner_user_id: i64,
    /// 具有同等管理权限的管理员 Telegram User ID 列表。
    #[serde(default)]
    pub admin_user_ids: Vec<i64>,
    /// TDLib 跨客户端共享的基础默认参数。
    pub tdlib_defaults: TdlibDefaults,
    /// 业务 SQLite 数据库存储配置。
    #[serde(default)]
    pub storage: StorageConfig,
    /// 固定客户端集合配置（User 执行器与 Bot 客户端）。
    pub clients: ClientsConfig,
    /// 工作流角色分配配置。
    #[serde(default)]
    pub workflow: WorkflowConfig,
    /// 默认目标频道与别名映射配置。
    #[serde(default)]
    pub targets: TargetsConfig,
    /// 转存调度与 GC 运行时配置。
    #[serde(default)]
    pub transfer_config: TransferConfig,
}

/// 运行时单个 TDLib 客户端的组装就绪配置。
#[derive(Debug, Clone)]
pub struct RuntimeClientConfig {
    /// 客户端角色类型（User 或 Bot）。
    pub role: ClientRole,
    /// 完整的 TDLib 启动参数。
    pub tdlib_config: TdlibConfig,
    /// 登录鉴权方式。
    pub login_info: LoginInfo,
    /// TDLib 内核日志详细级别。
    pub log_verbosity_level: i32,
}

/// 运行期分配给各个角色的 TDLib Client ID 记录容器。
#[derive(Debug, Clone, Default)]
pub struct RuntimeClientIds {
    /// 用户执行器分配到的 Client ID。
    pub user: Option<i32>,
    /// 官方 Bot 分配到的 Client ID。
    pub bot: Option<i32>,
}

impl RuntimeClientIds {
    /// 写入指定角色对应的 TDLib Client ID。
    pub fn set(&mut self, role: ClientRole, client_id: i32) {
        match role {
            ClientRole::User => self.user = Some(client_id),
            ClientRole::Bot => self.bot = Some(client_id),
        }
    }

    /// 按角色读取分配的 TDLib Client ID。
    pub fn get(&self, role: ClientRole) -> Option<i32> {
        match role {
            ClientRole::User => self.user,
            ClientRole::Bot => self.bot,
        }
    }

    /// 反查 TDLib Client ID 对应的角色。
    pub fn role_for_client_id(&self, client_id: i32) -> Option<ClientRole> {
        if self.user == Some(client_id) {
            return Some(ClientRole::User);
        }
        if self.bot == Some(client_id) {
            return Some(ClientRole::Bot);
        }
        None
    }
}

/// 转存执行所需的各环节 Client ID 聚合视图。
///
/// `download` 是旧 workflow 字段对应的兼容 client；真实源读取/下载会跟随每个任务的
/// `source_client_role`，可通过 `get(ClientRole)` 取得实际 client id。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferClientIds {
    /// 负责发送交互消息与进度卡片的 Client ID（固定为 Bot）。
    pub interaction: i32,
    /// 默认下载环节使用的 Client ID。
    pub download: i32,
    /// 负责将文件上传至目标频道的 Client ID（固定为 Bot）。
    pub upload: i32,
    /// 已就绪的用户执行器 Client ID（若已登录）。
    pub user: Option<i32>,
    /// 已就绪的官方 Bot Client ID。
    pub bot: Option<i32>,
}

impl TransferClientIds {
    /// 按角色获取对应的 TDLib Client ID，若未登录或未就绪则返回明确错误提示。
    pub fn get(self, role: ClientRole) -> anyhow::Result<i32> {
        match role {
            ClientRole::User => self.user.ok_or_else(|| {
                anyhow::anyhow!("执行器未登录；请由 owner 在“管理 -> 执行器”中完成登录")
            }),
            ClientRole::Bot => self
                .bot
                .ok_or_else(|| anyhow::anyhow!("bot client is not ready")),
        }
    }
}

/// 机器人应用程序运行时核心配置上下文视图。
///
/// 业务代码直接与该视图交互，无需关心底层配置是 v1 还是 v2 格式。
#[derive(Debug, Clone, Default)]
pub struct BotConfig {
    /// 始终允许与 Bot 私聊交互的拥有者 Telegram User ID。
    pub owner_user_id: i64,

    /// 与拥有者具有同等权限的管理员 Telegram User ID 白名单集合。
    pub admin_user_ids: BTreeSet<i64>,

    /// 机器人业务数据库持久化配置。
    pub storage: StorageConfig,

    /// 启动时用于向数据库播种（Seed）的初始目标频道配置。
    pub targets: TargetsConfig,

    /// 转存调度与 GC 运行时控制参数。
    pub transfer_config: TransferConfig,

    /// 工作流执行角色配置。
    pub workflow: WorkflowConfig,

    /// 运行期动态分配的 Client ID 映射集合。
    pub client_ids: RuntimeClientIds,

    /// User 与 Bot 各自完整的运行时 TDLib 参数与鉴权信息映射。
    pub runtime_clients: HashMap<ClientRole, RuntimeClientConfig>,
}

impl BotConfig {
    /// 从 JSON 文本解析运行时配置（支持包含 `//` 或 `/* */` 注释）。
    ///
    /// 这里集中处理 v1/v2 兼容，业务模块只使用运行时视图，避免命令层散落配置版本判断。
    pub fn from_json_str(text: &str) -> anyhow::Result<Self> {
        let stripped = strip_json_comments(text);
        if is_v2_config(&stripped)? {
            let config = serde_json::from_str::<BotConfigV2>(&stripped)?;
            return Self::from_v2(config);
        }

        anyhow::bail!(
            "config_version 2 is required because bot-only interaction needs explicit user/bot clients"
        )
    }

    /// 当前进程启动时必须创建哪些 TDLib client。
    ///
    /// 用户执行器由所有者在 Bot 内按需发起二维码登录，不能阻塞 Bot 的正常使用。
    pub fn required_client_roles(&self) -> Vec<ClientRole> {
        vec![ClientRole::Bot]
    }

    /// 返回某个角色的运行期 TDLib 配置。
    pub fn runtime_client(&self, role: ClientRole) -> anyhow::Result<&RuntimeClientConfig> {
        self.runtime_clients
            .get(&role)
            .ok_or_else(|| anyhow::anyhow!("runtime client not configured: {}", role.as_str()))
    }

    /// 获取交互 client id。
    pub fn interaction_client_id(&self) -> anyhow::Result<i32> {
        self.client_ids
            .get(ClientRole::Bot)
            .ok_or_else(|| anyhow::anyhow!("interaction client is not ready"))
    }

    /// 根据已就绪角色构造转存执行 client 组合。
    ///
    /// Bot 是唯一必需角色，承担默认读取、下载和上传；用户执行器仅在已经登录时
    /// 填入 `user`，供私有源或 Bot 权限不足时回退使用。
    pub fn transfer_client_ids_for_ready_roles(
        &self,
        ready_roles: &BTreeSet<ClientRole>,
    ) -> anyhow::Result<TransferClientIds> {
        let bot = ready_roles
            .contains(&ClientRole::Bot)
            .then_some(self.client_ids.bot)
            .flatten()
            .ok_or_else(|| anyhow::anyhow!("bot client is not ready"))?;
        let user = ready_roles
            .contains(&ClientRole::User)
            .then_some(self.client_ids.user)
            .flatten();

        Ok(TransferClientIds {
            interaction: bot,
            download: bot,
            upload: bot,
            user,
            bot: Some(bot),
        })
    }

    /// 获取当前已创建 client 的转存执行组合。
    ///
    /// 仅供启动兼容代码使用；处理消息时必须从 `TransferRuntimeState` 读取，避免
    /// 使用已经退出的执行器 client id。
    pub fn transfer_client_ids(&self) -> anyhow::Result<TransferClientIds> {
        let mut ready_roles = BTreeSet::new();
        if self.client_ids.bot.is_some() {
            ready_roles.insert(ClientRole::Bot);
        }
        if self.client_ids.user.is_some() {
            ready_roles.insert(ClientRole::User);
        }
        self.transfer_client_ids_for_ready_roles(&ready_roles)
    }

    /// 写入某个角色的 TDLib client id。
    pub fn set_client_id(&mut self, role: ClientRole, client_id: i32) {
        self.client_ids.set(role, client_id);
    }

    /// 判断所有 workflow 依赖的 client 是否已经 Ready。
    pub fn all_required_clients_ready(&self, ready_roles: &BTreeSet<ClientRole>) -> bool {
        self.required_client_roles()
            .into_iter()
            .all(|role| ready_roles.contains(&role))
    }

    /// 根据请求 chat 与发送者 user 判断是否允许交互。
    ///
    /// 项目明确只支持私聊 bot 交互，不处理群聊命令，避免多人共用一个 chat_id
    /// 时产生任务归属和菜单草稿混乱。
    pub fn request_actor(&self, request_chat_id: i64, sender_user_id: i64) -> Option<RequestActor> {
        (sender_user_id > 0
            && request_chat_id == sender_user_id
            && (sender_user_id == self.owner_user_id
                || self.admin_user_ids.contains(&sender_user_id)))
        .then_some(RequestActor {
            request_chat_id,
            user_id: sender_user_id,
        })
    }

    /// 从 v2 配置构造运行时视图。
    fn from_v2(config: BotConfigV2) -> anyhow::Result<Self> {
        config.validate()?;

        let owner_user_id = config.owner_user_id;
        if owner_user_id <= 0 {
            anyhow::bail!("owner_user_id must be positive");
        }
        let admin_user_ids = config.admin_user_ids.iter().copied().collect();

        let mut runtime_clients = HashMap::new();
        runtime_clients.insert(
            ClientRole::User,
            RuntimeClientConfig {
                role: ClientRole::User,
                tdlib_config: config
                    .clients
                    .user
                    .tdlib
                    .to_tdlib_config(&config.tdlib_defaults),
                login_info: config.clients.user.login_info.clone(),
                log_verbosity_level: config.tdlib_defaults.log_verbosity_level,
            },
        );
        runtime_clients.insert(
            ClientRole::Bot,
            RuntimeClientConfig {
                role: ClientRole::Bot,
                tdlib_config: config
                    .clients
                    .bot
                    .tdlib
                    .to_tdlib_config(&config.tdlib_defaults),
                login_info: LoginInfo::Token(config.clients.bot.token.clone()),
                log_verbosity_level: config.tdlib_defaults.log_verbosity_level,
            },
        );

        Ok(Self {
            owner_user_id,
            admin_user_ids,
            storage: config.storage,
            targets: config.targets,
            transfer_config: config.transfer_config,
            workflow: config.workflow,
            client_ids: RuntimeClientIds::default(),
            runtime_clients,
        })
    }
}

impl BotConfigV2 {
    /// 校验 v2 配置中的角色和目录关系。
    fn validate(&self) -> anyhow::Result<()> {
        if self.config_version != 2 {
            anyhow::bail!("unsupported config_version: {}", self.config_version);
        }

        if self.storage.database_url.trim().is_empty() {
            anyhow::bail!("storage.database_url cannot be empty");
        }

        if self.owner_user_id <= 0 {
            anyhow::bail!("owner_user_id must be positive");
        }
        if self.admin_user_ids.iter().any(|user_id| *user_id <= 0) {
            anyhow::bail!("admin_user_ids must contain only positive IDs");
        }

        if self.clients.bot.token.trim().is_empty() {
            anyhow::bail!("clients.bot.token is required");
        }
        if !looks_like_bot_token(self.clients.bot.token.trim()) {
            anyhow::bail!("clients.bot.token format is invalid");
        }

        if !matches!(self.clients.user.login_info, LoginInfo::Ocr) {
            anyhow::bail!(
                "clients.user.login_info only supports QR login; remove the field or use legacy OCR"
            );
        }

        if self.workflow.upload_client != ClientRole::Bot {
            anyhow::bail!(
                "workflow.upload_client only supports bot; remove the field or set it to bot"
            );
        }

        let user_db = self.clients.user.tdlib.database_directory.trim();
        let user_files = self.clients.user.tdlib.files_directory.trim();
        if user_db.is_empty() || user_files.is_empty() {
            anyhow::bail!("clients.user.tdlib directories cannot be empty");
        }

        let bot_db = self.clients.bot.tdlib.database_directory.trim();
        let bot_files = self.clients.bot.tdlib.files_directory.trim();
        if bot_db.is_empty() || bot_files.is_empty() {
            anyhow::bail!("clients.bot.tdlib directories cannot be empty");
        }
        if user_db == bot_db {
            anyhow::bail!("user and bot database_directory must be different");
        }
        if user_files == bot_files {
            anyhow::bail!("user and bot files_directory must be different");
        }

        Ok(())
    }
}

/// 初始化运行时配置文件路径。
pub fn init_runtime_config_path(path: impl Into<PathBuf>) {
    let _ = CONFIG_FILE_PATH.set(path.into());
}

/// 去除 JSON 文本中的单行注释（`//`）与多行注释（`/* ... */`），支持字符串内部包含斜杠等转义情况。
pub fn strip_json_comments(json: &str) -> String {
    let mut result = String::with_capacity(json.len());
    let mut chars = json.chars().peekable();
    let mut in_string = false;
    let mut escape = false;

    while let Some(ch) = chars.next() {
        if in_string {
            result.push(ch);
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == '"' {
                in_string = false;
            }
        } else if ch == '"' {
            in_string = true;
            result.push(ch);
        } else if ch == '/' {
            match chars.peek() {
                Some('/') => {
                    // 单行注释：跳过当前行直到换行符
                    chars.next();
                    for next_ch in chars.by_ref() {
                        if next_ch == '\n' {
                            result.push('\n');
                            break;
                        }
                    }
                }
                Some('*') => {
                    // 多行注释：跳过直到遇到 */
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            break;
                        }
                    }
                }
                _ => {
                    result.push(ch);
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// 判断配置文件是否是 v2 结构（自动剥离注释后校验）。
fn is_v2_config(text: &str) -> anyhow::Result<bool> {
    let stripped = strip_json_comments(text);
    let value = serde_json::from_str::<serde_json::Value>(&stripped)?;
    Ok(value
        .get("config_version")
        .and_then(|v| v.as_i64())
        .is_some_and(|version| version >= 2))
}

/// 默认后台转存任务并发数（默认值为 2）。
fn default_transfer_job_concurrency() -> usize {
    2
}

/// 默认本地文件延迟删除缓冲时间（默认值为 2 分钟）。
fn default_transfer_file_delete_delay_minutes() -> i64 {
    2
}

/// 默认垃圾文件定期回收（GC）检查循环间隔（默认值为 60 秒）。
fn default_transfer_file_gc_interval_seconds() -> u64 {
    60
}

/// 默认转存进度消息编辑刷新间隔时间（默认值为 2 秒）。
fn default_progress_edit_interval_seconds() -> u64 {
    2
}

/// 默认 `/downloads` 列表命令的分页条目数（默认值为 8 条）。
fn default_downloads_page_size() -> u64 {
    8
}

/// 默认交互式菜单等待用户输入操作的超时时间（默认值为 10 分钟）。
fn default_menu_input_timeout_seconds() -> u64 {
    10 * 60
}

/// 默认 TDLib 内核日志详细度输出级别（默认值为 1，仅警告与错误）。
fn default_tdlib_log_verbosity_level() -> i32 {
    1
}

/// 默认业务 SQLite 本地数据库连接 URL。
fn default_storage_database_url() -> String {
    "sqlite://tg/app/transfer.sqlite?mode=rwc".to_owned()
}

/// 粗略校验 BotFather token 格式。
///
/// 这里只检查公开结构 `<数字 bot id>:<token secret>`，不向 Telegram 校验真伪。
/// 目的是在启动前拦截明显的占位符或误填 user token，避免 TDLib 登录阶段长时间无反馈。
fn looks_like_bot_token(token: &str) -> bool {
    let Some((bot_id, secret)) = token.split_once(':') else {
        return false;
    };
    !bot_id.is_empty()
        && bot_id.bytes().all(|b| b.is_ascii_digit())
        && secret.len() >= 20
        && secret
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

/// 工作流的上传端客户端角色默认值为 Bot。
fn default_client_role_bot() -> ClientRole {
    ClientRole::Bot
}

#[cfg(test)]
mod tests {
    use super::*;

    // 旧版 v1 单 client 配置不能再启动。
    // 当前配置需要显式 user/bot client，继续兼容 v1 会隐藏配置错误。
    #[test]
    fn test_legacy_config_is_rejected() {
        let bot_config_str = r#"
        {
          "tdlib_config": {
            "use_test_dc": false,
            "database_directory": "tg/db",
            "files_directory": "tg/file",
            "database_encryption_key": "",
            "use_file_database": false,
            "use_chat_info_database": false,
            "use_message_database": false,
            "use_secret_chats": false,
            "api_id": 0,
            "api_hash": "",
            "system_language_code": "",
            "device_model": "",
            "system_version": "",
            "application_version": ""
          },
          "admin_ids": [],
          "target_map": {
            "1234": 1234
          },
          "transfer_config": {
            "job_concurrency": 2,
            "file_delete_delay_minutes": 2,
            "file_gc_interval_seconds": 60
          },
          "login_info": {
            "type": "PHONE",
            "data": "1234"
          }
        }"#;

        let err = BotConfig::from_json_str(bot_config_str).unwrap_err();

        assert!(err.to_string().contains("config_version 2 is required"));
    }

    // v2 配置启动时只创建 Bot；用户执行器由所有者在 Bot 内按需登录。
    #[test]
    fn test_v2_config_maps_to_runtime_clients() {
        let config = BotConfig::from_json_str(v2_config_text()).unwrap();

        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
        assert_eq!(config.required_client_roles(), vec![ClientRole::Bot]);
        assert!(matches!(
            config.runtime_client(ClientRole::Bot).unwrap().login_info,
            LoginInfo::Token(_)
        ));
        assert!(config.runtime_client(ClientRole::User).is_ok());
        assert!(config.runtime_client(ClientRole::Bot).is_ok());
        assert_eq!(
            config.storage.database_url,
            "sqlite://tg/app/transfer.sqlite?mode=rwc"
        );
        assert!(config.targets.is_empty());
        assert!(config.request_actor(123, 1).is_none());
        assert!(config.request_actor(1, 1).is_some());
        assert!(config.request_actor(999, 1).is_none());
        assert!(config.request_actor(2, 2).is_none());
        assert!(config.request_actor(3, 3).is_none());
    }

    // 新配置不再需要声明 user 登录方式；执行器固定由 Bot 触发 QR 登录。
    #[test]
    fn test_v2_user_login_info_is_optional_and_defaults_to_qr() {
        let config_text = v2_config_text().replace(
            "              \"login_info\": {\n                \"type\": \"OCR\"\n              },\n",
            "",
        );

        let config = BotConfig::from_json_str(&config_text).unwrap();

        assert!(matches!(
            config.runtime_client(ClientRole::User).unwrap().login_info,
            LoginInfo::Ocr
        ));
    }

    #[test]
    fn test_v2_omits_legacy_executor_login_and_upload_selection() {
        let config_text = v2_config_text()
            .replace(
                "              \"login_info\": {\n                \"type\": \"OCR\"\n              },\n",
                "",
            )
            .replace(
                ",\n          \"workflow\": {\n            \"upload_client\": \"bot\"\n          }",
                "",
            );

        let config = BotConfig::from_json_str(&config_text).unwrap();

        assert!(matches!(
            config.runtime_client(ClientRole::User).unwrap().login_info,
            LoginInfo::Ocr
        ));
        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
    }

    #[test]
    fn test_v2_phone_login_is_rejected_with_qr_migration_hint() {
        let config_text = v2_config_text().replace(
            "\"type\": \"OCR\"",
            "\"type\": \"PHONE\", \"data\": \"+8613800000000\"",
        );

        let err = BotConfig::from_json_str(&config_text).unwrap_err();

        assert!(err.to_string().contains("QR"));
        assert!(err.to_string().contains("clients.user.login_info"));
    }

    #[test]
    fn test_v2_owner_and_admin_user_ids_are_private_actors() {
        let config_text = v2_config_text().replace(
            "\"owner_user_id\": 1",
            "\"owner_user_id\": 1,\n          \"admin_user_ids\": [2, 3]",
        );
        let config = BotConfig::from_json_str(&config_text).unwrap();

        assert_eq!(config.owner_user_id, 1);
        assert!(config.request_actor(1, 1).is_some());
        assert!(config.request_actor(2, 2).is_some());
        assert!(config.request_actor(3, 3).is_some());
        assert!(config.request_actor(4, 4).is_none());
        assert!(config.request_actor(999, 1).is_none());
    }

    #[test]
    fn test_default_config_does_not_authorize_zero_actor() {
        assert!(BotConfig::default().request_actor(0, 0).is_none());
    }

    #[test]
    fn test_v2_rejects_missing_owner_user_id() {
        let config_text = v2_config_text().replace("\"owner_user_id\": 1", "\"owner_user_id\": 0");

        let err = BotConfig::from_json_str(&config_text).unwrap_err();

        assert!(err.to_string().contains("owner_user_id must be positive"));
    }

    #[test]
    fn test_v2_rejects_non_positive_admin_user_ids() {
        let config_text = v2_config_text().replace(
            "\"owner_user_id\": 1",
            "\"owner_user_id\": 1,\n          \"admin_user_ids\": [2, 0]",
        );

        let err = BotConfig::from_json_str(&config_text).unwrap_err();

        assert!(
            err.to_string()
                .contains("admin_user_ids must contain only positive IDs")
        );
    }

    // 文件配置只保留启动级字段时仍可启动；targets / transfer_config 后续由数据库运行态接管。
    #[test]
    fn test_v2_config_accepts_database_owned_runtime_defaults() {
        let config = BotConfig::from_json_str(v2_config_text()).unwrap();

        assert_eq!(config.owner_user_id, 1);
        assert!(config.targets.is_empty());
        assert_eq!(
            config.transfer_config.job_concurrency,
            default_transfer_job_concurrency()
        );
    }

    // 上传由 Bot 首选执行，旧的 user 上传配置必须显式迁移，不能静默改变历史部署。
    #[test]
    fn test_v2_rejects_legacy_user_upload_client() {
        let err = BotConfig::from_json_str(
            &v2_config_text().replace("\"upload_client\": \"bot\"", "\"upload_client\": \"user\""),
        )
        .unwrap_err();

        assert!(err.to_string().contains("workflow.upload_client"));
        assert!(err.to_string().contains("bot"));
    }

    // bot 默认负责上传和源读取；user 登录完成后才作为链接源 fallback client。
    // 查重维度保持 source_link + target_chat_id，不因为上传者变化而分裂历史结果。
    #[test]
    fn test_v2_supports_bot_source_with_bot_upload() {
        let config = BotConfig::from_json_str(v2_config_text()).unwrap();
        let ids = {
            let mut config = config.clone();
            config.set_client_id(ClientRole::User, 10);
            config.set_client_id(ClientRole::Bot, 20);
            config.transfer_client_ids().unwrap()
        };

        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
        assert_eq!(ids.interaction, 20);
        assert_eq!(ids.download, 20);
        assert_eq!(ids.upload, 20);
    }

    // Bot 登录完成后必须能够独立受理任务；用户执行器尚未登录时只是不提供回退能力。
    #[test]
    fn test_bot_ready_builds_bot_only_transfer_clients() {
        let mut config = BotConfig::from_json_str(v2_config_text()).unwrap();
        config.set_client_id(ClientRole::User, 10);
        config.set_client_id(ClientRole::Bot, 20);
        let ready_roles = BTreeSet::from([ClientRole::Bot]);

        let ids = config
            .transfer_client_ids_for_ready_roles(&ready_roles)
            .unwrap();

        assert_eq!(ids.interaction, 20);
        assert_eq!(ids.download, 20);
        assert_eq!(ids.upload, 20);
        assert_eq!(ids.bot, Some(20));
        assert_eq!(ids.user, None);
    }

    // 运行时默认 workflow 也保持 bot-first + bot-upload，避免测试或兜底构造误用 user 上传。
    #[test]
    fn test_workflow_default_uses_bot_for_upload() {
        let workflow = WorkflowConfig::default();

        assert_eq!(workflow.upload_client, ClientRole::Bot);
    }

    // 新模板只保留真正需要选择的 upload_client。
    #[test]
    fn test_v2_accepts_simplified_workflow_and_fixed_defaults() {
        let config = BotConfig::from_json_str(v2_config_text()).unwrap();

        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
        assert!(config.runtime_client(ClientRole::Bot).is_ok());
    }

    // 业务数据库连接串不能为空，避免运行时才发现 SeaORM 无法连接。
    #[test]
    fn test_v2_rejects_empty_storage_database_url() {
        let err = BotConfig::from_json_str(&v2_config_text().replace(
            "\"database_url\": \"sqlite://tg/app/transfer.sqlite?mode=rwc\"",
            "\"database_url\": \"\"",
        ))
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("storage.database_url cannot be empty")
        );
    }

    // bot token 明显不是 BotFather 格式时应在配置阶段失败，避免 TDLib 登录阶段无明确反馈。
    #[test]
    fn test_v2_rejects_invalid_bot_token_shape() {
        let err = BotConfig::from_json_str(&v2_config_text().replace(
            "\"token\": \"123456789:abcdefghijklmnopqrstuvwxyzABCDEF\"",
            "\"token\": \"bot-token\"",
        ))
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("clients.bot.token format is invalid")
        );
    }

    // 旧配置键兼容：避免用户本地 config.json 还没改名时启动失败。
    #[test]
    fn test_transfer_config_accepts_old_delay_key_as_minutes() {
        let text = r#"{
            "job_concurrency": 2,
            "file_delete_delay_hours": 5,
            "file_gc_interval_seconds": 60
        }"#;

        let cfg: TransferConfig = serde_json::from_str(text).unwrap();
        assert_eq!(cfg.file_delete_delay_minutes, 5);
        let serialized = serde_json::to_string(&cfg).unwrap();
        assert!(serialized.contains("file_delete_delay_minutes"));
        assert!(!serialized.contains("file_delete_delay_hours"));
    }

    /// 验证支持解析带有 `//` 和 `/* */` 注释的 JSON 配置文件。
    #[test]
    fn test_v2_config_accepts_comments() {
        let text_with_comments = r#"
        // 顶层配置文件
        {
          /* 配置文件协议版本 */
          "config_version": 2,
          "owner_user_id": 1, // 超管用户 ID
          "tdlib_defaults": {
            "use_test_dc": false,
            "api_id": 1,
            "api_hash": "hash//with-slashes",
            "system_language_code": "zh-hans",
            "device_model": "tg_transfer_bot",
            "system_version": "1.8.62",
            "application_version": "0.0.1",
            "use_secret_chats": false,
            "log_verbosity_level": 1
          },
          "storage": {
            // 数据库连接串
            "database_url": "sqlite://tg/app/transfer.sqlite?mode=rwc"
          },
          "clients": {
            "user": {
              "login_info": {
                "type": "OCR"
              },
              "tdlib": {
                "database_directory": "tg/user/db",
                "files_directory": "tg/user/files",
                "database_encryption_key": "user-key",
                "use_file_database": true,
                "use_chat_info_database": true,
                "use_message_database": true
              }
            },
            "bot": {
              "token": "123456789:abcdefghijklmnopqrstuvwxyzABCDEF",
              "tdlib": {
                "database_directory": "tg/bot/db",
                "files_directory": "tg/bot/files",
                "database_encryption_key": "bot-key",
                "use_file_database": true,
                "use_chat_info_database": true,
                "use_message_database": true
              }
            }
          },
          "workflow": {
            "upload_client": "bot"
          }
        }"#;

        let config = BotConfig::from_json_str(text_with_comments).expect("must parse with comments");
        assert_eq!(config.owner_user_id, 1);
        assert_eq!(
            config.runtime_client(ClientRole::Bot).unwrap().tdlib_config.api_hash,
            "hash//with-slashes"
        );
    }

    /// 验证仓库根目录的 `config.example.json` 模板能够被正确解析且符合规范（不带注释的标准 JSON）。
    #[test]
    fn test_config_example_json_is_valid() {
        let example_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("config.example.json");
        let content = std::fs::read_to_string(example_path).expect("read config.example.json");
        // 确保纯原生 serde_json 也能直接解析（无任何注释）
        let _raw: serde_json::Value = serde_json::from_str(&content).expect("raw serde_json must parse config.example.json");
        let config = BotConfig::from_json_str(&content).expect("config.example.json must parse successfully");
        assert_eq!(config.owner_user_id, 123456789);
        assert_eq!(config.admin_user_ids.len(), 0);
        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
    }

    /// 验证仓库根目录的 `config.example.jsonc` 模板能够被正确解析且符合规范（带有详细注释）。
    #[test]
    fn test_config_example_jsonc_is_valid() {
        let example_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("config.example.jsonc");
        let content = std::fs::read_to_string(example_path).expect("read config.example.jsonc");
        let config = BotConfig::from_json_str(&content).expect("config.example.jsonc must parse successfully");
        assert_eq!(config.owner_user_id, 123456789);
        assert_eq!(config.admin_user_ids.len(), 0);
        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
    }

    fn v2_config_text() -> &'static str {
        r#"
        {
          "config_version": 2,
          "tdlib_defaults": {
            "use_test_dc": false,
            "api_id": 1,
            "api_hash": "hash",
            "system_language_code": "zh-hans",
            "device_model": "tg_transfer_bot",
            "system_version": "1.8.62",
            "application_version": "0.0.1",
            "use_secret_chats": false,
            "log_verbosity_level": 1
          },
          "storage": {
            "database_url": "sqlite://tg/app/transfer.sqlite?mode=rwc"
          },
          "clients": {
            "user": {
              "login_info": {
                "type": "OCR"
              },
              "tdlib": {
                "database_directory": "tg/user/db",
                "files_directory": "tg/user/files",
                "database_encryption_key": "user-key",
                "use_file_database": true,
                "use_chat_info_database": true,
                "use_message_database": true
              }
            },
            "bot": {
              "token": "123456789:abcdefghijklmnopqrstuvwxyzABCDEF",
              "tdlib": {
                "database_directory": "tg/bot/db",
                "files_directory": "tg/bot/files",
                "database_encryption_key": "bot-key",
                "use_file_database": true,
                "use_chat_info_database": true,
                "use_message_database": true
              }
            }
          },
          "workflow": {
            "upload_client": "bot"
          },
          "owner_user_id": 1
        }"#
    }
}
