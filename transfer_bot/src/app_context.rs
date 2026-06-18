use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;

use crate::config::{
    AccessControlConfig, ActorRole, BillingConfig, ClientRole, RequestActor, TargetsConfig,
    TransferClientIds, TransferConfig,
};

static APP_CONTEXT: LazyLock<Arc<AppContext>> = LazyLock::new(|| Arc::new(AppContext::default()));

pub(crate) fn app_context() -> Arc<AppContext> {
    APP_CONTEXT.clone()
}

pub struct AppContext {
    pub(crate) transfer_runtime: Arc<TransferRuntimeState>,
    pub(crate) billing_runtime: Arc<BillingRuntimeState>,
    pub(crate) targets_runtime: Arc<TargetsRuntimeState>,
    pub(crate) access_control_runtime: Arc<AccessControlRuntimeState>,
    pub(crate) download_progress: Arc<DownloadProgressStore>,
    pub(crate) inflight_downloads: Arc<InflightDownloadRegistry>,
    pub(crate) transfer_guards: Arc<TransferExecutionGuards>,
    pub(crate) send_capabilities: Arc<SendCapabilities>,
    pub(crate) home_announcement: Arc<HomeAnnouncementState>,
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            transfer_runtime: Arc::new(TransferRuntimeState::default()),
            billing_runtime: Arc::new(BillingRuntimeState::default()),
            targets_runtime: Arc::new(TargetsRuntimeState::default()),
            access_control_runtime: Arc::new(AccessControlRuntimeState::default()),
            download_progress: Arc::new(DownloadProgressStore::default()),
            inflight_downloads: Arc::new(InflightDownloadRegistry::default()),
            transfer_guards: Arc::new(TransferExecutionGuards::default()),
            send_capabilities: Arc::new(SendCapabilities::default()),
            home_announcement: Arc::new(HomeAnnouncementState::default()),
        }
    }
}

#[derive(Default)]
pub struct TargetsRuntimeState {
    runtime_config: RwLock<TargetsConfig>,
    runtime_default_config: RwLock<TargetsConfig>,
}

impl TargetsRuntimeState {
    pub fn init_runtime_config(&self, config: TargetsConfig, default_config: TargetsConfig) {
        self.set_runtime_default_config(default_config);
        self.update_runtime_config(config);
    }

    pub fn update_runtime_config(&self, config: TargetsConfig) {
        *recover_rwlock_write(&self.runtime_config, "targets runtime config") = config;
    }

    pub fn runtime_config(&self) -> TargetsConfig {
        recover_rwlock_read(&self.runtime_config, "targets runtime config").clone()
    }

    pub fn set_runtime_default_config(&self, config: TargetsConfig) {
        *recover_rwlock_write(
            &self.runtime_default_config,
            "targets runtime default config",
        ) = config;
    }

    pub fn runtime_default_config(&self) -> TargetsConfig {
        recover_rwlock_read(
            &self.runtime_default_config,
            "targets runtime default config",
        )
        .clone()
    }
}

#[derive(Default)]
pub struct AccessControlRuntimeState {
    runtime_config: RwLock<AccessControlConfig>,
    runtime_default_config: RwLock<AccessControlConfig>,
}

impl AccessControlRuntimeState {
    pub fn init_runtime_config(
        &self,
        config: AccessControlConfig,
        default_config: AccessControlConfig,
    ) {
        self.set_runtime_default_config(default_config);
        self.update_runtime_config(config);
    }

    pub fn update_runtime_config(&self, config: AccessControlConfig) {
        *recover_rwlock_write(&self.runtime_config, "access control runtime config") = config;
    }

    pub fn runtime_config(&self) -> AccessControlConfig {
        recover_rwlock_read(&self.runtime_config, "access control runtime config").clone()
    }

    pub fn set_runtime_default_config(&self, config: AccessControlConfig) {
        *recover_rwlock_write(
            &self.runtime_default_config,
            "access control runtime default config",
        ) = config;
    }

    pub fn runtime_default_config(&self) -> AccessControlConfig {
        recover_rwlock_read(
            &self.runtime_default_config,
            "access control runtime default config",
        )
        .clone()
    }

