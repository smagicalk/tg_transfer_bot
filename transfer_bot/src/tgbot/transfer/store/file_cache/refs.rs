// file_cache 引用计数管理。
// 这里使用 SQL 原子表达式，避免并发任务完成时发生 active_refs 读后写覆盖。

use std::collections::HashMap;

use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseBackend, EntityTrait, QueryFilter, Statement};

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
    let mut refs: HashMap<String, i32> = HashMap::new();
    let mut released_item_ids = Vec::new();
    for item in items {
        if item.file_ref_released || is_text_file_key(&item.file_key) {
            continue;
        }
        released_item_ids.push(item.id);
        *refs.entry(item.file_key).or_insert(0) += 1;
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
    refs: HashMap<String, i32>,
    delay_minutes: i64,
) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    let now = now_utc8();
    let delete_after = now + chrono::Duration::minutes(delay_minutes.max(0));

    for (file_key, dec) in refs {
        // 使用单条 UPDATE 表达式完成扣减，避免并发完成任务时读后写覆盖 active_refs。
        conn.execute_raw(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"
                UPDATE file_cache
                SET
                    active_refs = CASE
                        WHEN active_refs > ? THEN active_refs - ?
                        ELSE 0
                    END,
                    last_ref_zero_at = CASE
                        WHEN active_refs <= ? THEN ?
                        ELSE NULL
                    END,
                    delete_after = CASE
                        WHEN active_refs <= ? THEN ?
                        ELSE NULL
                    END,
                    updated_at = ?,
                    last_used_at = ?
                WHERE file_key = ?
                "#,
            vec![
                dec.into(),
                dec.into(),
                dec.into(),
                now.into(),
                dec.into(),
                delete_after.into(),
                now.into(),
                now.into(),
                file_key.into(),
            ],
        ))
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
        if try_acquire_file_ref_on_conn(db_conn, file_key).await? {
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
    file_key: &str,
) -> anyhow::Result<bool>
where
    C: ConnectionTrait,
{
    let now = now_utc8();
    let rs = conn
        .execute_raw(Statement::from_sql_and_values(
            DatabaseBackend::Sqlite,
            r#"
            INSERT INTO file_cache (
                file_key,
                status,
                size_bytes,
                td_file_id,
                local_path,
                last_error,
                active_refs,
                last_ref_zero_at,
                delete_after,
                created_at,
                updated_at,
                last_used_at
            )
            VALUES (?, ?, NULL, NULL, NULL, NULL, 1, NULL, NULL, ?, ?, ?)
            ON CONFLICT(file_key) DO UPDATE SET
                active_refs = file_cache.active_refs + 1,
                last_ref_zero_at = NULL,
                delete_after = NULL,
                updated_at = excluded.updated_at,
                last_used_at = excluded.last_used_at,
                last_error = CASE
                    WHEN file_cache.status = ? THEN NULL
                    ELSE file_cache.last_error
                END,
                status = CASE
                    WHEN file_cache.status = ? THEN ?
                    ELSE file_cache.status
                END
            WHERE file_cache.status <> ?
            "#,
            vec![
                file_key.to_owned().into(),
                FILE_CACHE_STATUS_PENDING.into(),
                now.into(),
                now.into(),
                now.into(),
                FILE_CACHE_STATUS_DELETE_FAILED.into(),
                FILE_CACHE_STATUS_DELETE_FAILED.into(),
                FILE_CACHE_STATUS_PENDING.into(),
                FILE_CACHE_STATUS_DELETING.into(),
            ],
        ))
        .await?;

    Ok(rs.rows_affected() > 0)
}
