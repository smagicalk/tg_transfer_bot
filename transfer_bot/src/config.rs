// 配置模型定义：
// 负责 JSON <-> Rust 结构体映射。
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

// 运行时配置文件路径：
// - 主程序启动时写入
// - `/config` 命令读取/保存配置时复用
static CONFIG_FILE_PATH: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

// TDLib client 角色。
// bot 固定负责命令、卡片和按钮交互；user 负责读取源消息和下载兜底。
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ClientRole {
    User,
    Bot,
}

impl ClientRole {
    /// 角色名用于日志，不包含任何敏感信息。
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Bot => "bot",
        }
    }
}

// TDLib 参数配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct TdlibConfig {
    pub use_test_dc: bool,
    pub database_directory: String,
    pub files_directory: String,
    pub database_encryption_key: String,
    pub use_file_database: bool,
    pub use_chat_info_database: bool,
    pub use_message_database: bool,
    pub use_secret_chats: bool,
    pub api_id: i32,
    pub api_hash: String,
    pub system_language_code: String,
    pub device_model: String,
    pub system_version: String,
    pub application_version: String,
}

// 转存运行时配置。
// 这类参数原先散落在环境变量里，现在统一放进 config.json，便于修改和持久化。
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct TransferConfig {
    // 后台转存重任务并发数。
    #[serde(default = "default_transfer_job_concurrency")]
    pub job_concurrency: usize,

    // 文件引用归零后的延迟删除分钟数。
    // `alias` 兼容旧配置键，旧值会按“分钟”解释，保存后会写成新键。
    #[serde(
        default = "default_transfer_file_delete_delay_minutes",
        alias = "file_delete_delay_hours"
    )]
    pub file_delete_delay_minutes: i64,

    // 文件 GC 扫描间隔（秒）。
    #[serde(default = "default_transfer_file_gc_interval_seconds")]
    pub file_gc_interval_seconds: u64,

    // 进度消息编辑间隔（秒）。
    // 默认值与旧代码常量保持一致，后续可通过 `/cfg` 开放动态修改。
    #[serde(default = "default_progress_edit_interval_seconds")]
    pub progress_edit_interval_seconds: u64,

    // 下载列表默认分页大小。
    #[serde(default = "default_downloads_page_size")]
    pub downloads_default_page_size: u64,

    // 菜单等待用户输入的超时时间（秒）。
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

// 登录方式配置。
// JSON 示例：{ "type": "PHONE", "data": "..." }
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(
    rename = "login_info",
    rename_all = "UPPERCASE",
    tag = "type",
    content = "data"
)]
pub enum LoginInfo {
    // 手机号登录，序列化后仍保持配置文件中的 PHONE。
    Phone(String),
    // Bot token 登录，序列化后仍保持配置文件中的 TOKEN。
    Token(String),
    #[default]
    // 交互式 OCR 登录，序列化后仍保持配置文件中的 OCR。
    Ocr,
}

// TDLib 公共默认参数。
// v2 配置把“公共参数”和“客户端本地目录”拆开，避免 bot/user 误用同一个 TDLib 目录。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct TdlibDefaults {
    pub use_test_dc: bool,
    pub api_id: i32,
    pub api_hash: String,
    pub system_language_code: String,
    pub device_model: String,
    pub system_version: String,
    pub application_version: String,
    pub use_secret_chats: bool,
    #[serde(default = "default_tdlib_log_verbosity_level")]
    pub log_verbosity_level: i32,
}

// 机器人自身的本地存储配置。
// TDLib 的 database_directory 只属于 Telegram client；转存任务、文件引用和恢复状态使用这里的 SQLite。
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StorageConfig {
    // SeaORM/SQLx 使用的数据库连接串。
    // 当前推荐 SQLite 文件：sqlite://tg/app/transfer.sqlite?mode=rwc
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

// 单个 TDLib client 独有的本地配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct ClientTdlibConfig {
    pub database_directory: String,
    pub files_directory: String,
    pub database_encryption_key: String,
    pub use_file_database: bool,
    pub use_chat_info_database: bool,
    pub use_message_database: bool,
}

impl ClientTdlibConfig {
    /// 把 v2 的公共参数 + client 本地参数合成为 TDLib 启动参数。
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

// 用户号 client 配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct UserClientConfig {
    pub login_info: LoginInfo,
    pub tdlib: ClientTdlibConfig,
}

// bot client 配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct BotClientConfig {
    // bot 是当前交互模式的必需 client；该开关只保留给旧配置兼容。
    // 新配置省略时按启用处理，避免模板里出现一个不能实际关闭的误导项。
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub enabled: bool,
    pub token: String,
    pub tdlib: ClientTdlibConfig,
}

// v2 固定 client 集合。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct ClientsConfig {
    pub user: UserClientConfig,
    pub bot: BotClientConfig,
}

