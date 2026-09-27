use std::path::PathBuf;

use crate::config::{ClientRole, TargetsConfig};

/// 转存子系统运行时初始化快照。
///
/// 启动阶段先从文件读取默认值，再从数据库载入当前运行值，最后一次性灌进 AppContext。
pub struct RuntimeInitBundle {
    /// 经过持久化加载与合并后的转存动态运行时配置。
    pub transfer_config: crate::config::TransferConfig,
    /// 初始静态配置文件（或硬编码）定义的转存默认配置，用于“重置为默认值”能力。
    pub transfer_default_config: crate::config::TransferConfig,
    /// 经过持久化加载与合并后的目标频道/群组动态配置。
    pub targets_config: TargetsConfig,
    /// 初始静态配置文件定义的目标配置，用于恢复默认目标列表。
    pub targets_default_config: TargetsConfig,
    /// 针对不同客户端角色（User、Bot、Worker 等）在磁盘上分配的 TDLib 文件根目录映射。
    pub tdlib_files_directories: std::collections::HashMap<ClientRole, PathBuf>,
}

/// 从指定上下文读取 transfer 运行时配置。
///
/// 菜单和管理页在已经拿到 `AppContext` 时优先用这个版本，避免重复抓全局。
///
/// # 参数
/// - `app`: 全局应用程序上下文。
///
/// # 返回值
/// - 当前生效的转存运行时配置拷贝。
pub fn runtime_config_on(app: &crate::app_context::AppContext) -> crate::config::TransferConfig {
    app.transfer_runtime.runtime_config()
}

/// 从指定上下文读取 transfer 默认配置。
///
/// # 参数
/// - `app`: 全局应用程序上下文。
///
/// # 返回值
/// - 初始默认转存配置拷贝。
pub fn runtime_default_config_on(
    app: &crate::app_context::AppContext,
) -> crate::config::TransferConfig {
    app.transfer_runtime.runtime_default_config()
}

/// 从指定上下文读取 targets 运行时配置。
///
/// # 参数
/// - `app`: 全局应用程序上下文。
///
/// # 返回值
/// - 当前生效的目标频道/群组配置。
pub fn targets_runtime_config_on(app: &crate::app_context::AppContext) -> TargetsConfig {
    app.targets_runtime.runtime_config()
}

/// 从指定上下文读取 targets 默认配置。
///
/// # 参数
/// - `app`: 全局应用程序上下文。
///
/// # 返回值
/// - 初始默认的目标配置。
pub fn targets_runtime_default_config_on(app: &crate::app_context::AppContext) -> TargetsConfig {
    app.targets_runtime.runtime_default_config()
}

/// 在指定上下文上初始化 transfer 和 targets 运行时配置。
///
/// # 参数
/// - `app`: 全局应用程序上下文。
/// - `bundle`: 包含转存配置、目标配置和文件路径映射的初始化快照。
pub fn init_runtime_config_on(app: &crate::app_context::AppContext, bundle: RuntimeInitBundle) {
    app.transfer_runtime.init_runtime_config(
        bundle.transfer_config,
        bundle.transfer_default_config,
        bundle.tdlib_files_directories,
    );
    app.targets_runtime
        .init_runtime_config(bundle.targets_config, bundle.targets_default_config);
}

/// 在指定上下文上更新 transfer 运行时配置。
///
/// # 参数
/// - `app`: 全局应用程序上下文。
/// - `config`: 新的转存运行时配置。
pub fn update_runtime_config_on(
    app: &crate::app_context::AppContext,
    config: crate::config::TransferConfig,
) {
    app.transfer_runtime.update_runtime_config(config);
}

/// 在指定上下文上更新 targets 运行时配置。
///
/// # 参数
/// - `app`: 全局应用程序上下文。
/// - `config`: 新的目标频道配置。
pub fn update_targets_runtime_config_on(
    app: &crate::app_context::AppContext,
    config: TargetsConfig,
) {
    app.targets_runtime.update_runtime_config(config);
}
