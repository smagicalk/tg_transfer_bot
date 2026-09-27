//! 进程内全局共享运行时状态管理。
//!
//! 提供转存机器人生命周期内所需的全部共享组件，包括：
//! - 权限与动态白名单控制 (`AccessControlState`)
//! - 转存任务并发槽位与排空状态 (`TransferRuntimeState`)
//! - 目标频道映射与运行时配置 (`TargetsRuntimeState`)
//! - TDLib 下载进度与上传进度监听快照 (`DownloadProgressStore`, `UploadProgressStore`)
//! - 下载防重合并与 Singleflight 调度 (`InflightDownloadRegistry`)
//! - 防重执行 Guard 互斥原语 (`TransferExecutionGuards`)
//! - 机器人键盘/按钮等发送能力开关 (`SendCapabilities`)
//! - 用户执行器按需登录与生命周期状态 (`ExecutorRuntimeState`)
//! - 解析失败重试上下文与二次确认计划卡片缓存 (`LookupRetryState`, `RetransferConfirmState`)

use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::time::Duration;

use crate::config::{ClientRole, TargetsConfig, TransferClientIds, TransferConfig};

/// 全局懒加载单例实例。
static APP_CONTEXT: LazyLock<Arc<AppContext>> = LazyLock::new(|| Arc::new(AppContext::default()));

/// 获取全局唯一的应用上下文实例句柄。
pub(crate) fn app_context() -> Arc<AppContext> {
    APP_CONTEXT.clone()
}

/// 应用程序全局状态集合体。
///
/// 内部所有字段均使用 `Arc` 包裹，并在其内部封装适当的并发原语（`RwLock`、`Mutex` 或原子类型），
/// 因而整体结构体克隆成本极低，可安全在多个异步任务与 TDLib 事件循环间传递。
#[derive(Clone)]
pub struct AppContext {
    /// 用户鉴权与动态白名单访问控制状态。
    pub(crate) access_control: Arc<AccessControlState>,
    /// 转存任务并发数限制、排空控制及运行时转存配置。
    pub(crate) transfer_runtime: Arc<TransferRuntimeState>,
    /// 目标频道与映射分类运行时配置。
    pub(crate) targets_runtime: Arc<TargetsRuntimeState>,
    /// TDLib 文件下载进度内存快照存储。
    pub(crate) download_progress: Arc<DownloadProgressStore>,
    /// TDLib 文件上传进度内存快照存储及反向索引。
    pub(crate) upload_progress: Arc<UploadProgressStore>,
    /// 下载中的文件防击穿/Singleflight 调度注册表。
    pub(crate) inflight_downloads: Arc<InflightDownloadRegistry>,
    /// 任务与目标频道的互斥执行 Guard 集合，防止相同任务重复拉起。
    pub(crate) transfer_guards: Arc<TransferExecutionGuards>,
    /// 机器人消息发送能力特性探测（例如是否支持内联按钮 reply_markup）。
    pub(crate) send_capabilities: Arc<SendCapabilities>,
    /// 按需登录的用户执行器运行时生命周期与登录凭据状态。
    pub(crate) executor_runtime: Arc<ExecutorRuntimeState>,
    /// 链接解析失败后供重试回调查询的会话上下文缓存。
    pub(crate) lookup_retry: Arc<LookupRetryState>,
    /// 再次转存确认卡片的短期计划缓存（避免 callback_data 超长）。
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

/// 按需登录用户执行器的运行状态阶段。
///
/// Bot 始终独立运行；用户执行器只在需要读取私有源或 Bot 权限不足时由 owner 登录。
/// 状态只保存 client 与交互定位，不保存二维码链接、密码或其他登录凭据。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecutorPhase {
    /// 离线状态：未登录或已登出。
    #[default]
    Offline,
    /// 正在启动执行器客户端。
    Starting,
    /// 等待用户扫描二维码登录。
    WaitingQr,
    /// 等待用户输入两步验证密码。
    WaitingPassword,
    /// 就绪状态：执行器已正常登录并可承接转存任务。
    Ready,
    /// 排空状态：正在等待正在处理的任务完成，准备登出或停机。
    Draining,
    /// 正在执行登出流程。
    LoggingOut,
}