    pub fn request_actor(&self, request_chat_id: i64, sender_user_id: i64) -> Option<RequestActor> {
        let config = self.runtime_config();
        if config.banned_user_ids.contains(&sender_user_id) {
            return None;
        }

        if is_effective_admin(&config, sender_user_id) {
            if admin_request_chat_allowed(request_chat_id, sender_user_id) {
                return Some(RequestActor {
                    request_chat_id,
                    user_id: sender_user_id,
                    role: ActorRole::Admin,
                });
            }
            return None;
        }

        if normal_user_request_allowed(&config, request_chat_id, sender_user_id) {
            return Some(RequestActor {
                request_chat_id,
                user_id: sender_user_id,
                role: ActorRole::User,
            });
        }

        None
    }
}

/// 判断用户是否属于当前生效管理员集合。
///
/// bootstrap admin 来自文件兜底，数据库管理员来自运行时配置；这里统一合并判断。
fn is_effective_admin(config: &AccessControlConfig, sender_user_id: i64) -> bool {
    config.bootstrap_admin_user_ids.contains(&sender_user_id)
        || config.admin_user_ids.contains(&sender_user_id)
}

/// admin 也只能私聊 bot 操作。
fn admin_request_chat_allowed(request_chat_id: i64, sender_user_id: i64) -> bool {
    request_chat_id == sender_user_id
}

/// 普通用户只允许私聊，且必须在白名单中或开启 allow_all_private_users。
fn normal_user_request_allowed(
    config: &AccessControlConfig,
    request_chat_id: i64,
    sender_user_id: i64,
) -> bool {
    request_chat_id == sender_user_id
        && (config.allow_all_private_users || config.allowed_user_ids.contains(&sender_user_id))
}

#[derive(Default)]
pub struct HomeAnnouncementState {
    announcement_text: RwLock<Option<String>>,
}

impl HomeAnnouncementState {
    pub fn set_announcement_text(&self, announcement_text: Option<String>) {
        *recover_rwlock_write(&self.announcement_text, "home announcement") = announcement_text;
    }

    pub fn announcement_text(&self) -> Option<String> {
        recover_rwlock_read(&self.announcement_text, "home announcement").clone()
    }
}

#[derive(Default)]
pub struct BillingRuntimeState {
    runtime_config: RwLock<BillingConfig>,
    runtime_default_config: RwLock<BillingConfig>,
}

impl BillingRuntimeState {
    pub fn init_runtime_config(&self, config: BillingConfig, default_config: BillingConfig) {
        self.set_runtime_default_config(default_config);
        self.update_runtime_config(config);
    }

    pub fn update_runtime_config(&self, config: BillingConfig) {
        *recover_rwlock_write(&self.runtime_config, "billing runtime config") = config;
    }

    pub fn runtime_config(&self) -> BillingConfig {
        recover_rwlock_read(&self.runtime_config, "billing runtime config").clone()
    }

    pub fn set_runtime_default_config(&self, config: BillingConfig) {
        *recover_rwlock_write(
            &self.runtime_default_config,
            "billing runtime default config",
        ) = config;
    }

    pub fn runtime_default_config(&self) -> BillingConfig {
        recover_rwlock_read(
            &self.runtime_default_config,
            "billing runtime default config",
        )
        .clone()
    }
}

pub struct SendCapabilities {
    reply_markup_enabled: AtomicBool,
}

impl Default for SendCapabilities {
    fn default() -> Self {
        Self {
            reply_markup_enabled: AtomicBool::new(true),
        }
    }
}

impl SendCapabilities {
    pub fn set_reply_markup_enabled(&self, enabled: bool) {
        self.reply_markup_enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn reply_markup_enabled(&self) -> bool {
        self.reply_markup_enabled.load(Ordering::Relaxed)
    }
}

#[derive(Default)]
pub struct TransferRuntimeState {
    runtime_config: RwLock<TransferConfig>,
    runtime_default_config: RwLock<TransferConfig>,
    tdlib_files_directories: RwLock<HashMap<ClientRole, PathBuf>>,
    active_transfer_jobs: AtomicUsize,
    transfer_slot_notify: tokio::sync::Notify,
    transfer_client_ids: RwLock<Option<TransferClientIds>>,
    background_services_started: AtomicBool,
}

impl TransferRuntimeState {
    pub fn init_runtime_config(
        &self,
        config: TransferConfig,
        default_config: TransferConfig,
        tdlib_files_directories: HashMap<ClientRole, PathBuf>,
    ) {
        self.set_runtime_default_config(default_config);
        self.update_runtime_config(config);
        self.update_tdlib_files_directories(tdlib_files_directories);
    }

