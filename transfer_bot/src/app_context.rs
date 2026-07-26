use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;

use crate::config::{ClientRole, TargetsConfig, TransferClientIds, TransferConfig};

static APP_CONTEXT: LazyLock<Arc<AppContext>> = LazyLock::new(|| Arc::new(AppContext::default()));

pub(crate) fn app_context() -> Arc<AppContext> {
    APP_CONTEXT.clone()
}

#[derive(Clone)]
pub struct AppContext {
    pub(crate) access_control: Arc<AccessControlState>,
    pub(crate) transfer_runtime: Arc<TransferRuntimeState>,
    pub(crate) targets_runtime: Arc<TargetsRuntimeState>,
    pub(crate) download_progress: Arc<DownloadProgressStore>,
    pub(crate) upload_progress: Arc<UploadProgressStore>,
    pub(crate) inflight_downloads: Arc<InflightDownloadRegistry>,
    pub(crate) transfer_guards: Arc<TransferExecutionGuards>,
    pub(crate) send_capabilities: Arc<SendCapabilities>,
    pub(crate) executor_runtime: Arc<ExecutorRuntimeState>,
    pub(crate) lookup_retry: Arc<LookupRetryState>,
    pub(crate) retransfer_confirm: Arc<RetransferConfirmState>,
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            access_control: Arc::new(AccessControlState::default()),
            transfer_runtime: Arc::new(TransferRuntimeState::default()),
            targets_runtime: Arc::new(TargetsRuntimeState::default()),
            download_progress: Arc::new(DownloadProgressStore::default()),
            upload_progress: Arc::new(UploadProgressStore::default()),
            inflight_downloads: Arc::new(InflightDownloadRegistry::default()),
            transfer_guards: Arc::new(TransferExecutionGuards::default()),
            send_capabilities: Arc::new(SendCapabilities::default()),
            executor_runtime: Arc::new(ExecutorRuntimeState::default()),
            lookup_retry: Arc::new(LookupRetryState::default()),
            retransfer_confirm: Arc::new(RetransferConfirmState::default()),
        }
    }
}

/// 按需登录用户执行器的运行状态。
///
/// Bot 始终独立运行；用户执行器只在需要读取私有源或 Bot 权限不足时由 owner 登录。
/// 状态只保存 client 与交互定位，不保存二维码链接、密码或其他登录凭据。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecutorPhase {
    #[default]
    Offline,
    Starting,
    WaitingQr,
    WaitingPassword,
    Ready,
    Draining,
    LoggingOut,
}

#[derive(Default)]
pub struct ExecutorRuntimeState {
    phase: RwLock<ExecutorPhase>,
    user_client_id: RwLock<Option<i32>>,
    owner_chat_id: RwLock<Option<i64>>,
    qr_image_path: RwLock<Option<PathBuf>>,
    qr_message_id: RwLock<Option<i64>>,
    password_prompt_message_id: RwLock<Option<i64>>,
    identity: RwLock<Option<ExecutorIdentity>>,
}

/// 已登录执行器的非敏感账号摘要，用于 owner 在面板中确认当前会话。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutorIdentity {
    pub user_id: i64,
    pub display_name: String,
    pub username: Option<String>,
}

impl ExecutorRuntimeState {
    pub fn phase(&self) -> ExecutorPhase {
        *recover_rwlock_read(&self.phase, "executor phase")
    }

    pub fn user_client_id(&self) -> Option<i32> {
        *recover_rwlock_read(&self.user_client_id, "executor user client id")
    }

    pub fn owner_chat_id(&self) -> Option<i64> {
        *recover_rwlock_read(&self.owner_chat_id, "executor owner chat id")
    }

    pub fn begin_login(&self, user_client_id: i32, owner_chat_id: i64) {
        *recover_rwlock_write(&self.user_client_id, "executor user client id") =
            Some(user_client_id);
        *recover_rwlock_write(&self.owner_chat_id, "executor owner chat id") = Some(owner_chat_id);
        *recover_rwlock_write(&self.qr_message_id, "executor qr message id") = None;
        *recover_rwlock_write(&self.identity, "executor identity") = None;
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Starting;
    }

    pub fn role_for_client_id(&self, client_id: i32) -> Option<ClientRole> {
        (self.user_client_id() == Some(client_id)).then_some(ClientRole::User)
    }

