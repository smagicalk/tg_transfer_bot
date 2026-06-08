// file_cache 延迟删除队列管理。
// GC 使用“先认领再删文件再删记录”的流程，避免和新任务引用同一文件发生竞态。

use sea_orm::sea_query::Expr;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::db;

use super::super::{FILE_CACHE_STATUS_DELETE_FAILED, FILE_CACHE_STATUS_DELETING, now_utc8};

/// 扫描已到期的 file_cache 删除队列项。
pub(in crate::tgbot::transfer) async fn list_due_file_cache(
    now: chrono::DateTime<chrono::FixedOffset>,
    limit: u64,
) -> anyhow::Result<Vec<db::file_cache::Model>> {
    let db_conn = db::get_db().await?;
    db::file_cache::Entity::find()
        .filter(db::file_cache::Column::ActiveRefs.eq(0))
        .filter(db::file_cache::Column::DeleteAfter.lte(now))
        .order_by_asc(db::file_cache::Column::DeleteAfter)
        .limit(limit)
        .all(db_conn)
        .await
        .map_err(Into::into)
}

/// 原子认领一条到期删除记录。
///
/// GC 先把状态改成 deleting，再删除本地文件；新增引用遇到 deleting 会等待，
/// 从而避免“刚被重新引用的文件仍被删除”的竞态。
pub(in crate::tgbot::transfer) async fn claim_file_cache_for_delete(
    file_key: &str,
    now: chrono::DateTime<chrono::FixedOffset>,
) -> anyhow::Result<Option<db::file_cache::Model>> {
    let db_conn = db::get_db().await?;
    let rs = db::file_cache::Entity::update_many()
        .col_expr(
            db::file_cache::Column::Status,
            Expr::value(FILE_CACHE_STATUS_DELETING),
        )
        .col_expr(db::file_cache::Column::UpdatedAt, Expr::value(now))
        .filter(db::file_cache::Column::FileKey.eq(file_key.to_owned()))
        .filter(db::file_cache::Column::ActiveRefs.eq(0))
        .filter(db::file_cache::Column::DeleteAfter.lte(now))
        .filter(db::file_cache::Column::Status.ne(FILE_CACHE_STATUS_DELETING))
        .exec(db_conn)
        .await?;

    if rs.rows_affected == 0 {
        return Ok(None);
    }

    db::file_cache::Entity::find_by_id(file_key.to_owned())
        .one(db_conn)
        .await
        .map_err(Into::into)
}

/// 删除 file_cache 记录（文件已清理后调用）。
pub(in crate::tgbot::transfer) async fn delete_file_cache(file_key: &str) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    db::file_cache::Entity::delete_many()
        .filter(db::file_cache::Column::FileKey.eq(file_key.to_owned()))
        .filter(db::file_cache::Column::ActiveRefs.eq(0))
        .filter(db::file_cache::Column::Status.eq(FILE_CACHE_STATUS_DELETING))
        .exec(db_conn)
        .await?;
    Ok(())
}

/// 记录删除失败信息，便于后续重试排查。
pub(in crate::tgbot::transfer) async fn mark_file_cache_delete_failed(
    file_key: &str,
    err: String,
    retry_after: chrono::DateTime<chrono::FixedOffset>,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    if let Some(model) = db::file_cache::Entity::find_by_id(file_key.to_owned())
        .one(db_conn)
        .await?
    {
        let mut active: db::file_cache::ActiveModel = model.into();
        active.status = sea_orm::ActiveValue::Set(FILE_CACHE_STATUS_DELETE_FAILED.to_owned());
        active.last_error = sea_orm::ActiveValue::Set(Some(err));
        active.updated_at = sea_orm::ActiveValue::Set(now_utc8());
        // 删除失败后延后重试，避免危险路径或磁盘错误在短 GC 间隔下反复刷日志。
        active.delete_after = sea_orm::ActiveValue::Set(Some(retry_after));
        active.update(db_conn).await?;
    }
    Ok(())
}
