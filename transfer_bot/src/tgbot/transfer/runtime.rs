use std::path::PathBuf;

use crate::config::{AccessControlConfig, BillingConfig, ClientRole, TargetsConfig};

/// 转存子系统运行时初始化快照。
///
/// 启动阶段先从文件读取默认值，再从数据库载入当前运行值，最后一次性灌进 AppContext。
pub struct RuntimeInitBundle {
    pub transfer_config: crate::config::TransferConfig,
    pub transfer_default_config: crate::config::TransferConfig,
    pub billing_config: BillingConfig,
    pub billing_default_config: BillingConfig,
    pub targets_config: TargetsConfig,
    pub targets_default_config: TargetsConfig,
    pub access_control_config: AccessControlConfig,
    pub access_control_default_config: AccessControlConfig,
    pub tdlib_files_directories: std::collections::HashMap<ClientRole, PathBuf>,
}

/// 从指定上下文读取 transfer 运行时配置。
///
/// 菜单和管理页在已经拿到 `AppContext` 时优先用这个版本，避免重复抓全局。
pub fn runtime_config_on(app: &crate::app_context::AppContext) -> crate::config::TransferConfig {
    app.transfer_runtime.runtime_config()
}

/// 从指定上下文读取 transfer 默认配置。
pub fn runtime_default_config_on(
    app: &crate::app_context::AppContext,
) -> crate::config::TransferConfig {
    app.transfer_runtime.runtime_default_config()
}

/// 从指定上下文读取 targets 运行时配置。
pub fn targets_runtime_config_on(app: &crate::app_context::AppContext) -> TargetsConfig {
    app.targets_runtime.runtime_config()
}

/// 从指定上下文读取 targets 默认配置。
pub fn targets_runtime_default_config_on(app: &crate::app_context::AppContext) -> TargetsConfig {
    app.targets_runtime.runtime_default_config()
}

/// 从指定上下文读取 billing 运行时配置。
pub fn billing_runtime_config_on(app: &crate::app_context::AppContext) -> BillingConfig {
    app.billing_runtime.runtime_config()
}

/// 从指定上下文读取 billing 默认配置。
pub fn billing_runtime_default_config_on(app: &crate::app_context::AppContext) -> BillingConfig {
    app.billing_runtime.runtime_default_config()
}

/// 从指定上下文读取 ACL 运行时配置。
pub fn access_control_runtime_config_on(
    app: &crate::app_context::AppContext,
) -> AccessControlConfig {
    app.access_control_runtime.runtime_config()
}

/// 从指定上下文读取 ACL 默认配置。
pub fn access_control_runtime_default_config_on(
    app: &crate::app_context::AppContext,
) -> AccessControlConfig {
    app.access_control_runtime.runtime_default_config()
}

/// 在指定上下文上初始化 transfer 运行时配置。
pub fn init_runtime_config_on(app: &crate::app_context::AppContext, bundle: RuntimeInitBundle) {
    app.transfer_runtime.init_runtime_config(
        bundle.transfer_config,
        bundle.transfer_default_config,
        bundle.tdlib_files_directories,
    );
    app.billing_runtime
        .init_runtime_config(bundle.billing_config, bundle.billing_default_config);
    app.targets_runtime
        .init_runtime_config(bundle.targets_config, bundle.targets_default_config);
    app.access_control_runtime.init_runtime_config(
        bundle.access_control_config,
        bundle.access_control_default_config,
    );
}

/// 在指定上下文上更新 transfer 运行时配置。
pub fn update_runtime_config_on(
    app: &crate::app_context::AppContext,
    config: crate::config::TransferConfig,
) {
    app.transfer_runtime.update_runtime_config(config);
}

/// 在指定上下文上更新 targets 运行时配置。
pub fn update_targets_runtime_config_on(
    app: &crate::app_context::AppContext,
    config: TargetsConfig,
) {
    app.targets_runtime.update_runtime_config(config);
}

/// 在指定上下文上更新 ACL 运行时配置。
pub fn update_access_control_runtime_config_on(
    app: &crate::app_context::AppContext,
    config: AccessControlConfig,
) {
    app.access_control_runtime.update_runtime_config(config);
}

/// 在指定上下文上更新 billing 运行时配置。
pub fn update_billing_runtime_config_on(
    app: &crate::app_context::AppContext,
    config: BillingConfig,
) {
    app.billing_runtime.update_runtime_config(config);
}
