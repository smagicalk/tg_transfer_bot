// 配置模型定义：
// 负责 JSON <-> Rust 结构体映射。
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

// 运行时配置文件路径：
// - 主程序启动时写入
// - 目前只保留给需要知道“当前配置文件来自哪里”的流程使用
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
    // 默认值与旧代码常量保持一致，后续可通过运行态管理命令动态修改。
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

impl TransferConfig {
    /// 转成数据库单行配置使用的整数视图。
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

    /// 从数据库单行配置恢复运行时配置。
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
    // 上传端是 workflow 中唯一需要用户主动选择的角色。
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

// 所有者的一次 bot 交互上下文。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestActor {
    pub request_chat_id: i64,
    pub user_id: i64,
}

// 默认目标配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct TargetsConfig {
    pub default_chat_id: i64,
    #[serde(default)]
    pub aliases: HashMap<String, i64>,
}

impl TargetsConfig {
    /// 判断 targets 默认值是否为空。
    pub fn is_empty(&self) -> bool {
        self.default_chat_id == 0 && self.aliases.is_empty()
    }
}

// v2 原始配置。
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct BotConfigV2 {
    pub config_version: i32,
    #[serde(default)]
    pub owner_user_id: i64,
    pub tdlib_defaults: TdlibDefaults,
    #[serde(default)]
    pub storage: StorageConfig,
    pub clients: ClientsConfig,
    #[serde(default)]
    pub workflow: WorkflowConfig,
    #[serde(default)]
    pub targets: TargetsConfig,
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
#[derive(Debug, Clone, Default)]
pub struct BotConfig {
    // 唯一允许与 bot 私聊交互的所有者 Telegram user ID。
    pub owner_user_id: i64,

    // 机器人业务数据库配置。
    pub storage: StorageConfig,

    // 启动时用于 seed 数据库的目标配置。
    pub targets: TargetsConfig,

    // 转存相关运行参数。
    pub transfer_config: TransferConfig,

    // v2 原始配置中提取出的工作流角色。
    pub workflow: WorkflowConfig,

    // 运行期 client id 集合。
    pub client_ids: RuntimeClientIds,

    // user/bot 的运行期 TDLib 配置。
    pub runtime_clients: HashMap<ClientRole, RuntimeClientConfig>,
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
        roles.insert(ClientRole::User);
        roles.insert(ClientRole::Bot);
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
            .get(ClientRole::Bot)
            .ok_or_else(|| anyhow::anyhow!("interaction client is not ready"))
    }

    /// 获取转存执行链需要的 client id。
    pub fn transfer_client_ids(&self) -> anyhow::Result<TransferClientIds> {
        Ok(TransferClientIds {
            interaction: self.interaction_client_id()?,
            download: self.interaction_client_id()?,
            upload: self
                .client_ids
                .get(self.workflow.upload_client)
                .ok_or_else(|| anyhow::anyhow!("upload client is not ready"))?,
            user: self.client_ids.user,
            bot: self.client_ids.bot,
        })
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
        (self.owner_user_id != 0
            && request_chat_id == sender_user_id
            && sender_user_id == self.owner_user_id)
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

        if self.clients.bot.token.trim().is_empty() {
            anyhow::bail!("clients.bot.token is required");
        }
        if !looks_like_bot_token(self.clients.bot.token.trim()) {
            anyhow::bail!("clients.bot.token format is invalid");
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

/// 判断配置文件是否是 v2 结构。
fn is_v2_config(text: &str) -> anyhow::Result<bool> {
    let value = serde_json::from_str::<serde_json::Value>(text)?;
    Ok(value
        .get("config_version")
        .and_then(|v| v.as_i64())
        .is_some_and(|version| version >= 2))
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

// workflow 的上传端默认 bot。
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

    // v2 配置应固定启动 user/bot，并保留可选上传角色。
    #[test]
    fn test_v2_config_maps_to_runtime_clients() {
        let config = BotConfig::from_json_str(v2_config_text()).unwrap();

        assert_eq!(config.workflow.upload_client, ClientRole::Bot);
        assert_eq!(
            config.required_client_roles(),
            vec![ClientRole::User, ClientRole::Bot]
        );
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

    #[test]
    fn test_v2_owner_user_id_is_the_only_private_actor() {
        let config_text = v2_config_text();
        let config = BotConfig::from_json_str(config_text).unwrap();

        assert_eq!(config.owner_user_id, 1);
        assert!(config.request_actor(1, 1).is_some());
        assert!(config.request_actor(2, 2).is_none());
        assert!(config.request_actor(999, 1).is_none());
    }

    #[test]
    fn test_v2_rejects_missing_owner_user_id() {
        let config_text = v2_config_text().replace("\"owner_user_id\": 1", "\"owner_user_id\": 0");

        let err = BotConfig::from_json_str(&config_text).unwrap_err();

        assert!(err.to_string().contains("owner_user_id must be positive"));
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

    // 同一个链接转到同一个目标是否复用由数据库层 source_link + target_chat_id 决定，
    // 配置里的 upload_client 只决定谁上传，不应被要求参与查重键。
    #[test]
    fn test_v2_upload_client_does_not_affect_target_defaults() {
        let bot_upload = BotConfig::from_json_str(v2_config_text()).unwrap();
        let user_upload = BotConfig::from_json_str(
            &v2_config_text().replace("\"upload_client\": \"bot\"", "\"upload_client\": \"user\""),
        )
        .unwrap();

        assert_eq!(bot_upload.workflow.upload_client, ClientRole::Bot);
        assert_eq!(user_upload.workflow.upload_client, ClientRole::User);
        assert_eq!(bot_upload.targets, user_upload.targets);
    }

    // bot 可以只负责命令交互，上传仍由 user 执行。
    // 这对应“bot 做菜单和按钮，用户号做下载和上传”的保守部署模式。
    #[test]
    fn test_v2_supports_bot_interaction_with_user_upload() {
        let config = BotConfig::from_json_str(
            &v2_config_text().replace("\"upload_client\": \"bot\"", "\"upload_client\": \"user\""),
        )
        .unwrap();

        assert_eq!(config.workflow.upload_client, ClientRole::User);
        assert_eq!(
            config.required_client_roles(),
            vec![ClientRole::User, ClientRole::Bot]
        );
    }

    // bot 默认负责上传和源读取；user 仍会作为链接源 fallback client 启动。
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
