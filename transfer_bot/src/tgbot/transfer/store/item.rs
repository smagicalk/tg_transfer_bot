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
use crate::tgbot::transfer::types::{TransferBundle, client_role_as_str};

use super::super::file;
use super::file_cache::{release_file_ref_counts_on_conn, try_acquire_file_ref_on_conn};
use super::{
    FILE_CACHE_DELETING_RETRY_DELAY_MS, FILE_CACHE_DELETING_RETRY_LIMIT, ITEM_STATUS_OBSOLETE,
    ITEM_STATUS_PENDING, is_text_file_key, now_utc8,
};

/// 按当前抓取到的 bundle 对齐创建或复用 transfer_item 子项：
/// - 已存在的条目直接复用已有数据库 Model；
/// - 新出现的条目执行插入并在事务中为对应媒体增加 file_cache 引用；
/// - 纯文本条目不增加物理文件引用，默认标记 `file_ref_released = true`。
///
/// # 参数
/// - `job_id`: 主任务 ID。
/// - `bundle`: 从源聊天中爬取解析得到的消息组合包。
///
/// # 返回值
/// - 排序并对齐后的 `transfer_item` 列表。
pub(in crate::tgbot::transfer) async fn ensure_items_for_bundle(
    job_id: i64,
    bundle: &TransferBundle,
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

    let mut result = Vec::with_capacity(bundle.messages.len());
    for msg in &bundle.messages {
        let key = (msg.chat_id, msg.id);
        if let Some(old) = old_map.get(&key) {
            result.push(old.clone());
            continue;
        }

        let file_key = file_key_for_message(msg);
        let model = insert_item_with_optional_file_ref(
            db_conn,
            job_id,
            msg,
            file_key,
            client_role_as_str(bundle.source_client_role),
        )
        .await?;
        result.push(model);
    }

    Ok(result)
}

/// 按恢复执行时重新抓取到的 bundle 对齐 transfer_item 和 file_cache 引用。
///
/// 对齐与一致性保证规则：
/// 1. 新消息：新增子项并为其新 file_key 增加引用计数；
/// 2. 同一消息但 file_key 或角色变化：释放旧文件引用、为新文件增加引用，并将该子项状态重置为 pending；
/// 3. 旧消息在新 bundle 中消失（可能在源频道被撤回/删除）：标记为 `obsolete`（已废弃），提前释放其文件引用；
/// 4. 已提前释放引用的子项在数据库中记录 `file_ref_released = true`，后续任务完成或取消时不再重复扣减引用；
/// 5. 更新任务的主表快照（如条目总数 `total_items`、相册 ID 等）。
///
/// # 参数
/// - `job_id`: 主任务 ID。
/// - `bundle`: 重新抓取的源消息包。
/// - `delay_minutes`: 旧引用释放时的删除缓冲时间（分钟）。
pub(in crate::tgbot::transfer) async fn reconcile_items_for_bundle(
    job_id: i64,
    bundle: &TransferBundle,
    delay_minutes: i64,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    let txn = db_conn.begin().await?;
    let old_items = list_items_by_job_on_conn(&txn, job_id).await?;

    let mut old_map: HashMap<(i64, i64), db::transfer_item::Model> = HashMap::new();
    for item in old_items {
        old_map.insert((item.source_chat_id, item.source_message_id), item);
    }

    for msg in &bundle.messages {
        let key = (msg.chat_id, msg.id);
        let file_key = file_key_for_message(msg);
        let owner = client_role_as_str(bundle.source_client_role);
        if let Some(old) = old_map.remove(&key) {
            reconcile_existing_item_on_conn(&txn, old, file_key, owner, delay_minutes).await?;
        } else {
            insert_item_with_optional_file_ref_on_conn(&txn, job_id, msg, file_key, owner).await?;
        }
    }

    for (_, old) in old_map {
        mark_item_obsolete_on_conn(&txn, old, delay_minutes).await?;
    }

    update_job_source_snapshot_on_conn(&txn, job_id, bundle).await?;
    txn.commit().await?;
    Ok(())
}

/// 原子创建子项并增加文件引用。
///
/// 这里把 `transfer_item` 插入和 `file_cache.active_refs + 1` 放进同一个事务：
/// - 防止“子项已插入但引用增加失败”导致后续取消/完成时错误扣引用；
/// - 防止“引用已增加但子项插入失败”导致文件永远不进删除队列。
///
/// # 参数
/// - `db_conn`: 数据库连接池。
/// - `job_id`: 主任务 ID。
/// - `msg`: 来源 TDLib 消息。
/// - `file_key`: 文件唯一标识。
/// - `file_owner_client_role`: 文件归属的客户端角色。
///
/// # 返回值
/// - 成功插入的 `transfer_item` Model 实体。
async fn insert_item_with_optional_file_ref(
    db_conn: &sea_orm::DatabaseConnection,
    job_id: i64,
    msg: &tdlib_rs::types::Message,
    file_key: String,
    file_owner_client_role: &str,
) -> anyhow::Result<db::transfer_item::Model> {
    for _ in 0..FILE_CACHE_DELETING_RETRY_LIMIT {
        let txn = db_conn.begin().await?;

        let insert_result = insert_item_with_optional_file_ref_on_conn(
            &txn,
            job_id,
            msg,
            file_key.clone(),
            file_owner_client_role,
        )
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
                if err.to_string().starts_with("file cache is being deleted") {
                    tokio::time::sleep(std::time::Duration::from_millis(
                        FILE_CACHE_DELETING_RETRY_DELAY_MS,
                    ))
                    .await;
                    continue;
                }
                return Err(err);
            }
        }
    }

    anyhow::bail!("file cache is being deleted, file_key={file_key}")
}

