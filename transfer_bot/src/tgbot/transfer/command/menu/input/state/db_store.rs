// `/menu` 输入状态的数据库辅助逻辑。
// 这里只放草稿行的读写、条件更新和过期清理，避免这些细节继续占满状态机 module。

use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseBackend, EntityTrait, QueryFilter, Statement,
    sea_query::OnConflict,
};

use crate::db;

use super::{
    DraftFields, DraftKey, MenuInputDraft, current_draft_values, input_ttl_seconds, now_utc8,
};

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
pub(super) async fn delete_draft_if_current(
    model: &db::menu_input_draft::Model,
) -> anyhow::Result<bool> {
    let result = db::get_db()
        .await?
        .execute_raw(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"
            DELETE FROM menu_input_draft
            WHERE request_chat_id = ?
              AND sender_user_id = ?
              AND step = ?
              AND input_kind IS ?
              AND job_action IS ?
              AND source_link IS ?
              AND target_chat_id IS ?
            "#,
            current_draft_values(model),
        ))
        .await?;
    Ok(result.rows_affected() == 1)
}

/// 仅当数据库行仍匹配刚才读到的业务字段时才推进到下一步。
pub(super) async fn update_draft_if_current(
    model: &db::menu_input_draft::Model,
    draft: MenuInputDraft,
) -> anyhow::Result<bool> {
    let now = now_utc8();
    let expires_at = now + chrono::Duration::seconds(input_ttl_seconds() as i64);
    let fields = DraftFields::from_step(draft.step);
    let mut values = vec![
        fields.step.to_owned().into(),
        fields.input_kind.map(str::to_owned).into(),
        fields.job_action.map(str::to_owned).into(),
        fields.source_link.into(),
        fields.target_chat_id.into(),
        now.into(),
        now.into(),
        expires_at.into(),
    ];
    values.extend(current_draft_values(model));

    let result = db::get_db()
        .await?
        .execute_raw(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"
            UPDATE menu_input_draft
            SET
                step = ?,
                input_kind = ?,
                job_action = ?,
                source_link = ?,
                target_chat_id = ?,
                created_at = ?,
                updated_at = ?,
                expires_at = ?
            WHERE request_chat_id = ?
              AND sender_user_id = ?
              AND step = ?
              AND input_kind IS ?
              AND job_action IS ?
              AND source_link IS ?
              AND target_chat_id IS ?
            "#,
            values,
        ))
        .await?;
    Ok(result.rows_affected() == 1)
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