/// 按需用户执行器（User Client）的运行时内存状态。
#[derive(Default)]
pub struct ExecutorRuntimeState {
    /// 当前执行器所处的生命周期阶段。
    phase: RwLock<ExecutorPhase>,
    /// 当前用户客户端在 TDLib 内部注册的 Client ID（若已创建）。
    user_client_id: RwLock<Option<i32>>,
    /// 拥有者（Owner）的 Telegram Chat ID，用于定向推送二维码和登录通知。
    owner_chat_id: RwLock<Option<i64>>,
    /// 本地生成的二维码图片临时路径。
    qr_image_path: RwLock<Option<PathBuf>>,
    /// 发送给用户的二维码消息 ID，刷新二维码时可直接编辑该消息。
    qr_message_id: RwLock<Option<i64>>,
    /// 提示输入两步验证密码的消息 ID。
    password_prompt_message_id: RwLock<Option<i64>>,
    /// 当前已登录用户执行器的身份摘要信息。
    identity: RwLock<Option<ExecutorIdentity>>,
}

/// 已登录执行器的非敏感账号摘要，用于 owner 在面板中确认当前会话。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutorIdentity {
    /// Telegram 用户 ID。
    pub user_id: i64,
    /// 账号昵称（First Name + Last Name）。
    pub display_name: String,
    /// 账号用户名（不带 @ 前缀）。
    pub username: Option<String>,
}

impl ExecutorRuntimeState {
    /// 获取当前用户执行器的运行生命周期阶段。
    pub fn phase(&self) -> ExecutorPhase {
        *recover_rwlock_read(&self.phase, "executor phase")
    }

    /// 获取当前绑定的 TDLib User Client ID。
    pub fn user_client_id(&self) -> Option<i32> {
        *recover_rwlock_read(&self.user_client_id, "executor user client id")
    }

    /// 获取发起登录的 Owner Chat ID。
    pub fn owner_chat_id(&self) -> Option<i64> {
        *recover_rwlock_read(&self.owner_chat_id, "executor owner chat id")
    }

    /// 标记开始登录流程，重置二维码与身份，并将阶段置为 `Starting`。
    pub fn begin_login(&self, user_client_id: i32, owner_chat_id: i64) {
        *recover_rwlock_write(&self.user_client_id, "executor user client id") =
            Some(user_client_id);
        *recover_rwlock_write(&self.owner_chat_id, "executor owner chat id") = Some(owner_chat_id);
        *recover_rwlock_write(&self.qr_message_id, "executor qr message id") = None;
        *recover_rwlock_write(&self.identity, "executor identity") = None;
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Starting;
    }

    /// 若给定的 client_id 与当前用户执行器相匹配，则返回 `ClientRole::User`。
    pub fn role_for_client_id(&self, client_id: i32) -> Option<ClientRole> {
        (self.user_client_id() == Some(client_id)).then_some(ClientRole::User)
    }

    /// 若当前阶段处于 `Starting`，将其转换至 `WaitingQr` 状态以请求二维码。
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

    /// 将执行器阶段设为等待两步验证密码 (`WaitingPassword`)。
    pub fn set_waiting_password(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::WaitingPassword;
        true
    }

    /// 标记执行器已就绪 (`Ready`)。
    pub fn mark_ready(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Ready;
        true
    }

    /// 标记执行器正在登出 (`LoggingOut`)。
    pub fn mark_logging_out(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::LoggingOut;
        true
    }

    /// 开始排空当前用户执行器中的存量转存任务。
    pub fn begin_draining(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) || self.phase() != ExecutorPhase::Ready {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Draining;
        true
    }

    /// 取消排空状态，恢复就绪 (`Ready`)。
    pub fn cancel_draining(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) || self.phase() != ExecutorPhase::Draining {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Ready;
        true
    }

    /// 登出失败时回滚阶段至就绪 (`Ready`)。
    pub fn restore_ready_after_logout_failure(&self, client_id: i32) -> bool {
        if self.user_client_id() != Some(client_id) || self.phase() != ExecutorPhase::LoggingOut {
            return false;
        }
        *recover_rwlock_write(&self.phase, "executor phase") = ExecutorPhase::Ready;
        true
    }

