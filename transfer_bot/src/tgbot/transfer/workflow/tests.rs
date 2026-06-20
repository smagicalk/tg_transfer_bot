// workflow 状态机和上传辅助逻辑测试。
// 测试放在独立文件，避免核心流程文件继续膨胀。

use super::super::file::UploadKind;
use super::super::store;
use super::TransferOutcome;
use super::control::apply_job_control;
use super::guard::{acquire_job_guard, acquire_source_target_create_guard};
use super::result_link::{
    build_private_supergroup_message_link, extract_tdlib_message_id_from_stored_link,
    fallback_result_message_locator, tdlib_message_id_to_visible_id,
};
use super::upload::{album_chunk_sizes, validate_album_kinds};
use crate::db;
use rand::RngExt;
use rand::distr::SampleString;
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::time::Duration;

/// 测试前确保表结构存在。
async fn prepare_test_schema() -> anyhow::Result<&'static sea_orm::DatabaseConnection> {
    let db_conn = db::get_db().await?;
    db::ensure_test_schema_current(db_conn).await?;
    Ok(db_conn)
}

/// 测试统一获取运行态上下文。
fn test_app_context() -> std::sync::Arc<crate::app_context::AppContext> {
    crate::app_context::app_context()
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
        source_kind: sea_orm::ActiveValue::Set("link".to_owned()),
        source_client_role: sea_orm::ActiveValue::Set("user".to_owned()),
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
        owner_client_role: sea_orm::ActiveValue::Set("user".to_owned()),
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
        file_owner_client_role: sea_orm::ActiveValue::Set("user".to_owned()),
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

// Telegram 单个 album 最多 10 条；正好 10 条应允许一次发送。
#[test]
fn test_validate_album_kinds_allows_ten_items() {
    let kinds = vec![UploadKind::Photo; 10];
    let rs = validate_album_kinds(&kinds);
    assert!(rs.is_ok());
}

// 超过 10 条会在上传阶段分成多个 album；类型校验本身不应该拒绝。
#[test]
fn test_validate_album_kinds_allows_more_than_ten_items() {
    let kinds = vec![UploadKind::Photo; 11];
    let rs = validate_album_kinds(&kinds);
    assert!(rs.is_ok());
}

// album 分组应避免尾部只剩 1 条，否则 11 条会退化成 10 条 album + 1 条单发。
#[test]
fn test_album_chunk_sizes_avoid_trailing_single_item() {
    assert_eq!(album_chunk_sizes(0), Vec::<usize>::new());
    assert_eq!(album_chunk_sizes(1), vec![1]);
    assert_eq!(album_chunk_sizes(10), vec![10]);
    assert_eq!(album_chunk_sizes(11), vec![9, 2]);
    assert_eq!(album_chunk_sizes(20), vec![10, 10]);
    assert_eq!(album_chunk_sizes(21), vec![10, 9, 2]);
    assert_eq!(album_chunk_sizes(31), vec![10, 10, 9, 2]);
}

// 语音消息不能放进 album；单条语音会走 send_message。
#[test]
fn test_validate_album_kinds_rejects_voice_in_album() {
    let rs = validate_album_kinds(&[UploadKind::Voice, UploadKind::Voice]);
    assert!(rs.is_err());
}

// GIF/animation 不能放进 album；单条 GIF 会走 send_message。
#[test]
fn test_validate_album_kinds_rejects_animation_in_album() {
    let rs = validate_album_kinds(&[UploadKind::Animation, UploadKind::Animation]);
    assert!(rs.is_err());
}

// 非 supergroup/channel 场景无法生成稳定链接时，只保留定位信息。
#[test]
fn test_fallback_result_message_locator() {
    let locator = fallback_result_message_locator(-5106953357, 769654784);
    assert_eq!(locator, "chat_id=-5106953357 message_id=769654784");
}

// TDLib 内部消息 ID 必须换算成 Telegram 链接里的可见消息 ID，否则 t.me/c 会点不开。
#[test]
fn test_tdlib_message_id_to_visible_id() {
    assert_eq!(tdlib_message_id_to_visible_id(769654784), Some(734));
    assert_eq!(tdlib_message_id_to_visible_id(0), None);
}

// 私有 supergroup/channel 兜底链接使用 t.me/c 和换算后的可见消息 ID。
#[test]
fn test_build_private_supergroup_message_link() {
    let link = build_private_supergroup_message_link(1835352976, 769654784);
    assert_eq!(link.as_deref(), Some("https://t.me/c/1835352976/734"));
}

// 历史保存的 tg:// 或定位字符串可以提取 message_id，用于刷新旧结果链接。
#[test]
fn test_extract_tdlib_message_id_from_stored_link() {
    assert_eq!(
        extract_tdlib_message_id_from_stored_link(
            "tg://openmessage?chat_id=-5106953357&message_id=769654784"
        ),
        Some(769654784)
    );
    assert_eq!(
        extract_tdlib_message_id_from_stored_link("chat_id=-5106953357 message_id=769654784"),
        Some(769654784)
    );
    assert_eq!(
        extract_tdlib_message_id_from_stored_link("https://t.me/c/1/2"),
        None
    );
}

// 同 source_link + target_chat_id 的创建锁应阻止并发穿透查重窗口。
#[tokio::test]
async fn test_source_target_create_guard_is_exclusive() {
    let app_context = test_app_context();
    let first = acquire_source_target_create_guard(
        app_context.as_ref(),
        "https://t.me/c/1/2".to_owned(),
        100,
    )
    .await;

    let waiting = tokio::spawn(async {
        let app_context = test_app_context();
        acquire_source_target_create_guard(
            app_context.as_ref(),
            "https://t.me/c/1/2".to_owned(),
            100,
        )
        .await
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
    let app_context = test_app_context();
    let first = acquire_job_guard(app_context.as_ref(), job_id)
        .await
        .expect("first guard should be acquired");
    assert!(
        acquire_job_guard(app_context.as_ref(), job_id)
            .await
            .is_none()
    );

    drop(first);
    let second = tokio::time::timeout(Duration::from_secs(1), async {
        let app_context = test_app_context();
        loop {
            if let Some(guard) = acquire_job_guard(app_context.as_ref(), job_id).await {
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
    let app_context = test_app_context();

    let outcome = apply_job_control(app_context.as_ref(), job.id)
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
    let app_context = test_app_context();

    let outcome = apply_job_control(app_context.as_ref(), job.id)
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

    let cache = db::file_cache::Entity::find_by_id(("user".to_owned(), file_key))
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 0);
    assert!(cache.delete_after.is_some());
    Ok(())
}
