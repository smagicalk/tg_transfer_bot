// 任务子项读写。

use std::collections::HashMap;

use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::TransactionTrait;
use sea_orm::sea_query::Expr;

use crate::db;

use super::super::file;
use super::file_cache::try_acquire_file_ref_on_conn;
use super::{
    FILE_CACHE_DELETING_RETRY_DELAY_MS, FILE_CACHE_DELETING_RETRY_LIMIT, ITEM_STATUS_PENDING,
    is_text_file_key, now_utc8,
};

/// 按当前 bundle 对齐 transfer_item：
/// - 已存在条目复用
/// - 新出现条目补创建
/// - 新建媒体条目会增加 file_cache 引用计数
pub(in crate::tgbot::transfer) async fn ensure_items_for_bundle(
    job_id: i64,
    messages: &[tdlib_rs::types::Message],
) -> anyhow::Result<Vec<db::transfer_item::Model>> {
    let db_conn = db::get_db().await?;
    let old_items = db::transfer_item::Entity::find()
        .filter(db::transfer_item::Column::JobId.eq(job_id))
        .all(db_conn)
        .await?;

    let mut old_map: HashMap<(i64, i64), db::transfer_item::Model> = HashMap::new();
    for item in old_items {
        old_map.insert((item.source_chat_id, item.source_message_id), item);
    }

    let mut result = Vec::with_capacity(messages.len());
    for msg in messages {
        let key = (msg.chat_id, msg.id);
        if let Some(old) = old_map.get(&key) {
            result.push(old.clone());
            continue;
        }

        let file_key = file::extract_file_key(msg)
            .unwrap_or_else(|| format!("text:{}:{}", msg.chat_id, msg.id));
        let model = insert_item_with_optional_file_ref(db_conn, job_id, msg, file_key).await?;
        result.push(model);
    }

    Ok(result)
}

/// 原子创建子项并增加文件引用。
///
/// 这里把 `transfer_item` 插入和 `file_cache.active_refs + 1` 放进同一个事务：
/// - 防止“子项已插入但引用增加失败”导致后续取消/完成时错误扣引用；
/// - 防止“引用已增加但子项插入失败”导致文件永远不进删除队列。
async fn insert_item_with_optional_file_ref(
    db_conn: &sea_orm::DatabaseConnection,
    job_id: i64,
    msg: &tdlib_rs::types::Message,
    file_key: String,
) -> anyhow::Result<db::transfer_item::Model> {
    let is_text = is_text_file_key(&file_key);
    for _ in 0..FILE_CACHE_DELETING_RETRY_LIMIT {
        let txn = db_conn.begin().await?;

        if !is_text && !try_acquire_file_ref_on_conn(&txn, &file_key).await? {
            txn.rollback().await?;
            tokio::time::sleep(std::time::Duration::from_millis(
                FILE_CACHE_DELETING_RETRY_DELAY_MS,
            ))
            .await;
            continue;
        }

        let now = now_utc8();
        let insert_result = db::transfer_item::ActiveModel {
            job_id: sea_orm::ActiveValue::Set(job_id),
            source_chat_id: sea_orm::ActiveValue::Set(msg.chat_id),
            source_message_id: sea_orm::ActiveValue::Set(msg.id),
            file_key: sea_orm::ActiveValue::Set(file_key.clone()),
            status: sea_orm::ActiveValue::Set(ITEM_STATUS_PENDING.to_owned()),
            retry_count: sea_orm::ActiveValue::Set(0),
            error_message: sea_orm::ActiveValue::Set(None),
            created_at: sea_orm::ActiveValue::Set(now),
            updated_at: sea_orm::ActiveValue::Set(now),
            ..Default::default()
        }
        .insert(&txn)
        .await;

        match insert_result {
            Ok(model) => {
                txn.commit().await?;
                return Ok(model);
            }
            Err(err) => {
                txn.rollback().await?;
                // 多实例或恢复竞争下可能已有相同子项，读取现有记录即可，不能重复增加引用。
                if let Some(existing) =
                    find_item_by_job_source(db_conn, job_id, msg.chat_id, msg.id).await?
                {
                    return Ok(existing);
                }
                return Err(err.into());
            }
        }
    }

    anyhow::bail!("file cache is being deleted, file_key={}", file_key)
}

/// 按任务内源消息定位子项。
async fn find_item_by_job_source<C>(
    conn: &C,
    job_id: i64,
    source_chat_id: i64,
    source_message_id: i64,
) -> anyhow::Result<Option<db::transfer_item::Model>>
where
    C: ConnectionTrait,
{
    db::transfer_item::Entity::find()
        .filter(db::transfer_item::Column::JobId.eq(job_id))
        .filter(db::transfer_item::Column::SourceChatId.eq(source_chat_id))
        .filter(db::transfer_item::Column::SourceMessageId.eq(source_message_id))
        .one(conn)
        .await
        .map_err(Into::into)
}

/// 查询任务所有子项。
pub(in crate::tgbot::transfer) async fn list_items_by_job(
    job_id: i64,
) -> anyhow::Result<Vec<db::transfer_item::Model>> {
    let db_conn = db::get_db().await?;
    list_items_by_job_on_conn(db_conn, job_id).await
}

/// 在指定连接/事务内查询任务子项。
pub(super) async fn list_items_by_job_on_conn<C>(
    conn: &C,
    job_id: i64,
) -> anyhow::Result<Vec<db::transfer_item::Model>>
where
    C: ConnectionTrait,
{
    db::transfer_item::Entity::find()
        .filter(db::transfer_item::Column::JobId.eq(job_id))
        .order_by_asc(db::transfer_item::Column::Id)
        .all(conn)
        .await
        .map_err(Into::into)
}

/// 更新 transfer_item 状态与错误信息。
pub(in crate::tgbot::transfer) async fn set_item_status(
    item_id: i64,
    status: &str,
    error_message: Option<String>,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    set_item_status_on_conn(db_conn, item_id, status, error_message).await
}

/// 在指定连接/事务内更新 transfer_item 状态。
///
/// 事务化 finish/cancel 会复用该函数，保证“子项状态 + 主任务终态 + 文件引用释放”
/// 要么一起提交，要么一起回滚。
pub(super) async fn set_item_status_on_conn<C>(
    conn: &C,
    item_id: i64,
    status: &str,
    error_message: Option<String>,
) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    db::transfer_item::Entity::update_many()
        .col_expr(
            db::transfer_item::Column::Status,
            Expr::value(status.to_owned()),
        )
        .col_expr(
            db::transfer_item::Column::UpdatedAt,
            Expr::value(now_utc8()),
        )
        .col_expr(
            db::transfer_item::Column::ErrorMessage,
            Expr::value(error_message),
        )
        .filter(db::transfer_item::Column::Id.eq(item_id))
        .exec(conn)
        .await?;
    Ok(())
}
