// `/menu` 输入状态的数据库辅助逻辑。
// 这里只放草稿行的读写、条件更新和过期清理，避免这些细节继续占满状态机 module。

use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, sea_query::OnConflict};

use crate::db;

use super::{DraftFields, DraftKey, MenuInputDraft, input_ttl_seconds, now_utc8};

/// 写回草稿的无锁实现。
///
/// 仅供已经持有草稿 key guard 的状态层函数调用；外部入口必须先经过状态机入口。
pub(super) async fn put_draft_unlocked(key: DraftKey, draft: MenuInputDraft) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    purge_expired().await?;
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

/// 仅当数据库行仍匹配刚才读到的业务字段时才删除。
///
/// 这里直接用 ORM 条件把普通值与 NULL 拆开表达，SQLite / PostgreSQL 都能复用同一套语义。
pub(super) async fn delete_draft_if_current(
    model: &db::menu_input_draft::Model,
) -> anyhow::Result<bool> {
    let result = db::menu_input_draft::Entity::delete_many()
        .filter(draft_match_condition(model))
        .exec(db::get_db().await?)
        .await?;
    Ok(result.rows_affected == 1)
}

/// 仅当数据库行仍匹配刚才读到的业务字段时才推进到下一步。
pub(super) async fn update_draft_if_current(
    model: &db::menu_input_draft::Model,
    draft: MenuInputDraft,
) -> anyhow::Result<bool> {
    let now = now_utc8();
    let expires_at = now + chrono::Duration::seconds(input_ttl_seconds() as i64);
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

/// 构造“数据库当前行仍等于我刚读到的快照”条件。
///
/// 这相当于一个轻量 CAS：
/// - 必填列用 `=`
/// - 可空列按 `Some(value)` / `NULL` 分支展开
///   避免不同数据库对空值比较语义的细节差异。
fn draft_match_condition(model: &db::menu_input_draft::Model) -> Condition {
    let mut condition = Condition::all()
        .add(db::menu_input_draft::Column::RequestChatId.eq(model.request_chat_id))
        .add(db::menu_input_draft::Column::SenderUserId.eq(model.sender_user_id))
        .add(db::menu_input_draft::Column::Step.eq(model.step.clone()));

    condition = match &model.input_kind {
        Some(input_kind) => condition.add(db::menu_input_draft::Column::InputKind.eq(input_kind)),
        None => condition.add(db::menu_input_draft::Column::InputKind.is_null()),
    };
    condition = match &model.job_action {
        Some(job_action) => condition.add(db::menu_input_draft::Column::JobAction.eq(job_action)),
        None => condition.add(db::menu_input_draft::Column::JobAction.is_null()),
    };
    condition = match &model.source_link {
        Some(source_link) => {
            condition.add(db::menu_input_draft::Column::SourceLink.eq(source_link))
        }
        None => condition.add(db::menu_input_draft::Column::SourceLink.is_null()),
    };
    match model.target_chat_id {
        Some(target_chat_id) => {
            condition.add(db::menu_input_draft::Column::TargetChatId.eq(target_chat_id))
        }
        None => condition.add(db::menu_input_draft::Column::TargetChatId.is_null()),
    }
}

/// 按主键读取草稿行。
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

/// 按主键删除草稿。
pub(super) async fn delete_draft(chat_id: i64, user_id: i64) -> anyhow::Result<()> {
    db::menu_input_draft::Entity::delete_many()
        .filter(db::menu_input_draft::Column::RequestChatId.eq(chat_id))
        .filter(db::menu_input_draft::Column::SenderUserId.eq(user_id))
        .exec(db::get_db().await?)
        .await?;
    Ok(())
}

/// 清理过期草稿。
pub(super) async fn purge_expired() -> anyhow::Result<()> {
    db::menu_input_draft::Entity::delete_many()
        .filter(db::menu_input_draft::Column::ExpiresAt.lte(now_utc8()))
        .exec(db::get_db().await?)
        .await?;
    Ok(())
}
