// `/menu` 输入草稿状态。
// 草稿持久化在业务数据库中，真实转存任务仍全部落 transfer_job。

mod db_store;
mod memory;

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use sea_orm::Set;

use crate::db;
use db_store::{
    delete_draft, delete_draft_if_current, find_draft_model, purge_expired, put_draft_unlocked,
    update_draft_if_current,
};

/// 输入草稿索引。
///
/// 输入草稿使用 `(chat_id, user_id)` 做隔离，避免不同会话互相覆盖。
pub(super) type DraftKey = (i64, i64);

/// 最近一次确认执行的目标。
///
/// 这是纯交互优化，不参与转存幂等和任务恢复；进程重启后丢失也不会影响真实任务。
static MENU_LAST_TARGETS: LazyLock<std::sync::Mutex<HashMap<DraftKey, i64>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));
/// 进程内草稿互斥键。
///
/// 数据库主键保证最终只有一行草稿；这里额外保证同进程内的“读出并删除”不会被两个 callback
/// 同时执行，避免确认按钮连点时同一份草稿被消费两次。
static MENU_DRAFT_ACTIVE_KEYS: LazyLock<std::sync::Mutex<HashSet<DraftKey>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashSet::new()));

#[cfg(test)]
pub(super) use memory::clear_last_targets;
pub(super) use memory::{acquire_draft_key_guard, last_target, remember_last_target};

/// 菜单输入流程。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot::transfer::command::menu) enum MenuInputKind {
    Transfer,
    TransferDefault,
    Lookup,
    LookupDefault,
}

impl MenuInputKind {
    /// 是否只需要源链接，目标 chat 交给配置默认值解析。
    pub(super) fn uses_default_target(self) -> bool {
        matches!(self, Self::TransferDefault | Self::LookupDefault)
    }

    /// 归一化到实际命令类型。
    pub(in crate::tgbot::transfer::command::menu) fn command_kind(self) -> Self {
        match self {
            Self::Transfer | Self::TransferDefault => Self::Transfer,
            Self::Lookup | Self::LookupDefault => Self::Lookup,
        }
    }

    /// 当前流程复用的命令名。
    ///
    /// bot 私聊场景优先展示长命令；短命令仍由上层路由兼容。
    pub(super) fn command_name(self) -> &'static str {
        match self {
            Self::Transfer | Self::TransferDefault => "/transfer",
            Self::Lookup | Self::LookupDefault => "/lookup",
        }
    }

    /// 目标选择页标题。
    ///
    /// 这里直接覆盖四种输入类型，不依赖 `command_kind()` 后的不可达分支，避免未来调整
    /// 快速入口时把页面渲染路径变成运行时 panic。
    pub(super) fn target_choice_title(self) -> &'static str {
        match self {
            Self::Transfer | Self::TransferDefault => "选择转存目标",
            Self::Lookup | Self::LookupDefault => "选择查询目标",
        }
    }

    /// 确认页标题。
    pub(super) fn confirm_title(self) -> &'static str {
        match self {
            Self::Transfer | Self::TransferDefault => "确认转存",
            Self::Lookup | Self::LookupDefault => "确认查询",
        }
    }

    /// 源链接输入标题。
    pub(in crate::tgbot::transfer::command::menu) fn source_title(self) -> &'static str {
        match self {
            Self::Transfer => "转存源链接",
            Self::TransferDefault => "快速转存",
            Self::Lookup => "查询源链接",
            Self::LookupDefault => "快速查询",
        }
    }

    /// 源链接输入说明。
    pub(in crate::tgbot::transfer::command::menu) fn source_detail(self) -> &'static str {
        match self {
            Self::Transfer => "请回复要转存的 Telegram 消息或相册链接。",
            Self::TransferDefault => "请回复源链接，目标将使用默认目标（未配置时为当前私聊）。",
            Self::Lookup => "请回复要查询的 Telegram 消息或相册链接。",
            Self::LookupDefault => "请回复源链接，目标将使用默认目标（未配置时为当前私聊）。",
        }
    }

    /// 来源输入在当前向导中的步骤位置。
    pub(in crate::tgbot::transfer::command::menu) fn source_step_label(self) -> &'static str {
        if self.uses_default_target() {
            "1/1"
        } else {
            "1/3"
        }
    }

    /// 日志中使用的输入流程名，避免直接打印 Debug 后未来重命名影响排查关键词。
    pub(in crate::tgbot::transfer::command::menu) fn log_name(self) -> &'static str {
        match self {
            Self::Transfer => "transfer",
            Self::TransferDefault => "transfer_default",
            Self::Lookup => "lookup",
            Self::LookupDefault => "lookup_default",
        }
    }

    /// 数据库持久化编码。
    fn code(self) -> &'static str {
        self.log_name()
    }

    /// 从数据库编码恢复输入类型。
    pub(in crate::tgbot::transfer::command::menu) fn parse(code: &str) -> Option<Self> {
        match code {
            "transfer" => Some(Self::Transfer),
            "transfer_default" => Some(Self::TransferDefault),
            "lookup" => Some(Self::Lookup),
            "lookup_default" => Some(Self::LookupDefault),
            _ => None,
        }
    }
}

/// 菜单里的任务控制动作。
///
/// 任务控制只需要用户补一个 `job_id`，因此独立于转存/查询的链接输入流程，
/// 最终仍会组装成 `/job <action> <job_id>` 并复用已有命令入口。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot::transfer::command::menu) enum MenuJobAction {
    Status,
    Pause,
    Resume,
    Stop,
}

