// store 测试夹具。
// 这里集中构造数据库记录和 TDLib 消息，测试用例只关注具体行为断言。

use super::super::*;
use crate::db;
use migration::MigratorTrait;
use rand::RngExt;
use rand::distr::SampleString;
use sea_orm::ActiveModelTrait;

/// 测试前确保表结构存在。
pub(super) async fn prepare_test_schema() -> anyhow::Result<&'static sea_orm::DatabaseConnection> {
    let db_conn = db::get_db().await?;
    migration::Migrator::up(db_conn, None).await?;
    Ok(db_conn)
}

/// 构造一个指定状态的 transfer_job。
pub(super) async fn insert_job(status: &str) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = prepare_test_schema().await?;
    let now = now_utc8();
    db::transfer_job::ActiveModel {
        request_chat_id: sea_orm::ActiveValue::Set(rand::rng().random_range(1..=1000000)),
        request_message_id: sea_orm::ActiveValue::Set(rand::rng().random_range(1..=1000000)),
        source_link: sea_orm::ActiveValue::Set(format!(
            "https://t.me/c/{}/{}",
            rand::rng().random_range(1..=1000000),
            rand::rng().random_range(1..=1000000)
        )),
        source_chat_id: sea_orm::ActiveValue::Set(rand::rng().random_range(1..=1000000)),
        source_message_id: sea_orm::ActiveValue::Set(rand::rng().random_range(1..=1000000)),
        source_album_id: sea_orm::ActiveValue::Set(0),
        target_chat_id: sea_orm::ActiveValue::Set(rand::rng().random_range(1..=1000000)),
        result_message_id: sea_orm::ActiveValue::Set(None),
        result_message_link: sea_orm::ActiveValue::Set(None),
        status: sea_orm::ActiveValue::Set(status.to_owned()),
        total_items: sea_orm::ActiveValue::Set(1),
        done_items: sea_orm::ActiveValue::Set(0),
        failed_items: sea_orm::ActiveValue::Set(0),
        retry_count: sea_orm::ActiveValue::Set(0),
        last_error: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        finished_at: sea_orm::ActiveValue::Set(None),
        ..Default::default()
    }
    .insert(db_conn)
    .await
    .map_err(Into::into)
}

/// 为任务插入一个真实媒体子项和对应 file_cache 引用。
pub(super) async fn insert_item_with_file_ref(job_id: i64) -> anyhow::Result<(i64, String)> {
    let db_conn = prepare_test_schema().await?;
    let now = now_utc8();
    let file_key = format!(
        "fk_{}",
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 24)
    );

    db::file_cache::ActiveModel {
        file_key: sea_orm::ActiveValue::Set(file_key.clone()),
        status: sea_orm::ActiveValue::Set("ready".to_owned()),
        size_bytes: sea_orm::ActiveValue::Set(Some(1024)),
        td_file_id: sea_orm::ActiveValue::Set(Some(100)),
        local_path: sea_orm::ActiveValue::Set(Some("tmp/test.bin".to_owned())),
        last_error: sea_orm::ActiveValue::Set(None),
        active_refs: sea_orm::ActiveValue::Set(1),
        last_ref_zero_at: sea_orm::ActiveValue::Set(None),
        delete_after: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        last_used_at: sea_orm::ActiveValue::Set(now),
    }
    .insert(db_conn)
    .await?;

    let item = db::transfer_item::ActiveModel {
        job_id: sea_orm::ActiveValue::Set(job_id),
        source_chat_id: sea_orm::ActiveValue::Set(rand::rng().random_range(1..=1000000)),
        source_message_id: sea_orm::ActiveValue::Set(rand::rng().random_range(1..=1000000)),
        file_key: sea_orm::ActiveValue::Set(file_key.clone()),
        status: sea_orm::ActiveValue::Set(JOB_STATUS_PENDING.to_owned()),
        retry_count: sea_orm::ActiveValue::Set(0),
        error_message: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        ..Default::default()
    }
    .insert(db_conn)
    .await?;

    Ok((item.id, file_key))
}

/// 插入一个引用指定 file_key 的子项，便于测试多个任务共享同一缓存文件。
pub(super) async fn insert_item_for_file_key(job_id: i64, file_key: &str) -> anyhow::Result<i64> {
    let db_conn = prepare_test_schema().await?;
    let now = now_utc8();
    let item = db::transfer_item::ActiveModel {
        job_id: sea_orm::ActiveValue::Set(job_id),
        source_chat_id: sea_orm::ActiveValue::Set(rand::rng().random_range(1..=1000000)),
        source_message_id: sea_orm::ActiveValue::Set(rand::rng().random_range(1..=1000000)),
        file_key: sea_orm::ActiveValue::Set(file_key.to_owned()),
        status: sea_orm::ActiveValue::Set(JOB_STATUS_PENDING.to_owned()),
        retry_count: sea_orm::ActiveValue::Set(0),
        error_message: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
        ..Default::default()
    }
    .insert(db_conn)
    .await?;
    Ok(item.id)
}

/// 构造带文档文件的 TDLib 消息，用于测试 ensure_items_for_bundle 的引用计数。
pub(super) fn message_with_document(
    source_chat_id: i64,
    source_message_id: i64,
    file_key: &str,
) -> tdlib_rs::types::Message {
    tdlib_rs::types::Message {
        id: source_message_id,
        sender_id: tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
            user_id: 1,
        }),
        chat_id: source_chat_id,
        sending_state: None,
        scheduling_state: None,
        is_outgoing: false,
        is_pinned: false,
        is_from_offline: false,
        can_be_saved: true,
        has_timestamped_media: false,
        is_channel_post: false,
        is_paid_star_suggested_post: false,
        is_paid_ton_suggested_post: false,
        contains_unread_mention: false,
        date: 0,
        edit_date: 0,
        forward_info: None,
        import_info: None,
        interaction_info: None,
        unread_reactions: vec![],
        fact_check: None,
        suggested_post_info: None,
        reply_to: None,
        topic_id: None,
        self_destruct_type: None,
        self_destruct_in: 0.0,
        auto_delete_in: 0.0,
        via_bot_user_id: 0,
        sender_business_bot_user_id: 0,
        sender_boost_count: 0,
        sender_tag: String::new(),
        paid_message_star_count: 0,
        author_signature: String::new(),
        media_album_id: 0,
        effect_id: 0,
        restriction_info: None,
        summary_language_code: String::new(),
        content: tdlib_rs::enums::MessageContent::MessageDocument(
            tdlib_rs::types::MessageDocument {
                document: tdlib_rs::types::Document {
                    file_name: "test.bin".to_owned(),
                    mime_type: "application/octet-stream".to_owned(),
                    minithumbnail: None,
                    thumbnail: None,
                    document: tdlib_rs::types::File {
                        id: 77,
                        size: 4096,
                        expected_size: 4096,
                        remote: tdlib_rs::types::RemoteFile {
                            unique_id: file_key.to_owned(),
                            id: format!("remote_{}", file_key),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                caption: tdlib_rs::types::FormattedText::default(),
            },
        ),
        reply_markup: None,
    }
}
