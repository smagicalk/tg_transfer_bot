// 主任务完成、上传完成和终态保护相关测试。

use super::super::*;
use super::fixtures::*;
use crate::db;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;

/// 主任务完成和文件引用释放必须在同一事务提交。
#[tokio::test]
async fn test_finish_job_releases_file_refs_in_same_flow() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_RUNNING).await?;
    let (_, file_key) = insert_item_with_file_ref(job.id).await?;

    let finished = finish_job(
        job.clone(),
        1,
        0,
        None,
        Some(700),
        Some("https://t.me/c/1/700".to_owned()),
        2,
    )
    .await?;

    assert!(finished);
    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_SUCCESS);
    assert!(job.finished_at.is_some());

    let cache = db::file_cache::Entity::find_by_id(user_cache_id(file_key))
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 0);
    assert!(cache.delete_after.is_some());
    Ok(())
}

/// 全部失败时应退回普通用户已扣积分，并把计费状态标记为 refunded。
#[tokio::test]
async fn test_finish_failed_job_refunds_charged_points() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_charged_job(JOB_STATUS_RUNNING, 5).await?;

    let finished = finish_job(
        job.clone(),
        0,
        1,
        Some("upload failed".to_owned()),
        None,
        None,
        2,
    )
    .await?;

    assert!(finished);
    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_FAILED);
    assert_eq!(job.billing_status, "refunded");

    let account = get_user_account(job.owner_user_id)
        .await?
        .expect("account should exist");
    assert_eq!(account.points_balance, 5);
    assert_eq!(account.total_points_spent, 0);
    Ok(())
}

/// 部分成功已经产生目标消息，只按失败条目占比退回部分积分。
#[tokio::test]
async fn test_finish_partial_job_refunds_failed_item_ratio() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let mut job = insert_charged_job(JOB_STATUS_RUNNING, 5).await?;
    let mut active: db::transfer_job::ActiveModel = job.clone().into();
    active.total_items = sea_orm::ActiveValue::Set(2);
    job = active.update(db_conn).await?;

    let finished = finish_job(
        job.clone(),
        1,
        1,
        Some("one item failed".to_owned()),
        Some(710),
        Some("https://t.me/c/1/710".to_owned()),
        2,
    )
    .await?;

    assert!(finished);
    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_PARTIAL);
    assert_eq!(job.billing_status, "refunded");

    let account = get_user_account(job.owner_user_id)
        .await?
        .expect("account should exist");
    assert_eq!(account.points_balance, 2);
    assert_eq!(account.total_points_spent, 3);
    Ok(())
}

/// 上传成功路径应同时写子项成功、主任务成功和文件引用释放。
#[tokio::test]
async fn test_finish_uploaded_job_with_item_statuses_is_consistent() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_RUNNING).await?;
    let (item_id, file_key) = insert_item_with_file_ref(job.id).await?;
    let result_link = "https://t.me/c/1/701".to_owned();

    let finished = finish_uploaded_job_with_item_statuses(
        job.clone(),
        FinishJobSummary {
            ok_count: 1,
            fail_count: 0,
            last_error: None,
            result_message_id: Some(701),
            result_message_link: Some(result_link.clone()),
            result_messages: vec![ResultMessageRecord {
                result_index: 0,
                target_chat_id: job.target_chat_id,
                message_id: 701,
                message_link: result_link.clone(),
                is_album: true,
                item_count: 1,
            }],
            delay_minutes: 2,
        },
        vec![(item_id, "success".to_owned(), None)],
    )
    .await?;

    assert!(finished);
    let item = db::transfer_item::Entity::find_by_id(item_id)
        .one(db_conn)
        .await?
        .expect("item must exist");
    assert_eq!(item.status, "success");

    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_SUCCESS);
    assert_eq!(job.result_message_link, Some(result_link));

    let result_rows = list_result_messages_by_job(job.id).await?;
    assert_eq!(result_rows.len(), 1);
    assert_eq!(result_rows[0].message_id, 701);
    assert_eq!(result_rows[0].message_link, "https://t.me/c/1/701");

    let cache = db::file_cache::Entity::find_by_id(user_cache_id(file_key))
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 0);
    assert!(cache.delete_after.is_some());
    Ok(())
}