impl MenuJobAction {
    /// 映射到 `/job` 的公开长动作参数。
    pub(super) fn command_action(self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Pause => "pause",
            Self::Resume => "resume",
            Self::Stop => "stop",
        }
    }

    /// 输入提示标题。
    pub(super) fn input_title(self) -> &'static str {
        match self {
            Self::Status => "任务详情",
            Self::Pause => "暂停任务",
            Self::Resume => "恢复任务",
            Self::Stop => "停止任务",
        }
    }

    /// 输入提示说明。
    pub(super) fn input_detail(self) -> &'static str {
        match self {
            Self::Status => "请回复要查看的 job_id，例如 42。",
            Self::Pause => "请回复要暂停的 job_id，例如 42。",
            Self::Resume => "请回复要恢复的 job_id，例如 42。",
            Self::Stop => "请回复要停止并清理的 job_id，例如 42。",
        }
    }

    /// 日志中使用的稳定动作名。
    pub(in crate::tgbot::transfer::command) fn log_name(self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Pause => "pause",
            Self::Resume => "resume",
            Self::Stop => "stop",
        }
    }

    /// 数据库持久化编码。
    fn code(self) -> &'static str {
        self.log_name()
    }

    /// 从数据库编码恢复任务动作。
    pub(in crate::tgbot::transfer::command) fn parse(code: &str) -> Option<Self> {
        match code {
            "status" => Some(Self::Status),
            "pause" => Some(Self::Pause),
            "resume" => Some(Self::Resume),
            "stop" => Some(Self::Stop),
            _ => None,
        }
    }
}

/// 菜单里的管理配置输入动作。
///
/// 大多数动作都是“单步输入”；`TargetsAliasName`
/// 是 targets 新增项的第一步，收到 A 后会继续进入第二步输入目标 B。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot::transfer::command) enum AdminInputAction {
    TargetsAliasName,
    TargetsAliasSearch,
    TargetsSetDefault,
    TargetsSetAlias,
    ConfigSetJobConcurrency,
    ConfigSetFileDeleteDelayMinutes,
    ConfigSetFileGcIntervalSeconds,
    ConfigSetProgressEditIntervalSeconds,
    ConfigSetDownloadsDefaultPageSize,
    ConfigSetMenuInputTimeoutSeconds,
}

impl AdminInputAction {
    /// 当前所有管理输入动作。
    ///
    /// 这个清单主要用于覆盖测试：新增动作时必须同步确认编码、文案和命令规格。
    #[cfg(test)]
    const ALL: &'static [Self] = &[
        Self::TargetsAliasName,
        Self::TargetsAliasSearch,
        Self::TargetsSetDefault,
        Self::TargetsSetAlias,
        Self::ConfigSetJobConcurrency,
        Self::ConfigSetFileDeleteDelayMinutes,
        Self::ConfigSetFileGcIntervalSeconds,
        Self::ConfigSetProgressEditIntervalSeconds,
        Self::ConfigSetDownloadsDefaultPageSize,
        Self::ConfigSetMenuInputTimeoutSeconds,
    ];
}

/// 配置输入动作从 `/config` 字段规格读取文案，保证按钮、help 和 ForceReply 一致。
fn config_field_spec(
    action: AdminInputAction,
) -> Option<&'static crate::tgbot::transfer::command::config_cmd::ConfigFieldSpec> {
    crate::tgbot::transfer::command::config_cmd::config_field_spec_for_admin_action(action)
}

/// 目标输入动作从 `/targets` 动作规格读取文案，保证按钮、help 和 ForceReply 一致。
fn targets_input_spec(
    action: AdminInputAction,
) -> Option<&'static crate::tgbot::transfer::command::targets::TargetsInputSpec> {
    crate::tgbot::transfer::command::targets::targets_input_spec_for_admin_action(action)
}

/// 带上下文的管理输入提示元数据。
///
/// targets 的新增路由/别名会先收集 A，再收集目标 B；旧的修改现有项也会把已选中的
/// request_chat_id 或 alias 放到上下文里。统一在这里渲染提示，避免“继续输入”和“错误重试”
/// 使用另一套文案。
#[derive(Debug, Clone)]
pub(super) struct AdminInputPromptMeta {
    pub(super) title: String,
    pub(super) detail: String,
    pub(super) placeholder: String,
}

/// 根据管理动作和上下文生成 ForceReply 提示。
pub(super) fn admin_input_prompt_meta(
    action: AdminInputAction,
    context_text: Option<&str>,
    _context_i64: Option<i64>,
) -> AdminInputPromptMeta {
    let (title, detail, placeholder) = match action {
        AdminInputAction::TargetsAliasName => (
            "新增别名".to_owned(),
            "请先回复别名（A），例如 archive；或发送 /cancel 取消。".to_owned(),
            "输入 alias，或发送 /cancel".to_owned(),
        ),
        AdminInputAction::TargetsAliasSearch => (
            "搜索别名".to_owned(),
            "请回复要搜索的别名关键字，例如 archive；或发送 /cancel 取消。".to_owned(),
            "输入别名关键字，或发送 /cancel".to_owned(),
        ),
        AdminInputAction::TargetsSetAlias if context_text.is_some() => (
            "修改目标别名".to_owned(),
            format!(
                "已选 alias：{}。请只回复新的目标私聊 chat_id；或发送 /cancel 取消。",
                context_text.expect("context_text checked above")
            ),
            "输入新的 target_chat_id，或发送 /cancel".to_owned(),
        ),
        _ => (
            action.input_title().to_owned(),
            action.input_detail().to_owned(),
            action.input_placeholder().to_owned(),
        ),
    };

    AdminInputPromptMeta {
        title,
        detail,
        placeholder,
    }
}

impl AdminInputAction {
    /// 对外展示的稳定标题。
    pub(super) fn input_title(self) -> &'static str {
        match self {
            Self::TargetsAliasName => return "新增别名",
            Self::TargetsAliasSearch => return "搜索别名",
            _ => {}
        }
        if let Some(spec) = targets_input_spec(self) {
            return spec.input_title;
        }
        if let Some(spec) = config_field_spec(self) {
            return spec.input_title;
        }