    pub fn request_qr_if_starting(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) {
            return false;
        }
        let mut phase = recover_rwlock_write(&self.phase, "executor phase");
        if *phase != ExecutorPhase::Starting {
            return false;
        }
        *phase = ExecutorPhase::WaitingQr;
        true
    }

    pub fn set_waiting_password(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::WaitingPassword;
        true
    }

    pub fn mark_ready(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Ready;
        true
    }

    pub fn mark_logging_out(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::LoggingOut;
        true
    }

    pub fn begin_draining(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) || self.phase() != ExecutorPhase::Ready {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Draining;
        true
    }

    pub fn cancel_draining(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) || self.phase() != ExecutorPhase::Draining {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Ready;
        true
    }

    pub fn restore_ready_after_logout_failure(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) || self.phase() != ExecutorPhase::LoggingOut {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Ready;
        true
    }

    pub fn clear_user_client_if(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) {
            return false;
        }
        *recover_rwlock_write(&self.user_client_id, "executor user client id") = None;
        *recover_rwlock_write(&self.owner_chat_id, "executor owner chat id") = None;
        *recover_rwlock_write(&self.qr_message_id, "executor qr message id") = None;
        *recover_rwlock_write(&self.identity, "executor identity") = None;
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Offline;
        true
    }

    pub fn replace_qr_image_path(&self, path: PathBuf) -> Option<PathBuf> {
        recover_rwlock_write(&self.qr_image_path, "executor qr image path").replace(path)
    }

    pub fn take_qr_image_path(&self) -> Option<PathBuf> {
        recover_rwlock_write(&self.qr_image_path, "executor qr image path").take()
    }

    /// 保存首次发送的二维码消息，后续二维码刷新时只编辑该消息。
    pub fn replace_qr_message_id(&self, message_id: i64) -> Option<i64> {
        recover_rwlock_write(&self.qr_message_id, "executor qr message id").replace(message_id)
    }

    pub fn qr_message_id(&self) -> Option<i64> {
        *recover_rwlock_read(&self.qr_message_id, "executor qr message id")
    }

    pub fn set_identity_if_ready(&self, client_id: i32, identity: ExecutorIdentity) -> bool {
        if self.user_client_id() != Some(client_id) || self.phase() != ExecutorPhase::Ready {
            return false;
        }
        *recover_rwlock_write(&self.identity, "executor identity") = Some(identity);
        true
    }

    pub fn identity(&self) -> Option<ExecutorIdentity> {
        recover_rwlock_read(&self.identity, "executor identity").clone()
    }

    pub fn replace_password_prompt_message_id(&self, message_id: i64) -> Option<i64> {
        recover_rwlock_write(
            &self.password_prompt_message_id,
            "executor password prompt message id",
        )
        .replace(message_id)
    }

    pub fn take_password_prompt_message_id(&self) -> Option<i64> {
        recover_rwlock_write(
            &self.password_prompt_message_id,
            "executor password prompt message id",
        )
        .take()
    }

    pub fn password_prompt_message_id(&self) -> Option<i64> {
        *recover_rwlock_read(
            &self.password_prompt_message_id,
            "executor password prompt message id",
        )
    }
}

/// 运行时动态授权名单；持久化由数据库访问层负责。
#[derive(Default)]
pub struct AccessControlState {
    authorized_user_ids: RwLock<HashSet<i64>>,
}

impl AccessControlState {
    /// 用启动时从数据库读取的完整名单替换当前状态。
    pub fn replace_authorized_user_ids(&self, user_ids: impl IntoIterator<Item = i64>) {
        let mut guard = recover_rwlock_write(&self.authorized_user_ids, "authorized user ids");
        guard.clear();
        guard.extend(user_ids.into_iter().filter(|user_id| *user_id > 0));
    }

    pub fn is_authorized(&self, user_id: i64) -> bool {
        user_id > 0
            && recover_rwlock_read(&self.authorized_user_ids, "authorized user ids")
                .contains(&user_id)
    }

    /// 把单个用户加入当前进程授权名单。
    pub fn authorize_user(&self, user_id: i64) -> bool {
        user_id > 0
            && recover_rwlock_write(&self.authorized_user_ids, "authorized user ids")
                .insert(user_id)
    }

