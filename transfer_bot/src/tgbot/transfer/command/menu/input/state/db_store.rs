//! `/menu` 菜单输入草稿持久化存储（Database Store）模块。
//!
//! 负责与数据库 `menu_input_draft` 表交互，处理会话草稿的读写、轻量乐观锁 CAS（Compare-And-Swap）条件更新、删除及过期数据物理清理。
//! 隔离底层 SQL 与 Sea-ORM 查询构建细节，使输入状态机核心代码保持整洁。

use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, sea_query::OnConflict};

use crate::db;

use super::{DraftFields, DraftKey, MenuInputDraft, input_ttl_seconds, now_utc8};

/// 无锁（Unlocked）方式将菜单输入草稿写入数据库。
///
/// 内部流程：
/// 1. 顺带清理已超时的全局过期草稿行（`purge_expired`）；
/// 2. 使用 Sea-ORM 的 `on_conflict` 机制，在 `(request_chat_id, sender_user_id)` 冲突时执行整行字段覆盖更新（Upsert）。
///
/// # 约定与权限
/// 本函数不自动申请进程内互斥锁，**仅供外部已经持有 `MenuDraftKeyGuard` 的状态层函数调用**。
///
/// # 参数说明
/// - `key`: 会话唯一标识元组 `(request_chat_id, sender_user_id)`
/// - `draft`: 待存入的菜单输入草稿对象
pub(super) async fn put_draft_unlocked(key: DraftKey, draft: MenuInputDraft) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    // 写入前顺便触发物理清理已过期的陈旧草稿
    purge_expired().await?;
    // 执行插入或冲突更新
    db::menu_input_draft::Entity::insert(draft.into_active_model(key))
        .on_conflict(
            OnConflict::columns([
                db::menu_input_draft::Column::RequestChatId,
                db::menu_input_draft::Column::SenderUserId,
            ])
            .update_columns([
                db::menu_input_draft::Column::Step,
                db::menu_input_draft::Column::InputKind,
                db::menu_input_draft::Column::JobAction,
                db::menu_input_draft::Column::SourceLink,
                db::menu_input_draft::Column::TargetChatId,
                db::menu_input_draft::Column::CreatedAt,
                db::menu_input_draft::Column::UpdatedAt,
                db::menu_input_draft::Column::ExpiresAt,
            ])
            .to_owned(),
        )
        .exec(db_conn)
        .await?;
    Ok(())
}

/// 乐观并发删除：仅当数据库当前行仍严格匹配之前读取到的业务模型快照时才执行物理删除。
///
/// 避免并发交互或超时竞争导致误删了用户刚刚新创建或已推进的草稿。
///
/// # 参数说明
/// - `model`: 此前查询并校验通过的数据库草稿模型快照
///
/// # 返回值
/// - `Ok(true)`: 成功删除且受影响行数为 1（确认本次操作消费了对应草稿）
/// - `Ok(false)`: 数据库行已被其他协程更改或已被删除，CAS 校验不通过
pub(super) async fn delete_draft_if_current(
    model: &db::menu_input_draft::Model,
) -> anyhow::Result<bool> {
    let result = db::menu_input_draft::Entity::delete_many()
        .filter(draft_match_condition(model))
        .exec(db::get_db().await?)
        .await?;
    Ok(result.rows_affected == 1)
}

/// 乐观并发更新：仅当数据库当前行仍严格等于之前读取到的快照时，才将其推进到下一步草稿状态。
///
/// 内部会刷新 `updated_at` 并根据系统配置重置 `expires_at` 超时时间戳。
///
/// # 参数说明
/// - `model`: 此前读取到的草稿模型快照
/// - `draft`: 包含新步骤与新字段的目标草稿对象
///
/// # 返回值
/// - `Ok(true)`: CAS 校验通过且成功更新
/// - `Ok(false)`: 行数据已被其他事件并发修改，更新失败
pub(super) async fn update_draft_if_current(
    model: &db::menu_input_draft::Model,
    draft: MenuInputDraft,
) -> anyhow::Result<bool> {
    let now = now_utc8();
    // 计算新的过期时间戳
    let expires_at = now + chrono::Duration::seconds(input_ttl_seconds() as i64);
    // 从目标步骤解构出待写入的各个数据库字段
    let fields = DraftFields::from_step(draft.step);
    let result = db::menu_input_draft::Entity::update_many()
        .col_expr(
            db::menu_input_draft::Column::Step,
            sea_orm::sea_query::Expr::value(fields.step),
        )
        .col_expr(
            db::menu_input_draft::Column::InputKind,
            sea_orm::sea_query::Expr::value(fields.input_kind.map(str::to_owned)),
        )
        .col_expr(
            db::menu_input_draft::Column::JobAction,
            sea_orm::sea_query::Expr::value(fields.job_action.map(str::to_owned)),
        )
        .col_expr(
            db::menu_input_draft::Column::SourceLink,
            sea_orm::sea_query::Expr::value(fields.source_link),
        )
        .col_expr(
            db::menu_input_draft::Column::TargetChatId,
            sea_orm::sea_query::Expr::value(fields.target_chat_id),
        )
        .col_expr(
            db::menu_input_draft::Column::CreatedAt,
            sea_orm::sea_query::Expr::value(now),
        )
        .col_expr(
            db::menu_input_draft::Column::UpdatedAt,
            sea_orm::sea_query::Expr::value(now),
        )
        .col_expr(
            db::menu_input_draft::Column::ExpiresAt,
            sea_orm::sea_query::Expr::value(expires_at),
        )
        .filter(draft_match_condition(model))
        .exec(db::get_db().await?)
        .await?;
    Ok(result.rows_affected == 1)
}