        match self {
            Self::TargetsAliasName | Self::TargetsAliasSearch => {
                unreachable!("two-step targets action title handled above")
            }
            Self::TargetsSetDefault | Self::TargetsSetAlias => {
                unreachable!("targets input title uses spec")
            }
            Self::ConfigSetJobConcurrency
            | Self::ConfigSetFileDeleteDelayMinutes
            | Self::ConfigSetFileGcIntervalSeconds
            | Self::ConfigSetProgressEditIntervalSeconds
            | Self::ConfigSetDownloadsDefaultPageSize
            | Self::ConfigSetMenuInputTimeoutSeconds => {
                unreachable!("config input title uses spec")
            }
        }
    }

    /// ForceReply 提示正文。
    pub(super) fn input_detail(self) -> &'static str {
        match self {
            Self::TargetsAliasName => {
                return "请先回复别名（A），例如 archive；或发送 /cancel 取消。";
            }
            Self::TargetsAliasSearch => {
                return "请回复要搜索的别名关键字，例如 archive；或发送 /cancel 取消。";
            }
            _ => {}
        }
        if let Some(spec) = targets_input_spec(self) {
            return spec.input_detail;
        }
        if let Some(spec) = config_field_spec(self) {
            return spec.input_detail;
        }

        match self {
            Self::TargetsAliasName | Self::TargetsAliasSearch => {
                unreachable!("two-step targets action detail handled above")
            }
            Self::TargetsSetDefault | Self::TargetsSetAlias => {
                unreachable!("targets input detail uses spec")
            }
            Self::ConfigSetJobConcurrency
            | Self::ConfigSetFileDeleteDelayMinutes
            | Self::ConfigSetFileGcIntervalSeconds
            | Self::ConfigSetProgressEditIntervalSeconds
            | Self::ConfigSetDownloadsDefaultPageSize
            | Self::ConfigSetMenuInputTimeoutSeconds => {
                unreachable!("config input detail uses spec")
            }
        }
    }

    /// 输入框占位文案。
    pub(super) fn input_placeholder(self) -> &'static str {
        match self {
            Self::TargetsAliasName => return "输入 alias，或发送 /cancel",
            Self::TargetsAliasSearch => return "输入别名关键字，或发送 /cancel",
            _ => {}
        }
        if let Some(spec) = targets_input_spec(self) {
            return spec.input_placeholder;
        }
        if let Some(spec) = config_field_spec(self) {
            return spec.input_placeholder;
        }

        match self {
            Self::TargetsAliasName | Self::TargetsAliasSearch => {
                unreachable!("two-step targets action placeholder handled above")
            }
            Self::TargetsSetDefault | Self::TargetsSetAlias => {
                unreachable!("targets input placeholder uses spec")
            }
            Self::ConfigSetJobConcurrency
            | Self::ConfigSetFileDeleteDelayMinutes
            | Self::ConfigSetFileGcIntervalSeconds
            | Self::ConfigSetProgressEditIntervalSeconds
            | Self::ConfigSetDownloadsDefaultPageSize
            | Self::ConfigSetMenuInputTimeoutSeconds => {
                unreachable!("config input placeholder uses spec")
            }
        }
    }

    /// 日志与持久化使用的稳定编码。
    pub(in crate::tgbot::transfer::command) fn log_name(self) -> &'static str {
        match self {
            Self::TargetsAliasName => "targets_new_alias_name",
            Self::TargetsAliasSearch => "targets_alias_search",
            Self::TargetsSetDefault => "targets_set_default",
            Self::TargetsSetAlias => "targets_set_alias",
            Self::ConfigSetJobConcurrency => "config_set_job_concurrency",
            Self::ConfigSetFileDeleteDelayMinutes => "config_set_delete_delay",
            Self::ConfigSetFileGcIntervalSeconds => "config_set_gc_interval",
            Self::ConfigSetProgressEditIntervalSeconds => "config_set_progress_interval",
            Self::ConfigSetDownloadsDefaultPageSize => "config_set_page_size",
            Self::ConfigSetMenuInputTimeoutSeconds => "config_set_menu_timeout",
        }
    }

    fn code(self) -> &'static str {
        self.log_name()
    }

    pub(in crate::tgbot::transfer::command) fn parse(code: &str) -> Option<Self> {
        match code {
            "targets_new_alias_name" => Some(Self::TargetsAliasName),
            "targets_alias_search" => Some(Self::TargetsAliasSearch),
            "targets_set_default" => Some(Self::TargetsSetDefault),
            "targets_set_alias" => Some(Self::TargetsSetAlias),
            "config_set_job_concurrency" => Some(Self::ConfigSetJobConcurrency),
            "config_set_delete_delay" => Some(Self::ConfigSetFileDeleteDelayMinutes),
            "config_set_gc_interval" => Some(Self::ConfigSetFileGcIntervalSeconds),
            "config_set_progress_interval" => Some(Self::ConfigSetProgressEditIntervalSeconds),
            "config_set_page_size" => Some(Self::ConfigSetDownloadsDefaultPageSize),
            "config_set_menu_timeout" => Some(Self::ConfigSetMenuInputTimeoutSeconds),
            _ => None,
        }
    }
}

/// 菜单输入阶段。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum MenuInputStep {
    SourceLink {
        kind: MenuInputKind,
    },
    TargetChoice {
        kind: MenuInputKind,
        source_link: String,
    },
    TargetChat {
        kind: MenuInputKind,
        source_link: String,
    },
    Confirm {
        kind: MenuInputKind,
        source_link: String,
        target_chat_id: i64,
    },
    JobId {
        action: MenuJobAction,
    },
    AdminInput {
        action: AdminInputAction,
        context_text: Option<String>,
        context_i64: Option<i64>,
    },
}

/// 菜单输入草稿。
#[derive(Debug, Clone)]
pub(super) struct MenuInputDraft {
    pub(super) step: MenuInputStep,
}

impl MenuInputDraft {
    /// 构造等待源链接的草稿。
    pub(super) fn source_link(kind: MenuInputKind) -> Self {
        Self::new(MenuInputStep::SourceLink { kind })
    }