    pub fn update_runtime_config(&self, config: TransferConfig) {
        *recover_rwlock_write(&self.runtime_config, "transfer runtime config") = config;
        self.transfer_slot_notify.notify_waiters();
    }

    pub fn runtime_config(&self) -> TransferConfig {
        recover_rwlock_read(&self.runtime_config, "transfer runtime config").clone()
    }

    pub fn set_runtime_default_config(&self, config: TransferConfig) {
        *recover_rwlock_write(
            &self.runtime_default_config,
            "transfer runtime default config",
        ) = config;
    }

    pub fn runtime_default_config(&self) -> TransferConfig {
        recover_rwlock_read(
            &self.runtime_default_config,
            "transfer runtime default config",
        )
        .clone()
    }

    pub fn tdlib_files_directory_for(&self, role: ClientRole) -> Option<PathBuf> {
        recover_rwlock_read(&self.tdlib_files_directories, "tdlib files directory")
            .get(&role)
            .cloned()
    }

    pub fn active_transfer_jobs_count(&self) -> usize {
        self.active_transfer_jobs.load(Ordering::SeqCst)
    }

    pub async fn acquire_transfer_slot(self: &Arc<Self>) -> TransferExecGuard {
        loop {
            let limit = self.runtime_config().job_concurrency.max(1);
            let active = self.active_transfer_jobs.load(Ordering::SeqCst);
            if active < limit {
                if self
                    .active_transfer_jobs
                    .compare_exchange(active, active + 1, Ordering::SeqCst, Ordering::SeqCst)
                    .is_ok()
                {
                    return TransferExecGuard {
                        state: self.clone(),
                    };
                }
                continue;
            }
            self.transfer_slot_notify.notified().await;
        }
    }

    pub fn set_transfer_client_ids(&self, client_ids: TransferClientIds) {
        *recover_rwlock_write(&self.transfer_client_ids, "transfer client ids") = Some(client_ids);
    }

    pub fn transfer_client_ids(&self) -> Option<TransferClientIds> {
        recover_rwlock_read(&self.transfer_client_ids, "transfer client ids")
            .as_ref()
            .copied()
    }

    pub fn mark_background_services_started(&self) -> bool {
        self.background_services_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    fn update_tdlib_files_directories(&self, paths: HashMap<ClientRole, PathBuf>) {
        let mut guard =
            recover_rwlock_write(&self.tdlib_files_directories, "tdlib files directory");
        guard.clear();
        for (role, path) in paths {
            if !path.as_os_str().is_empty() {
                guard.insert(role, path);
            }
        }
    }
}

pub struct TransferExecGuard {
    state: Arc<TransferRuntimeState>,
}

impl Drop for TransferExecGuard {
    fn drop(&mut self) {
        self.state
            .active_transfer_jobs
            .fetch_sub(1, Ordering::SeqCst);
        self.state.transfer_slot_notify.notify_one();
    }
}

#[derive(Debug, Clone, Default)]
pub struct DownloadProgressSnapshot {
    pub downloaded_size: i64,
    pub total_size: Option<i64>,
}

#[derive(Default)]
pub struct DownloadProgressStore {
    snapshots: RwLock<HashMap<(i32, i32), DownloadProgressSnapshot>>,
}

impl DownloadProgressStore {
    pub fn update_download_progress(&self, client_id: i32, file: &tdlib_rs::types::File) {
        let total_size = if file.size > 0 {
            Some(file.size)
        } else if file.expected_size > 0 {
            Some(file.expected_size)
        } else {
            None
        };

        let mut guard = recover_rwlock_write(&self.snapshots, "download progress");

        let key = (client_id, file.id);
        if file.local.is_downloading_completed {
            guard.remove(&key);
            return;
        }

        if !file.local.is_downloading_active && file.local.downloaded_size <= 0 {
            return;
        }

        guard.insert(
            key,
            DownloadProgressSnapshot {
                downloaded_size: file
                    .local
                    .downloaded_size
                    .max(file.local.downloaded_prefix_size),
                total_size,
            },
        );
    }