// 业务流程角色配置。
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct WorkflowConfig {
    // 交互端固定为 bot；保留字段是为了兼容旧配置和内部运行时视图。
    #[serde(
        default = "default_client_role_bot",
        skip_serializing_if = "is_bot_role"
    )]
    pub interaction_client: ClientRole,
    // 链接源真实策略是 bot-first + user fallback；该字段只作为旧配置兼容下载端。
    #[serde(
        default = "default_client_role_bot",
        skip_serializing_if = "is_bot_role"
    )]
    pub download_client: ClientRole,
    // 上传端是当前 workflow 中唯一需要用户主动选择的角色。
    #[serde(default = "default_client_role_bot")]
    pub upload_client: ClientRole,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            interaction_client: ClientRole::Bot,
            download_client: ClientRole::Bot,
            upload_client: ClientRole::Bot,
        }
    }
}

// 重复转存策略。
// 当前数据库查重固定使用 source_link + target_chat_id，不把 uploader 放进唯一语义。
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct DeduplicateConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub return_running_job: bool,
    #[serde(default = "default_true")]
    pub return_finished_result: bool,
}

impl Default for DeduplicateConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            return_running_job: true,
            return_finished_result: true,
        }
    }
}

impl DeduplicateConfig {
    /// 当前查重策略固定为 source_link + target_chat_id。
    /// 配置写回时隐藏固定值，避免用户误以为可以通过配置切换查重语义。
    fn is_fixed(&self) -> bool {
        self.enabled && self.return_running_job && self.return_finished_result
    }
}

// 交互用户角色。
// admin 不消耗积分且可管理全局；user 只能查看和控制自己的任务。
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActorRole {
    Admin,
    User,
}

impl ActorRole {
    /// 数据库存储值与日志字段统一使用小写英文。
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::User => "user",
        }
    }

    /// 普通用户权限判断的反向辅助。
    pub fn is_admin(self) -> bool {
        self == Self::Admin
    }
}

// 一次 bot 交互的身份上下文。
// chat_id 表示这条命令发到哪里；user_id 表示真正点击按钮或发送命令的人。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestActor {
    pub request_chat_id: i64,
    pub user_id: i64,
    pub role: ActorRole,
}

impl RequestActor {
    /// 是否管理员。
    pub fn is_admin(self) -> bool {
        self.role.is_admin()
    }

    /// 任务可见范围：None 表示 admin 全局可见，Some 表示普通用户只能看自己的任务。
    pub fn owner_scope(self) -> Option<i64> {
        if self.is_admin() {
            None
        } else {
            Some(self.user_id)
        }
    }
}

// 积分计费配置。
// 第一版按任务和条目数计费，不按文件大小计费，避免下载前无法准确估算成本。
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct BillingConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_billing_base_cost_points")]
    pub base_cost_points: i64,
    #[serde(default = "default_billing_item_cost_points")]
    pub item_cost_points: i64,
    #[serde(default)]
    pub initial_user_points: i64,
}

impl Default for BillingConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            base_cost_points: default_billing_base_cost_points(),
            item_cost_points: default_billing_item_cost_points(),
            initial_user_points: 0,
        }
    }
}

impl BillingConfig {
    /// 根据抓取到的消息数量计算本次转存成本。
    pub fn cost_for_items(&self, item_count: usize) -> i64 {
        if !self.enabled {
            return 0;
        }
        let item_count = i64::try_from(item_count).unwrap_or(i64::MAX);
        self.base_cost_points
            .saturating_add(self.item_cost_points.saturating_mul(item_count))
            .max(0)
    }
}

// 访问控制配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccessControlConfig {
    pub admin_user_ids: Vec<i64>,
    #[serde(default)]
    pub allowed_user_ids: Vec<i64>,
    #[serde(default)]
    pub allow_all_private_users: bool,
    #[serde(default)]
    pub banned_user_ids: Vec<i64>,
    #[serde(default)]
    pub allowed_request_chat_ids: Vec<i64>,
    #[serde(default)]
    pub allowed_target_chat_ids: Vec<i64>,
}

impl AccessControlConfig {
    /// 兼容旧代码的 admin_ids 判断：同时包含“允许发命令的 chat”和“管理员用户”。
    fn merged_admin_ids(&self) -> Vec<i64> {
        let mut ids = BTreeSet::new();
        for id in &self.admin_user_ids {
            ids.insert(*id);
        }
        for id in &self.allowed_request_chat_ids {
            ids.insert(*id);
        }
        ids.into_iter().collect()
    }
}

// 默认目标配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct TargetsConfig {
    pub default_chat_id: i64,
    #[serde(default)]
    pub by_request_chat_id: HashMap<i64, i64>,
    #[serde(default)]
    pub aliases: HashMap<String, i64>,
}