/// 在已有事务内创建子项，并按媒体/文本决定是否增加 file_cache 引用。
///
/// # 参数
/// - `conn`: 数据库连接或事务上下文。
/// - `job_id`: 任务 ID。
/// - `msg`: TDLib 消息。
/// - `file_key`: 文件键。
/// - `file_owner_client_role`: 客户端角色。
async fn insert_item_with_optional_file_ref_on_conn<C>(
    conn: &C,
    job_id: i64,
    msg: &tdlib_rs::types::Message,
    file_key: String,
    file_owner_client_role: &str,
) -> anyhow::Result<db::transfer_item::Model>
where
    C: ConnectionTrait,
{
    let is_text = is_text_file_key(&file_key);
    if !is_text && !try_acquire_file_ref_on_conn(conn, file_owner_client_role, &file_key).await? {
        anyhow::bail!("file cache is being deleted, file_key={file_key}");
    }

    let now = now_utc8();
    db::transfer_item::ActiveModel {
        job_id: sea_orm::ActiveValue::Set(job_id),
        source_chat_id: sea_orm::ActiveValue::Set(msg.chat_id),
        source_message_id: sea_orm::ActiveValue::Set(msg.id),
        file_key: sea_orm::ActiveValue::Set(file_key),
        file_owner_client_role: sea_orm::ActiveValue::Set(file_owner_client_role.to_owned()),
        status: sea_orm::ActiveValue::Set(ITEM_STATUS_PENDING.to_owned()),
        retry_count: sea_orm::ActiveValue::Set(0),
        error_message: sea_orm::ActiveValue::Set(None),
        file_ref_released: sea_orm::ActiveValue::Set(is_text),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        ..Default::default()
    }
    .insert(conn)
    .await
    .map_err(Into::into)
}

/// 对齐已经存在的子项。
///
/// file_key 不变时保留原状态；file_key 变化或旧引用已经释放时，重新引用当前文件并重置为 pending。
///
/// # 参数
/// - `conn`: 数据库连接或事务上下文。
/// - `old`: 原数据库子项记录。
/// - `new_file_key`: 重新抓取确定的新文件键。
/// - `new_file_owner_client_role`: 重新抓取确定的客户端角色。
/// - `delay_minutes`: 旧引用释放时的延迟删除分钟数。
async fn reconcile_existing_item_on_conn<C>(
    conn: &C,
    old: db::transfer_item::Model,
    new_file_key: String,
    new_file_owner_client_role: &str,
    delay_minutes: i64,
) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    let new_is_text = is_text_file_key(&new_file_key);
    let owner_changed = old.file_owner_client_role != new_file_owner_client_role;
    let needs_new_ref =
        !new_is_text && (old.file_key != new_file_key || owner_changed || old.file_ref_released);
    if needs_new_ref
        && !try_acquire_file_ref_on_conn(conn, new_file_owner_client_role, &new_file_key).await?
    {
        anyhow::bail!("file cache is being deleted, file_key={new_file_key}");
    }

    if (old.file_key != new_file_key || owner_changed)
        && !old.file_ref_released
        && !is_text_file_key(&old.file_key)
    {
        release_one_file_ref_on_conn(
            conn,
            old.file_owner_client_role.clone(),
            old.file_key.clone(),
            delay_minutes,
        )
        .await?;
    }

    let status =
        if old.file_key != new_file_key || owner_changed || old.status == ITEM_STATUS_OBSOLETE {
            ITEM_STATUS_PENDING.to_owned()
        } else {
            old.status
        };
    let file_ref_released = new_is_text;
    db::transfer_item::Entity::update_many()
        .col_expr(
            db::transfer_item::Column::FileKey,
            Expr::value(new_file_key),
        )
        .col_expr(
            db::transfer_item::Column::FileOwnerClientRole,
            Expr::value(new_file_owner_client_role.to_owned()),
        )
        .col_expr(db::transfer_item::Column::Status, Expr::value(status))
        .col_expr(
            db::transfer_item::Column::ErrorMessage,
            Expr::value(Option::<String>::None),
        )
        .col_expr(
            db::transfer_item::Column::FileRefReleased,
            Expr::value(file_ref_released),
        )
        .col_expr(
            db::transfer_item::Column::UpdatedAt,
            Expr::value(now_utc8()),
        )
        .filter(db::transfer_item::Column::Id.eq(old.id))
        .exec(conn)
        .await?;
    Ok(())
}