    /// 当前草稿的简短标题，供首页继续输入按钮展示。
    pub(super) fn continue_title(&self) -> &'static str {
        match &self.step {
            MenuInputStep::SourceLink { kind } => kind.source_title(),
            MenuInputStep::TargetChoice { .. } => "选择目标",
            MenuInputStep::TargetChat { .. } => "输入目标",
            MenuInputStep::Confirm { .. } => "确认执行",
            MenuInputStep::JobId { action } => action.input_title(),
            MenuInputStep::AdminInput { action, .. } => action.input_title(),
        }
    }

    /// 构造等待目标选择的草稿。
    pub(super) fn target_choice(kind: MenuInputKind, source_link: String) -> Self {
        Self::new(MenuInputStep::TargetChoice { kind, source_link })
    }

    /// 构造等待手动输入目标的草稿。
    pub(super) fn target_chat(kind: MenuInputKind, source_link: String) -> Self {
        Self::new(MenuInputStep::TargetChat { kind, source_link })
    }

    /// 构造等待确认执行的草稿。
    pub(super) fn confirm(kind: MenuInputKind, source_link: String, target_chat_id: i64) -> Self {
        Self::new(MenuInputStep::Confirm {
            kind,
            source_link,
            target_chat_id,
        })
    }

    /// 构造等待任务编号的草稿。
    pub(super) fn job_id(action: MenuJobAction) -> Self {
        Self::new(MenuInputStep::JobId { action })
    }

    /// 构造等待管理配置输入的草稿。
    pub(super) fn admin_input(
        action: AdminInputAction,
        context_text: Option<String>,
        context_i64: Option<i64>,
    ) -> Self {
        Self::new(MenuInputStep::AdminInput {
            action,
            context_text,
            context_i64,
        })
    }

    /// 每次写回草稿都刷新过期时间。
    fn new(step: MenuInputStep) -> Self {
        Self { step }
    }

    /// 转成数据库 ActiveModel。
    fn into_active_model(self, key: DraftKey) -> db::menu_input_draft::ActiveModel {
        let now = now_utc8();
        let expires_at = now + chrono::Duration::seconds(input_ttl_seconds() as i64);
        let fields = DraftFields::from_step(self.step);
        db::menu_input_draft::ActiveModel {
            request_chat_id: Set(key.0),
            sender_user_id: Set(key.1),
            step: Set(fields.step.to_owned()),
            input_kind: Set(fields.input_kind.map(str::to_owned)),
            job_action: Set(fields.job_action.map(str::to_owned)),
            source_link: Set(fields.source_link),
            target_chat_id: Set(fields.target_chat_id),
            created_at: Set(now),
            updated_at: Set(now),
            expires_at: Set(expires_at),
        }
    }

    /// 从数据库行恢复草稿。
    fn from_model(model: &db::menu_input_draft::Model) -> Option<Self> {
        let step = match model.step.as_str() {
            "source_link" => MenuInputStep::SourceLink {
                kind: MenuInputKind::parse(model.input_kind.as_deref()?)?,
            },
            "target_choice" => MenuInputStep::TargetChoice {
                kind: MenuInputKind::parse(model.input_kind.as_deref()?)?,
                source_link: model.source_link.clone()?,
            },
            "target_chat" => MenuInputStep::TargetChat {
                kind: MenuInputKind::parse(model.input_kind.as_deref()?)?,
                source_link: model.source_link.clone()?,
            },
            "confirm" => MenuInputStep::Confirm {
                kind: MenuInputKind::parse(model.input_kind.as_deref()?)?,
                source_link: model.source_link.clone()?,
                target_chat_id: model.target_chat_id?,
            },
            "job_id" => MenuInputStep::JobId {
                action: MenuJobAction::parse(model.job_action.as_deref()?)?,
            },
            "admin_input" => MenuInputStep::AdminInput {
                action: AdminInputAction::parse(model.job_action.as_deref()?)?,
                context_text: model.source_link.clone(),
                context_i64: model.target_chat_id,
            },
            _ => return None,
        };
        Some(Self { step })
    }
}

/// 持久化草稿时拆出的列值。
struct DraftFields {
    step: &'static str,
    input_kind: Option<&'static str>,
    job_action: Option<&'static str>,
    source_link: Option<String>,
    target_chat_id: Option<i64>,
}

impl DraftFields {
    /// 从运行时状态拆成数据库列。
    fn from_step(step: MenuInputStep) -> Self {
        match step {
            MenuInputStep::SourceLink { kind } => Self {
                step: "source_link",
                input_kind: Some(kind.code()),
                job_action: None,
                source_link: None,
                target_chat_id: None,
            },
            MenuInputStep::TargetChoice { kind, source_link } => Self {
                step: "target_choice",
                input_kind: Some(kind.code()),
                job_action: None,
                source_link: Some(source_link),
                target_chat_id: None,
            },
            MenuInputStep::TargetChat { kind, source_link } => Self {
                step: "target_chat",
                input_kind: Some(kind.code()),
                job_action: None,
                source_link: Some(source_link),
                target_chat_id: None,
            },
            MenuInputStep::Confirm {
                kind,
                source_link,
                target_chat_id,
            } => Self {
                step: "confirm",
                input_kind: Some(kind.code()),
                job_action: None,
                source_link: Some(source_link),
                target_chat_id: Some(target_chat_id),
            },
            MenuInputStep::JobId { action } => Self {
                step: "job_id",
                input_kind: None,
                job_action: Some(action.code()),
                source_link: None,
                target_chat_id: None,
            },
            MenuInputStep::AdminInput {
                action,
                context_text,
                context_i64,
            } => Self {
                step: "admin_input",
                input_kind: None,
                // 复用可空字符串列保存单步管理动作编码，避免为短草稿状态再单独加 schema。
                job_action: Some(action.code()),
                source_link: context_text,
                target_chat_id: context_i64,
            },
        }
    }
}

/// 取草稿的结果。
#[derive(Debug, Clone)]
pub(super) enum DraftTakeResult {
    None,
    Active(MenuInputDraft),
    Expired,
}

/// 目标选择上下文。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TargetContext {
    pub(super) kind: MenuInputKind,
    pub(super) source_link: String,
}