impl TargetsConfig {
    /// 兼容旧命令层 target_map：request_chat_id -> target_chat_id，0 表示全局兜底。
    fn to_target_map(&self) -> HashMap<i64, i64> {
        let mut target_map = self.by_request_chat_id.clone();
        if self.default_chat_id != 0 {
            target_map.entry(0).or_insert(self.default_chat_id);
        }
        target_map
    }
}

// v2 原始配置。
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct BotConfigV2 {
    pub config_version: i32,
    pub tdlib_defaults: TdlibDefaults,
    #[serde(default)]
    pub storage: StorageConfig,
    pub clients: ClientsConfig,
    #[serde(default)]
    pub workflow: WorkflowConfig,
    #[serde(default, skip_serializing_if = "DeduplicateConfig::is_fixed")]
    pub deduplicate: DeduplicateConfig,
    pub access_control: AccessControlConfig,
    pub targets: TargetsConfig,
    #[serde(default)]
    pub billing: BillingConfig,
    #[serde(default)]
    pub transfer_config: TransferConfig,
}

// 运行期单 client 配置。
#[derive(Debug, Clone)]
pub struct RuntimeClientConfig {
    pub role: ClientRole,
    pub tdlib_config: TdlibConfig,
    pub login_info: LoginInfo,
    pub log_verbosity_level: i32,
}

// 运行期 TDLib client id 集合。
#[derive(Debug, Clone, Default)]
pub struct RuntimeClientIds {
    pub user: Option<i32>,
    pub bot: Option<i32>,
}

impl RuntimeClientIds {
    /// 写入角色对应的 TDLib client id。
    pub fn set(&mut self, role: ClientRole, client_id: i32) {
        match role {
            ClientRole::User => self.user = Some(client_id),
            ClientRole::Bot => self.bot = Some(client_id),
        }
    }

    /// 按角色读取 TDLib client id。
    pub fn get(&self, role: ClientRole) -> Option<i32> {
        match role {
            ClientRole::User => self.user,
            ClientRole::Bot => self.bot,
        }
    }

    /// 反查 TDLib client id 对应的角色。
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

// 转存执行需要的 client id。
//
// `download` 是旧 workflow 字段对应的兼容 client；真实源读取/下载会跟随每个任务的
// `source_client_role`，可通过 `get(ClientRole)` 取得实际 client id。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferClientIds {
    pub interaction: i32,
    pub download: i32,
    pub upload: i32,
    pub user: Option<i32>,
    pub bot: Option<i32>,
}

impl TransferClientIds {
    /// 按角色获取 TDLib client id。
    pub fn get(self, role: ClientRole) -> anyhow::Result<i32> {
        match role {
            ClientRole::User => self.user,
            ClientRole::Bot => self.bot,
        }
        .ok_or_else(|| anyhow::anyhow!("{} client is not ready", role.as_str()))
    }
}

// 机器人运行时配置。
// 这里是业务代码读取的“视图”，不再要求和 config.json 结构一一对应。
#[derive(Debug, Clone)]
pub struct BotConfig {
    // 机器人业务数据库配置。
    pub storage: StorageConfig,

    // 兼容旧代码的默认 TDLib 配置，取 interaction client。
    pub tdlib_config: TdlibConfig,

    // 兼容旧代码的管理员 chat/user id 白名单。
    pub admin_ids: Vec<i64>,

    // 明确的管理员用户 ID；权限判断以 sender_user_id 为准，不再只看 chat_id。
    pub admin_user_ids: Vec<i64>,

    // 允许管理员在其中发命令的 chat ID。
    // 普通用户只允许私聊，避免群聊里多人共享同一个 request_chat_id。
    pub allowed_request_chat_ids: Vec<i64>,

    // 允许作为普通用户使用 bot 的用户 ID。
    pub allowed_user_ids: Vec<i64>,

    // 是否允许任意私聊用户作为普通用户使用。
    pub allow_all_private_users: bool,

    // 被禁止使用 bot 的用户 ID。
    pub banned_user_ids: Vec<i64>,

    // 兼容旧代码的默认 client id，取 interaction client。
    pub client_id: Option<i32>,

    // 默认转存目标映射：request_chat_id -> target_chat_id。
    // 可使用 key=0 作为兜底目标。
    pub target_map: HashMap<i64, i64>,

    // 目标 chat 别名：命令里可以用 `/t <link> archive` 替代数字 chat_id。
    pub target_aliases: HashMap<String, i64>,

    // 允许作为转存目标的 chat 白名单。
    // 空数组表示不限制；非空时显式参数、默认目标和别名都必须命中白名单。
    pub allowed_target_chat_ids: Vec<i64>,

    // 转存相关运行参数。
    pub transfer_config: TransferConfig,