    pub fn get_download_progress(
        &self,
        client_id: i32,
        file_id: i32,
    ) -> Option<DownloadProgressSnapshot> {
        recover_rwlock_read(&self.snapshots, "download progress")
            .get(&(client_id, file_id))
            .cloned()
    }
}

type DownloadResult = Result<(), String>;
type DownloadNotifier = tokio::sync::watch::Sender<Option<DownloadResult>>;
type InflightDownloadMap = HashMap<String, DownloadNotifier>;

#[derive(Default)]
pub struct InflightDownloadRegistry {
    inflight: Mutex<InflightDownloadMap>,
}

impl InflightDownloadRegistry {
    pub async fn run_singleflight<F, Fut>(
        self: &Arc<Self>,
        file_key: String,
        task: F,
    ) -> anyhow::Result<()>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = anyhow::Result<()>>,
    {
        let role = {
            let mut guard = recover_mutex_lock(&self.inflight, "inflight downloads");
            if let Some(tx) = guard.get(&file_key) {
                tracing::debug!(file_key = %file_key, "join inflight file download");
                InflightDownloadRole::Waiter(tx.subscribe())
            } else {
                let (tx, _rx) = tokio::sync::watch::channel(None);
                guard.insert(file_key.clone(), tx);
                tracing::debug!(file_key = %file_key, "start inflight file download");
                InflightDownloadRole::Executor(InflightExecutionGuard::new(self.clone(), file_key))
            }
        };

        let mut rx = match role {
            InflightDownloadRole::Executor(mut execute_guard) => {
                let result = task().await;
                let send_value = result.as_ref().map(|_| ()).map_err(|e| format!("{:#}", e));
                if let Err(err) = &send_value {
                    tracing::warn!(
                        file_key = %execute_guard.file_key,
                        error = %err,
                        "inflight file download failed"
                    );
                } else {
                    tracing::debug!(
                        file_key = %execute_guard.file_key,
                        "inflight file download completed"
                    );
                }
                execute_guard.finish(send_value);
                return result;
            }
            InflightDownloadRole::Waiter(rx) => rx,
        };

        loop {
            {
                let borrowed = rx.borrow();
                if let Some(value) = borrowed.as_ref() {
                    return value
                        .as_ref()
                        .map(|_| ())
                        .map_err(|e| anyhow::anyhow!("{}", e));
                }
            }

            if rx.changed().await.is_err() {
                anyhow::bail!("singleflight channel closed unexpectedly");
            }
        }
    }

    fn remove_and_notify(&self, file_key: &str, result: DownloadResult) {
        let mut guard = recover_mutex_lock(&self.inflight, "inflight downloads");
        if let Some(tx) = guard.remove(file_key) {
            let _ = tx.send(Some(result));
        }
    }
}

enum InflightDownloadRole {
    Executor(InflightExecutionGuard),
    Waiter(tokio::sync::watch::Receiver<Option<DownloadResult>>),
}

struct InflightExecutionGuard {
    registry: Arc<InflightDownloadRegistry>,
    file_key: String,
    finished: bool,
}

impl InflightExecutionGuard {
    fn new(registry: Arc<InflightDownloadRegistry>, file_key: String) -> Self {
        Self {
            registry,
            file_key,
            finished: false,
        }
    }

    fn finish(&mut self, result: DownloadResult) {
        self.finished = true;
        self.registry.remove_and_notify(&self.file_key, result);
    }
}

impl Drop for InflightExecutionGuard {
    fn drop(&mut self) {
        if self.finished {
            return;
        }

        self.registry.remove_and_notify(
            &self.file_key,
            Err("singleflight executor dropped before completion".to_owned()),
        );
    }
}

#[derive(Default)]
pub struct TransferExecutionGuards {
    running_job_ids: Mutex<HashSet<i64>>,
    creating_source_targets: Mutex<HashSet<(String, i64)>>,
}

impl TransferExecutionGuards {
    pub async fn is_job_running_in_process(&self, job_id: i64) -> bool {
        recover_mutex_lock(&self.running_job_ids, "running job id").contains(&job_id)
    }

    pub async fn acquire_job_guard(self: &Arc<Self>, job_id: i64) -> Option<TransferJobGuard> {
        let mut guard = recover_mutex_lock(&self.running_job_ids, "running job id");
        if guard.contains(&job_id) {
            return None;
        }
        guard.insert(job_id);
        Some(TransferJobGuard {
            guards: self.clone(),
            job_id,
        })
    }