    /// 若给定的 client_id 匹配，则清理全部用户执行器会话状态并重置为离线 (`Offline`)。
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

    /// 替换保存的二维码图片文件路径，返回先前的路径（若有）。
    pub fn replace_qr_image_path(&self, path: PathBuf) -> Option<PathBuf> {
        recover_rwlock_write(&self.qr_image_path, "executor qr image path").replace(path)
    }

    /// 取出并移除二维码图片文件路径。
    pub fn take_qr_image_path(&self) -> Option<PathBuf> {
        recover_rwlock_write(&self.qr_image_path, "executor qr image path").take()
    }

    /// 保存首次发送的二维码消息，后续二维码刷新时只编辑该消息。
    pub fn replace_qr_message_id(&self, message_id: i64) -> Option<i64> {
        recover_rwlock_write(&self.qr_message_id, "executor qr message id").replace(message_id)
    }

    /// 获取当前展示二维码的消息 ID。
    pub fn qr_message_id(&self) -> Option<i64> {
        *recover_rwlock_read(&self.qr_message_id, "executor qr message id")
    }

    /// 若执行器处于就绪状态，设置当前账号的身份摘要。
    pub fn set_identity_if_ready(&self, client_id: i32, identity: ExecutorIdentity) -> bool {
        if self.user_client_id() != Some(client_id) || self.phase() != ExecutorPhase::Ready {
            return false;
        }
        *recover_rwlock_write(&self.identity, "executor identity") = Some(identity);
        true
    }

    /// 获取当前登录账号的身份摘要（若已就绪）。
    pub fn identity(&self) -> Option<ExecutorIdentity> {
        recover_rwlock_read(&self.identity, "executor identity").clone()
    }

    /// 保存或替换两步验证密码提示消息的 ID。
    pub fn replace_password_prompt_message_id(&self, message_id: i64) -> Option<i64> {
        recover_rwlock_write(
            &self.password_prompt_message_id,
            "executor password prompt message id",
        )
        .replace(message_id)
    }

    /// 取出并清除两步验证密码提示消息的 ID。
    pub fn take_password_prompt_message_id(&self) -> Option<i64> {
        recover_rwlock_write(
            &self.password_prompt_message_id,
            "executor password prompt message id",
        )
        .take()
    }

    /// 获取当前两步验证密码提示消息的 ID。
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
    /// 已授权使用机器人的 Telegram 用户 ID 集合。
    authorized_user_ids: RwLock<HashSet<i64>>,
}

impl AccessControlState {
    /// 用启动时从数据库读取的完整名单替换当前状态。
    pub fn replace_authorized_user_ids(&self, user_ids: impl IntoIterator<Item = i64>) {
        let mut guard = recover_rwlock_write(&self.authorized_user_ids, "authorized user ids");
        guard.clear();
        guard.extend(user_ids.into_iter().filter(|user_id| *user_id > 0));
    }

    /// 检查指定用户 ID 是否在授权白名单中。
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

/// 链接解析重试的会话上下文。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupRetryContext {
    /// 触发解析失败的原始源链接。
    pub source_link: String,
    /// 用户选定或默认的目标频道/群组 Chat ID。
    pub target_chat_id: i64,
}

/// 解析失败重试上下文管理器。
#[derive(Default)]
pub struct LookupRetryState {
    /// 按 `(request_chat_id, sender_user_id, message_id)` 定位的上下文条目映射。
    by_message: RwLock<HashMap<(i64, i64, i64), LookupRetryEntry>>,
    /// 单调递增的序列号生成器，用于淘汰最旧条目。
    sequence: AtomicUsize,
}

/// 携带插入顺序序号的重试上下文条目。
#[derive(Debug, Clone, PartialEq, Eq)]
struct LookupRetryEntry {
    /// 解析重试上下文。
    context: LookupRetryContext,
    /// 插入时的全局时序编号。
    sequence: usize,
}

impl LookupRetryState {
    /// 存入解析重试上下文，并限制每个用户的历史条目数量。
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