/// 上传超过 10 条拆成多个 album 时，所有结果入口必须和主任务终态一起写入。
#[tokio::test]
async fn test_finish_uploaded_job_persists_multiple_result_messages() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_RUNNING).await?;
    let (item_id, file_key) = insert_item_with_file_ref(job.id).await?;
    let first_link = "https://t.me/c/1/1001".to_owned();
    let second_link = "https://t.me/c/1/1002".to_owned();

    let finished = finish_uploaded_job_with_item_statuses(
        job.clone(),
        FinishJobSummary {
            ok_count: 11,
            fail_count: 0,
            last_error: None,
            result_message_id: Some(1001),
            result_message_link: Some(first_link.clone()),
            result_messages: vec![
                ResultMessageRecord {
                    result_index: 0,
                    target_chat_id: job.target_chat_id,
                    message_id: 1001,
                    message_link: first_link.clone(),
                    is_album: true,
                    item_count: 9,
                },
                ResultMessageRecord {
                    result_index: 1,
                    target_chat_id: job.target_chat_id,
                    message_id: 1002,
                    message_link: second_link.clone(),
                    is_album: true,
                    item_count: 2,
                },
            ],
            delay_minutes: 2,
        },
        vec![(item_id, "success".to_owned(), None)],
    )
    .await?;

    assert!(finished);
    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_SUCCESS);
    assert_eq!(job.result_message_id, Some(1001));
    assert_eq!(job.result_message_link, Some(first_link));

    let result_rows = list_result_messages_by_job(job.id).await?;
    assert_eq!(result_rows.len(), 2);
    assert_eq!(result_rows[0].message_id, 1001);
    assert_eq!(result_rows[0].item_count, 9);
    assert!(result_rows[0].is_album);
    assert_eq!(result_rows[1].message_id, 1002);
    assert_eq!(result_rows[1].item_count, 2);
    assert!(result_rows[1].is_album);

    let cache = db::file_cache::Entity::find_by_id(user_cache_id(file_key))
        .one(db_conn)
        .await?
        .expect("file cache must exist");
    assert_eq!(cache.active_refs, 0);
    Ok(())
}

/// finish_job 不能覆盖用户已经发出的停止请求。
#[tokio::test]
async fn test_finish_job_does_not_overwrite_cancelling() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_CANCELLING).await?;

    let finished = finish_job(
        job.clone(),
        1,
        0,
        None,
        Some(900),
        Some("https://t.me/c/1/900".to_owned()),
        2,
    )
    .await?;

    assert!(!finished);
    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_CANCELLING);
    assert!(job.result_message_link.is_none());
    Ok(())
}

/// finish_job 也不能覆盖暂停任务，暂停只能由恢复命令重新进入执行队列。
#[tokio::test]
async fn test_finish_job_does_not_overwrite_paused() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_PAUSED).await?;

    let finished = finish_job(
        job.clone(),
        1,
        0,
        None,
        Some(901),
        Some("https://t.me/c/1/901".to_owned()),
        2,
    )
    .await?;

    assert!(!finished);
    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_PAUSED);
    assert!(job.result_message_link.is_none());
    Ok(())
}

/// 上传成功后，即使控制状态刚好变成 paused，也必须保存已经发出的目标消息链接。
#[tokio::test]
async fn test_finish_uploaded_job_can_finalize_paused() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_PAUSED).await?;
    let result_link = "https://t.me/c/1/902".to_owned();

    let finished = finish_uploaded_job(
        job.clone(),
        1,
        0,
        None,
        Some(902),
        Some(result_link.clone()),
        2,
    )
    .await?;

    assert!(finished);
    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_SUCCESS);
    assert_eq!(job.result_message_id, Some(902));
    assert_eq!(job.result_message_link, Some(result_link));
    assert!(job.finished_at.is_some());
    Ok(())
}

/// 上传成功后，如果停止请求刚好到达，也以真实上传结果为准完成任务。
#[tokio::test]
async fn test_finish_uploaded_job_can_finalize_cancelling() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    let db_conn = prepare_test_schema().await?;
    let job = insert_job(JOB_STATUS_CANCELLING).await?;
    let result_link = "https://t.me/c/1/903".to_owned();

    let finished = finish_uploaded_job(
        job.clone(),
        1,
        0,
        None,
        Some(903),
        Some(result_link.clone()),
        2,
    )
    .await?;

    assert!(finished);
    let job = db::transfer_job::Entity::find_by_id(job.id)
        .one(db_conn)
        .await?
        .expect("job must exist");
    assert_eq!(job.status, JOB_STATUS_SUCCESS);
    assert_eq!(job.result_message_id, Some(903));
    assert_eq!(job.result_message_link, Some(result_link));
    assert!(job.finished_at.is_some());
    Ok(())
}