    pub async fn acquire_source_target_create_guard(
        self: &Arc<Self>,
        source_link: String,
        target_chat_id: i64,
    ) -> SourceTargetCreateGuard {
        let key = (source_link, target_chat_id);
        loop {
            {
                let mut guard = recover_mutex_lock(&self.creating_source_targets, "source-target");
                if guard.insert(key.clone()) {
                    return SourceTargetCreateGuard {
                        guards: self.clone(),
                        key,
                    };
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

pub struct TransferJobGuard {
    guards: Arc<TransferExecutionGuards>,
    job_id: i64,
}

impl Drop for TransferJobGuard {
    fn drop(&mut self) {
        let mut guard = recover_mutex_lock(&self.guards.running_job_ids, "running job id");
        guard.remove(&self.job_id);
    }
}

pub struct SourceTargetCreateGuard {
    guards: Arc<TransferExecutionGuards>,
    key: (String, i64),
}

impl Drop for SourceTargetCreateGuard {
    fn drop(&mut self) {
        let mut guard = recover_mutex_lock(&self.guards.creating_source_targets, "source-target");
        guard.remove(&self.key);
    }
}

/// 恢复被 panic 标记为 poisoned 的互斥锁。
///
/// 这些锁只保护进程内缓存/guard；继续使用内部数据比让后续所有交互一起 panic 更可控。
fn recover_mutex_lock<'a, T>(mutex: &'a Mutex<T>, name: &str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!(lock = name, "recover poisoned mutex");
            poisoned.into_inner()
        }
    }
}

/// 恢复被 panic 标记为 poisoned 的读锁。
fn recover_rwlock_read<'a, T>(lock: &'a RwLock<T>, name: &str) -> RwLockReadGuard<'a, T> {
    match lock.read() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!(lock = name, "recover poisoned rwlock read");
            poisoned.into_inner()
        }
    }
}

/// 恢复被 panic 标记为 poisoned 的写锁。
fn recover_rwlock_write<'a, T>(lock: &'a RwLock<T>, name: &str) -> RwLockWriteGuard<'a, T> {
    match lock.write() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!(lock = name, "recover poisoned rwlock write");
            poisoned.into_inner()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_capabilities_default_to_enabled() {
        let capabilities = SendCapabilities::default();
        assert!(capabilities.reply_markup_enabled());
        capabilities.set_reply_markup_enabled(false);
        assert!(!capabilities.reply_markup_enabled());
    }

    #[test]
    fn download_progress_store_is_isolated_by_client_id() {
        let store = DownloadProgressStore::default();
        let file_id = 42;
        store.update_download_progress(10, &test_file(file_id, 100, 1000, false, true));
        store.update_download_progress(20, &test_file(file_id, 700, 1000, false, true));

        let first = store
            .get_download_progress(10, file_id)
            .expect("first client progress");
        let second = store
            .get_download_progress(20, file_id)
            .expect("second client progress");

        assert_eq!(first.downloaded_size, 100);
        assert_eq!(second.downloaded_size, 700);
    }

    #[tokio::test]
    async fn transfer_guards_block_duplicate_job_and_release_on_drop() {
        let guards = Arc::new(TransferExecutionGuards::default());
        let first = guards.acquire_job_guard(7).await;
        assert!(first.is_some());
        assert!(guards.is_job_running_in_process(7).await);
        let second = guards.acquire_job_guard(7).await;
        assert!(second.is_none());
        drop(first);
        assert!(!guards.is_job_running_in_process(7).await);
        assert!(guards.acquire_job_guard(7).await.is_some());
    }

    fn test_file(
        id: i32,
        downloaded_size: i64,
        size: i64,
        is_downloading_completed: bool,
        is_downloading_active: bool,
    ) -> tdlib_rs::types::File {
        tdlib_rs::types::File {
            id,
            size,
            expected_size: 0,
            local: tdlib_rs::types::LocalFile {
                path: String::new(),
                can_be_downloaded: true,
                can_be_deleted: true,
                is_downloading_active,
                is_downloading_completed,
                download_offset: 0,
                downloaded_prefix_size: 0,
                downloaded_size,
            },
            remote: tdlib_rs::types::RemoteFile {
                id: String::new(),
                unique_id: String::new(),
                is_uploading_active: false,
                is_uploading_completed: false,
                uploaded_size: 0,
            },
        }
    }
}
