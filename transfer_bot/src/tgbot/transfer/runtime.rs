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

pub fn init_runtime_config(bundle: RuntimeInitBundle) {
    let app = crate::app_context::app_context();
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

pub fn update_runtime_config(config: crate::config::TransferConfig) {
    crate::app_context::app_context()
        .transfer_runtime
        .update_runtime_config(config);
}

pub fn update_targets_runtime_config(config: TargetsConfig) {
    crate::app_context::app_context()
        .targets_runtime
        .update_runtime_config(config);
}

pub fn update_access_control_runtime_config(config: AccessControlConfig) {
    crate::app_context::app_context()
        .access_control_runtime
        .update_runtime_config(config);
}

pub fn update_billing_runtime_config(config: BillingConfig) {
    crate::app_context::app_context()
        .billing_runtime
        .update_runtime_config(config);
}

pub(in crate::tgbot::transfer) fn runtime_config() -> crate::config::TransferConfig {
    crate::app_context::app_context()
        .transfer_runtime
        .runtime_config()
}

pub(in crate::tgbot::transfer) fn runtime_default_config() -> crate::config::TransferConfig {
    crate::app_context::app_context()
        .transfer_runtime
        .runtime_default_config()
}

pub(in crate::tgbot::transfer) fn targets_runtime_config() -> TargetsConfig {
    crate::app_context::app_context()
        .targets_runtime
        .runtime_config()
}

pub(in crate::tgbot::transfer) fn targets_runtime_default_config() -> TargetsConfig {
    crate::app_context::app_context()
        .targets_runtime
        .runtime_default_config()
}

pub(in crate::tgbot) fn billing_runtime_config() -> crate::config::BillingConfig {
    crate::app_context::app_context()
        .billing_runtime
        .runtime_config()
}

pub(in crate::tgbot::transfer) fn billing_runtime_default_config() -> BillingConfig {
    crate::app_context::app_context()
        .billing_runtime
        .runtime_default_config()
}

pub(in crate::tgbot) fn access_control_runtime_config() -> AccessControlConfig {
    crate::app_context::app_context()
        .access_control_runtime
        .runtime_config()
}

pub(in crate::tgbot::transfer) fn access_control_runtime_default_config() -> AccessControlConfig {
    crate::app_context::app_context()
        .access_control_runtime
        .runtime_default_config()
}