/// 将新 bundle 中已经不存在的旧子项标记为 obsolete，并释放其持有的文件引用。
///
/// # 参数
/// - `conn`: 数据库连接或事务上下文。
/// - `old`: 待标记废弃的旧子项模型。
/// - `delay_minutes`: 文件引用释放后的延迟清理时间（分钟）。
async fn mark_item_obsolete_on_conn<C>(
    conn: &C,
    old: db::transfer_item::Model,
    delay_minutes: i64,
) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    if !old.file_ref_released && !is_text_file_key(&old.file_key) {
        release_one_file_ref_on_conn(
            conn,
            old.file_owner_client_role.clone(),
            old.file_key.clone(),
            delay_minutes,
        )
        .await?;
    }

    db::transfer_item::Entity::update_many()
        .col_expr(
            db::transfer_item::Column::Status,
            Expr::value(ITEM_STATUS_OBSOLETE),
        )
        .col_expr(
            db::transfer_item::Column::ErrorMessage,
            Expr::value(Some("source message missing after recovery".to_owned())),
        )
        .col_expr(
            db::transfer_item::Column::FileRefReleased,
            Expr::value(true),
        )
        .col_expr(
            db::transfer_item::Column::UpdatedAt,
            Expr::value(now_utc8()),
        )
        .filter(db::transfer_item::Column::Id.eq(old.id))
        .exec(conn)
        .await?;
    Ok(())
}

/// 更新主任务的源消息快照和当前条目数，保证恢复后的展示与本次 spider 结果一致。
///
/// # 参数
/// - `conn`: 数据库连接或事务上下文。
/// - `job_id`: 主任务 ID。
/// - `bundle`: 最新的抓取消息包。
async fn update_job_source_snapshot_on_conn<C>(
    conn: &C,
    job_id: i64,
    bundle: &TransferBundle,
) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::SourceChatId,
            Expr::value(bundle.source_chat_id),
        )
        .col_expr(
            db::transfer_job::Column::SourceMessageId,
            Expr::value(bundle.source_message_id),
        )
        .col_expr(
            db::transfer_job::Column::SourceAlbumId,
            Expr::value(bundle.source_album_id),
        )
        .col_expr(
            db::transfer_job::Column::SourceClientRole,
            Expr::value(client_role_as_str(bundle.source_client_role)),
        )
        .col_expr(
            db::transfer_job::Column::TotalItems,
            Expr::value(bundle.messages.len() as i32),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now_utc8()))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .exec(conn)
        .await?;
    Ok(())
}

/// 释放单个 file_key 的一次引用。
///
/// # 参数
/// - `conn`: 数据库连接或事务句柄。
/// - `owner_client_role`: 客户端角色。
/// - `file_key`: 文件键。
/// - `delay_minutes`: 延迟删除分钟数。
async fn release_one_file_ref_on_conn<C>(
    conn: &C,
    owner_client_role: String,
    file_key: String,
    delay_minutes: i64,
) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    let mut refs = HashMap::new();
    refs.insert((owner_client_role, file_key), 1);
    release_file_ref_counts_on_conn(conn, refs, delay_minutes).await
}

/// 获取消息当前对应的 file_key；纯文本消息使用稳定文本占位键。
///
/// # 参数
/// - `msg`: TDLib 消息引用。
///
/// # 返回值
/// - 提取的文件唯一键或形如 `text:{chat_id}:{msg_id}` 的文本占位键。
fn file_key_for_message(msg: &tdlib_rs::types::Message) -> String {
    file::extract_file_key(msg).unwrap_or_else(|| format!("text:{}:{}", msg.chat_id, msg.id))
}

/// 按任务内源消息定位子项。
///
/// # 参数
/// - `conn`: 数据库连接或事务上下文。
/// - `job_id`: 主任务 ID。
/// - `source_chat_id`: 源会话 ID。
/// - `source_message_id`: 源消息 ID。
///
/// # 返回值
/// - 查找到的对应子项（若存在）。
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
///
/// # 参数
/// - `job_id`: 主任务 ID。
///
/// # 返回值
/// - 按子项主键升序排列的 `transfer_item` 列表。
pub(in crate::tgbot::transfer) async fn list_items_by_job(
    job_id: i64,
) -> anyhow::Result<Vec<db::transfer_item::Model>> {
    let db_conn = db::get_db().await?;
    list_items_by_job_on_conn(db_conn, job_id).await
}

/// 在指定连接/事务内查询任务子项。
///
/// # 参数
/// - `conn`: 数据库连接或事务上下文。
/// - `job_id`: 主任务 ID。
///
/// # 返回值
/// - 子项列表。
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
///
/// # 参数
/// - `item_id`: 子项主键 ID。
/// - `status`: 新状态（如 "preparing"、"uploading"、"success" 等）。
/// - `error_message`: 可选的错误信息描述。
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
///
/// # 参数
/// - `conn`: 数据库连接或事务上下文。
/// - `item_id`: 子项主键 ID。
/// - `status`: 新状态。
/// - `error_message`: 可选的错误描述。
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
