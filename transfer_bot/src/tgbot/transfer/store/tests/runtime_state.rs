// targets 运行时持久化测试。

use super::super::{
    ensure_targets_runtime_config, load_targets_runtime_config, save_targets_runtime_config,
};
use super::fixtures::rebuild_empty_test_schema;
use crate::config::TargetsConfig;
use crate::db;

#[tokio::test]
async fn test_ensure_targets_runtime_config_seeds_default_once() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    rebuild_empty_test_schema().await?;

    let default_config = TargetsConfig {
        default_chat_id: -100,
        aliases: std::collections::HashMap::from([("archive".to_owned(), -300)]),
    };

    let seeded = ensure_targets_runtime_config(&default_config).await?;
    assert_eq!(seeded.default_chat_id, -100);

    let stored = load_targets_runtime_config()
        .await?
        .expect("targets config row");
    assert_eq!(stored.default_chat_id, -100);
    assert_eq!(stored.aliases.get("archive"), Some(&-300));
    Ok(())
}

#[tokio::test]
async fn test_save_targets_runtime_config_overwrites_existing_rows() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    rebuild_empty_test_schema().await?;

    ensure_targets_runtime_config(&TargetsConfig {
        default_chat_id: -100,
        aliases: std::collections::HashMap::from([("archive".to_owned(), -300)]),
    })
    .await?;

    save_targets_runtime_config(&TargetsConfig {
        default_chat_id: -400,
        aliases: std::collections::HashMap::from([("backup".to_owned(), -600)]),
    })
    .await?;

    let stored = load_targets_runtime_config()
        .await?
        .expect("targets config row");
    assert_eq!(stored.default_chat_id, -400);
    assert_eq!(stored.aliases.len(), 1);
    assert_eq!(stored.aliases.get("backup"), Some(&-600));
    Ok(())
}