    // 普通用户积分计费参数。
    pub billing: BillingConfig,

    // 兼容旧代码的默认登录方式，取 interaction client。
    pub login_info: LoginInfo,

    // v2 原始配置中提取出的工作流角色。
    pub workflow: WorkflowConfig,

    // 运行期 client id 集合。
    pub client_ids: RuntimeClientIds,

    // user/bot 的运行期 TDLib 配置。
    pub runtime_clients: HashMap<ClientRole, RuntimeClientConfig>,

    // TDLib 日志级别。
    pub log_verbosity_level: i32,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            tdlib_config: TdlibConfig::default(),
            storage: StorageConfig::default(),
            admin_ids: Vec::new(),
            admin_user_ids: Vec::new(),
            allowed_request_chat_ids: Vec::new(),
            allowed_user_ids: Vec::new(),
            allow_all_private_users: false,
            banned_user_ids: Vec::new(),
            client_id: None,
            target_map: HashMap::new(),
            target_aliases: HashMap::new(),
            allowed_target_chat_ids: Vec::new(),
            transfer_config: TransferConfig::default(),
            billing: BillingConfig::default(),
            login_info: LoginInfo::default(),
            workflow: WorkflowConfig::default(),
            client_ids: RuntimeClientIds::default(),
            runtime_clients: HashMap::new(),
            log_verbosity_level: default_tdlib_log_verbosity_level(),
        }
    }
}

impl BotConfig {
    /// 从 JSON 文本解析运行时配置。
    ///
    /// 这里集中处理 v1/v2 兼容，业务模块只使用运行时视图，避免命令层散落配置版本判断。
    pub fn from_json_str(text: &str) -> anyhow::Result<Self> {
        if is_v2_config(text)? {
            let config = serde_json::from_str::<BotConfigV2>(text)?;
            return Self::from_v2(config);
        }

        anyhow::bail!(
            "config_version 2 is required because bot-only interaction needs explicit user/bot clients"
        )
    }

    /// 当前进程需要启动哪些 TDLib client。
    pub fn required_client_roles(&self) -> Vec<ClientRole> {
        let mut roles = BTreeSet::new();
        // 链接源采用 bot-first + user fallback；user 始终作为私有源链接的兜底读取/下载端启动。
        // download_client 是旧配置兼容字段，真实下载端会跟随每个 job 的 source_client_role。
        roles.insert(ClientRole::User);
        roles.insert(self.workflow.interaction_client);
        roles.insert(self.workflow.download_client);
        roles.insert(self.workflow.upload_client);
        roles.into_iter().collect()
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
            .get(self.workflow.interaction_client)
            .ok_or_else(|| anyhow::anyhow!("interaction client is not ready"))
    }

    /// 获取转存执行链需要的 client id。
    pub fn transfer_client_ids(&self) -> anyhow::Result<TransferClientIds> {
        Ok(TransferClientIds {
            interaction: self.interaction_client_id()?,
            download: self
                .client_ids
                .get(self.workflow.download_client)
                .ok_or_else(|| anyhow::anyhow!("download client is not ready"))?,
            upload: self
                .client_ids
                .get(self.workflow.upload_client)
                .ok_or_else(|| anyhow::anyhow!("upload client is not ready"))?,
            user: self.client_ids.user,
            bot: self.client_ids.bot,
        })
    }

    /// 写入某个角色的 TDLib client id，并同步兼容字段。
    pub fn set_client_id(&mut self, role: ClientRole, client_id: i32) {
        self.client_ids.set(role, client_id);
        if role == self.workflow.interaction_client {
            self.client_id = Some(client_id);
            if let Some(runtime) = self.runtime_clients.get(&role) {
                self.tdlib_config = runtime.tdlib_config.clone();
                self.login_info = runtime.login_info.clone();
                self.log_verbosity_level = runtime.log_verbosity_level;
            }
        }
    }

    /// 判断所有 workflow 依赖的 client 是否已经 Ready。
    pub fn all_required_clients_ready(&self, ready_roles: &BTreeSet<ClientRole>) -> bool {
        self.required_client_roles()
            .into_iter()
            .all(|role| ready_roles.contains(&role))
    }

    /// 是否支持 TDLib inline keyboard/callback。
    pub fn supports_reply_markup(&self) -> bool {
        self.workflow.interaction_client == ClientRole::Bot
    }