    /// 取出并移除指定消息的解析重试上下文（一次性消费）。
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

/// 单个用户在内存中保留的最近解析重试上下文上限。
const LOOKUP_RETRY_CONTEXT_LIMIT_PER_USER: usize = 8;

/// “再次转存”确认卡的短期上下文。
///
/// callback_data 不能容纳完整源链接，因此按卡片消息定位保存计划；确认后立即消费。
#[derive(Default)]
pub struct RetransferConfirmState {
    /// 键为 `(request_chat_id, sender_user_id, message_id)`，值为对应的转存执行计划。
    by_message: RwLock<HashMap<(i64, i64, i64), crate::tgbot::transfer::types::TransferPlan>>,
}

impl RetransferConfirmState {
    /// 存入二次确认计划卡片上下文。
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

    /// 取出并消费二次确认转存计划。
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

/// 淘汰超出限制的最旧解析重试上下文条目。
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

/// 目标频道配置的运行时动态容器。
#[derive(Default)]
pub struct TargetsRuntimeState {
    /// 当前生效的目标频道配置。
    runtime_config: RwLock<TargetsConfig>,
    /// 配置文件或数据库中持久化的初始默认配置（用于重置/比对）。
    runtime_default_config: RwLock<TargetsConfig>,
}

impl TargetsRuntimeState {
    /// 同时初始化运行时配置与默认基准配置。
    pub fn init_runtime_config(&self, config: TargetsConfig, default_config: TargetsConfig) {
        self.set_runtime_default_config(default_config);
        self.update_runtime_config(config);
    }

    /// 动态热更新当前目标频道配置。
    pub fn update_runtime_config(&self, config: TargetsConfig) {
        *recover_rwlock_write(&self.runtime_config, "targets runtime config") = config;
    }

    /// 获取当前生效的目标频道配置快照。
    pub fn runtime_config(&self) -> TargetsConfig {
        recover_rwlock_read(&self.runtime_config, "targets runtime config").clone()
    }

    /// 设置默认目标频道配置。
    pub fn set_runtime_default_config(&self, config: TargetsConfig) {
        *recover_rwlock_write(
            &self.runtime_default_config,
            "targets runtime default config",
        ) = config;
    }

    /// 获取默认目标频道配置快照。
    pub fn runtime_default_config(&self) -> TargetsConfig {
        recover_rwlock_read(
            &self.runtime_default_config,
            "targets runtime default config",
        )
        .clone()
    }
}

/// 消息发送能力探测开关。
pub struct SendCapabilities {
    /// 是否支持附带内联键盘等 reply_markup 发送消息。
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
    /// 设置是否允许发送 reply_markup。
    pub fn set_reply_markup_enabled(&self, enabled: bool) {
        self.reply_markup_enabled.store(enabled, Ordering::Relaxed);
    }

    /// 查询当前是否允许发送 reply_markup。
    pub fn reply_markup_enabled(&self) -> bool {
        self.reply_markup_enabled.load(Ordering::Relaxed)
    }
}

/// 转存核心引擎的运行时控制状态。
///
/// 负责并发度管控（槽位申请与释放）、平滑排空（Drain）、双客户端 ID 关联以及本地文件路径索引。
pub struct TransferRuntimeState {
    /// 当前生效的转存运行时配置（如并发数限制、单任务最大重试次数等）。
    runtime_config: RwLock<TransferConfig>,
    /// 默认基准配置（用于重置或比对偏差）。
    runtime_default_config: RwLock<TransferConfig>,
    /// 按客户端角色（Bot / User）索引的 TDLib 本地工作目录与文件缓存路径。
    tdlib_files_directories: RwLock<HashMap<ClientRole, PathBuf>>,
    /// 当前正在执行中的任务数（占用实际并发槽位）。
    active_transfer_jobs: AtomicUsize,
    /// 槽位释放或并发上限调大时的唤醒通知器。
    transfer_slot_notify: tokio::sync::Notify,
    /// 是否接纳新的转存请求（平滑停机或排空维护时置为 `false`）。
    accepting_new_transfers: AtomicBool,
    /// 已被系统接纳进入调度生命周期的任务总数。
    admitted_transfer_jobs: AtomicUsize,
    /// 接纳状态变动或接纳任务数归零时的排空通知器。
    transfer_admission_notify: tokio::sync::Notify,
    /// 当前启用的双客户端 ID 对（包含 Bot Client 与可选的 User Client）。
    transfer_client_ids: RwLock<Option<TransferClientIds>>,
    /// 标记后台恢复与定期清理服务是否已拉起（确保单例执行）。
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
    /// 启动时批量初始化转存配置、默认基准配置与 TDLib 目录映射。
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

