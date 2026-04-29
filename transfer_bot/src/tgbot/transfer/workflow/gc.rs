// 文件缓存删除队列：
// - 扫描引用计数为 0 且到期的 file_cache
// - 删除本地文件与 TDLib 缓存
// - 删除成功后移除数据库记录

use std::time::Duration;

use crate::tgbot::transfer::store;

/// 文件删除队列后台循环（持续运行）。
pub(in crate::tgbot::transfer) async fn run_file_gc_loop(client_id: i32) {
    let interval = cleanup_interval_seconds();
    loop {
        if let Err(err) = run_file_gc_once(client_id).await {
            tracing::error!("file gc round failed: {:#}", err);
        }
        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}

/// 执行一轮文件删除队列消费。
pub(in crate::tgbot::transfer) async fn run_file_gc_once(client_id: i32) -> anyhow::Result<()> {
    let due_rows = store::list_due_file_cache(store::now_utc8(), 100).await?;
    if due_rows.is_empty() {
        return Ok(());
    }
    tracing::info!(due_count = due_rows.len(), "file gc found due cache rows");

    for row in due_rows {
        // 删除前先原子认领，避免扫描到期记录后又被新任务重新引用。
        let Some(row) =
            store::claim_file_cache_for_delete(&row.file_key, store::now_utc8()).await?
        else {
            continue;
        };

        if let Some(path) = row.local_path.clone()
            && !path.is_empty()
        {
            match tokio::fs::remove_file(path).await {
                Ok(_) => {}
                // 文件已不存在视为成功，继续清理数据库记录。
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
                Err(err) => {
                    store::mark_file_cache_delete_failed(
                        &row.file_key,
                        format!("remove local file failed: {}", err),
                    )
                    .await?;
                    continue;
                }
            }
        }

        // 如果保存了 td_file_id，再尝试通知 TDLib 清理缓存（失败不阻断）。
        if let Some(file_id) = row.td_file_id
            && let Err(err) = tdlib_rs::functions::delete_file(file_id, client_id).await
        {
            tracing::warn!(
                "delete_file from tdlib failed, file_key={}, file_id={}, err={:?}",
                row.file_key,
                file_id,
                err
            );
        }

        store::delete_file_cache(&row.file_key).await?;
        tracing::info!(
            file_key = %row.file_key,
            td_file_id = row.td_file_id,
            "file cache deleted"
        );
    }

    Ok(())
}

/// 删除队列扫描间隔（秒）：
/// 从 config.json 读取 `transfer_config.file_gc_interval_seconds`。
fn cleanup_interval_seconds() -> u64 {
    super::super::runtime_config()
        .file_gc_interval_seconds
        .max(1)
}