/// 目标选择按钮要推进到的下一步。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TargetDraftAdvance {
    TargetChoice,
    TargetChat,
    Confirm { target_chat_id: i64 },
}

/// 目标选择推进结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TargetContextAdvanceResult {
    None,
    Active(TargetContext),
    Expired,
    WrongStep,
}

/// 确认执行上下文。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ConfirmContext {
    pub(super) kind: MenuInputKind,
    pub(super) source_link: String,
    pub(super) target_chat_id: i64,
}

/// 确认按钮消费结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ConfirmContextTakeResult {
    None,
    Active(ConfirmContext),
    Expired,
    WrongStep,
}

/// 开始一个菜单输入流程。
pub(in crate::tgbot::transfer::command::menu) async fn start_menu_input(
    chat_id: i64,
    user_id: i64,
    kind: MenuInputKind,
) -> anyhow::Result<()> {
    put_draft((chat_id, user_id), MenuInputDraft::source_link(kind)).await?;
    tracing::debug!(
        chat_id,
        user_id,
        input_kind = kind.log_name(),
        "menu input draft started"
    );
    Ok(())
}

/// 取消一个菜单输入流程。
pub(in crate::tgbot::transfer::command::menu) async fn cancel_menu_input(
    chat_id: i64,
    user_id: i64,
) -> anyhow::Result<bool> {
    let _guard = acquire_draft_key_guard((chat_id, user_id)).await;
    purge_expired().await?;
    let Some(removed) = find_draft_model(chat_id, user_id).await? else {
        return Ok(false);
    };
    if !delete_draft_if_current(&removed).await? {
        tracing::debug!(chat_id, user_id, "menu input draft cancel lost write race");
        return Ok(false);
    }
    tracing::debug!(chat_id, user_id, "menu input draft cancelled");
    Ok(true)
}

/// 取出当前草稿；若草稿过期，则清理后返回过期状态。
pub(super) async fn take_current_draft(key: DraftKey) -> anyhow::Result<DraftTakeResult> {
    let _guard = acquire_draft_key_guard(key).await;
    purge_expired().await?;
    let Some(model) = find_draft_model(key.0, key.1).await? else {
        return Ok(DraftTakeResult::None);
    };
    if model.expires_at <= now_utc8() {
        delete_draft(key.0, key.1).await?;
        purge_expired().await?;
        return Ok(DraftTakeResult::Expired);
    }
    let Some(draft) = MenuInputDraft::from_model(&model) else {
        tracing::warn!(
            chat_id = key.0,
            user_id = key.1,
            "menu input draft row is invalid, deleting"
        );
        return Ok(DraftTakeResult::None);
    };
    if !delete_draft_if_current(&model).await? {
        tracing::debug!(
            chat_id = key.0,
            user_id = key.1,
            "menu input draft was already consumed by another worker"
        );
        return Ok(DraftTakeResult::None);
    }
    Ok(DraftTakeResult::Active(draft))
}

/// 在同一把草稿互斥下推进目标选择上下文。
///
/// callback 连点时，如果调用方自己 `take_current_draft` 后再 `put_draft`，中间会有短暂空窗；
/// 另一个 callback 可能误判为“没有待输入”。这里把读取和写回收敛到状态层，保证同进程内
/// 同一个 chat + user 的目标选择推进串行完成。
pub(super) async fn advance_target_context(
    key: DraftKey,
    advance: TargetDraftAdvance,
) -> anyhow::Result<TargetContextAdvanceResult> {
    let _guard = acquire_draft_key_guard(key).await;
    purge_expired().await?;
    let Some(model) = find_draft_model(key.0, key.1).await? else {
        return Ok(TargetContextAdvanceResult::None);
    };
    if model.expires_at <= now_utc8() {
        delete_draft(key.0, key.1).await?;
        purge_expired().await?;
        return Ok(TargetContextAdvanceResult::Expired);
    }

    let Some(draft) = MenuInputDraft::from_model(&model) else {
        tracing::warn!(
            chat_id = key.0,
            user_id = key.1,
            "menu target draft row is invalid, deleting"
        );
        delete_draft(key.0, key.1).await?;
        return Ok(TargetContextAdvanceResult::None);
    };
    let Some((kind, source_link)) = target_context_from_step(&draft.step) else {
        return Ok(TargetContextAdvanceResult::WrongStep);
    };

    let next = match advance {
        TargetDraftAdvance::TargetChoice => {
            MenuInputDraft::target_choice(kind, source_link.clone())
        }
        TargetDraftAdvance::TargetChat => MenuInputDraft::target_chat(kind, source_link.clone()),
        TargetDraftAdvance::Confirm { target_chat_id } => {
            MenuInputDraft::confirm(kind, source_link.clone(), target_chat_id)
        }
    };
    if !update_draft_if_current(&model, next).await? {
        tracing::debug!(
            chat_id = key.0,
            user_id = key.1,
            "menu target draft advance lost write race"
        );
        return Ok(TargetContextAdvanceResult::None);
    }
    Ok(TargetContextAdvanceResult::Active(TargetContext {
        kind,
        source_link,
    }))
}

/// 消费确认态草稿。
///
/// 只有处于 Confirm 阶段才删除草稿并返回可执行上下文；其它阶段保持原草稿不变，
/// 避免用户点错旧按钮后丢失当前输入流程。
pub(super) async fn take_confirm_context(
    key: DraftKey,
) -> anyhow::Result<ConfirmContextTakeResult> {
    let _guard = acquire_draft_key_guard(key).await;
    purge_expired().await?;
    let Some(model) = find_draft_model(key.0, key.1).await? else {
        return Ok(ConfirmContextTakeResult::None);
    };
    if model.expires_at <= now_utc8() {
        delete_draft(key.0, key.1).await?;
        purge_expired().await?;
        return Ok(ConfirmContextTakeResult::Expired);
    }

    let Some(draft) = MenuInputDraft::from_model(&model) else {
        tracing::warn!(
            chat_id = key.0,
            user_id = key.1,
            "menu confirm draft row is invalid, deleting"
        );
        delete_draft(key.0, key.1).await?;
        return Ok(ConfirmContextTakeResult::None);
    };
    let MenuInputStep::Confirm {
        kind,
        source_link,
        target_chat_id,
    } = draft.step
    else {
        return Ok(ConfirmContextTakeResult::WrongStep);
    };

    if !delete_draft_if_current(&model).await? {
        tracing::debug!(
            chat_id = key.0,
            user_id = key.1,
            "menu confirm draft was already consumed by another worker"
        );
        return Ok(ConfirmContextTakeResult::None);
    }
    Ok(ConfirmContextTakeResult::Active(ConfirmContext {
        kind,
        source_link,
        target_chat_id,
    }))
}