    /// 根据请求 chat 与发送者 user 判断是否允许交互。
    ///
    /// 普通用户第一版只支持私聊 bot，避免群聊里多人共用同一个 chat_id 时产生任务归属混乱。
    /// admin 可在配置允许的 request chat 中管理全局任务。
    pub fn request_actor(&self, request_chat_id: i64, sender_user_id: i64) -> Option<RequestActor> {
        if self.banned_user_ids.contains(&sender_user_id) {
            return None;
        }

        if self.admin_user_ids.contains(&sender_user_id) {
            if self.admin_request_chat_allowed(request_chat_id, sender_user_id) {
                return Some(RequestActor {
                    request_chat_id,
                    user_id: sender_user_id,
                    role: ActorRole::Admin,
                });
            }
            return None;
        }

        if self.normal_user_request_allowed(request_chat_id, sender_user_id) {
            return Some(RequestActor {
                request_chat_id,
                user_id: sender_user_id,
                role: ActorRole::User,
            });
        }

        None
    }

    /// admin 可在私聊或显式允许的请求 chat 中操作。
    fn admin_request_chat_allowed(&self, request_chat_id: i64, sender_user_id: i64) -> bool {
        request_chat_id == sender_user_id
            || self.allowed_request_chat_ids.contains(&request_chat_id)
    }

    /// 普通用户只允许私聊，且必须在白名单中或开启 allow_all_private_users。
    fn normal_user_request_allowed(&self, request_chat_id: i64, sender_user_id: i64) -> bool {
        request_chat_id == sender_user_id
            && (self.allow_all_private_users || self.allowed_user_ids.contains(&sender_user_id))
    }

    /// 从 v2 配置构造运行时视图。
    fn from_v2(config: BotConfigV2) -> anyhow::Result<Self> {
        config.validate()?;

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
        if config.clients.bot.enabled {
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
        }

        let interaction_runtime = runtime_clients
            .get(&config.workflow.interaction_client)
            .ok_or_else(|| anyhow::anyhow!("interaction client is not configured"))?
            .clone();

        Ok(Self {
            tdlib_config: interaction_runtime.tdlib_config.clone(),
            storage: config.storage,
            admin_ids: config.access_control.merged_admin_ids(),
            admin_user_ids: config.access_control.admin_user_ids,
            allowed_request_chat_ids: config.access_control.allowed_request_chat_ids,
            allowed_user_ids: config.access_control.allowed_user_ids,
            allow_all_private_users: config.access_control.allow_all_private_users,
            banned_user_ids: config.access_control.banned_user_ids,
            client_id: None,
            target_map: config.targets.to_target_map(),
            target_aliases: config.targets.aliases,
            allowed_target_chat_ids: config.access_control.allowed_target_chat_ids,
            transfer_config: config.transfer_config,
            billing: config.billing,
            login_info: interaction_runtime.login_info.clone(),
            workflow: config.workflow,
            client_ids: RuntimeClientIds::default(),
            runtime_clients,
            log_verbosity_level: interaction_runtime.log_verbosity_level,
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

        if self.billing.base_cost_points < 0 || self.billing.item_cost_points < 0 {
            anyhow::bail!("billing cost points cannot be negative");
        }

        // 交互链路依赖 bot-only 的 inline keyboard、callback 和 copy-text 按钮。
        // 这里提前拒绝 user 交互，避免启动后菜单卡片没有按钮、callback 永远进不来。
        if self.workflow.interaction_client != ClientRole::Bot {
            anyhow::bail!(
                "workflow.interaction_client must be bot because Telegram interactive cards require bot reply markup"
            );
        }

        // 当前查重语义固定为 source_link + target_chat_id：
        // 上传者只记录实际执行者，不参与“是否同一个转存”的判断。
        if !self.deduplicate.enabled
            || !self.deduplicate.return_running_job
            || !self.deduplicate.return_finished_result
        {
            anyhow::bail!("deduplicate options are fixed to true for source_link + target_chat_id");
        }

        for role in [
            self.workflow.interaction_client,
            self.workflow.download_client,
            self.workflow.upload_client,
        ] {
            if role == ClientRole::Bot && !self.clients.bot.enabled {
                anyhow::bail!("workflow requires bot client but clients.bot.enabled is false");
            }
        }

        if self.clients.bot.enabled && self.clients.bot.token.trim().is_empty() {
            anyhow::bail!("clients.bot.token is required when bot is enabled");
        }
        if self.clients.bot.enabled && !looks_like_bot_token(self.clients.bot.token.trim()) {
            anyhow::bail!("clients.bot.token format is invalid");
        }

        let user_db = self.clients.user.tdlib.database_directory.trim();
        let user_files = self.clients.user.tdlib.files_directory.trim();
        if user_db.is_empty() || user_files.is_empty() {
            anyhow::bail!("clients.user.tdlib directories cannot be empty");
        }

        if self.clients.bot.enabled {
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
        }

        Ok(())
    }
}

// 旧版 config.json 结构。
// 启动配置已要求 v2；这里仅用于 `/cfg` 写回旧 transfer_config 片段时保持兼容。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
struct LegacyBotConfig {
    #[serde(default)]
    pub storage: StorageConfig,
    pub tdlib_config: TdlibConfig,
    pub admin_ids: Vec<i64>,
    #[serde(default)]
    pub target_map: HashMap<i64, i64>,
    #[serde(default)]
    pub transfer_config: TransferConfig,
    pub login_info: LoginInfo,
}

/// 初始化运行时配置文件路径。
pub fn init_runtime_config_path(path: impl Into<PathBuf>) {
    let _ = CONFIG_FILE_PATH.set(path.into());
}

/// 加载当前运行所使用的配置文件。
/// 仅支持明文 JSON 配置，不支持直接改写加密配置文件。
pub async fn load_runtime_bot_config() -> anyhow::Result<BotConfig> {
    let path = runtime_config_path()?;
    validate_runtime_config_path(path)?;
    let text = tokio::fs::read_to_string(path).await?;
    BotConfig::from_json_str(&text)
}

/// 保存当前运行所使用的配置文件。
/// 只写回允许动态修改的 `transfer_config`，避免把 v2 原始配置序列化成运行时视图。
pub async fn save_runtime_bot_config(config: &BotConfig) -> anyhow::Result<()> {
    let path = runtime_config_path()?;
    validate_runtime_config_path(path)?;
    let text = tokio::fs::read_to_string(path).await?;
    let text = update_runtime_transfer_config_in_text(&text, &config.transfer_config)?;
    tokio::fs::write(path, text).await?;
    Ok(())
}

/// 判断配置文件是否是 v2 结构。
fn is_v2_config(text: &str) -> anyhow::Result<bool> {
    let value = serde_json::from_str::<serde_json::Value>(text)?;
    Ok(value
        .get("config_version")
        .and_then(|v| v.as_i64())
        .is_some_and(|version| version >= 2))
}

/// 在原始 JSON 文本中只替换 `transfer_config` 字段。
///
/// `/cfg` 属于运行时安全参数修改，不能顺手重写 token、TDLib 目录等敏感结构。
fn update_runtime_transfer_config_in_text(
    text: &str,
    transfer_config: &TransferConfig,
) -> anyhow::Result<String> {
    if is_v2_config(text)? {
        let mut config = serde_json::from_str::<BotConfigV2>(text)?;
        config.transfer_config = transfer_config.clone();
        return Ok(serde_json::to_string_pretty(&config)?);
    }

    let mut config = serde_json::from_str::<LegacyBotConfig>(text)?;
    config.transfer_config = transfer_config.clone();
    Ok(serde_json::to_string_pretty(&config)?)
}

/// 获取运行时配置文件路径。
fn runtime_config_path() -> anyhow::Result<&'static PathBuf> {
    CONFIG_FILE_PATH
        .get()
        .ok_or_else(|| anyhow::anyhow!("runtime config path not initialized"))
}

/// 校验当前配置文件是否允许运行时写回。
/// 目前只允许明文 JSON 文件，避免把 `.enc` 覆盖成明文。
fn validate_runtime_config_path(path: &Path) -> anyhow::Result<()> {
    let is_encrypted = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("enc"))
        .unwrap_or(false);
    if is_encrypted {
        anyhow::bail!("encrypted config doesn't support runtime update")
    }
    Ok(())
}