    /// 热更新当前转存配置，并唤醒可能正在等待并发槽位的任务。
    pub fn update_runtime_config(&self, config: TransferConfig) {
        *recover_rwlock_write(&self.runtime_config, "transfer runtime config") = config;
        self.transfer_slot_notify.notify_waiters();
    }

    /// 获取当前转存运行时配置快照。
    pub fn runtime_config(&self) -> TransferConfig {
        recover_rwlock_read(&self.runtime_config, "transfer runtime config").clone()
    }

    /// 设置默认转存基准配置。
    pub fn set_runtime_default_config(&self, config: TransferConfig) {
        *recover_rwlock_write(
            &self.runtime_default_config,
            "transfer runtime default config",
        ) = config;
    }

    /// 获取默认转存基准配置快照。
    pub fn runtime_default_config(&self) -> TransferConfig {
        recover_rwlock_read(
            &self.runtime_default_config,
            "transfer runtime default config",
        )
        .clone()
    }

    /// 查询指定角色客户端在本地使用的 TDLib 文件目录路径。
    pub fn tdlib_files_directory_for(&self, role: ClientRole) -> Option<PathBuf> {
        recover_rwlock_read(&self.tdlib_files_directories, "tdlib files directory")
            .get(&role)
            .cloned()
    }

    /// 查询当前占用并发槽位的活跃任务数。
    pub fn active_transfer_jobs_count(&self) -> usize {
        self.active_transfer_jobs.load(Ordering::SeqCst)
    }

    /// 异步申请一个任务执行并发槽位。若达到并发上限则挂起等待直到有槽位释放。
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

    /// 开启平滑排空，停止接纳任何新的转存任务。
    pub fn begin_transfer_drain(&self) {
        self.accepting_new_transfers.store(false, Ordering::SeqCst);
        self.transfer_admission_notify.notify_waiters();
    }

    /// 取消平滑排空，重新开始接纳转存任务。
    pub fn cancel_transfer_drain(&self) {
        self.accepting_new_transfers.store(true, Ordering::SeqCst);
        self.transfer_admission_notify.notify_waiters();
    }

    /// 异步等待当前已接纳的所有存量任务全部执行结束（接纳计数归零）。
    pub async fn wait_for_transfer_drain(&self) {
        loop {
            let notified = self.transfer_admission_notify.notified();
            if self.admitted_transfer_jobs.load(Ordering::SeqCst) == 0 {
                return;
            }
            notified.await;
        }
    }

    /// 设置双客户端 ID 映射关系。
    pub fn set_transfer_client_ids(&self, client_ids: TransferClientIds) {
        *recover_rwlock_write(&self.transfer_client_ids, "transfer client ids") = Some(client_ids);
    }

    /// 获取双客户端 ID 映射快照。
    pub fn transfer_client_ids(&self) -> Option<TransferClientIds> {
        recover_rwlock_read(&self.transfer_client_ids, "transfer client ids")
            .as_ref()
            .copied()
    }

