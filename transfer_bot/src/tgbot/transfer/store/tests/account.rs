// 用户账号与积分账本测试。
// 这里覆盖余额初始化、扣费幂等和余额不足，避免计费逻辑只靠命令层测试保护。

use super::super::*;
use super::fixtures::*;
use crate::config::ActorRole;
use crate::db;
use crate::tgbot::transfer::store::account::partial_refund_points;
use rand::RngExt;

/// 普通用户账号首次创建时会拿到配置的初始积分；重复 ensure 不应重复发放。
#[tokio::test]
async fn test_ensure_user_account_initial_points_are_one_time() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;

    let user_id = test_user_id();
    let first = ensure_user_account(user_id, ActorRole::User, 10).await?;
    let second = ensure_user_account(user_id, ActorRole::User, 99).await?;

    assert_eq!(first.points_balance, 10);
    assert_eq!(second.points_balance, 10);
    assert_eq!(second.total_points_added, 10);
    Ok(())
}

/// 管理员账号不需要积分消费，初始化时也不应被发放普通用户初始积分。
#[tokio::test]
async fn test_admin_account_does_not_receive_initial_points() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;

    let user_id = test_user_id();
    let account = ensure_user_account(user_id, ActorRole::Admin, 99).await?;

    assert_eq!(account.role, "admin");
    assert_eq!(account.points_balance, 0);
    assert_eq!(account.total_points_added, 0);
    Ok(())
}

/// 带 idempotency_key 的扣费重复执行时只扣一次，防止同一条请求因重试被重复收费。
#[tokio::test]
async fn test_change_points_idempotency_prevents_double_charge() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;
    let user_id = test_user_id();
    ensure_user_account(user_id, ActorRole::User, 10).await?;

    let first = change_points(PointsChange {
        telegram_user_id: user_id,
        delta: -3,
        reason: "transfer_charge".to_owned(),
        job_id: None,
        request_chat_id: Some(user_id),
        request_message_id: Some(1),
        idempotency_key: Some(format!("request:{}:1", user_id)),
        created_by: Some(user_id),
    })
    .await?;
    let second = change_points(PointsChange {
        telegram_user_id: user_id,
        delta: -3,
        reason: "transfer_charge".to_owned(),
        job_id: None,
        request_chat_id: Some(user_id),
        request_message_id: Some(1),
        idempotency_key: Some(format!("request:{}:1", user_id)),
        created_by: Some(user_id),
    })
    .await?;

    assert!(!first.idempotent_replay);
    assert!(second.idempotent_replay);
    assert_eq!(second.account.points_balance, 7);
    assert_eq!(second.account.total_points_spent, 3);
    Ok(())
}

/// 余额不足时拒绝扣费，并保持原余额不变。
#[tokio::test]
async fn test_change_points_rejects_insufficient_balance() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;
    let user_id = test_user_id();
    ensure_user_account(user_id, ActorRole::User, 2).await?;

    let err = change_points(PointsChange {
        telegram_user_id: user_id,
        delta: -3,
        reason: "transfer_charge".to_owned(),
        job_id: None,
        request_chat_id: Some(user_id),
        request_message_id: Some(1),
        idempotency_key: Some(format!("request:{}:1", user_id)),
        created_by: Some(user_id),
    })
    .await
    .unwrap_err();
    let account = get_user_account(user_id)
        .await?
        .expect("account should still exist");

    assert!(err.to_string().contains("insufficient points"));
    assert_eq!(account.points_balance, 2);
    assert_eq!(account.total_points_spent, 0);
    Ok(())
}

/// 积分流水应按最新记录倒序分页，只暴露命令展示需要的轻量字段。
#[tokio::test]
async fn test_list_point_ledger_page_paginates_recent_entries() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;
    let user_id = test_user_id();
    ensure_user_account(user_id, ActorRole::User, 0).await?;

    for amount in [5, 7, 9] {
        change_points(PointsChange {
            telegram_user_id: user_id,
            delta: amount,
            reason: format!("admin_adjust_{amount}"),
            job_id: None,
            request_chat_id: Some(user_id),
            request_message_id: None,
            idempotency_key: None,
            created_by: Some(user_id),
        })
        .await?;
    }

    let page = list_point_ledger_page(user_id, 2, 1).await?;

    assert_eq!(page.telegram_user_id, user_id);
    assert_eq!(page.total, 3);
    assert_eq!(page.total_pages, 2);
    assert_eq!(page.entries.len(), 2);
    assert_eq!(page.entries[0].delta, 9);
    assert_eq!(page.entries[1].delta, 7);
    assert_eq!(page.entries[0].balance_after, 21);
    assert_eq!(page.entries[0].reason, "admin_adjust_9");
    Ok(())
}

/// 生成测试用户 ID，避免本地旧测试库里的账号记录影响重复运行。
fn test_user_id() -> i64 {
    rand::rng().random_range(10_000_000..=99_999_999)
}

/// 部分成功退款按失败条目占比计算；只要存在失败且扣过费，至少退 1 分。
#[test]
fn test_partial_refund_points() {
    assert_eq!(partial_refund_points(10, 5, 2), 4);
    assert_eq!(partial_refund_points(2, 10, 1), 1);
    assert_eq!(partial_refund_points(2, 10, 0), 0);
    assert_eq!(partial_refund_points(2, 0, 1), 0);
    assert_eq!(partial_refund_points(2, 10, 99), 2);
}
