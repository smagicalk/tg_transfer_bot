// 文件缓存删除队列：
// - 扫描引用计数为 0 且到期的 file_cache
// - 删除本地文件与 TDLib 缓存
// - 删除成功后移除数据库记录

use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use crate::tgbot::transfer::store;

/// 文件删除队列后台循环（持续运行）。
pub(in crate::tgbot::transfer) async fn run_file_gc_loop(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    client_ids: crate::config::TransferClientIds,
) {
    loop {
        if let Err(err) = run_file_gc_once(app_context.clone(), client_ids).await {
            tracing::error!("file gc round failed: {:#}", err);
        }
        // 每轮 sleep 前重新读取运行时配置，保证 `/config set file_gc_interval_seconds`
        // 对已经启动的 GC 循环也能生效。
        let interval = cleanup_interval_seconds(&app_context);
        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}

/// 执行一轮文件删除队列消费。
pub(in crate::tgbot::transfer) async fn run_file_gc_once(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<()> {
    let retry_delay_seconds = cleanup_interval_seconds(&app_context);
    let due_rows = store::list_due_file_cache(store::now_utc8(), 100).await?;
    if due_rows.is_empty() {
        return Ok(());
    }
    tracing::info!(due_count = due_rows.len(), "file gc found due cache rows");

    for row in due_rows {
        // 删除前先原子认领，避免扫描到期记录后又被新任务重新引用。
        let Some(row) = store::claim_file_cache_for_delete(
            &row.owner_client_role,
            &row.file_key,
            store::now_utc8(),
        )
        .await?
        else {
            continue;
        };

        let mut cleanup_confirmed = row.local_path.as_deref().is_none_or(str::is_empty);
        if let Some(path) = row.local_path.as_deref().filter(|path| !path.is_empty()) {
            match safe_local_file_path(app_context.as_ref(), &row.owner_client_role, path) {
                Ok(Some(path)) => match tokio::fs::remove_file(&path).await {
                    Ok(_) => {
                        cleanup_confirmed = true;
                    }
                    // 文件已不存在视为成功，继续清理数据库记录。
                    Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                        cleanup_confirmed = true;
                    }
                    Err(err) => {
                        mark_delete_failed_retry_later(
                            &row.file_key,
                            &row.owner_client_role,
                            format!("remove local file failed: {}", err),
                            retry_delay_seconds,
                        )
                        .await?;
                        continue;
                    }
                },
                Ok(None) => {}
                Err(err) => {
                    // local_path 来自数据库，删除前必须确认它仍落在 TDLib 文件目录内。
                    // 如果有 td_file_id，后面仍可交给 TDLib 自己删除；否则保留记录等待人工排查。
                    tracing::warn!(
                        file_key = %row.file_key,
                        local_path = %path,
                        error = %err,
                        "skip unsafe local file removal"
                    );
                    if row.td_file_id.is_none() {
                        mark_delete_failed_retry_later(
                            &row.file_key,
                            &row.owner_client_role,
                            format!("unsafe local path refused: {}", err),
                            retry_delay_seconds,
                        )
                        .await?;
                        continue;
                    }
                }
            }
        }

        // 如果保存了 td_file_id，再尝试通知 TDLib 清理缓存（失败不阻断）。
        if let Some(file_id) = row.td_file_id {
            let Some(client_id) = client_id_for_owner(client_ids, &row.owner_client_role) else {
                mark_delete_failed_retry_later(
                    &row.file_key,
                    &row.owner_client_role,
                    format!("owner client isn't ready: {}", row.owner_client_role),
                    retry_delay_seconds,
                )
                .await?;
                continue;
            };
            match tdlib_rs::functions::delete_file(file_id, client_id).await {
                Ok(_) => {}
                Err(err) => {
                    tracing::warn!(
                        "delete_file from tdlib failed, file_key={}, file_id={}, err={:?}",
                        row.file_key,
                        file_id,
                        err
                    );
                    if !cleanup_confirmed {
                        mark_delete_failed_retry_later(
                            &row.file_key,
                            &row.owner_client_role,
                            format!("tdlib delete_file failed: {:?}", err),
                            retry_delay_seconds,
                        )
                        .await?;
                        continue;
                    }
                }
            }
        }

        store::delete_file_cache(&row.owner_client_role, &row.file_key).await?;
        tracing::info!(
            file_key = %row.file_key,
            td_file_id = row.td_file_id,
            "file cache deleted"
        );
    }

    Ok(())
}

/// 把删除失败项延后到下一轮之后重试，避免配置了很短 GC 间隔时刷屏热循环。
async fn mark_delete_failed_retry_later(
    file_key: &str,
    owner_client_role: &str,
    err: String,
    retry_delay_seconds: u64,
) -> anyhow::Result<()> {
    let retry_after = store::now_utc8()
        + chrono::Duration::seconds(retry_delay_seconds.min(i64::MAX as u64) as i64);
    store::mark_file_cache_delete_failed(owner_client_role, file_key, err, retry_after).await
}

/// 删除队列扫描间隔（秒）：
/// 从 config.json 读取 `transfer_config.file_gc_interval_seconds`。
fn cleanup_interval_seconds(app_context: &crate::app_context::AppContext) -> u64 {
    app_context
        .transfer_runtime
        .runtime_config()
        .file_gc_interval_seconds
        .max(1)
}

/// 解析并校验本地文件路径。
///
/// 只允许删除位于 `tdlib_config.files_directory` 下的文件；如果配置为空或路径越界，
/// 调用方必须拒绝 `remove_file`。这里做的是不依赖文件存在性的词法规范化，
/// 这样文件已经被 TDLib 或人工删掉时也能得到稳定判断。
fn safe_local_file_path(
    app_context: &crate::app_context::AppContext,
    owner_client_role: &str,
    local_path: &str,
) -> anyhow::Result<Option<PathBuf>> {
    let role = crate::tgbot::transfer::types::client_role_from_str(owner_client_role)
        .ok_or_else(|| anyhow::anyhow!("invalid owner_client_role: {}", owner_client_role))?;
    let Some(tdlib_root) = app_context.transfer_runtime.tdlib_files_directory_for(role) else {
        anyhow::bail!("tdlib files_directory is empty");
    };
    let cwd = std::env::current_dir()?;
    resolve_safe_local_file_path(local_path, &tdlib_root, &cwd)
}

/// 根据文件 owner role 找到对应 TDLib client id。
fn client_id_for_owner(
    client_ids: crate::config::TransferClientIds,
    owner_client_role: &str,
) -> Option<i32> {
    crate::tgbot::transfer::types::client_role_from_str(owner_client_role)
        .and_then(|role| client_ids.get(role).ok())
}

/// 带 cwd 参数的路径校验纯函数，便于测试覆盖相对路径和 `..` 越界。
fn resolve_safe_local_file_path(
    local_path: &str,
    tdlib_root: &Path,
    cwd: &Path,
) -> anyhow::Result<Option<PathBuf>> {
    if local_path.is_empty() {
        return Ok(None);
    }
    if tdlib_root.as_os_str().is_empty() {
        anyhow::bail!("tdlib files_directory is empty");
    }

    let root = absolute_normalized_path(tdlib_root, cwd)?;
    let candidate = absolute_normalized_path(Path::new(local_path), cwd)?;
    if candidate == root {
        anyhow::bail!(
            "refuse to remove tdlib files_directory itself: {}",
            candidate.display()
        );
    }
    if !path_starts_with_platform(&candidate, &root) {
        anyhow::bail!(
            "local path is outside tdlib files_directory: path={}, root={}",
            candidate.display(),
            root.display()
        );
    }
    Ok(Some(candidate))
}

/// 把相对路径基于 cwd 转为绝对路径，并折叠 `.` / `..`。
fn absolute_normalized_path(path: &Path, cwd: &Path) -> anyhow::Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    };
    normalize_path_lexically(&absolute)
        .ok_or_else(|| anyhow::anyhow!("path escapes above filesystem root: {}", path.display()))
}