    /// 尝试原子标记后台定时服务已拉起，若此前未拉起则返回 `true`。
    pub fn mark_background_services_started(&self) -> bool {
        self.background_services_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    /// 更新各角色对应的本地目录路径。
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

/// 并发槽位持有凭证（Guard）。
///
/// 当该凭证 Drop 时，自动释放活跃并发槽位并唤醒等待队列中的下一个任务。
pub struct TransferExecGuard {
    /// 关联的转存运行时状态引用。
    state: Arc<TransferRuntimeState>,
}

/// 从任务创建到后台 workflow 结束的接纳凭证。
///
/// 当任务完全退出或异常结束并释放该 Guard 时，接纳任务计数减一并触发排空检查通知。
pub struct TransferAdmissionGuard {
    /// 关联的转存运行时状态引用。
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

/// 单个文件的 TDLib 下载进度快照。
#[derive(Debug, Clone, Default)]
pub struct DownloadProgressSnapshot {
    /// 本地已下载字节数。
    pub downloaded_size: i64,
    /// 文件总大小（字节），未知时为 `None`。
    pub total_size: Option<i64>,
}

/// TDLib 文件下载进度内存存储。
#[derive(Default)]
pub struct DownloadProgressStore {
    /// 按 `(client_id, file_id)` 索引的下载进度快照表。
    snapshots: RwLock<HashMap<(i32, i32), DownloadProgressSnapshot>>,
}

impl DownloadProgressStore {
    /// 根据 TDLib 的 `UpdateFile` 事件更新指定文件的下载进度快照。
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

    /// 获取指定客户端和文件 ID 的当前下载进度快照。
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

/// 转存任务关联的所有文件上传进度聚合快照。
#[derive(Debug, Clone, Default)]
pub struct JobUploadProgressSnapshot {
    /// 当前正在并发上传的文件数。
    pub active_files: i32,
    /// 任务内所有活跃上传文件的已上传字节总和。
    pub uploaded_size: i64,
    /// 任务内所有已知大小文件的总字节数。
    pub total_size: i64,
    /// 是否存在部分文件总大小尚未确定的情况。
    pub has_unknown_total: bool,
}

/// 单个上传文件的临时进度快照。
#[derive(Debug, Clone)]
struct UploadFileProgressSnapshot {
    /// TDLib 中的当前 File ID。
    file_id: i32,
    /// 已上传字节数（保证单调递增）。
    uploaded_size: i64,
    /// 文件总大小。
    total_size: Option<i64>,
}

/// 上传进度内存状态结构体。
#[derive(Default)]
struct UploadProgressState {
    /// 逻辑上传项是聚合主键；TDLib 替换 File ID 时不会增加文件数。
    /// 键为 `(client_id, job_id, item_id)`。
    by_item: HashMap<(i32, i64, i64), UploadFileProgressSnapshot>,
    /// UpdateFile 只携带 client/file ID，用反向索引定位所属任务条目。
    /// 键为 `(client_id, file_id)`，值为 `(job_id, item_id)`。
    by_file: HashMap<(i32, i32), (i64, i64)>,
}

/// TDLib 上传 file ID 与转存任务的运行时关联。
///
/// 上传 file ID 由 `sendMessage` 返回，不能从下载阶段的 file_cache 推导；这里按 client
/// 隔离保存，并在任务详情读取时按 job 聚合。
#[derive(Default)]
pub struct UploadProgressStore {
    /// 上传进度内部状态读写锁。
    state: RwLock<UploadProgressState>,
}

impl UploadProgressStore {
    /// 注册正在发送的文件与逻辑任务条目的关联。
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

    /// 接收 TDLib 的 UpdateFile 事件更新指定文件的上传进度。
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

    /// 聚合计算指定任务下所有活跃上传文件的整体进度快照。
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

    /// 清除指定任务的所有上传进度记录。
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

    /// 创建一个绑定任务生命周期的上传进度 Guard，在任务结束 Drop 时自动清理记录。
    pub fn job_guard(self: &Arc<Self>, client_id: i32, job_id: i64) -> UploadProgressJobGuard {
        UploadProgressJobGuard {
            store: self.clone(),
            client_id,
            job_id,
        }
    }
}

/// 上传进度清理 Guard。
pub struct UploadProgressJobGuard {
    /// 关联的上传进度存储引用。
    store: Arc<UploadProgressStore>,
    /// 客户端 ID。
    client_id: i32,
    /// 任务 ID。
    job_id: i64,
}

impl Drop for UploadProgressJobGuard {
    fn drop(&mut self) {
        self.store.clear_job(self.client_id, self.job_id);
    }
}

/// 从 TDLib File 对象中提取已知的文件总大小（字节）。
fn file_total_size(file: &tdlib_rs::types::File) -> Option<i64> {
    if file.size > 0 {
        Some(file.size)
    } else if file.expected_size > 0 {
        Some(file.expected_size)
    } else {
        None
    }
}

/// 下载结果类型别名（成功或错误字符串）。
type DownloadResult = Result<(), String>;
/// Singleflight 广播通知发送器。
type DownloadNotifier = tokio::sync::watch::Sender<Option<DownloadResult>>;
/// 正在执行中的下载映射表。
type InflightDownloadMap = HashMap<String, DownloadNotifier>;

/// 下载防重击穿（Singleflight）注册中心。
///
/// 当多个转存任务需要同时下载相同源文件时，仅拉起一次实际 TDLib 下载操作，
/// 其余并发请求挂起并订阅首个下载者的完成广播。
#[derive(Default)]
pub struct InflightDownloadRegistry {
    /// 正在进行的下载跟踪表。
    inflight: Mutex<InflightDownloadMap>,
}

impl InflightDownloadRegistry {
    /// 执行 singleflight 任务：如果该 `file_key` 已经在下载中，等待其完成；否则作为执行者执行 `task`。
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

    /// 移除下载项并向所有等待者广播完成结果。
    fn remove_and_notify(&self, file_key: &str, result: DownloadResult) {
        let mut guard = recover_mutex_lock(&self.inflight, "inflight downloads");
        if let Some(tx) = guard.remove(file_key) {
            let _ = tx.send(Some(result));
        }
    }
}

/// Singleflight 请求角色划分。
enum InflightDownloadRole {
    /// 执行者：首个发起请求者，负责实际执行下载任务。
    Executor(InflightExecutionGuard),
    /// 等待者：订阅现有任务的结果广播。
    Waiter(tokio::sync::watch::Receiver<Option<DownloadResult>>),
}

/// 执行者任务守卫，确保无论正常退出还是 panic，都能通知等待者并清理注册表。
struct InflightExecutionGuard {
    /// 注册中心引用。
    registry: Arc<InflightDownloadRegistry>,
    /// 文件唯一键。
    file_key: String,
    /// 是否已正常显式结束。
    finished: bool,
}

impl InflightExecutionGuard {
    /// 创建新的执行者守卫。
    fn new(registry: Arc<InflightDownloadRegistry>, file_key: String) -> Self {
        Self {
            registry,
            file_key,
            finished: false,
        }
    }

    /// 显式标记完成并广播最终结果。
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

/// 转存执行互斥守卫集合。
///
/// 防止同一任务 ID 或同一（源链接 + 目标频道）并发创建导致竞态。
#[derive(Default)]
pub struct TransferExecutionGuards {
    /// 当前正在进程内执行的任务 ID 集合。
    running_job_ids: Mutex<HashSet<i64>>,
    /// 正在创建或排队的 `(source_link, target_chat_id)` 键集合。
    creating_source_targets: Mutex<HashSet<(String, i64)>>,
}

impl TransferExecutionGuards {
    /// 检查指定任务 ID 当前是否正在本进程中运行。
    pub async fn is_job_running_in_process(&self, job_id: i64) -> bool {
        recover_mutex_lock(&self.running_job_ids, "running job id").contains(&job_id)
    }

    /// 尝试获取任务执行独占锁。若已被锁定则返回 `None`。
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

    /// 轮询获取（源链接 + 目标频道）创建锁，防止瞬时并发重复提交。
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

/// 任务执行互斥持有凭证。
pub struct TransferJobGuard {
    /// 守卫集合引用。
    guards: Arc<TransferExecutionGuards>,
    /// 锁定的任务 ID。
    job_id: i64,
}

impl Drop for TransferJobGuard {
    fn drop(&mut self) {
        let mut guard = recover_mutex_lock(&self.guards.running_job_ids, "running job id");
        guard.remove(&self.job_id);
    }
}

/// 源-目标创建防重锁持有凭证。
pub struct SourceTargetCreateGuard {
    /// 守卫集合引用。
    guards: Arc<TransferExecutionGuards>,
    /// 锁定的源链接与目标频道键。
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
