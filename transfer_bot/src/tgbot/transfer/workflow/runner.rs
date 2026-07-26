// 已持有 job 运行锁后的核心执行逻辑。
// 该模块负责下载/准备、整批上传、终态写入和文件引用释放。

use std::collections::HashMap;

use crate::db;
use crate::tgbot::queue;
use sea_orm::EntityTrait;

use super::super::file;
use super::super::file::PreparedUpload;
use super::super::spider;
use super::super::store::{
    self, ITEM_STATUS_FAILED, ITEM_STATUS_PREPARED, ITEM_STATUS_PREPARING, ITEM_STATUS_SUCCESS,
    ITEM_STATUS_UPLOADING,
};
use super::super::types::{SourceKind, TransferBundle, client_role_from_str};
use super::TransferOutcome;
use super::control::{apply_job_control, finish_skipped_by_control};
use super::result_link::build_result_message_link;
use super::upload::{is_initial_upload_rejected, upload_prepared};

/// 已持有 job 运行锁后的核心执行逻辑：
/// 1. 准备所有上传内容（包括下载与缓存回填）
/// 2. 若全部成功，再进行上传（单条 send_message，多条 send_message_album）
/// 3. 结束后释放引用，进入延迟删除队列
pub(super) async fn run_job_inner(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    job: db::transfer_job::Model,
    messages: Vec<tdlib_rs::types::Message>,
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<TransferOutcome> {
    run_job_inner_with_fallback(app_context, job, messages, client_ids, true).await
}

/// 执行任务，并在链接源 bot 准备失败时切 user 源重试一次。
///
/// bot-first 策略分两段：
/// - spider 阶段 bot 失败会立即 fallback 到 user；
/// - bot spider 成功但后续下载/准备失败时，可能是 bot 对文件权限或文件状态受限，此处重新用 user spider。
///
/// `allow_prepare_fallback` 防止 user 重试失败后无限递归。BotMessage 源不 fallback，因为 user 通常无法读取
/// bot 私聊或 bot 可见消息的本地 message_id。
async fn run_job_inner_with_fallback(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    job: db::transfer_job::Model,
    messages: Vec<tdlib_rs::types::Message>,
    client_ids: crate::config::TransferClientIds,
    allow_prepare_fallback: bool,
) -> anyhow::Result<TransferOutcome> {
    match run_job_inner_once(app_context.clone(), job.clone(), messages, client_ids).await {
        Ok(outcome) => Ok(outcome),
        Err(err) => {
            if !allow_prepare_fallback || !should_fallback_prepare_to_user(&job, &err) {
                return Err(err);
            }
            tracing::warn!(
                job_id = job.id,
                source_link = %job.source_link,
                error = %err,
                "bot source prepare failed, fallback to user source"
            );
            let fallback_result = match spider::spider_message(
                job.source_link.clone(),
                client_ids.get(crate::config::ClientRole::User)?,
                crate::config::ClientRole::User,
            )
            .await
            {
                Ok(bundle) => {
                    match reconcile_job_source_for_fallback(app_context.as_ref(), job.id, &bundle)
                        .await
                    {
                        Ok(()) => Ok(bundle),
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            };
            let bundle = match fallback_result {
                Ok(bundle) => bundle,
                Err(fallback_err) => {
                    finish_prepare_fallback_failed_job(
                        app_context.as_ref(),
                        &job,
                        &err,
                        &fallback_err,
                    )
                    .await?;
                    anyhow::bail!(
                        "transfer failed during bot prepare and user fallback, job_id={}, bot_error={:#}, fallback_error={:#}",
                        job.id,
                        err,
                        fallback_err
                    );
                }
            };
            let refreshed_job = db::transfer_job::Entity::find_by_id(job.id)
                .one(crate::db::get_db().await?)
                .await?
                .ok_or_else(|| {
                    anyhow::anyhow!("transfer job disappeared during fallback: {}", job.id)
                })?;
            run_job_inner_once(app_context, refreshed_job, bundle.messages, client_ids).await
        }
    }
}

/// 执行一次任务，不做 source fallback。
async fn run_job_inner_once(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    job: db::transfer_job::Model,
    messages: Vec<tdlib_rs::types::Message>,
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<TransferOutcome> {
    tracing::info!(
        job_id = job.id,
        target_chat_id = job.target_chat_id,
        message_count = messages.len(),
        source_client_role = %job.source_client_role,
        upload_client_id = client_ids.upload,
        "transfer job execution started"
    );

    let source_client_role = client_role_from_str(&job.source_client_role)
        .ok_or_else(|| anyhow::anyhow!("invalid source_client_role: {}", job.source_client_role))?;
    let source_client_id = client_ids.get(source_client_role)?;

    if let Some(outcome) = apply_job_control(app_context.as_ref(), job.id).await? {
        tracing::info!(
            job_id = job.id,
            "transfer job stopped by control before prepare"
        );
        return Ok(outcome);
    }

    let items = store::list_items_by_job(job.id).await?;
    let mut item_map: HashMap<(i64, i64), db::transfer_item::Model> = HashMap::new();
    for item in items {
        item_map.insert((item.source_chat_id, item.source_message_id), item);
    }
    let base_done = item_map
        .values()
        .filter(|it| it.status == ITEM_STATUS_SUCCESS)
        .count() as i32;
    // 非 success 子项都会在本轮重新准备，旧 failed 不能继续计入最终失败数。
    let base_failed = 0i32;

    let mut prepared: Vec<(i64, PreparedUpload)> = Vec::with_capacity(messages.len());
    let mut prepare_fail_count = 0i32;
    let mut last_error: Option<String> = None;

    // 第一阶段：全部准备完成（下载与构建 InputMessageContent）。
    for msg in &messages {
        if let Some(outcome) = apply_job_control(app_context.as_ref(), job.id).await? {
            tracing::info!(
                job_id = job.id,
                "transfer job stopped by control during prepare"
            );
            return Ok(outcome);
        }

        let Some(item) = item_map.get(&(msg.chat_id, msg.id)) else {
            prepare_fail_count += 1;
            last_error = Some(format!(
                "missing transfer_item for source message {}",
                msg.id
            ));
            continue;
        };

        if item.status == ITEM_STATUS_SUCCESS {
            // 已经成功的子项跳过（容错：历史状态）。
            continue;
        }

        store::set_item_status(item.id, ITEM_STATUS_PREPARING, None).await?;
        if let Some(outcome) = apply_job_control(app_context.as_ref(), job.id).await? {
            tracing::info!(
                job_id = job.id,
                item_id = item.id,
                "transfer job stopped by control before item prepare"
            );
            return Ok(outcome);
        }

        let file_key = file::extract_file_key(msg);
        let mut cached_meta = None;
        if let Some(seed) = file::extract_download_seed(msg) {
            cached_meta =
                store::find_ready_file_cache(&job.source_client_role, &seed.file_key).await?;
            if cached_meta.is_none() {
                store::mark_file_cache_downloading(&job.source_client_role, &seed).await?;
                // TDLib 的 file id/local path 隶属于具体 client，bot/user 不能共享同一个下载 future。
                let singleflight_key = format!("{}:{}", job.source_client_role, seed.file_key);
                let download_result = queue::run_singleflight(singleflight_key, || async {
                    file::ensure_media_downloaded(msg, source_client_id, job.id).await
                })
                .await;
                if let Err(err) = download_result {
                    if let Some(outcome) = apply_job_control(app_context.as_ref(), job.id).await? {
                        tracing::info!(
                            job_id = job.id,
                            item_id = item.id,
                            outcome = ?outcome,
                            "transfer job download interrupted by control"
                        );
                        return Ok(outcome);
                    }
                    let err_str = format!("{err:#}");
                    tracing::warn!(
                        job_id = job.id,
                        item_id = item.id,
                        file_key = %seed.file_key,
                        error = %err_str,
                        "prepare item download failed"
                    );
                    store::set_item_status(item.id, ITEM_STATUS_FAILED, Some(err_str.clone()))
                        .await?;
                    store::mark_file_cache_failed(
                        &job.source_client_role,
                        &seed.file_key,
                        err_str.clone(),
                    )
                    .await?;
                    prepare_fail_count += 1;
                    last_error = Some(err_str);
                    continue;
                }
            }
            if let Some(outcome) = apply_job_control(app_context.as_ref(), job.id).await? {
                tracing::info!(
                    job_id = job.id,
                    item_id = item.id,
                    "transfer job stopped by control after download"
                );
                return Ok(outcome);
            }
        }

        match file::prepare_upload_content(msg, source_client_id, cached_meta.as_ref()).await {
            Ok(prepared_one) => {
                if let Some(meta) = &prepared_one.cache_meta {
                    store::mark_file_cache_ready(&job.source_client_role, meta).await?;
                }
                store::set_item_status(item.id, ITEM_STATUS_PREPARED, None).await?;
                prepared.push((item.id, prepared_one));
                if let Some(outcome) = apply_job_control(app_context.as_ref(), job.id).await? {
                    tracing::info!(
                        job_id = job.id,
                        item_id = item.id,
                        "transfer job stopped by control after item prepare"
                    );
                    return Ok(outcome);
                }
            }
            Err(err) => {
                let err_str = format!("{err:#}");
                tracing::warn!(
                    job_id = job.id,
                    item_id = item.id,
                    file_key = file_key.as_deref().unwrap_or("none"),
                    error = %err_str,
                    "prepare upload content failed"
                );
                store::set_item_status(item.id, ITEM_STATUS_FAILED, Some(err_str.clone())).await?;
                if let Some(key) = file_key {
                    store::mark_file_cache_failed(&job.source_client_role, &key, err_str.clone())
                        .await?;
                }
                prepare_fail_count += 1;
                last_error = Some(err_str);
            }
        }
    }

    // 准备失败时不进入上传阶段，直接结束任务并释放引用。
    if prepare_fail_count > 0 {
        tracing::warn!(
            job_id = job.id,
            prepare_fail_count,
            prepared_count = prepared.len(),
            "transfer job prepare failed"
        );
        if should_return_prepare_error_for_fallback(&job) {
            anyhow::bail!(
                "transfer failed during prepare, job_id={}, error={}",
                job.id,
                last_error.unwrap_or_else(|| "unknown error".to_owned())
            );
        }
        let total_fail = base_failed + prepare_fail_count + (prepared.len() as i32);
        let item_updates = prepared
            .iter()
            .map(|(item_id, _)| {
                (
                    *item_id,
                    ITEM_STATUS_FAILED.to_owned(),
                    Some("batch prepare failed".to_owned()),
                )
            })
            .collect::<Vec<_>>();
        if !store::finish_job_with_item_statuses(
            job.clone(),
            store::FinishJobSummary {
                ok_count: base_done,
                fail_count: total_fail,
                last_error: last_error.clone(),
                result_message_id: None,
                result_message_link: None,
                result_messages: Vec::new(),
                delay_minutes: super::file_delete_delay_minutes(app_context.as_ref()),
            },
            item_updates,
        )
        .await?
        {
            return finish_skipped_by_control(app_context.as_ref(), job.id).await;
        }
        anyhow::bail!(
            "transfer failed during prepare, job_id={}, error={}",
            job.id,
            last_error.unwrap_or_else(|| "unknown error".to_owned())
        );
    }

    // 若本轮无需再上传（例如恢复时所有子项已是 success），直接收敛任务状态。
    if prepared.is_empty() {
        tracing::info!(
            job_id = job.id,
            ok_count = base_done,
            "transfer job already has all items uploaded"
        );
        let link = job
            .result_message_link
            .clone()
            .ok_or_else(|| anyhow::anyhow!("job finished but result_message_link missing"))?;
        if !store::finish_job(
            job.clone(),
            base_done,
            base_failed,
            last_error.clone(),
            job.result_message_id,
            job.result_message_link.clone(),
            super::file_delete_delay_minutes(app_context.as_ref()),
        )
        .await?
        {
            return finish_skipped_by_control(app_context.as_ref(), job.id).await;
        }
        return Ok(TransferOutcome::Completed {
            job_id: job.id,
            link,
        });
    }

    // 第二阶段：所有项准备成功后再上传。
    if let Some(outcome) = apply_job_control(app_context.as_ref(), job.id).await? {
        tracing::info!(
            job_id = job.id,
            "transfer job stopped by control before upload"
        );
        return Ok(outcome);
    }
    for (item_id, _) in &prepared {
        store::set_item_status(*item_id, ITEM_STATUS_UPLOADING, None).await?;
    }
    tracing::info!(
        job_id = job.id,
        target_chat_id = job.target_chat_id,
        prepared_count = prepared.len(),
        upload_client_id = client_ids.upload,
        "transfer job upload started"
    );

    // guard 覆盖上传与最终状态落库；任何成功或错误返回都会清理运行时 file_id 映射。
    // 恢复任务可能来自进程重启或上一次中断，先清除同 job 的陈旧 file_id/字节快照，
    // 避免新一轮上传从旧进度起算。
    let mut upload_client_id = client_ids.upload;
    let initial_upload_result = {
        app_context
            .upload_progress
            .clear_job(upload_client_id, job.id);
        let _upload_progress_guard = app_context
            .upload_progress
            .job_guard(upload_client_id, job.id);
        upload_prepared(
            app_context.as_ref(),
            job.id,
            job.target_chat_id,
            &prepared,
            upload_client_id,
        )
        .await
    };
    let upload_result = match initial_upload_result {
        Err(error)
            if client_ids.bot == Some(upload_client_id)
                && client_ids.user.is_some()
                && is_initial_upload_rejected(&error) =>
        {
            let user_client_id = client_ids.get(crate::config::ClientRole::User)?;
            tracing::warn!(
                job_id = job.id,
                target_chat_id = job.target_chat_id,
                bot_upload_client_id = upload_client_id,
                user_upload_client_id = user_client_id,
                error = %error,
                "bot initial upload rejected, fallback to user before target acceptance"
            );
            upload_client_id = user_client_id;
            app_context
                .upload_progress
                .clear_job(upload_client_id, job.id);
            let _upload_progress_guard = app_context
                .upload_progress
                .job_guard(upload_client_id, job.id);
            upload_prepared(
                app_context.as_ref(),
                job.id,
                job.target_chat_id,
                &prepared,
                upload_client_id,
            )
            .await
        }
        result => result,
    };
    match upload_result {
        Ok(upload_result) => {
            // 上传成功后目标消息已经真实发出，不能再用 pause/stop 把数据库隐藏成未完成。
            let mut result_messages = Vec::with_capacity(upload_result.entries.len());
            for (idx, entry) in upload_result.entries.iter().enumerate() {
                let result_link = build_result_message_link(
                    job.target_chat_id,
                    entry.message_id,
                    entry.is_album,
                    upload_client_id,
                )
                .await?;
                result_messages.push(store::ResultMessageRecord {
                    result_index: idx as i32,
                    target_chat_id: job.target_chat_id,
                    message_id: entry.message_id,
                    message_link: result_link,
                    is_album: entry.is_album,
                    item_count: entry.item_count,
                });
            }
            let first_result = result_messages
                .first()
                .ok_or_else(|| anyhow::anyhow!("upload succeeded without result message"))?;
            let item_updates = prepared
                .iter()
                .map(|(item_id, _)| (*item_id, ITEM_STATUS_SUCCESS.to_owned(), None))
                .collect::<Vec<_>>();
            if !store::finish_uploaded_job_with_item_statuses(
                job.clone(),
                store::FinishJobSummary {
                    ok_count: base_done + prepared.len() as i32,
                    fail_count: base_failed,
                    last_error: None,
                    result_message_id: Some(first_result.message_id),
                    result_message_link: Some(first_result.message_link.clone()),
                    result_messages: result_messages.clone(),
                    delay_minutes: super::file_delete_delay_minutes(app_context.as_ref()),
                },
                item_updates,
            )
            .await?
            {
                return finish_skipped_by_control(app_context.as_ref(), job.id).await;
            }
            tracing::info!(
                job_id = job.id,
                target_chat_id = job.target_chat_id,
                result_message_id = first_result.message_id,
                result_count = result_messages.len(),
                "transfer job completed"
            );
            Ok(TransferOutcome::Completed {
                job_id: job.id,
                link: first_result.message_link.clone(),
            })
        }
        Err(err) => {
            // 上传等待会在 pause/stop 时主动返回；必须先交给控制状态收敛，
            // 不能把用户控制误记成上传失败，更不能随后覆盖成 success。
            if let Some(outcome) = apply_job_control(app_context.as_ref(), job.id).await? {
                tracing::info!(
                    job_id = job.id,
                    target_chat_id = job.target_chat_id,
                    outcome = ?outcome,
                    "transfer upload interrupted by job control"
                );
                return Ok(outcome);
            }
            let err_str = format!("{err:#}");
            tracing::error!(
                job_id = job.id,
                target_chat_id = job.target_chat_id,
                error = %err_str,
                "transfer job upload failed"
            );
            let item_updates = prepared
                .iter()
                .map(|(item_id, _)| {
                    (
                        *item_id,
                        ITEM_STATUS_FAILED.to_owned(),
                        Some(err_str.clone()),
                    )
                })
                .collect::<Vec<_>>();
            if !store::finish_job_with_item_statuses(
                job.clone(),
                store::FinishJobSummary {
                    ok_count: base_done,
                    fail_count: base_failed + prepared.len() as i32,
                    last_error: Some(err_str),
                    result_message_id: None,
                    result_message_link: None,
                    result_messages: Vec::new(),
                    delay_minutes: super::file_delete_delay_minutes(app_context.as_ref()),
                },
                item_updates,
            )
            .await?
            {
                return finish_skipped_by_control(app_context.as_ref(), job.id).await;
            }
            anyhow::bail!("transfer upload failed, job_id={}", job.id)
        }
    }
}

/// 判断一次准备失败是否允许从 bot 源切到 user 源重试。
fn should_fallback_prepare_to_user(job: &db::transfer_job::Model, err: &anyhow::Error) -> bool {
    if job.source_kind != SourceKind::Link.as_str() || job.source_client_role != "bot" {
        return false;
    }
    // 是否允许 user 账号读取私有源链接，由任务创建时的读取策略持久化决定。
    if !job.allow_user_fallback {
        return false;
    }
    // 只对准备阶段失败 fallback；上传失败说明文件已准备好，问题在目标发送，不应重新下载。
    err.to_string().contains("transfer failed during prepare")
}

/// bot 链接源准备失败时先把错误交给外层 fallback，不立刻写失败终态。
fn should_return_prepare_error_for_fallback(job: &db::transfer_job::Model) -> bool {
    job.source_kind == SourceKind::Link.as_str()
        && job.source_client_role == "bot"
        && job.allow_user_fallback
}

/// 将 bot 源任务切换为 user 源任务。
///
/// 复用恢复流程的 reconcile：新增/消失/owner 变化都会在同一事务里更新 item 和 file_cache 引用。
async fn reconcile_job_source_for_fallback(
    app_context: &crate::app_context::AppContext,
    job_id: i64,
    bundle: &TransferBundle,
) -> anyhow::Result<()> {
    if let Some(outcome) = apply_job_control(app_context, job_id).await? {
        anyhow::bail!("transfer job control requested during fallback: {outcome:?}");
    }
    store::reconcile_items_for_bundle(
        job_id,
        bundle,
        super::file_delete_delay_minutes(app_context),
    )
    .await
}

/// bot 准备失败且 user fallback 也失败时，必须把任务收敛成失败并释放已有引用。
async fn finish_prepare_fallback_failed_job(
    app_context: &crate::app_context::AppContext,
    job: &db::transfer_job::Model,
    bot_err: &anyhow::Error,
    fallback_err: &anyhow::Error,
) -> anyhow::Result<()> {
    let items = store::list_items_by_job(job.id).await?;
    let item_updates = items
        .iter()
        .filter(|item| item.status != ITEM_STATUS_SUCCESS)
        .map(|item| {
            (
                item.id,
                ITEM_STATUS_FAILED.to_owned(),
                Some("bot prepare failed and user fallback failed".to_owned()),
            )
        })
        .collect::<Vec<_>>();
    let ok_count = items
        .iter()
        .filter(|item| item.status == ITEM_STATUS_SUCCESS)
        .count() as i32;
    let fail_count = (items.len() as i32 - ok_count).max(1);
    let last_error = Some(format!(
        "bot prepare failed: {bot_err:#}; user fallback failed: {fallback_err:#}"
    ));
    if !store::finish_job_with_item_statuses(
        job.clone(),
        store::FinishJobSummary {
            ok_count,
            fail_count,
            last_error,
            result_message_id: None,
            result_message_link: None,
            result_messages: Vec::new(),
            delay_minutes: super::file_delete_delay_minutes(app_context),
        },
        item_updates,
    )
    .await?
    {
        let _ = finish_skipped_by_control(app_context, job.id).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 只有 link + bot 源的准备阶段失败才允许切 user 重试。
    #[test]
    fn test_should_fallback_prepare_to_user_only_for_bot_link_prepare_error() {
        let mut job = fallback_test_job("link", "bot");
        let prepare_err = anyhow::anyhow!("transfer failed during prepare, job_id=1, error=x");
        let upload_err = anyhow::anyhow!("transfer upload failed, job_id=1");

        assert!(should_fallback_prepare_to_user(&job, &prepare_err));
        assert!(!should_fallback_prepare_to_user(&job, &upload_err));

        job.source_client_role = "user".to_owned();
        assert!(!should_fallback_prepare_to_user(&job, &prepare_err));

        job.source_client_role = "bot".to_owned();
        job.source_kind = "bot_message".to_owned();
        assert!(!should_fallback_prepare_to_user(&job, &prepare_err));
    }

    fn fallback_test_job(source_kind: &str, source_client_role: &str) -> db::transfer_job::Model {
        db::transfer_job::Model {
            id: 1,
            request_chat_id: 10,
            request_message_id: 20,
            owner_user_id: 10,
            source_link: "https://t.me/c/1/2".to_owned(),
            source_kind: source_kind.to_owned(),
            source_client_role: source_client_role.to_owned(),
            allow_user_fallback: true,
            source_chat_id: 30,
            source_message_id: 40,
            source_album_id: 0,
            target_chat_id: 50,
            result_message_id: None,
            result_message_link: None,
            status: store::JOB_STATUS_RUNNING.to_owned(),
            total_items: 1,
            done_items: 0,
            failed_items: 0,
            retry_count: 0,
            last_error: None,
            created_at: store::now_utc8(),
            updated_at: store::now_utc8(),
            finished_at: None,
        }
    }
}
