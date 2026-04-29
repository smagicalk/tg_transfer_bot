// 配置模型定义：
// 负责 JSON <-> Rust 结构体映射。
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// 运行时配置文件路径：
// - 主程序启动时写入
// - `/config` 命令读取/保存配置时复用
static CONFIG_FILE_PATH: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

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

    // 文件引用归零后的延迟删除小时数。
    #[serde(default = "default_transfer_file_delete_delay_hours")]
    pub file_delete_delay_hours: i64,

    // 文件 GC 扫描间隔（秒）。
    #[serde(default = "default_transfer_file_gc_interval_seconds")]
    pub file_gc_interval_seconds: u64,
}

impl Default for TransferConfig {
    fn default() -> Self {
        Self {
            job_concurrency: default_transfer_job_concurrency(),
            file_delete_delay_hours: default_transfer_file_delete_delay_hours(),
            file_gc_interval_seconds: default_transfer_file_gc_interval_seconds(),
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

// 机器人整体配置。
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct BotConfig {
    // TDLib 配置。
    pub tdlib_config: TdlibConfig,

    // 管理员 chat/user id 白名单。
    pub admin_ids: Vec<i64>,

    // 运行时 client id，不参与配置序列化。
    #[serde(skip)]
    pub client_id: Option<i32>,

    // 默认转存目标映射：source_chat_id -> target_chat_id。
    // 可使用 key=0 作为兜底目标。
    pub target_map: HashMap<i64, i64>,

    // 转存相关运行参数。
    #[serde(default)]
    pub transfer_config: TransferConfig,

    // 登录方式。
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
    Ok(serde_json::from_str::<BotConfig>(&text)?)
}

/// 保存当前运行所使用的配置文件。
pub async fn save_runtime_bot_config(config: &BotConfig) -> anyhow::Result<()> {
    let path = runtime_config_path()?;
    validate_runtime_config_path(path)?;
    let text = serde_json::to_string_pretty(config)?;
    tokio::fs::write(path, text).await?;
    Ok(())
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

// 默认文件延迟删除小时数。
fn default_transfer_file_delete_delay_hours() -> i64 {
    2
}

// 默认文件 GC 扫描间隔秒数。
fn default_transfer_file_gc_interval_seconds() -> u64 {
    60
}

#[cfg(test)]
mod tests {
    use super::*;

    // 序列化样例。
    #[test]
    fn test_config() {
        let mut bot_config = BotConfig {
            login_info: LoginInfo::Phone("1234".to_string()),
            ..Default::default()
        };
        bot_config.target_map.entry(1234).or_insert(1234);
        let text = serde_json::to_string_pretty(&bot_config).unwrap();
        assert!(text.contains("\"type\": \"PHONE\""));
        assert!(text.contains("\"1234\": 1234"));
    }

    // 反序列化样例。
    #[test]
    fn test_config2() {
        let bot_config_str = r#"
        {
          "tdlib_config": {
            "use_test_dc": false,
            "database_directory": "",
            "files_directory": "",
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
            "file_delete_delay_hours": 2,
            "file_gc_interval_seconds": 60
          },
          "login_info": {
            "type": "PHONE",
            "data": "1234"
          }
        }"#;

        let bot_config: BotConfig = serde_json::from_str(bot_config_str).unwrap();
        assert_eq!(bot_config.target_map.get(&1234), Some(&1234));
        assert_eq!(bot_config.transfer_config.job_concurrency, 2);
    }
}
