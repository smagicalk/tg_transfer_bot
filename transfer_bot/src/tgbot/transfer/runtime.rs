use std::path::PathBuf;

use crate::config::ClientRole;

pub fn init_runtime_config(
    config: crate::config::TransferConfig,
    tdlib_files_directories: std::collections::HashMap<ClientRole, PathBuf>,
) {
    crate::app_context::app_context()
        .transfer_runtime
        .init_runtime_config(config, tdlib_files_directories);
}

pub fn update_runtime_config(config: crate::config::TransferConfig) {
    crate::app_context::app_context()
        .transfer_runtime
        .update_runtime_config(config);
}

pub(in crate::tgbot::transfer) fn runtime_config() -> crate::config::TransferConfig {
    crate::app_context::app_context()
        .transfer_runtime
        .runtime_config()
}