/// 词法规范化路径，不访问文件系统，避免目标文件不存在时无法判断安全边界。
fn normalize_path_lexically(path: &Path) -> Option<PathBuf> {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir => {
                if !normalized.pop() {
                    return None;
                }
            }
        }
    }
    Some(normalized)
}

/// Windows 路径大小写不敏感；其他平台直接使用 Path 的组件级 starts_with。
#[cfg(windows)]
fn path_starts_with_platform(path: &Path, root: &Path) -> bool {
    let path = normalized_path_string(path);
    let root = normalized_path_string(root);
    path == root
        || path
            .strip_prefix(&root)
            .map(|rest| rest.starts_with('/'))
            .unwrap_or(false)
}

#[cfg(not(windows))]
fn path_starts_with_platform(path: &Path, root: &Path) -> bool {
    path.starts_with(root)
}

#[cfg(windows)]
fn normalized_path_string(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .trim_end_matches('/')
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::resolve_safe_local_file_path;
    use std::path::Path;

    // 相对 local_path 会按当前工作目录解析；只要仍在 TDLib 文件目录下就允许删除。
    #[test]
    fn test_safe_local_file_path_accepts_relative_path_under_tdlib_root() {
        let cwd = std::env::current_dir().expect("current dir");
        let resolved = resolve_safe_local_file_path("tg/file/a.bin", Path::new("tg/file"), &cwd)
            .expect("path should be safe")
            .expect("path should exist logically");
        assert!(resolved.ends_with(Path::new("tg/file/a.bin")));
    }

    // TDLib 也可能返回绝对路径；绝对路径同样必须落在配置的 TDLib 文件目录下。
    #[test]
    fn test_safe_local_file_path_accepts_absolute_path_under_tdlib_root() {
        let cwd = std::env::current_dir().expect("current dir");
        let root = cwd.join("tg/file");
        let local_path = root.join("a.bin");
        let resolved = resolve_safe_local_file_path(
            local_path.to_str().expect("test path utf-8"),
            &root,
            &cwd,
        )
        .expect("absolute path should be safe")
        .expect("path should exist logically");
        assert!(resolved.ends_with(Path::new("tg/file/a.bin")));
    }

    // `..` 跳出 TDLib 文件目录时必须拒绝，避免数据库路径污染导致误删项目文件。
    #[test]
    fn test_safe_local_file_path_rejects_parent_escape() {
        let cwd = std::env::current_dir().expect("current dir");
        let err = resolve_safe_local_file_path("tg/file/../secret.bin", Path::new("tg/file"), &cwd)
            .expect_err("parent escape should be refused");
        assert!(err.to_string().contains("outside tdlib files_directory"));
    }

    // 字符串前缀相似不代表同一目录，例如 tg/file2 不是 tg/file 的子目录。
    #[test]
    fn test_safe_local_file_path_rejects_sibling_prefix() {
        let cwd = std::env::current_dir().expect("current dir");
        let err = resolve_safe_local_file_path("tg/file2/a.bin", Path::new("tg/file"), &cwd)
            .expect_err("sibling prefix should be refused");
        assert!(err.to_string().contains("outside tdlib files_directory"));
    }

    // 只允许删除目录下的文件，不允许把根目录自身交给 remove_file。
    #[test]
    fn test_safe_local_file_path_rejects_root_itself() {
        let cwd = std::env::current_dir().expect("current dir");
        let err = resolve_safe_local_file_path("tg/file", Path::new("tg/file"), &cwd)
            .expect_err("root itself should be refused");
        assert!(err.to_string().contains("refuse to remove"));
    }

    // 配置为空时不能退化到当前目录，否则会把工作区当成可删除根目录。
    #[test]
    fn test_safe_local_file_path_rejects_empty_tdlib_root() {
        let cwd = std::env::current_dir().expect("current dir");
        let err = resolve_safe_local_file_path("tg/file/a.bin", Path::new(""), &cwd)
            .expect_err("empty root should be refused");
        assert!(err.to_string().contains("files_directory is empty"));
    }
}
