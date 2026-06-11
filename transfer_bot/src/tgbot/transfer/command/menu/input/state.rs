// `/menu` 输入草稿状态。
// 草稿持久化在业务数据库中，真实转存任务仍全部落 transfer_job。

use std::collections::HashMap;
use std::sync::LazyLock;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::db;

/// 输入草稿索引。
///
/// 同一个管理员可能在多个管理 chat 中操作，因此用 `(chat_id, user_id)` 做隔离。
pub(super) type DraftKey = (i64, i64);

/// 最近一次确认执行的目标。
///
/// 这是纯交互优化，不参与转存幂等和任务恢复；进程重启后丢失也不会影响真实任务。
static MENU_LAST_TARGETS: LazyLock<std::sync::Mutex<HashMap<DraftKey, i64>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

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
    pub(super) fn command_kind(self) -> Self {
        match self {
            Self::Transfer | Self::TransferDefault => Self::Transfer,
            Self::Lookup | Self::LookupDefault => Self::Lookup,
        }
    }

    /// 当前流程复用的命令名。
    ///
    /// bot 私聊场景优先展示长命令；短命令仍由上层路由兼容。
    pub(super) fn command_name(self) -> &'static str {
        match self.command_kind() {
            Self::Transfer => "/transfer",
            Self::Lookup => "/lookup",
            Self::TransferDefault | Self::LookupDefault => unreachable!("kind is normalized"),
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
            Self::TransferDefault => "请回复源链接，目标 chat 将使用配置默认值。",
            Self::Lookup => "请回复要查询的 Telegram 消息或相册链接。",
            Self::LookupDefault => "请回复源链接，目标 chat 将使用配置默认值。",
        }
    }

    /// 日志中使用的输入流程名，避免直接打印 Debug 后未来重命名影响排查关键词。
    pub(super) fn log_name(self) -> &'static str {
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
    fn parse(code: &str) -> Option<Self> {
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
    /// 映射到 `/job` 已支持的短动作参数。
    pub(super) fn command_action(self) -> &'static str {
        match self {
            Self::Status => "st",
            Self::Pause => "p",
            Self::Resume => "r",
            Self::Stop => "s",
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
    pub(super) fn log_name(self) -> &'static str {
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
    fn parse(code: &str) -> Option<Self> {
        match code {
            "status" => Some(Self::Status),
            "pause" => Some(Self::Pause),
            "resume" => Some(Self::Resume),
            "stop" => Some(Self::Stop),
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
    ChatPicker {
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
    PointLedgerUserId,
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
            MenuInputStep::ChatPicker { .. } => "选择群组",
            MenuInputStep::Confirm { .. } => "确认执行",
            MenuInputStep::JobId { action } => action.input_title(),
            MenuInputStep::PointLedgerUserId => "用户积分流水",
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

    /// 构造等待 Telegram 原生选群结果的草稿。
    pub(super) fn chat_picker(kind: MenuInputKind, source_link: String) -> Self {
        Self::new(MenuInputStep::ChatPicker { kind, source_link })
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

    /// 构造等待用户 ID 的积分流水草稿。
    pub(super) fn point_ledger_user_id() -> Self {
        Self::new(MenuInputStep::PointLedgerUserId)
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
    fn from_model(model: db::menu_input_draft::Model) -> Option<Self> {
        let step = match model.step.as_str() {
            "source_link" => MenuInputStep::SourceLink {
                kind: MenuInputKind::parse(model.input_kind.as_deref()?)?,
            },
            "target_choice" => MenuInputStep::TargetChoice {
                kind: MenuInputKind::parse(model.input_kind.as_deref()?)?,
                source_link: model.source_link?,
            },
            "target_chat" => MenuInputStep::TargetChat {
                kind: MenuInputKind::parse(model.input_kind.as_deref()?)?,
                source_link: model.source_link?,
            },
            "chat_picker" => MenuInputStep::ChatPicker {
                kind: MenuInputKind::parse(model.input_kind.as_deref()?)?,
                source_link: model.source_link?,
            },
            "confirm" => MenuInputStep::Confirm {
                kind: MenuInputKind::parse(model.input_kind.as_deref()?)?,
                source_link: model.source_link?,
                target_chat_id: model.target_chat_id?,
            },
            "job_id" => MenuInputStep::JobId {
                action: MenuJobAction::parse(model.job_action.as_deref()?)?,
            },
            "point_ledger_user_id" => MenuInputStep::PointLedgerUserId,
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
            MenuInputStep::ChatPicker { kind, source_link } => Self {
                step: "chat_picker",
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
            MenuInputStep::PointLedgerUserId => Self {
                step: "point_ledger_user_id",
                input_kind: None,
                job_action: None,
                source_link: None,
                target_chat_id: None,
            },
        }
    }
}

/// 取消草稿后的附加信息。
///
/// 目前只有 Telegram 原生选群阶段会展示 reply keyboard；取消这类草稿时需要额外发送
/// `replyMarkupRemoveKeyboard`，否则客户端输入框下方可能继续残留“选择群组”按钮。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot::transfer::command::menu) struct CancelledMenuInput {
    pub(in crate::tgbot::transfer::command::menu) needs_reply_keyboard_cleanup: bool,
}

/// 取草稿的结果。
#[derive(Debug, Clone)]
pub(super) enum DraftTakeResult {
    None,
    Active(MenuInputDraft),
    Expired,
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
    Ok(cancel_menu_input_with_state(chat_id, user_id)
        .await?
        .is_some())
}

/// 取消一个菜单输入流程，并返回是否需要清理 reply keyboard。
pub(in crate::tgbot::transfer::command::menu) async fn cancel_menu_input_with_state(
    chat_id: i64,
    user_id: i64,
) -> anyhow::Result<Option<CancelledMenuInput>> {
    purge_expired().await?;
    let Some(removed) = find_draft_model(chat_id, user_id).await? else {
        return Ok(None);
    };
    delete_draft(chat_id, user_id).await?;
    let removed = match MenuInputDraft::from_model(removed) {
        Some(removed) => removed,
        None => {
            return Ok(Some(CancelledMenuInput {
                needs_reply_keyboard_cleanup: false,
            }));
        }
    };
    let needs_reply_keyboard_cleanup = step_uses_reply_keyboard(&removed.step);
    tracing::debug!(
        chat_id,
        user_id,
        needs_reply_keyboard_cleanup,
        "menu input draft cancelled"
    );
    Ok(Some(CancelledMenuInput {
        needs_reply_keyboard_cleanup,
    }))
}

/// 取出当前草稿；若草稿过期，则清理后返回过期状态。
pub(super) async fn take_current_draft(key: DraftKey) -> anyhow::Result<DraftTakeResult> {
    purge_expired().await?;
    let Some(model) = find_draft_model(key.0, key.1).await? else {
        return Ok(DraftTakeResult::None);
    };
    if model.expires_at <= now_utc8() {
        delete_draft(key.0, key.1).await?;
        purge_expired().await?;
        return Ok(DraftTakeResult::Expired);
    }
    delete_draft(key.0, key.1).await?;
    let Some(draft) = MenuInputDraft::from_model(model) else {
        tracing::warn!(
            chat_id = key.0,
            user_id = key.1,
            "menu input draft row is invalid, deleting"
        );
        return Ok(DraftTakeResult::None);
    };
    Ok(DraftTakeResult::Active(draft))
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
    let Some(draft) = MenuInputDraft::from_model(model) else {
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
    let db_conn = db::get_db().await?;
    purge_expired().await?;
    delete_draft(key.0, key.1).await?;
    db::menu_input_draft::Entity::insert(draft.into_active_model(key))
        .exec(db_conn)
        .await?;
    Ok(())
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

/// 记录用户最近一次确认执行的目标 chat。
pub(super) fn remember_last_target(chat_id: i64, user_id: i64, target_chat_id: i64) {
    let mut targets = MENU_LAST_TARGETS
        .lock()
        .expect("menu last target mutex poisoned");
    targets.insert((chat_id, user_id), target_chat_id);
    tracing::debug!(
        chat_id,
        user_id,
        target_chat_id,
        "menu last target remembered"
    );
}

/// 读取用户最近一次确认执行的目标 chat。
pub(super) fn last_target(chat_id: i64, user_id: i64) -> Option<i64> {
    let targets = MENU_LAST_TARGETS
        .lock()
        .expect("menu last target mutex poisoned");
    targets.get(&(chat_id, user_id)).copied()
}

/// 从输入阶段提取目标选择上下文。
pub(super) fn target_context_from_step(step: &MenuInputStep) -> Option<(MenuInputKind, String)> {
    match step {
        MenuInputStep::TargetChoice { kind, source_link }
        | MenuInputStep::TargetChat { kind, source_link }
        | MenuInputStep::ChatPicker { kind, source_link }
        | MenuInputStep::Confirm {
            kind, source_link, ..
        } => Some((*kind, source_link.clone())),
        MenuInputStep::SourceLink { .. }
        | MenuInputStep::JobId { .. }
        | MenuInputStep::PointLedgerUserId => None,
    }
}

/// 判断当前阶段是否曾展示 reply keyboard。
pub(super) fn step_uses_reply_keyboard(step: &MenuInputStep) -> bool {
    matches!(step, MenuInputStep::ChatPicker { .. })
}

/// 按主键读取草稿行。
async fn find_draft_model(
    chat_id: i64,
    user_id: i64,
) -> anyhow::Result<Option<db::menu_input_draft::Model>> {
    Ok(db::menu_input_draft::Entity::find()
        .filter(db::menu_input_draft::Column::RequestChatId.eq(chat_id))
        .filter(db::menu_input_draft::Column::SenderUserId.eq(user_id))
        .one(db::get_db().await?)
        .await?)
}

/// 按主键删除草稿。
async fn delete_draft(chat_id: i64, user_id: i64) -> anyhow::Result<()> {
    db::menu_input_draft::Entity::delete_many()
        .filter(db::menu_input_draft::Column::RequestChatId.eq(chat_id))
        .filter(db::menu_input_draft::Column::SenderUserId.eq(user_id))
        .exec(db::get_db().await?)
        .await?;
    Ok(())
}

/// 清理过期草稿。
async fn purge_expired() -> anyhow::Result<()> {
    db::menu_input_draft::Entity::delete_many()
        .filter(db::menu_input_draft::Column::ExpiresAt.lte(now_utc8()))
        .exec(db::get_db().await?)
        .await?;
    Ok(())
}

/// 统一生成 UTC+8 时间戳。
fn now_utc8() -> chrono::DateTime<chrono::FixedOffset> {
    chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap())
}

/// 菜单输入草稿超时时间（秒）。
fn input_ttl_seconds() -> u64 {
    crate::tgbot::transfer::runtime_config()
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

    // 草稿应按 chat + user 隔离，避免多个管理员互相覆盖输入。
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

    // admin 查询用户积分流水的输入草稿也必须持久化，保证重启后首页可继续输入。
    #[tokio::test]
    async fn test_point_ledger_user_id_draft_roundtrip() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (900_007, 900_008);
        put_draft(key, MenuInputDraft::point_ledger_user_id()).await?;

        let draft = take_current_draft(key).await?;

        assert!(matches!(
            draft,
            DraftTakeResult::Active(MenuInputDraft {
                step: MenuInputStep::PointLedgerUserId
            })
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

    // 原生选群阶段会展示 reply keyboard，因此取消时需要额外清理键盘。
    #[test]
    fn test_step_uses_reply_keyboard_only_for_chat_picker() {
        let source_link = "https://t.me/c/1/2".to_owned();

        assert!(step_uses_reply_keyboard(&MenuInputStep::ChatPicker {
            kind: MenuInputKind::Transfer,
            source_link: source_link.clone(),
        }));
        assert!(!step_uses_reply_keyboard(&MenuInputStep::TargetChoice {
            kind: MenuInputKind::Transfer,
            source_link,
        }));
        assert!(!step_uses_reply_keyboard(&MenuInputStep::JobId {
            action: MenuJobAction::Status,
        }));
    }

    // 菜单任务动作应稳定映射到 `/job` 的短参数，避免交互入口和命令入口语义分叉。
    #[test]
    fn test_menu_job_action_command_action() {
        assert_eq!(MenuJobAction::Status.command_action(), "st");
        assert_eq!(MenuJobAction::Pause.command_action(), "p");
        assert_eq!(MenuJobAction::Resume.command_action(), "r");
        assert_eq!(MenuJobAction::Stop.command_action(), "s");
    }

    // 上次目标只是交互捷径，按 chat + user 隔离，避免多个管理员互相覆盖。
    #[test]
    fn test_last_target_is_isolated_by_chat_and_user() {
        remember_last_target(10, 20, -100);

        assert_eq!(last_target(10, 20), Some(-100));
        assert_eq!(last_target(10, 21), None);
        assert_eq!(last_target(11, 20), None);
    }
}
