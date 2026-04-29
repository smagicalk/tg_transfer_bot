// 命令模块入口：
// - 按命令职责拆分子模块
// - 对外保持统一导出，避免上层调用方感知文件结构变化

pub(super) mod common;
mod config_cmd;
mod downloads;
mod help;
mod job;
mod lookup;
mod transfer_cmd;

pub use config_cmd::config_command;
pub use downloads::{downloads_callback_query, downloads_command};
pub use help::help_command;
pub use job::job_command;
pub use lookup::lookup_command;
pub use transfer_cmd::transfer_command;