    /// 从当前进程授权名单移除单个用户。
    pub fn revoke_user(&self, user_id: i64) -> bool {
        recover_rwlock_write(&self.authorized_user_ids, "authorized user ids").remove(&user_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupRetryContext {
    pub source_link: String,
    pub target_chat_id: i64,
}

#[derive(Default)]
pub struct LookupRetryState {
    by_message: RwLock<HashMap<(i64, i64, i64), LookupRetryEntry>>,
    sequence: AtomicUsize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LookupRetryEntry {
    context: LookupRetryContext,
    sequence: usize,
}

impl LookupRetryState {
    pub fn put_context(
        &self,
        request_chat_id: i64,
        sender_user_id: i64,
        message_id: i64,
        context: LookupRetryContext,
    ) {
        let mut guard = recover_rwlock_write(&self.by_message, "lookup retry context");
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
        guard.insert(
            (request_chat_id, sender_user_id, message_id),
            LookupRetryEntry { context, sequence },
        );
        prune_lookup_retry_entries(&mut guard, request_chat_id, sender_user_id);
    }

    pub fn take_context(
        &self,
        request_chat_id: i64,
        sender_user_id: i64,
        message_id: i64,
    ) -> Option<LookupRetryContext> {
        recover_rwlock_write(&self.by_message, "lookup retry context")
            .remove(&(request_chat_id, sender_user_id, message_id))
            .map(|entry| entry.context)
    }
}

const LOOKUP_RETRY_CONTEXT_LIMIT_PER_USER: usize = 8;

/// “再次转存”确认卡的短期上下文。
///
/// callback_data 不能容纳完整源链接，因此按卡片消息定位保存计划；确认后立即消费。
#[derive(Default)]
pub struct RetransferConfirmState {
    by_message: RwLock<HashMap<(i64, i64, i64), crate::tgbot::transfer::types::TransferPlan>>,
}

impl RetransferConfirmState {
    pub(crate) fn put_plan(
        &self,
        request_chat_id: i64,
        sender_user_id: i64,
        message_id: i64,
        plan: crate::tgbot::transfer::types::TransferPlan,
    ) {
        let mut guard = recover_rwlock_write(&self.by_message, "retransfer confirm context");
        guard.insert((request_chat_id, sender_user_id, message_id), plan);
        // 每个会话只保留最近的少量确认卡，防止长期运行后无界增长。
        let mut scoped = guard
            .keys()
            .filter(|(chat_id, user_id, _)| {
                *chat_id == request_chat_id && *user_id == sender_user_id
            })
            .copied()
            .collect::<Vec<_>>();
        scoped.sort_by_key(|(_, _, message_id)| *message_id);
        let remove_count = scoped.len().saturating_sub(8);
        for key in scoped.into_iter().take(remove_count) {
            guard.remove(&key);
        }
    }

    pub(crate) fn take_plan(
        &self,
        request_chat_id: i64,
        sender_user_id: i64,
        message_id: i64,
    ) -> Option<crate::tgbot::transfer::types::TransferPlan> {
        recover_rwlock_write(&self.by_message, "retransfer confirm context").remove(&(
            request_chat_id,
            sender_user_id,
            message_id,
        ))
    }
}

fn prune_lookup_retry_entries(
    entries: &mut HashMap<(i64, i64, i64), LookupRetryEntry>,
    request_chat_id: i64,
    sender_user_id: i64,
) {
    let mut scoped = entries
        .iter()
        .filter(|((chat_id, user_id, _), _)| {
            *chat_id == request_chat_id && *user_id == sender_user_id
        })
        .map(|(key, entry)| (*key, entry.sequence))
        .collect::<Vec<_>>();
    if scoped.len() <= LOOKUP_RETRY_CONTEXT_LIMIT_PER_USER {
        return;
    }
    scoped.sort_by_key(|(_, sequence)| *sequence);
    let remove_count = scoped.len() - LOOKUP_RETRY_CONTEXT_LIMIT_PER_USER;
    for (key, _) in scoped.into_iter().take(remove_count) {
        entries.remove(&key);
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

pub struct TransferRuntimeState {
    runtime_config: RwLock<TransferConfig>,
    runtime_default_config: RwLock<TransferConfig>,
    tdlib_files_directories: RwLock<HashMap<ClientRole, PathBuf>>,
    active_transfer_jobs: AtomicUsize,
    transfer_slot_notify: tokio::sync::Notify,
    accepting_new_transfers: AtomicBool,
    admitted_transfer_jobs: AtomicUsize,
    transfer_admission_notify: tokio::sync::Notify,
    transfer_client_ids: RwLock<Option<TransferClientIds>>,
    background_services_started: AtomicBool,
}

impl Default for TransferRuntimeState {
    fn default() -> Self {
        Self {
            runtime_config: RwLock::new(TransferConfig::default()),
            runtime_default_config: RwLock::new(TransferConfig::default()),
            tdlib_files_directories: RwLock::new(HashMap::new()),
            active_transfer_jobs: AtomicUsize::new(0),
            transfer_slot_notify: tokio::sync::Notify::new(),
            accepting_new_transfers: AtomicBool::new(true),
            admitted_transfer_jobs: AtomicUsize::new(0),
            transfer_admission_notify: tokio::sync::Notify::new(),
            transfer_client_ids: RwLock::new(None),
            background_services_started: AtomicBool::new(false),
        }
    }
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

    /// 接纳一项新建或恢复转存。执行器排空期间返回 `None`，已接纳的任务不受影响。
    pub fn try_admit_transfer(self: &Arc<Self>) -> Option<TransferAdmissionGuard> {
        loop {
            if !self.accepting_new_transfers.load(Ordering::SeqCst) {
                return None;
            }
            let current = self.admitted_transfer_jobs.load(Ordering::SeqCst);
            if self
                .admitted_transfer_jobs
                .compare_exchange(current, current + 1, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                // `begin_transfer_drain` 可能刚好发生在 compare-exchange 之后；撤销这次
                // 接纳，确保排空开始后的新任务不会漏进来。
                if self.accepting_new_transfers.load(Ordering::SeqCst) {
                    return Some(TransferAdmissionGuard {
                        state: self.clone(),
                    });
                }
                self.admitted_transfer_jobs.fetch_sub(1, Ordering::SeqCst);
                self.transfer_admission_notify.notify_waiters();
                return None;
            }
        }
    }

    pub fn begin_transfer_drain(&self) {
        self.accepting_new_transfers.store(false, Ordering::SeqCst);
        self.transfer_admission_notify.notify_waiters();
    }

    pub fn cancel_transfer_drain(&self) {
        self.accepting_new_transfers.store(true, Ordering::SeqCst);
        self.transfer_admission_notify.notify_waiters();
    }

    pub async fn wait_for_transfer_drain(&self) {
        loop {
            let notified = self.transfer_admission_notify.notified();
            if self.admitted_transfer_jobs.load(Ordering::SeqCst) == 0 {
                return;
            }
            notified.await;
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

/// 从任务创建到后台 workflow 结束的接纳凭证。
pub struct TransferAdmissionGuard {
    state: Arc<TransferRuntimeState>,
}

impl Drop for TransferAdmissionGuard {
    fn drop(&mut self) {
        self.state
            .admitted_transfer_jobs
            .fetch_sub(1, Ordering::SeqCst);
        self.state.transfer_admission_notify.notify_waiters();
    }
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

#[derive(Debug, Clone, Default)]
pub struct JobUploadProgressSnapshot {
    pub active_files: i32,
    pub uploaded_size: i64,
    pub total_size: i64,
    pub has_unknown_total: bool,
}

#[derive(Debug, Clone)]
struct UploadFileProgressSnapshot {
    file_id: i32,
    uploaded_size: i64,
    total_size: Option<i64>,
}

#[derive(Default)]
struct UploadProgressState {
    /// 逻辑上传项是聚合主键；TDLib 替换 File ID 时不会增加文件数。
    by_item: HashMap<(i32, i64, i64), UploadFileProgressSnapshot>,
    /// UpdateFile 只携带 client/file ID，用反向索引定位所属任务条目。
    by_file: HashMap<(i32, i32), (i64, i64)>,
}

/// TDLib 上传 file ID 与转存任务的运行时关联。
///
/// 上传 file ID 由 `sendMessage` 返回，不能从下载阶段的 file_cache 推导；这里按 client
/// 隔离保存，并在任务详情读取时按 job 聚合。
#[derive(Default)]
pub struct UploadProgressStore {
    state: RwLock<UploadProgressState>,
}

impl UploadProgressStore {
    pub fn register_upload_file(
        &self,
        client_id: i32,
        job_id: i64,
        item_id: i64,
        file: &tdlib_rs::types::File,
    ) {
        let mut guard = recover_rwlock_write(&self.state, "upload progress");
        let item_key = (client_id, job_id, item_id);
        let previous = guard.by_item.remove(&item_key);
        if let Some(previous) = &previous {
            guard.by_file.remove(&(client_id, previous.file_id));
        }
        let previous_uploaded_size = previous
            .as_ref()
            .map(|snapshot| snapshot.uploaded_size)
            .unwrap_or(0);
        let previous_total_size = previous.as_ref().and_then(|snapshot| snapshot.total_size);

        guard.by_item.insert(
            item_key,
            UploadFileProgressSnapshot {
                file_id: file.id,
                // TDLib 替换临时 File ID 时，新对象的 uploaded_size 可能短暂回到 0。
                // 同一逻辑上传项的已上传字节必须保持单调递增。
                uploaded_size: file.remote.uploaded_size.max(previous_uploaded_size).max(0),
                total_size: file_total_size(file).or(previous_total_size),
            },
        );
        guard
            .by_file
            .insert((client_id, file.id), (job_id, item_id));
    }

    pub fn update_upload_progress(&self, client_id: i32, file: &tdlib_rs::types::File) {
        let mut guard = recover_rwlock_write(&self.state, "upload progress");
        let Some((job_id, item_id)) = guard.by_file.get(&(client_id, file.id)).copied() else {
            return;
        };
        let Some(snapshot) = guard.by_item.get_mut(&(client_id, job_id, item_id)) else {
            return;
        };
        // UpdateFile 可能乱序到达；旧快照不能让已经展示的上传进度倒退。
        snapshot.uploaded_size = snapshot.uploaded_size.max(file.remote.uploaded_size).max(0);
        if snapshot.total_size.is_none() {
            snapshot.total_size = file_total_size(file);
        }
    }

    /// 消息确认发送成功后，把对应逻辑项收敛到已知总大小。
    ///
    /// TDLib 不保证在 MessageSendSucceeded 之前再发一条 uploaded_size == size 的
    /// UpdateFile，因此不能只依赖文件事件显示最终 100%。
    pub fn mark_upload_item_complete(&self, client_id: i32, job_id: i64, item_id: i64) {
        let mut guard = recover_rwlock_write(&self.state, "upload progress");
        let Some(snapshot) = guard.by_item.get_mut(&(client_id, job_id, item_id)) else {
            return;
        };
        if let Some(total_size) = snapshot.total_size {
            snapshot.uploaded_size = snapshot.uploaded_size.max(total_size);
        }
    }

    pub fn get_job_upload_progress(
        &self,
        client_id: i32,
        job_id: i64,
    ) -> Option<JobUploadProgressSnapshot> {
        let guard = recover_rwlock_read(&self.state, "upload progress");
        let mut progress = JobUploadProgressSnapshot::default();
        for snapshot in guard.by_item.iter().filter_map(
            |((snapshot_client_id, snapshot_job_id, _), snapshot)| {
                (*snapshot_client_id == client_id && *snapshot_job_id == job_id).then_some(snapshot)
            },
        ) {
            progress.active_files = progress.active_files.saturating_add(1);
            progress.uploaded_size = progress
                .uploaded_size
                .saturating_add(snapshot.uploaded_size.max(0));
            if let Some(total_size) = snapshot.total_size {
                progress.total_size = progress.total_size.saturating_add(total_size.max(0));
            } else {
                progress.has_unknown_total = true;
            }
        }
        (progress.active_files > 0).then_some(progress)
    }

    pub fn clear_job(&self, client_id: i32, job_id: i64) {
        let mut guard = recover_rwlock_write(&self.state, "upload progress");
        guard
            .by_item
            .retain(|(snapshot_client_id, snapshot_job_id, _), _| {
                *snapshot_client_id != client_id || *snapshot_job_id != job_id
            });
        guard
            .by_file
            .retain(|(snapshot_client_id, _), (snapshot_job_id, _)| {
                *snapshot_client_id != client_id || *snapshot_job_id != job_id
            });
    }

    pub fn job_guard(self: &Arc<Self>, client_id: i32, job_id: i64) -> UploadProgressJobGuard {
        UploadProgressJobGuard {
            store: self.clone(),
            client_id,
            job_id,
        }
    }
}

pub struct UploadProgressJobGuard {
    store: Arc<UploadProgressStore>,
    client_id: i32,
    job_id: i64,
}

impl Drop for UploadProgressJobGuard {
    fn drop(&mut self) {
        self.store.clear_job(self.client_id, self.job_id);
    }
}

fn file_total_size(file: &tdlib_rs::types::File) -> Option<i64> {
    if file.size > 0 {
        Some(file.size)
    } else if file.expected_size > 0 {
        Some(file.expected_size)
    } else {
        None
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
                let send_value = result.as_ref().map(|_| ()).map_err(|e| format!("{e:#}"));
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
                        .map_err(|e| anyhow::anyhow!("{e}"));
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
    fn executor_runtime_keeps_user_client_lifecycle_separate_from_bot() {
        let state = ExecutorRuntimeState::default();

        state.begin_login(71, 1001);
        assert_eq!(state.phase(), ExecutorPhase::Starting);
        assert_eq!(state.role_for_client_id(71), Some(ClientRole::User));
        assert!(state.request_qr_if_starting(71));
        assert_eq!(state.phase(), ExecutorPhase::WaitingQr);
        assert!(state.set_waiting_password(71));
        assert_eq!(state.phase(), ExecutorPhase::WaitingPassword);
        assert!(state.mark_ready(71));
        assert_eq!(state.phase(), ExecutorPhase::Ready);
        assert!(state.begin_draining(71));
        assert_eq!(state.phase(), ExecutorPhase::Draining);
        assert!(state.cancel_draining(71));
        assert_eq!(state.phase(), ExecutorPhase::Ready);
        assert!(state.clear_user_client_if(71));
        assert_eq!(state.phase(), ExecutorPhase::Offline);
        assert_eq!(state.user_client_id(), None);
    }

    #[test]
    fn executor_runtime_keeps_qr_message_and_identity_with_current_session() {
        let state = ExecutorRuntimeState::default();
        state.begin_login(71, 1001);
        assert_eq!(state.replace_qr_message_id(500), None);
        assert_eq!(state.qr_message_id(), Some(500));
        assert!(state.mark_ready(71));
        assert!(state.set_identity_if_ready(
            71,
            ExecutorIdentity {
                user_id: 2002,
                display_name: "测试账号".to_owned(),
                username: Some("tester".to_owned()),
            },
        ));
        assert_eq!(state.identity().expect("executor identity").user_id, 2002);

        assert!(state.clear_user_client_if(71));
        assert_eq!(state.qr_message_id(), None);
        assert_eq!(state.identity(), None);
    }

    #[tokio::test]
    async fn transfer_admission_rejects_new_work_while_drain_waits_for_existing_work() {
        let state = Arc::new(TransferRuntimeState::default());
        let guard = state.try_admit_transfer().expect("initial admission");

        state.begin_transfer_drain();
        assert!(state.try_admit_transfer().is_none());
        assert!(
            tokio::time::timeout(Duration::from_millis(10), state.wait_for_transfer_drain())
                .await
                .is_err()
        );

        drop(guard);
        tokio::time::timeout(Duration::from_millis(100), state.wait_for_transfer_drain())
            .await
            .expect("drain should finish after admitted task exits");
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

    #[test]
    fn upload_progress_store_clears_stale_snapshot_before_restart() {
        let store = UploadProgressStore::default();
        let mut first = test_file(51, 0, 1000, false, false);
        first.remote.is_uploading_active = true;
        first.remote.uploaded_size = 250;
        let mut second = test_file(52, 0, 3000, false, false);
        second.remote.is_uploading_active = true;
        second.remote.uploaded_size = 750;

        store.register_upload_file(10, 7, 101, &first);
        store.register_upload_file(10, 7, 102, &second);
        store.update_upload_progress(10, &first);
        store.update_upload_progress(10, &second);

        let progress = store
            .get_job_upload_progress(10, 7)
            .expect("job upload progress");
        assert_eq!(progress.active_files, 2);
        assert_eq!(progress.uploaded_size, 1000);
        assert_eq!(progress.total_size, 4000);
        assert!(!progress.has_unknown_total);
        assert!(store.get_job_upload_progress(20, 7).is_none());

        store.clear_job(10, 7);
        assert!(store.get_job_upload_progress(10, 7).is_none());

        // 暂停后恢复会创建一轮新的 TDLib 上传；只能从新文件快照重新计数。
        let mut restarted = test_file(53, 0, 2000, false, false);
        restarted.remote.is_uploading_active = true;
        restarted.remote.uploaded_size = 100;
        store.register_upload_file(10, 7, 101, &restarted);

        let restarted_progress = store
            .get_job_upload_progress(10, 7)
            .expect("restarted job upload progress");
        assert_eq!(restarted_progress.active_files, 1);
        assert_eq!(restarted_progress.uploaded_size, 100);
        assert_eq!(restarted_progress.total_size, 2000);
        assert!(!restarted_progress.has_unknown_total);
    }

    // TDLib 可能在消息发送完成后替换 File ID；同一上传项只能计数一次。
    #[test]
    fn upload_progress_store_replaces_file_id_for_same_item() {
        let store = UploadProgressStore::default();
        let mut temporary = test_file(61, 0, 1000, false, false);
        temporary.remote.is_uploading_active = true;
        temporary.remote.uploaded_size = 300;
        let mut final_file = test_file(62, 0, 1000, false, false);
        final_file.remote.is_uploading_active = true;
        final_file.remote.uploaded_size = 700;

        store.register_upload_file(10, 7, 101, &temporary);
        store.register_upload_file(10, 7, 101, &final_file);
        // 旧 File ID 的迟到事件不能覆盖当前上传项。
        temporary.remote.uploaded_size = 900;
        store.update_upload_progress(10, &temporary);

        let progress = store
            .get_job_upload_progress(10, 7)
            .expect("job upload progress");
        assert_eq!(progress.active_files, 1);
        assert_eq!(progress.uploaded_size, 700);
        assert_eq!(progress.total_size, 1000);
    }

    // 最终 File 对象可能从 0 重新开始上报；替换 ID 时不能让进度倒退。
    #[test]
    fn upload_progress_store_keeps_progress_when_replacement_starts_at_zero() {
        let store = UploadProgressStore::default();
        let mut temporary = test_file(71, 0, 1000, false, false);
        temporary.remote.uploaded_size = 700;
        let mut final_file = test_file(72, 0, 1000, false, false);
        final_file.remote.uploaded_size = 0;

        store.register_upload_file(10, 7, 101, &temporary);
        store.register_upload_file(10, 7, 101, &final_file);

        let progress = store
            .get_job_upload_progress(10, 7)
            .expect("job upload progress");
        assert_eq!(progress.uploaded_size, 700);
        assert_eq!(progress.total_size, 1000);
    }

    // 消息发送成功是权威完成信号，即使最后一条 UpdateFile 缺失也应显示 100%。
    #[test]
    fn upload_progress_store_marks_item_complete() {
        let store = UploadProgressStore::default();
        let mut file = test_file(81, 0, 1000, false, false);
        file.remote.uploaded_size = 700;
        store.register_upload_file(10, 7, 101, &file);

        store.mark_upload_item_complete(10, 7, 101);

        let progress = store
            .get_job_upload_progress(10, 7)
            .expect("job upload progress");
        assert_eq!(progress.uploaded_size, 1000);
        assert_eq!(progress.total_size, 1000);
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

    #[test]
    fn lookup_retry_state_is_scoped_by_chat_user_and_message() {
        let state = LookupRetryState::default();
        state.put_context(
            1,
            2,
            3,
            LookupRetryContext {
                source_link: "https://t.me/c/1/2".to_owned(),
                target_chat_id: -100,
            },
        );

        assert!(state.take_context(1, 2, 4).is_none());
        assert_eq!(
            state.take_context(1, 2, 3),
            Some(LookupRetryContext {
                source_link: "https://t.me/c/1/2".to_owned(),
                target_chat_id: -100,
            })
        );
        assert!(state.take_context(1, 2, 3).is_none());
    }

    #[test]
    fn lookup_retry_state_keeps_multiple_recent_contexts_and_prunes_oldest() {
        let state = LookupRetryState::default();
        for index in 0..=LOOKUP_RETRY_CONTEXT_LIMIT_PER_USER {
            state.put_context(
                1,
                2,
                i64::try_from(index).expect("index should fit i64"),
                LookupRetryContext {
                    source_link: format!("https://t.me/c/1/{index}"),
                    target_chat_id: -100 - i64::try_from(index).expect("index should fit i64"),
                },
            );
        }

        assert!(state.take_context(1, 2, 0).is_none());
        assert_eq!(
            state.take_context(1, 2, 1),
            Some(LookupRetryContext {
                source_link: "https://t.me/c/1/1".to_owned(),
                target_chat_id: -101,
            })
        );
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