/// 构造轻量 CAS 匹配条件表达式。
///
/// 校验数据库当前行的每个字段是否均与读取时的 `model` 一致：
/// - 非空列：使用严格相等（`.eq(val)`）；
/// - 可空列：若 `Some(val)` 则匹配 `.eq(val)`，若 `None` 则匹配 `.is_null()`。
/// 这种设计确保在不同数据库方言（SQLite / PostgreSQL）下空值语义统一无歧义。
///
/// # 参数说明
/// - `model`: 之前读取的草稿数据行模型引用
fn draft_match_condition(model: &db::menu_input_draft::Model) -> Condition {
    let mut condition = Condition::all()
        .add(db::menu_input_draft::Column::RequestChatId.eq(model.request_chat_id))
        .add(db::menu_input_draft::Column::SenderUserId.eq(model.sender_user_id))
        .add(db::menu_input_draft::Column::Step.eq(model.step.clone()));

    // 匹配 input_kind 字段
    condition = match &model.input_kind {
        Some(input_kind) => condition.add(db::menu_input_draft::Column::InputKind.eq(input_kind)),
        None => condition.add(db::menu_input_draft::Column::InputKind.is_null()),
    };
    // 匹配 job_action 字段
    condition = match &model.job_action {
        Some(job_action) => condition.add(db::menu_input_draft::Column::JobAction.eq(job_action)),
        None => condition.add(db::menu_input_draft::Column::JobAction.is_null()),
    };
    // 匹配 source_link 字段
    condition = match &model.source_link {
        Some(source_link) => {
            condition.add(db::menu_input_draft::Column::SourceLink.eq(source_link))
        }
        None => condition.add(db::menu_input_draft::Column::SourceLink.is_null()),
    };
    // 匹配 target_chat_id 字段
    match model.target_chat_id {
        Some(target_chat_id) => {
            condition.add(db::menu_input_draft::Column::TargetChatId.eq(target_chat_id))
        }
        None => condition.add(db::menu_input_draft::Column::TargetChatId.is_null()),
    }
}

/// 根据会话主键 `(chat_id, user_id)` 查询数据库中的草稿行。
///
/// # 参数说明
/// - `chat_id`: 交互会话 ID
/// - `user_id`: 用户 Telegram ID
///
/// # 返回值
/// 若存在则返回 `Some(Model)`，否则返回 `None`。
pub(super) async fn find_draft_model(
    chat_id: i64,
    user_id: i64,
) -> anyhow::Result<Option<db::menu_input_draft::Model>> {
    Ok(db::menu_input_draft::Entity::find()
        .filter(db::menu_input_draft::Column::RequestChatId.eq(chat_id))
        .filter(db::menu_input_draft::Column::SenderUserId.eq(user_id))
        .one(db::get_db().await?)
        .await?)
}

/// 根据会话主键 `(chat_id, user_id)` 无条件删除对应的草稿记录。
///
/// 常用于流程主动取消或用户强行重置会话。
pub(super) async fn delete_draft(chat_id: i64, user_id: i64) -> anyhow::Result<()> {
    db::menu_input_draft::Entity::delete_many()
        .filter(db::menu_input_draft::Column::RequestChatId.eq(chat_id))
        .filter(db::menu_input_draft::Column::SenderUserId.eq(user_id))
        .exec(db::get_db().await?)
        .await?;
    Ok(())
}

/// 物理清理所有已到达过期时间（`expires_at <= now`）的陈旧输入草稿行。
pub(super) async fn purge_expired() -> anyhow::Result<()> {
    db::menu_input_draft::Entity::delete_many()
        .filter(db::menu_input_draft::Column::ExpiresAt.lte(now_utc8()))
        .exec(db::get_db().await?)
        .await?;
    Ok(())
}
