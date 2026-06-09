// file_cache 下载状态回填。
// 这些函数只更新缓存记录本身，不改变 active_refs，避免状态写入影响引用生命周期。

use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::db;
use crate::tgbot::transfer::file::{DownloadSeed, PreparedCacheMeta};

use super::super::{
    FILE_CACHE_STATUS_DELETING, FILE_CACHE_STATUS_DOWNLOADING, FILE_CACHE_STATUS_FAILED,
    FILE_CACHE_STATUS_READY, now_utc8,
};

/// 将 file_cache 标记为“下载中”。
/// 同时预写入 td_file_id 和 size_bytes，供实时进度查询使用。
pub(in crate::tgbot::transfer) async fn mark_file_cache_downloading(
    owner_client_role: &str,
    seed: &DownloadSeed,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    let now = now_utc8();
    let mut update = db::file_cache::Entity::update_many()
        .col_expr(
            db::file_cache::Column::Status,
            Expr::value(FILE_CACHE_STATUS_DOWNLOADING),
        )
        .col_expr(
            db::file_cache::Column::TdFileId,
            Expr::value(Some(seed.td_file_id)),
        )
        .col_expr(db::file_cache::Column::UpdatedAt, Expr::value(now))
        .col_expr(db::file_cache::Column::LastUsedAt, Expr::value(now))
        .col_expr(
            db::file_cache::Column::LastError,
            Expr::value(Option::<String>::None),
        );
    if seed.size_bytes.is_some() {
        update = update.col_expr(
            db::file_cache::Column::SizeBytes,
            Expr::value(seed.size_bytes),
        );
    }
    update
        .filter(db::file_cache::Column::OwnerClientRole.eq(owner_client_role.to_owned()))
        .filter(db::file_cache::Column::FileKey.eq(seed.file_key.clone()))
        .filter(db::file_cache::Column::ActiveRefs.gt(0))
        .filter(db::file_cache::Column::Status.ne(FILE_CACHE_STATUS_DELETING))
        .exec(db_conn)
        .await?;
    Ok(())
}

/// 回填 file_cache 的就绪信息（路径/文件ID/大小）。
pub(in crate::tgbot::transfer) async fn mark_file_cache_ready(
    owner_client_role: &str,
    meta: &PreparedCacheMeta,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    let now = now_utc8();
    db::file_cache::Entity::update_many()
        .col_expr(
            db::file_cache::Column::Status,
            Expr::value(FILE_CACHE_STATUS_READY),
        )
        .col_expr(
            db::file_cache::Column::TdFileId,
            Expr::value(Some(meta.td_file_id)),
        )
        .col_expr(
            db::file_cache::Column::LocalPath,
            Expr::value(Some(meta.local_path.clone())),
        )
        .col_expr(
            db::file_cache::Column::SizeBytes,
            Expr::value(meta.size_bytes),
        )
        .col_expr(
            db::file_cache::Column::LastError,
            Expr::value(Option::<String>::None),
        )
        .col_expr(db::file_cache::Column::UpdatedAt, Expr::value(now))
        .col_expr(db::file_cache::Column::LastUsedAt, Expr::value(now))
        .filter(db::file_cache::Column::OwnerClientRole.eq(owner_client_role.to_owned()))
        .filter(db::file_cache::Column::FileKey.eq(meta.file_key.clone()))
        .filter(db::file_cache::Column::ActiveRefs.gt(0))
        .filter(db::file_cache::Column::Status.ne(FILE_CACHE_STATUS_DELETING))
        .exec(db_conn)
        .await?;
    Ok(())
}

/// 标记 file_cache 失败信息（不变更引用计数）。
pub(in crate::tgbot::transfer) async fn mark_file_cache_failed(
    owner_client_role: &str,
    file_key: &str,
    err: String,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    let now = now_utc8();
    db::file_cache::Entity::update_many()
        .col_expr(
            db::file_cache::Column::Status,
            Expr::value(FILE_CACHE_STATUS_FAILED),
        )
        .col_expr(db::file_cache::Column::LastError, Expr::value(Some(err)))
        .col_expr(db::file_cache::Column::UpdatedAt, Expr::value(now))
        .col_expr(db::file_cache::Column::LastUsedAt, Expr::value(now))
        .filter(db::file_cache::Column::OwnerClientRole.eq(owner_client_role.to_owned()))
        .filter(db::file_cache::Column::FileKey.eq(file_key.to_owned()))
        .filter(db::file_cache::Column::ActiveRefs.gt(0))
        .filter(db::file_cache::Column::Status.ne(FILE_CACHE_STATUS_DELETING))
        .exec(db_conn)
        .await?;
    Ok(())
}
