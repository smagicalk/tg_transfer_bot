// 运行参数数据库持久化测试。

use super::super::{
    ensure_transfer_runtime_config, load_transfer_runtime_config, save_transfer_runtime_config,
};
use super::fixtures::{prepare_test_schema, rebuild_empty_test_schema};
use crate::config::TransferConfig;
use crate::db;

#[tokio::test]
async fn test_ensure_transfer_runtime_config_seeds_default_row_once() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    rebuild_empty_test_schema().await?;

    let default_config = TransferConfig {
        job_concurrency: 3,
        file_delete_delay_minutes: 5,
        file_gc_interval_seconds: 90,
        progress_edit_interval_seconds: 4,
        downloads_default_page_size: 9,
        menu_input_timeout_seconds: 700,
    };

    let seeded = ensure_transfer_runtime_config(&default_config).await?;
    assert_eq!(seeded.job_concurrency, 3);

    let stored = load_transfer_runtime_config()
        .await?
        .expect("runtime config row");
    assert_eq!(stored.job_concurrency, 3);
    assert_eq!(stored.file_delete_delay_minutes, 5);
    Ok(())
}

#[tokio::test]
async fn test_save_transfer_runtime_config_overwrites_existing_row() -> anyhow::Result<()> {
    let _guard = db::TEST_DB_LOCK.lock().await;
    prepare_test_schema().await?;

    let first = TransferConfig {
        job_concurrency: 2,
        file_delete_delay_minutes: 2,
        file_gc_interval_seconds: 60,
        progress_edit_interval_seconds: 2,
        downloads_default_page_size: 8,
        menu_input_timeout_seconds: 600,
    };
    ensure_transfer_runtime_config(&first).await?;

    let second = TransferConfig {
        job_concurrency: 6,
        file_delete_delay_minutes: 12,
        file_gc_interval_seconds: 120,
        progress_edit_interval_seconds: 5,
        downloads_default_page_size: 15,
        menu_input_timeout_seconds: 900,
    };
    save_transfer_runtime_config(&second).await?;

    let stored = load_transfer_runtime_config()
        .await?
        .expect("runtime config row");
    let roundtrip = TransferConfig::from_db_model(&stored)?;
    assert_eq!(roundtrip.job_concurrency, 6);
    assert_eq!(roundtrip.file_delete_delay_minutes, 12);
    assert_eq!(roundtrip.file_gc_interval_seconds, 120);
    assert_eq!(roundtrip.progress_edit_interval_seconds, 5);
    assert_eq!(roundtrip.downloads_default_page_size, 15);
    assert_eq!(roundtrip.menu_input_timeout_seconds, 900);
    Ok(())
}
