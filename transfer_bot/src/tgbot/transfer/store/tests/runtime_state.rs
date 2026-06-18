// targets / access_control / billing 运行时持久化测试。

use super::super::{
    ensure_access_control_runtime_config, ensure_billing_runtime_config,
    ensure_targets_runtime_config, load_access_control_runtime_config, load_billing_runtime_config,
    load_targets_runtime_config, save_access_control_runtime_config, save_billing_runtime_config,
    save_targets_runtime_config,
};
use super::fixtures::prepare_test_schema;
use crate::config::{AccessControlConfig, BillingConfig, TargetsConfig};
use crate::db;

#[tokio::test]
async fn test_ensure_targets_runtime_config_seeds_default_once() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;

    let default_config = TargetsConfig {
        default_chat_id: -100,
        by_request_chat_id: std::collections::HashMap::from([(1, -200)]),
        aliases: std::collections::HashMap::from([("archive".to_owned(), -300)]),
    };

    let seeded = ensure_targets_runtime_config(&default_config).await?;
    assert_eq!(seeded.default_chat_id, -100);

    let stored = load_targets_runtime_config()
        .await?
        .expect("targets config row");
    assert_eq!(stored.default_chat_id, -100);
    assert_eq!(stored.by_request_chat_id.get(&1), Some(&-200));
    assert_eq!(stored.aliases.get("archive"), Some(&-300));
    Ok(())
}

#[tokio::test]
async fn test_save_targets_runtime_config_overwrites_existing_rows() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;

    ensure_targets_runtime_config(&TargetsConfig {
        default_chat_id: -100,
        by_request_chat_id: std::collections::HashMap::from([(1, -200)]),
        aliases: std::collections::HashMap::from([("archive".to_owned(), -300)]),
    })
    .await?;

    save_targets_runtime_config(&TargetsConfig {
        default_chat_id: -400,
        by_request_chat_id: std::collections::HashMap::from([(2, -500)]),
        aliases: std::collections::HashMap::from([("backup".to_owned(), -600)]),
    })
    .await?;

    let stored = load_targets_runtime_config()
        .await?
        .expect("targets config row");
    assert_eq!(stored.default_chat_id, -400);
    assert_eq!(stored.by_request_chat_id.len(), 1);
    assert_eq!(stored.by_request_chat_id.get(&2), Some(&-500));
    assert_eq!(stored.aliases.len(), 1);
    assert_eq!(stored.aliases.get("backup"), Some(&-600));
    Ok(())
}

#[tokio::test]
async fn test_ensure_access_control_runtime_config_seeds_default_once() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;

    let default_config = AccessControlConfig {
        bootstrap_admin_user_ids: vec![1],
        admin_user_ids: vec![2],
        allowed_user_ids: vec![3],
        allow_all_private_users: true,
        banned_user_ids: vec![4],
        allowed_request_chat_ids: vec![5],
        allowed_target_chat_ids: vec![6],
    };

    let seeded = ensure_access_control_runtime_config(&default_config).await?;
    assert_eq!(seeded.admin_user_ids, vec![2]);

    let stored = load_access_control_runtime_config()
        .await?
        .expect("access control row");
    assert!(stored.bootstrap_admin_user_ids.is_empty());
    assert_eq!(stored.admin_user_ids, vec![2]);
    assert_eq!(stored.allowed_user_ids, vec![3]);
    assert!(stored.allow_all_private_users);
    assert_eq!(stored.banned_user_ids, vec![4]);
    assert_eq!(stored.allowed_request_chat_ids, vec![5]);
    assert_eq!(stored.allowed_target_chat_ids, vec![6]);
    Ok(())
}

#[tokio::test]
async fn test_save_access_control_runtime_config_overwrites_existing_rows() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;

    ensure_access_control_runtime_config(&AccessControlConfig {
        bootstrap_admin_user_ids: vec![1],
        admin_user_ids: vec![2],
        allowed_user_ids: vec![3],
        allow_all_private_users: true,
        banned_user_ids: vec![4],
        allowed_request_chat_ids: vec![5],
        allowed_target_chat_ids: vec![6],
    })
    .await?;

    save_access_control_runtime_config(&AccessControlConfig {
        bootstrap_admin_user_ids: vec![1],
        admin_user_ids: vec![7],
        allowed_user_ids: vec![8],
        allow_all_private_users: false,
        banned_user_ids: vec![9],
        allowed_request_chat_ids: vec![10],
        allowed_target_chat_ids: vec![11],
    })
    .await?;

    let stored = load_access_control_runtime_config()
        .await?
        .expect("access control row");
    assert_eq!(stored.admin_user_ids, vec![7]);
    assert_eq!(stored.allowed_user_ids, vec![8]);
    assert!(!stored.allow_all_private_users);
    assert_eq!(stored.banned_user_ids, vec![9]);
    assert_eq!(stored.allowed_request_chat_ids, vec![10]);
    assert_eq!(stored.allowed_target_chat_ids, vec![11]);
    Ok(())
}

#[tokio::test]
async fn test_ensure_billing_runtime_config_seeds_default_once() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;

    let default_config = BillingConfig {
        enabled: true,
        base_cost_points: 1,
        item_cost_points: 2,
        initial_user_points: 3,
        announcement_text: Some("hello".to_owned()),
    };

    let seeded = ensure_billing_runtime_config(&default_config).await?;
    assert_eq!(seeded.base_cost_points, 1);

    let stored = load_billing_runtime_config()
        .await?
        .expect("billing config row");
    let roundtrip = BillingConfig::from_db_model(&stored);
    assert_eq!(roundtrip.base_cost_points, 1);
    assert_eq!(roundtrip.item_cost_points, 2);
    assert_eq!(roundtrip.initial_user_points, 3);
    assert_eq!(roundtrip.announcement_text.as_deref(), Some("hello"));
    Ok(())
}

#[tokio::test]
async fn test_save_billing_runtime_config_overwrites_existing_row() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;

    ensure_billing_runtime_config(&BillingConfig {
        enabled: true,
        base_cost_points: 1,
        item_cost_points: 2,
        initial_user_points: 3,
        announcement_text: Some("hello".to_owned()),
    })
    .await?;

    save_billing_runtime_config(&BillingConfig {
        enabled: false,
        base_cost_points: 4,
        item_cost_points: 5,
        initial_user_points: 6,
        announcement_text: Some("world".to_owned()),
    })
    .await?;

    let stored = load_billing_runtime_config()
        .await?
        .expect("billing config row");
    let roundtrip = BillingConfig::from_db_model(&stored);
    assert!(!roundtrip.enabled);
    assert_eq!(roundtrip.base_cost_points, 4);
    assert_eq!(roundtrip.item_cost_points, 5);
    assert_eq!(roundtrip.initial_user_points, 6);
    assert_eq!(roundtrip.announcement_text.as_deref(), Some("world"));
    Ok(())
}