/// 读取当前草稿但不消费。
///
/// 首页“继续输入”只需要判断是否存在草稿；真正消费仍发生在用户回复文本或确认按钮时。
pub(super) async fn peek_current_draft(key: DraftKey) -> anyhow::Result<DraftTakeResult> {
    purge_expired().await?;
    let Some(model) = find_draft_model(key.0, key.1).await? else {
        return Ok(DraftTakeResult::None);
    };
    if model.expires_at <= now_utc8() {
        delete_draft(key.0, key.1).await?;
        purge_expired().await?;
        return Ok(DraftTakeResult::Expired);
    }
    let Some(draft) = MenuInputDraft::from_model(&model) else {
        tracing::warn!(
            chat_id = key.0,
            user_id = key.1,
            "menu input draft row is invalid, deleting"
        );
        delete_draft(key.0, key.1).await?;
        return Ok(DraftTakeResult::None);
    };
    Ok(DraftTakeResult::Active(draft))
}

/// 写回草稿。
pub(super) async fn put_draft(key: DraftKey, draft: MenuInputDraft) -> anyhow::Result<()> {
    let _guard = acquire_draft_key_guard(key).await;
    put_draft_unlocked(key, draft).await
}

/// 写入目标选择草稿。
pub(super) async fn put_target_choice_draft(
    key: DraftKey,
    kind: MenuInputKind,
    source_link: String,
) -> anyhow::Result<()> {
    put_draft(key, MenuInputDraft::target_choice(kind, source_link)).await
}

/// 写入确认草稿。
pub(super) async fn put_confirm_draft(
    key: DraftKey,
    kind: MenuInputKind,
    source_link: String,
    target_chat_id: i64,
) -> anyhow::Result<()> {
    put_draft(
        key,
        MenuInputDraft::confirm(kind, source_link, target_chat_id),
    )
    .await
}

/// 从输入阶段提取目标选择上下文。
pub(super) fn target_context_from_step(step: &MenuInputStep) -> Option<(MenuInputKind, String)> {
    match step {
        MenuInputStep::TargetChoice { kind, source_link }
        | MenuInputStep::TargetChat { kind, source_link }
        | MenuInputStep::Confirm {
            kind, source_link, ..
        } => Some((*kind, source_link.clone())),
        MenuInputStep::SourceLink { .. }
        | MenuInputStep::JobId { .. }
        | MenuInputStep::AdminInput { .. } => None,
    }
}

/// 统一生成 UTC+8 时间戳。
fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    let Some(offset) = chrono::FixedOffset::east_opt(8 * 3600) else {
        tracing::error!("failed to build menu input UTC+8 fixed offset, fallback to UTC");
        return chrono::Utc::now().fixed_offset();
    };
    chrono::Utc::now().with_timezone(&offset)
}

