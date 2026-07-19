// file_cache 引用计数管理。
// 这里优先使用 SeaORM / SeaQuery 的表达式构造原子更新，避免并发任务完成时发生
// active_refs 读后写覆盖，同时尽量减少数据库方言分支。

use std::collections::HashMap;

use sea_orm::sea_query::{Expr, OnConflict};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter, TryInsertResult};

use crate::db;

use super::super::item::list_items_by_job_on_conn;
#[cfg(test)]
use super::super::{FILE_CACHE_DELETING_RETRY_DELAY_MS, FILE_CACHE_DELETING_RETRY_LIMIT};
use super::super::{
    FILE_CACHE_STATUS_DELETE_FAILED, FILE_CACHE_STATUS_DELETING, FILE_CACHE_STATUS_PENDING,
    is_text_file_key, now_utc8,
};

/// 任务完成后释放本任务引用：
/// - active_refs 归零后进入“延迟删除队列”
/// - delete_after = now + delay_minutes
#[cfg(test)]
pub(in crate::tgbot::transfer) async fn release_job_file_refs(
    job_id: i64,
    delay_minutes: i64,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    release_job_file_refs_on_conn(db_conn, job_id, delay_minutes).await
}

/// 在指定连接/事务内释放任务持有的文件引用。
pub(in crate::tgbot::transfer::store) async fn release_job_file_refs_on_conn<C>(
    conn: &C,
    job_id: i64,
    delay_minutes: i64,
) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    let items = list_items_by_job_on_conn(conn, job_id).await?;
    let mut refs: HashMap<(String, String), i32> = HashMap::new();
    let mut released_item_ids = Vec::new();
    for item in items {
        if item.file_ref_released || is_text_file_key(&item.file_key) {
            continue;
        }
        released_item_ids.push(item.id);
        *refs
            .entry((item.file_owner_client_role, item.file_key))
            .or_insert(0) += 1;
    }

    if refs.is_empty() {
        return Ok(());
    }

    release_file_ref_counts_on_conn(conn, refs, delay_minutes).await?;
    db::transfer_item::Entity::update_many()
        .col_expr(
            db::transfer_item::Column::FileRefReleased,
            Expr::value(true),
        )
        .col_expr(
            db::transfer_item::Column::UpdatedAt,
            Expr::value(now_utc8()),
        )
        .filter(db::transfer_item::Column::Id.is_in(released_item_ids))
        .exec(conn)
        .await?;
    Ok(())
}

/// 在指定连接/事务内按 file_key 批量扣减引用计数。
pub(in crate::tgbot::transfer::store) async fn release_file_ref_counts_on_conn<C>(
    conn: &C,
    refs: HashMap<(String, String), i32>,
    delay_minutes: i64,
) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    let now = now_utc8();
    let delete_after = now + chrono::Duration::minutes(std::cmp::max(delay_minutes, 0));

    for ((owner_client_role, file_key), dec) in refs {
        // 单条 UPDATE 表达式内完成扣减与归零后的删除计划写入，避免并发任务读后写覆盖。
        db::file_cache::Entity::update_many()
            .col_expr(
                db::file_cache::Column::ActiveRefs,
                Expr::case(
                    Expr::col(db::file_cache::Column::ActiveRefs).gt(dec),
                    Expr::col(db::file_cache::Column::ActiveRefs).sub(dec),
                )
                .finally(0)
                .into(),
            )
            .col_expr(
                db::file_cache::Column::LastRefZeroAt,
                Expr::case(Expr::col(db::file_cache::Column::ActiveRefs).lte(dec), now)
                    .finally(Expr::null())
                    .into(),
            )
            .col_expr(
                db::file_cache::Column::DeleteAfter,
                Expr::case(
                    Expr::col(db::file_cache::Column::ActiveRefs).lte(dec),
                    delete_after,
                )
                .finally(Expr::null())
                .into(),
            )
            .col_expr(db::file_cache::Column::UpdatedAt, Expr::value(now))
            .col_expr(db::file_cache::Column::LastUsedAt, Expr::value(now))
            .filter(db::file_cache::Column::OwnerClientRole.eq(owner_client_role))
            .filter(db::file_cache::Column::FileKey.eq(file_key))
            .exec(conn)
            .await?;
    }

    Ok(())
}