// 默认后台转存并发数。
fn default_transfer_job_concurrency() -> usize {
    2
}

// 默认文件延迟删除分钟数。
fn default_transfer_file_delete_delay_minutes() -> i64 {
    2
}

// 默认文件 GC 扫描间隔秒数。
fn default_transfer_file_gc_interval_seconds() -> u64 {
    60
}

// 默认进度编辑间隔秒数。
fn default_progress_edit_interval_seconds() -> u64 {
    2
}

// 默认下载列表分页大小。
fn default_downloads_page_size() -> u64 {
    8
}

// 默认单次转存基础积分成本。
fn default_billing_base_cost_points() -> i64 {
    1
}

// 默认每条消息积分成本。
fn default_billing_item_cost_points() -> i64 {
    1
}

// 默认菜单输入超时时间秒数。
fn default_menu_input_timeout_seconds() -> u64 {
    10 * 60
}

// 默认 TDLib 日志级别。
fn default_tdlib_log_verbosity_level() -> i32 {
    1
}

// 默认业务 SQLite 数据库路径。
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

// serde 默认 true 辅助函数。
fn default_true() -> bool {
    true
}

// serde 跳过默认 true 字段时使用，避免配置写回时保留冗余开关。
fn is_true(value: &bool) -> bool {
    *value
}

// workflow 的交互端和兼容下载端都默认 bot。
fn default_client_role_bot() -> ClientRole {
    ClientRole::Bot
}

// workflow 固定 bot 的字段在写回时隐藏，只保留真正需要选择的 upload_client。
fn is_bot_role(role: &ClientRole) -> bool {
    *role == ClientRole::Bot
}

#[cfg(test)]
mod tests {
    use super::*;

