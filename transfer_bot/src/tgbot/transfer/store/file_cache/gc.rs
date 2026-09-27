// file_cache 延迟删除队列管理。
// GC 使用“先认领再删文件再删记录”的流程，避免和新任务引用同一文件发生竞态。

use sea_orm::sea_query::Expr;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::db;

use super::super::{FILE_CACHE_STATUS_DELETE_FAILED, FILE_CACHE_STATUS_DELETING, now_utc8};

/// 扫描已到期的 file_cache 删除队列项。
///
/// 筛选条件为：活跃引用数归零（`active_refs = 0`）且删除期限已到达（`delete_after <= now`）。
/// 结果按计划删除时间升序排列，以便优先处理最早到期的文件。
///
/// # 参数
/// - `now`: 当前参考时间（通常为 UTC+8）。
/// - `limit`: 单批次扫描并返回的最大记录数。
///
/// # 返回值
/// - 符合到期条件的缓存记录列表。
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
///
/// # 参数
/// - `owner_client_role`: 文件归属的客户端角色（如 "user"、"bot"）。
/// - `file_key`: 文件的唯一标识键。
/// - `now`: 认领时间戳。
///
/// # 返回值
/// - `Some(Model)` 表示成功认领该记录的所有权；`None` 表示已被其它并发 GC 认领或被新任务重新引用。
pub(in crate::tgbot::transfer) async fn claim_file_cache_for_delete(
    owner_client_role: &str,
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
        .filter(db::file_cache::Column::OwnerClientRole.eq(owner_client_role.to_owned()))
        .filter(db::file_cache::Column::FileKey.eq(file_key.to_owned()))
        .filter(db::file_cache::Column::ActiveRefs.eq(0))
        .filter(db::file_cache::Column::DeleteAfter.lte(now))
        .filter(db::file_cache::Column::Status.ne(FILE_CACHE_STATUS_DELETING))
        .exec(db_conn)
        .await?;

    if rs.rows_affected == 0 {
        return Ok(None);
    }

    db::file_cache::Entity::find_by_id((owner_client_role.to_owned(), file_key.to_owned()))
        .one(db_conn)
        .await
        .map_err(Into::into)
}

/// 删除 file_cache 数据库记录（物理磁盘文件已清理后调用）。
///
/// 仅删除状态为 `deleting` 且引用仍为 0 的记录。
///
/// # 参数
/// - `owner_client_role`: 客户端角色。
/// - `file_key`: 文件键。
pub(in crate::tgbot::transfer) async fn delete_file_cache(
    owner_client_role: &str,
    file_key: &str,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    db::file_cache::Entity::delete_many()
        .filter(db::file_cache::Column::OwnerClientRole.eq(owner_client_role.to_owned()))
        .filter(db::file_cache::Column::FileKey.eq(file_key.to_owned()))
        .filter(db::file_cache::Column::ActiveRefs.eq(0))
        .filter(db::file_cache::Column::Status.eq(FILE_CACHE_STATUS_DELETING))
        .exec(db_conn)
        .await?;
    Ok(())
}

/// 记录删除失败信息，便于后续重试排查。
///
/// 将状态变更为 `delete_failed`，并设定延迟重试时间戳，防止密集轮询报错。
///
/// # 参数
/// - `owner_client_role`: 客户端角色。
/// - `file_key`: 文件键。
/// - `err`: 失败的错误原因描述。
/// - `retry_after`: 下一次允许重试删除的时间。
pub(in crate::tgbot::transfer) async fn mark_file_cache_delete_failed(
    owner_client_role: &str,
    file_key: &str,
    err: String,
    retry_after: chrono::DateTime<chrono::FixedOffset>,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    if let Some(model) =
        db::file_cache::Entity::find_by_id((owner_client_role.to_owned(), file_key.to_owned()))
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