/// 为 file_key 增加引用计数：
/// - 新记录：active_refs = 1
/// - 旧记录：active_refs + 1，并清除删除计划
#[cfg(test)]
pub(in crate::tgbot::transfer) async fn acquire_file_ref(file_key: &str) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    for _ in 0..FILE_CACHE_DELETING_RETRY_LIMIT {
        if try_acquire_file_ref_on_conn(db_conn, "user", file_key).await? {
            return Ok(());
        }

        // GC 正在删除同一 file_key；等待旧记录删除或标记失败后再重新引用。
        tokio::time::sleep(std::time::Duration::from_millis(
            FILE_CACHE_DELETING_RETRY_DELAY_MS,
        ))
        .await;
    }

    anyhow::bail!("file cache is being deleted, file_key={}", file_key)
}

/// 尝试在指定连接/事务内增加一次文件引用。
///
/// 返回 false 表示该 file_key 正被 GC 标记为 deleting，调用方应回滚并稍后重试。
pub(in crate::tgbot::transfer::store) async fn try_acquire_file_ref_on_conn<C>(
    conn: &C,
    owner_client_role: &str,
    file_key: &str,
) -> anyhow::Result<bool>
where
    C: ConnectionTrait,
{
    let now = now_utc8();
    let result = db::file_cache::Entity::insert(db::file_cache::ActiveModel {
        owner_client_role: sea_orm::ActiveValue::Set(owner_client_role.to_owned()),
        file_key: sea_orm::ActiveValue::Set(file_key.to_owned()),
        status: sea_orm::ActiveValue::Set(FILE_CACHE_STATUS_PENDING.to_owned()),
        size_bytes: sea_orm::ActiveValue::Set(None),
        td_file_id: sea_orm::ActiveValue::Set(None),
        local_path: sea_orm::ActiveValue::Set(None),
        last_error: sea_orm::ActiveValue::Set(None),
        active_refs: sea_orm::ActiveValue::Set(1),
        last_ref_zero_at: sea_orm::ActiveValue::Set(None),
        delete_after: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        last_used_at: sea_orm::ActiveValue::Set(now),
    })
    .on_conflict(
        OnConflict::columns([
            db::file_cache::Column::OwnerClientRole,
            db::file_cache::Column::FileKey,
        ])
        .values([
            (
                db::file_cache::Column::ActiveRefs,
                Expr::col(db::file_cache::Column::ActiveRefs).add(1),
            ),
            (db::file_cache::Column::LastRefZeroAt, Expr::null()),
            (db::file_cache::Column::DeleteAfter, Expr::null()),
            (db::file_cache::Column::UpdatedAt, Expr::value(now)),
            (db::file_cache::Column::LastUsedAt, Expr::value(now)),
            (
                db::file_cache::Column::LastError,
                Expr::case(
                    Expr::col(db::file_cache::Column::Status).eq(FILE_CACHE_STATUS_DELETE_FAILED),
                    Expr::null(),
                )
                .finally(Expr::col(db::file_cache::Column::LastError))
                .into(),
            ),
            (
                db::file_cache::Column::Status,
                Expr::case(
                    Expr::col(db::file_cache::Column::Status).eq(FILE_CACHE_STATUS_DELETE_FAILED),
                    FILE_CACHE_STATUS_PENDING,
                )
                .finally(Expr::col(db::file_cache::Column::Status))
                .into(),
            ),
        ])
        .action_and_where(Expr::col(db::file_cache::Column::Status).ne(FILE_CACHE_STATUS_DELETING))
        .to_owned(),
    )
    .try_insert()
    .exec_without_returning(conn)
    .await?;

    Ok(matches!(result, TryInsertResult::Inserted(rows) if rows > 0))
}
