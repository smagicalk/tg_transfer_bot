// workflow 状态机和上传辅助逻辑测试。
// 测试放在独立文件，避免核心流程文件继续膨胀。

use super::super::file::UploadKind;
use super::super::store;
use super::TransferOutcome;
use super::control::apply_job_control;
use super::guard::{acquire_job_guard, acquire_source_target_create_guard};
use super::upload::{fallback_result_message_link, validate_album_kinds};
use crate::db;
use migration::MigratorTrait;
use rand::RngExt;
use rand::distr::SampleString;
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::time::Duration;

/// 测试前确保表结构存在。
async fn prepare_test_schema() -> anyhow::Result<&'static sea_orm::DatabaseConnection> {
    let db_conn = db::get_db().await?;
    migration::Migrator::up(db_conn, None).await?;
    Ok(db_conn)
}

/// 构造一个指定状态的 transfer_job。
async fn insert_job(status: &str) -> anyhow::Result<db::transfer_job::Model> {
    let db_conn = prepare_test_schema().await?;
    let now = store::now_utc8();
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

/// 为任务插入一个媒体子项和对应 file_cache 引用。
async fn insert_item_with_file_ref(job_id: i64) -> anyhow::Result<(i64, String)> {
    let db_conn = prepare_test_schema().await?;
    let now = store::now_utc8();
    let file_key = format!(
        "fk_{}",
        rand::distr::Alphanumeric.sample_string(&mut rand::rng(), 24)
    );

    db::file_cache::ActiveModel {
        file_key: sea_orm::ActiveValue::Set(file_key.clone()),
        status: sea_orm::ActiveValue::Set("ready".to_owned()),
        size_bytes: sea_orm::ActiveValue::Set(Some(2048)),
        td_file_id: sea_orm::ActiveValue::Set(Some(200)),
        local_path: sea_orm::ActiveValue::Set(Some("tmp/workflow-test.bin".to_owned())),
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
        status: sea_orm::ActiveValue::Set(store::JOB_STATUS_PENDING.to_owned()),
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

// 文档与图片混合时应拒绝 album。
#[test]
fn test_validate_album_kinds_for_document_mix() {
    let rs = validate_album_kinds(&[UploadKind::Document, UploadKind::Photo]);
    assert!(rs.is_err());
}

// 图片+视频允许混合 album。
#[test]
fn test_validate_album_kinds_for_photo_video_mix() {
    let rs = validate_album_kinds(&[UploadKind::Photo, UploadKind::Video]);
    assert!(rs.is_ok());
}

// 语音消息不能放进 album；单条语音会走 send_message。
#[test]
fn test_validate_album_kinds_rejects_voice_in_album() {
    let rs = validate_album_kinds(&[UploadKind::Voice, UploadKind::Voice]);
    assert!(rs.is_err());
}

// 私有超级群/频道使用 t.me/c 兜底链接，避免 get_message_link 失败后丢失结果入口。
#[test]
fn test_fallback_result_message_link_for_channel_chat() {
    let link = fallback_result_message_link(-1001234567890, 88);
    assert_eq!(link, "https://t.me/c/1234567890/88");
}

// 普通 chat 无法构造 t.me/c 时，退回 Telegram 客户端 deeplink。
#[test]
fn test_fallback_result_message_link_for_regular_chat() {
    let link = fallback_result_message_link(12345, 88);
    assert_eq!(link, "tg://openmessage?chat_id=12345&message_id=88");
}

// 同 source_link + target_chat_id 的创建锁应阻止并发穿透查重窗口。
#[tokio::test]
async fn test_source_target_create_guard_is_exclusive() {
    let first = acquire_source_target_create_guard("https://t.me/c/1/2".to_owned(), 100).await;

    let waiting = tokio::spawn(async {
        acquire_source_target_create_guard("https://t.me/c/1/2".to_owned(), 100).await
    });
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(!waiting.is_finished());

    drop(first);
    let second = tokio::time::timeout(Duration::from_secs(1), waiting)
        .await
        .expect("guard should be released")
        .expect("join should succeed");
    drop(second);
}

// job 运行锁应在同一进程内阻止同一任务被重复执行。
#[tokio::test]
async fn test_job_guard_is_exclusive() {
    let job_id = rand::rng().random_range(1_000_000..=2_000_000);
    let first = acquire_job_guard(job_id)
        .await
        .expect("first guard should be acquired");
    assert!(acquire_job_guard(job_id).await.is_none());

    drop(first);
    let second = tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            if let Some(guard) = acquire_job_guard(job_id).await {
                return guard;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("guard should be released");
    drop(second);
}

// workflow 控制检查遇到 paused 时应立即返回暂停结果，不继续执行后续流程。
#[tokio::test]
async fn test_apply_job_control_paused() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let job = insert_job(store::JOB_STATUS_PAUSED).await?;

    let outcome = apply_job_control(job.id)
        .await?
        .expect("paused job should stop workflow");
    match outcome {
        TransferOutcome::Paused { job_id } => assert_eq!(job_id, job.id),
        other => panic!("unexpected outcome: {:?}", other),
    }
    Ok(())
}

// workflow 控制检查遇到 cancelling 时应收敛任务，并释放文件引用。
#[tokio::test]
async fn test_apply_job_control_cancelling() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(store::JOB_STATUS_CANCELLING).await?;
    let (item_id, file_key) = insert_item_with_file_ref(job.id).await?;

    let outcome = apply_job_control(job.id)
        .await?
        .expect("cancelling job should stop workflow");
    match outcome {
        TransferOutcome::Cancelled { job_id } => assert_eq!(job_id, job.id),
        other => panic!("unexpected outcome: {:?}", other),
    }

    let status = store::get_job_status(job.id)
        .await?
        .expect("job status must exist");
    assert_eq!(status, store::JOB_STATUS_CANCELLED);

    let item = db::transfer_item::Entity::find_by_id(item_id)
        .one(db_conn)
        .await?
        .expect("item must exist");
    assert_eq!(item.status, "cancelled");

    let cache = db::file_cache::Entity::find_by_id(file_key)
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 0);
    assert!(cache.delete_after.is_some());
    Ok(())
}