    // 旧版 v1 单 client 配置不能再启动。
    // 当前交互固定 bot，下载/上传又需要显式 user/bot client，继续兼容 v1 会隐藏配置错误。
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

    // v2 配置应解析出 bot 交互、bot 兼容下载、bot 上传这三个角色。
    #[test]
    fn test_v2_config_maps_to_runtime_clients() {
        let config = BotConfig::from_json_str(v2_config_text()).unwrap();

        assert_eq!(config.workflow.interaction_client, ClientRole::Bot);
        assert_eq!(config.workflow.download_client, ClientRole::Bot);
        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
        assert_eq!(
            config.required_client_roles(),
            vec![ClientRole::User, ClientRole::Bot]
        );
        assert_eq!(config.target_map.get(&0), Some(&-100));
        assert_eq!(config.target_map.get(&123), Some(&-200));
        assert!(matches!(config.login_info, LoginInfo::Token(_)));
        assert!(config.supports_reply_markup());
        assert!(config.runtime_client(ClientRole::User).is_ok());
        assert!(config.runtime_client(ClientRole::Bot).is_ok());
        assert_eq!(
            config.storage.database_url,
            "sqlite://tg/app/transfer.sqlite?mode=rwc"
        );
        assert_eq!(config.billing.base_cost_points, 1);
        assert_eq!(config.billing.item_cost_points, 1);
        assert!(
            config
                .request_actor(123, 1)
                .is_some_and(|actor| actor.is_admin())
        );
        assert!(
            config
                .request_actor(1, 1)
                .is_some_and(|actor| actor.is_admin())
        );
        assert!(config.request_actor(999, 1).is_none());
        assert!(
            config
                .request_actor(2, 2)
                .is_some_and(|actor| !actor.is_admin())
        );
        assert!(config.request_actor(3, 3).is_none());
    }

    // 同一个链接转到同一个目标是否复用由数据库层 source_link + target_chat_id 决定，
    // 配置里的 upload_client 只决定谁上传，不应被要求参与查重键。
    #[test]
    fn test_v2_upload_client_does_not_affect_deduplicate_config() {
        let bot_upload = BotConfig::from_json_str(v2_config_text()).unwrap();
        let user_upload = BotConfig::from_json_str(
            &v2_config_text().replace("\"upload_client\": \"bot\"", "\"upload_client\": \"user\""),
        )
        .unwrap();

        assert_eq!(bot_upload.workflow.upload_client, ClientRole::Bot);
        assert_eq!(user_upload.workflow.upload_client, ClientRole::User);
        assert_eq!(bot_upload.target_map, user_upload.target_map);
    }

    // bot 可以只负责命令交互，上传仍由 user 执行。
    // 这对应“bot 做菜单和按钮，用户号做下载和上传”的保守部署模式。
    #[test]
    fn test_v2_supports_bot_interaction_with_user_upload() {
        let config = BotConfig::from_json_str(
            &v2_config_text().replace("\"upload_client\": \"bot\"", "\"upload_client\": \"user\""),
        )
        .unwrap();

        assert_eq!(config.workflow.interaction_client, ClientRole::Bot);
        assert_eq!(config.workflow.download_client, ClientRole::Bot);
        assert_eq!(config.workflow.upload_client, ClientRole::User);
        assert_eq!(
            config.required_client_roles(),
            vec![ClientRole::User, ClientRole::Bot]
        );
        assert!(config.supports_reply_markup());
    }

