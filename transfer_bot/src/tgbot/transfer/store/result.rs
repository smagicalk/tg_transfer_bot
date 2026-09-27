// 上传结果入口读写。
// 一次转存可能因为 Telegram album 10 条限制产生多个结果入口。

use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;

use crate::db;

use super::now_utc8;

/// 上传完成后可持久化的单个结果入口。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::tgbot::transfer) struct ResultMessageRecord {
    /// 分组序号，从 0 开始。
    pub result_index: i32,
    /// 目标 chat_id。
    pub target_chat_id: i64,
    /// 入口消息 ID；album 保存该分组首条消息。
    pub message_id: i64,
    /// 入口链接或定位信息。
    pub message_link: String,
    /// 是否是 album 入口。
    pub is_album: bool,
    /// 该分组包含的源条目数。
    pub item_count: i32,
}

/// 查询任务的所有结果入口。
///
/// # 参数
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - 该任务关联的全部结果消息记录列表（按分组序号升序排列）。
pub(in crate::tgbot::transfer) async fn list_result_messages_by_job(
    job_id: i64,
) -> anyhow::Result<Vec<ResultMessageRecord>> {
    let db_conn = db::get_db().await?;
    list_result_messages_by_job_on_conn(db_conn, job_id).await
}

/// 在指定事务/连接内原子重建任务结果入口。
///
/// 上传成功后调用；先全量删除原旧结果再批量写入新结果，避免恢复/重试路径留下过期的悬挂入口。
///
/// # 参数
/// - `conn`: 数据库连接或事务上下文。
/// - `job_id`: 任务主键 ID。
/// - `records`: 待持久化的新结果消息记录切片。
pub(in crate::tgbot::transfer) async fn replace_result_messages_on_conn<C>(
    conn: &C,
    job_id: i64,
    records: &[ResultMessageRecord],
) -> anyhow::Result<()>
where
    C: ConnectionTrait,
{
    db::transfer_result_message::Entity::delete_many()
        .filter(db::transfer_result_message::Column::JobId.eq(job_id))
        .exec(conn)
        .await?;

    let now = now_utc8();
    for record in records {
        db::transfer_result_message::ActiveModel {
            job_id: sea_orm::ActiveValue::Set(job_id),
            result_index: sea_orm::ActiveValue::Set(record.result_index),
            target_chat_id: sea_orm::ActiveValue::Set(record.target_chat_id),
            message_id: sea_orm::ActiveValue::Set(record.message_id),
            message_link: sea_orm::ActiveValue::Set(record.message_link.clone()),
            is_album: sea_orm::ActiveValue::Set(record.is_album),
            item_count: sea_orm::ActiveValue::Set(record.item_count),
            created_at: sea_orm::ActiveValue::Set(now),
            updated_at: sea_orm::ActiveValue::Set(now),
            ..Default::default()
        }
        .insert(conn)
        .await?;
    }
    Ok(())
}

/// 在指定事务/连接内查询任务的结果入口。
///
/// # 参数
/// - `conn`: 数据库连接或事务上下文。
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - 查出的结果消息记录列表。
async fn list_result_messages_by_job_on_conn<C>(
    conn: &C,
    job_id: i64,
) -> anyhow::Result<Vec<ResultMessageRecord>>
where
    C: ConnectionTrait,
{
    let records = db::transfer_result_message::Entity::find()
        .filter(db::transfer_result_message::Column::JobId.eq(job_id))
        .order_by_asc(db::transfer_result_message::Column::ResultIndex)
        .all(conn)
        .await?
        .into_iter()
        .map(|model| ResultMessageRecord {
            result_index: model.result_index,
            target_chat_id: model.target_chat_id,
            message_id: model.message_id,
            message_link: model.message_link,
            is_album: model.is_album,
            item_count: model.item_count,
        })
        .collect();
    Ok(records)
}

/// 更新单条具体结果入口的链接。
///
/// 当旧链接失效或不可点击时，lookup 或重复转存解析出新的可用 URL 后，同步写回
/// `transfer_result_message` 表，确保多相册/多卡片跳转按钮都能使用最新有效链接。
///
/// # 参数
/// - `job_id`: 任务主键 ID。
/// - `message_id`: 目标消息 ID。
/// - `message_link`: 新的结果消息链接。
pub(in crate::tgbot::transfer) async fn update_result_message_record_link(
    job_id: i64,
    message_id: i64,
    message_link: String,
) -> anyhow::Result<()> {
    let db_conn = db::get_db().await?;
    db::transfer_result_message::Entity::update_many()
        .set(db::transfer_result_message::ActiveModel {
            message_link: sea_orm::ActiveValue::Set(message_link),
            updated_at: sea_orm::ActiveValue::Set(now_utc8()),
            ..Default::default()
        })
        .filter(db::transfer_result_message::Column::JobId.eq(job_id))
        .filter(db::transfer_result_message::Column::MessageId.eq(message_id))
        .exec(db_conn)
        .await?;
    Ok(())
}