/// 菜单输入草稿超时时间（秒）。
fn input_ttl_seconds() -> u64 {
    // 草稿 TTL 是状态模块自己的基础规则；这里集中读取一次运行态配置，
    // 比把 `AppContext` 继续透传到所有状态读写 API 更能保持状态层接口简洁。
    let app_context = crate::app_context::app_context();
    crate::tgbot::transfer::runtime_config_on(app_context.as_ref())
        .menu_input_timeout_seconds
        .max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试前准备业务库 schema，并串行化 DB 测试。
    async fn prepare_schema() -> anyhow::Result<tokio::sync::MutexGuard<'static, ()>> {
        let guard = crate::db::TEST_DB_LOCK.lock().await;
        let db = crate::db::get_db().await?;
        crate::db::ensure_test_schema_current(db).await?;
        Ok(guard)
    }

    // 草稿应按 chat + user 隔离，避免不同会话互相覆盖输入。
    #[tokio::test]
    async fn test_start_and_cancel_menu_input() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        start_menu_input(900_001, 900_002, MenuInputKind::Transfer).await?;

        assert!(cancel_menu_input(900_001, 900_002).await?);
        assert!(!cancel_menu_input(900_001, 900_002).await?);
        Ok(())
    }

    // 草稿持久化后应能恢复为运行时状态；取出后即删除，避免同一条回复被重复消费。
    #[tokio::test]
    async fn test_take_current_draft_reads_persisted_row_once() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_003, 900_004);
        put_draft(
            key,
            MenuInputDraft::target_choice(MenuInputKind::Transfer, "https://t.me/c/1/2".to_owned()),
        )
        .await?;

        let draft = take_current_draft(key).await?;
        assert!(matches!(
            draft,
            DraftTakeResult::Active(MenuInputDraft {
                step: MenuInputStep::TargetChoice { .. }
            })
        ));
        assert!(matches!(
            take_current_draft(key).await?,
            DraftTakeResult::None
        ));
        Ok(())
    }

    // 首页“继续输入”只读取草稿摘要，不应消费草稿；真正消费必须等用户回复或确认按钮。
    #[tokio::test]
    async fn test_peek_current_draft_does_not_consume_row() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_005, 900_006);
        put_draft(key, MenuInputDraft::job_id(MenuJobAction::Pause)).await?;

        let peeked = peek_current_draft(key).await?;
        assert!(matches!(
            peeked,
            DraftTakeResult::Active(MenuInputDraft {
                step: MenuInputStep::JobId {
                    action: MenuJobAction::Pause
                }
            })
        ));

        let taken = take_current_draft(key).await?;
        assert!(matches!(
            taken,
            DraftTakeResult::Active(MenuInputDraft {
                step: MenuInputStep::JobId {
                    action: MenuJobAction::Pause
                }
            })
        ));
        assert!(matches!(
            take_current_draft(key).await?,
            DraftTakeResult::None
        ));
        Ok(())
    }

    // 并发按钮点击会多次写回同一个 chat + user 草稿；写入必须是 upsert，不能暴露主键冲突。
    #[tokio::test]
    async fn test_put_draft_concurrent_writes_keep_single_active_row() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_021, 900_022);

        let first = tokio::spawn(async move {
            put_draft(key, MenuInputDraft::job_id(MenuJobAction::Pause)).await
        });
        let second = tokio::spawn(async move {
            put_draft(
                key,
                MenuInputDraft::target_choice(
                    MenuInputKind::Transfer,
                    "https://t.me/c/1/2".to_owned(),
                ),
            )
            .await
        });

        first.await??;
        second.await??;

        let active = find_draft_model(key.0, key.1).await?;
        assert!(active.is_some());
        assert!(matches!(
            take_current_draft(key).await?,
            DraftTakeResult::Active(_)
        ));
        assert!(matches!(
            take_current_draft(key).await?,
            DraftTakeResult::None
        ));
        Ok(())
    }

    // 并发消费同一份草稿时，只允许一个调用拿到 Active，另一个必须看到 None。
    #[tokio::test]
    async fn test_take_current_draft_concurrent_reads_consume_once() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_023, 900_024);
        put_draft(key, MenuInputDraft::job_id(MenuJobAction::Stop)).await?;

        let first = tokio::spawn(async move { take_current_draft(key).await });
        let second = tokio::spawn(async move { take_current_draft(key).await });

        let first = first.await??;
        let second = second.await??;
        let active_count = [first, second]
            .into_iter()
            .filter(|result| matches!(result, DraftTakeResult::Active(_)))
            .count();

        assert_eq!(active_count, 1);
        assert!(matches!(
            take_current_draft(key).await?,
            DraftTakeResult::None
        ));
        Ok(())
    }

    // 目标选择按钮的推进必须在状态层原子完成，避免按钮连点时短暂出现“没有草稿”。
    #[tokio::test]
    async fn test_advance_target_context_concurrent_writes_keep_valid_confirm() -> anyhow::Result<()>
    {
        let _guard = prepare_schema().await?;
        let key = (900_025, 900_026);
        put_draft(
            key,
            MenuInputDraft::target_choice(MenuInputKind::Transfer, "https://t.me/c/1/2".to_owned()),
        )
        .await?;

        let first = tokio::spawn(async move {
            advance_target_context(
                key,
                TargetDraftAdvance::Confirm {
                    target_chat_id: -100,
                },
            )
            .await
        });
        let second = tokio::spawn(async move {
            advance_target_context(
                key,
                TargetDraftAdvance::Confirm {
                    target_chat_id: -200,
                },
            )
            .await
        });

        assert!(matches!(
            first.await??,
            TargetContextAdvanceResult::Active(_)
        ));
        assert!(matches!(
            second.await??,
            TargetContextAdvanceResult::Active(_)
        ));

        let confirm = take_confirm_context(key).await?;
        assert!(matches!(
            confirm,
            ConfirmContextTakeResult::Active(ConfirmContext {
                kind: MenuInputKind::Transfer,
                ..
            })
        ));
        Ok(())
    }

    // “执行”按钮连点时只有一个调用能消费确认草稿，另一个必须看到 None。
    #[tokio::test]
    async fn test_take_confirm_context_concurrent_reads_consume_once() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_027, 900_028);
        put_draft(
            key,
            MenuInputDraft::confirm(
                MenuInputKind::Transfer,
                "https://t.me/c/1/2".to_owned(),
                -100,
            ),
        )
        .await?;

        let first = tokio::spawn(async move { take_confirm_context(key).await });
        let second = tokio::spawn(async move { take_confirm_context(key).await });

        let first = first.await??;
        let second = second.await??;
        let active_count = [first, second]
            .into_iter()
            .filter(|result| matches!(result, ConfirmContextTakeResult::Active(_)))
            .count();

        assert_eq!(active_count, 1);
        assert!(matches!(
            take_confirm_context(key).await?,
            ConfirmContextTakeResult::None
        ));
        Ok(())
    }

    // 条件删除必须拒绝旧快照，避免多进程下旧 worker 删除已经被新输入覆盖的草稿。
    #[tokio::test]
    async fn test_delete_draft_if_current_rejects_stale_snapshot() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_029, 900_030);
        put_draft(key, MenuInputDraft::job_id(MenuJobAction::Pause)).await?;
        let stale = find_draft_model(key.0, key.1).await?.expect("draft exists");

        put_draft(key, MenuInputDraft::job_id(MenuJobAction::Resume)).await?;

        assert!(!delete_draft_if_current(&stale).await?);
        let current = take_current_draft(key).await?;
        assert!(matches!(
            current,
            DraftTakeResult::Active(MenuInputDraft {
                step: MenuInputStep::JobId {
                    action: MenuJobAction::Resume
                }
            })
        ));
        Ok(())
    }

    // 条件更新必须拒绝旧快照，避免旧按钮覆盖较新的输入阶段。
    #[tokio::test]
    async fn test_update_draft_if_current_rejects_stale_snapshot() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_031, 900_032);
        put_draft(
            key,
            MenuInputDraft::target_choice(MenuInputKind::Transfer, "https://t.me/c/1/2".to_owned()),
        )
        .await?;
        let stale = find_draft_model(key.0, key.1).await?.expect("draft exists");

        put_draft(
            key,
            MenuInputDraft::target_chat(MenuInputKind::Transfer, "https://t.me/c/1/2".to_owned()),
        )
        .await?;

        assert!(
            !update_draft_if_current(
                &stale,
                MenuInputDraft::confirm(
                    MenuInputKind::Transfer,
                    "https://t.me/c/1/2".to_owned(),
                    -100
                ),
            )
            .await?
        );
        let current = take_current_draft(key).await?;
        assert!(matches!(
            current,
            DraftTakeResult::Active(MenuInputDraft {
                step: MenuInputStep::TargetChat { .. }
            })
        ));
        Ok(())
    }

    // 多步草稿应能按“源链接 -> 目标选择 -> 确认 -> 消费确认”完整推进，避免状态机只在局部测试里成立。
    #[tokio::test]
    async fn test_multi_step_draft_flow_roundtrip() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_041, 900_042);

        put_draft(key, MenuInputDraft::source_link(MenuInputKind::Transfer)).await?;
        assert!(matches!(
            peek_current_draft(key).await?,
            DraftTakeResult::Active(MenuInputDraft {
                step: MenuInputStep::SourceLink {
                    kind: MenuInputKind::Transfer
                }
            })
        ));

        put_target_choice_draft(
            key,
            MenuInputKind::Transfer,
            "https://t.me/c/1/2".to_owned(),
        )
        .await?;
        assert!(matches!(
            advance_target_context(
                key,
                TargetDraftAdvance::Confirm {
                    target_chat_id: -100
                }
            )
            .await?,
            TargetContextAdvanceResult::Active(TargetContext {
                kind: MenuInputKind::Transfer,
                ..
            })
        ));

        assert!(matches!(
            take_confirm_context(key).await?,
            ConfirmContextTakeResult::Active(ConfirmContext {
                kind: MenuInputKind::Transfer,
                target_chat_id: -100,
                ..
            })
        ));
        assert!(matches!(
            peek_current_draft(key).await?,
            DraftTakeResult::None
        ));
        Ok(())
    }

    // 手动输入目标后返回选择页时，必须保留输入类型和来源链接。
    #[tokio::test]
    async fn test_target_chat_can_return_to_target_choice() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_043, 900_044);
        let source_link = "https://t.me/c/1/9";
        put_draft(
            key,
            MenuInputDraft::target_chat(MenuInputKind::Transfer, source_link.to_owned()),
        )
        .await?;

        assert_eq!(
            advance_target_context(key, TargetDraftAdvance::TargetChoice).await?,
            TargetContextAdvanceResult::Active(TargetContext {
                kind: MenuInputKind::Transfer,
                source_link: source_link.to_owned(),
            })
        );
        assert!(matches!(
            peek_current_draft(key).await?,
            DraftTakeResult::Active(MenuInputDraft {
                step: MenuInputStep::TargetChoice {
                    kind: MenuInputKind::Transfer,
                    source_link: stored_source,
                }
            }) if stored_source == source_link
        ));
        Ok(())
    }

    // 不同输入流程应使用对应的长命令，最终复用已有命令入口。
    #[test]
    fn test_menu_input_kind_command_name() {
        assert_eq!(MenuInputKind::Transfer.command_name(), "/transfer");
        assert_eq!(MenuInputKind::TransferDefault.command_name(), "/transfer");
        assert_eq!(MenuInputKind::Lookup.command_name(), "/lookup");
        assert_eq!(MenuInputKind::LookupDefault.command_name(), "/lookup");
    }

    // 快速流程只输入一次来源；指定目标流程仍保留来源、目标、确认三步。
    #[test]
    fn test_menu_input_kind_source_step_label() {
        assert_eq!(MenuInputKind::Transfer.source_step_label(), "1/3");
        assert_eq!(MenuInputKind::Lookup.source_step_label(), "1/3");
        assert_eq!(MenuInputKind::TransferDefault.source_step_label(), "1/1");
        assert_eq!(MenuInputKind::LookupDefault.source_step_label(), "1/1");
    }

    // 菜单任务动作应稳定映射到 `/job` 的公开长参数，避免交互入口和命令入口语义分叉。
    #[test]
    fn test_menu_job_action_command_action() {
        assert_eq!(MenuJobAction::Status.command_action(), "status");
        assert_eq!(MenuJobAction::Pause.command_action(), "pause");
        assert_eq!(MenuJobAction::Resume.command_action(), "resume");
        assert_eq!(MenuJobAction::Stop.command_action(), "stop");
    }

    #[test]
    fn test_admin_input_action_prompt_meta() {
        assert!(
            AdminInputAction::TargetsAliasName
                .input_detail()
                .contains("别名")
        );
        assert_eq!(
            AdminInputAction::TargetsSetDefault.input_title(),
            "设置默认目标"
        );
    }

    #[test]
    fn test_admin_input_prompt_meta_uses_targets_context() {
        let alias =
            admin_input_prompt_meta(AdminInputAction::TargetsSetAlias, Some("archive"), None);

        assert!(alias.detail.contains("alias：archive"));
        assert_eq!(alias.title, "修改目标别名");
    }

    #[test]
    fn test_admin_input_action_all_roundtrip_and_prompt_meta() {
        for action in AdminInputAction::ALL {
            assert_eq!(
                AdminInputAction::parse(action.log_name()),
                Some(*action),
                "admin input action code should roundtrip: {}",
                action.log_name()
            );
            assert!(
                !action.input_title().trim().is_empty(),
                "admin input action title should not be empty: {}",
                action.log_name()
            );
            assert!(
                !action.input_detail().trim().is_empty(),
                "admin input action detail should not be empty: {}",
                action.log_name()
            );
            assert!(
                !action.input_placeholder().trim().is_empty(),
                "admin input action placeholder should not be empty: {}",
                action.log_name()
            );
        }
    }

    // 上次目标只是交互捷径，按 chat + user 隔离，避免不同会话互相覆盖。
    #[test]
    fn test_last_target_is_isolated_by_chat_and_user() {
        remember_last_target(10, 20, -100);

        assert_eq!(last_target(10, 20), Some(-100));
        assert_eq!(last_target(10, 21), None);
        assert_eq!(last_target(11, 20), None);
    }
}