    // 交互端必须是 bot：菜单按钮、callback、copy-text 都依赖 bot reply_markup。
    #[test]
    fn test_v2_rejects_user_interaction_client() {
        let err = BotConfig::from_json_str(&v2_config_text().replace(
            "\"interaction_client\": \"bot\"",
            "\"interaction_client\": \"user\"",
        ))
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("workflow.interaction_client must be bot")
        );
    }

    // bot 默认负责上传和兼容下载端；user 仍会作为链接源 fallback client 启动。
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

        assert_eq!(config.workflow.download_client, ClientRole::Bot);
        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
        assert_eq!(ids.interaction, 20);
        assert_eq!(ids.download, 20);
        assert_eq!(ids.upload, 20);
    }

    // 运行时默认 workflow 也保持 bot-first + bot-upload，避免测试或兜底构造误用 user 上传。
    #[test]
    fn test_workflow_default_uses_bot_for_interaction_download_and_upload() {
        let workflow = WorkflowConfig::default();

        assert_eq!(workflow.interaction_client, ClientRole::Bot);
        assert_eq!(workflow.download_client, ClientRole::Bot);
        assert_eq!(workflow.upload_client, ClientRole::Bot);
    }

    // 新模板只保留真正需要选择的 upload_client：
    // - bot.enabled 省略时默认启用
    // - interaction/download 省略时默认 bot
    // - deduplicate 省略时使用固定查重策略
    #[test]
    fn test_v2_accepts_simplified_workflow_and_fixed_defaults() {
        let simplified = v2_config_text()
            .replace("              \"enabled\": true,\n", "")
            .replace(
                "            \"interaction_client\": \"bot\",\n            \"download_client\": \"bot\",\n",
                "",
            )
            .replace(
                "          \"deduplicate\": {\n            \"enabled\": true,\n            \"return_running_job\": true,\n            \"return_finished_result\": true\n          },\n",
                "",
            );

        let config = BotConfig::from_json_str(&simplified).unwrap();

        assert_eq!(config.workflow.interaction_client, ClientRole::Bot);
        assert_eq!(config.workflow.download_client, ClientRole::Bot);
        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
        assert!(config.runtime_client(ClientRole::Bot).is_ok());
    }

    // 当前重复转存策略固定开启，避免配置写成 false 但数据库仍按固定规则查重。
    #[test]
    fn test_v2_rejects_disabled_deduplicate() {
        let err = BotConfig::from_json_str(&v2_config_text().replace(
            "\"deduplicate\": {\n            \"enabled\": true",
            "\"deduplicate\": {\n            \"enabled\": false",
        ))
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("deduplicate options are fixed to true")
        );
    }

    // v2 允许 user 作为兼容下载端；但 user 无论如何都会作为链接源 fallback client 启动。
    #[test]
    fn test_v2_supports_user_download_client_with_user_fallback() {
        let config = BotConfig::from_json_str(&v2_config_text().replace(
            "\"download_client\": \"bot\"",
            "\"download_client\": \"user\"",
        ))
        .unwrap();

        assert_eq!(config.workflow.download_client, ClientRole::User);
        assert_eq!(
            config.required_client_roles(),
            vec![ClientRole::User, ClientRole::Bot]
        );
    }

    // workflow 使用 bot 时必须启用 bot client，避免启动后才发现缺少 token/client。
    #[test]
    fn test_v2_rejects_disabled_required_bot() {
        let err = BotConfig::from_json_str(&v2_config_text().replace(
            "\"bot\": {\n              \"enabled\": true",
            "\"bot\": {\n              \"enabled\": false",
        ))
        .unwrap_err();

        assert!(
            err.to_string()
                .contains("workflow requires bot client but clients.bot.enabled is false")
        );
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

    // `/cfg` 写回只能替换 transfer_config，不能把 v2 结构写成运行时视图。
    #[test]
    fn test_update_runtime_transfer_config_keeps_v2_shape() {
        let mut runtime = BotConfig::from_json_str(v2_config_text()).unwrap();
        runtime.transfer_config.job_concurrency = 5;
        runtime.transfer_config.file_delete_delay_minutes = 7;

        let updated =
            update_runtime_transfer_config_in_text(v2_config_text(), &runtime.transfer_config)
                .unwrap();
        let raw: BotConfigV2 = serde_json::from_str(&updated).unwrap();

        assert_eq!(raw.config_version, 2);
        assert_eq!(
            raw.clients.bot.token,
            "123456789:abcdefghijklmnopqrstuvwxyzABCDEF"
        );
        assert!(raw.clients.bot.enabled);
        assert_eq!(raw.transfer_config.job_concurrency, 5);
        assert_eq!(raw.transfer_config.file_delete_delay_minutes, 7);
        assert!(raw.billing.enabled);
        assert!(!updated.contains("\"bot\": {\n      \"enabled\": true"));
        assert!(!updated.contains("\"interaction_client\""));
        assert!(!updated.contains("\"download_client\""));
        assert!(!updated.contains("\"deduplicate\""));
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
              "enabled": true,
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
            "interaction_client": "bot",
            "download_client": "bot",
            "upload_client": "bot"
          },
          "deduplicate": {
            "enabled": true,
            "return_running_job": true,
            "return_finished_result": true
          },
          "access_control": {
            "admin_user_ids": [1],
            "allowed_user_ids": [2],
            "allow_all_private_users": false,
            "banned_user_ids": [],
            "allowed_request_chat_ids": [123],
            "allowed_target_chat_ids": [-100]
          },
          "targets": {
            "default_chat_id": -100,
            "by_request_chat_id": {
              "123": -200
            },
            "aliases": {
              "archive": -100
            }
          },
          "billing": {
            "enabled": true,
            "base_cost_points": 1,
            "item_cost_points": 1,
            "initial_user_points": 0
          },
          "transfer_config": {
            "job_concurrency": 2,
            "file_delete_delay_minutes": 2,
            "file_gc_interval_seconds": 60,
            "progress_edit_interval_seconds": 2,
            "downloads_default_page_size": 8,
            "menu_input_timeout_seconds": 600
          }
        }"#
    }
}
