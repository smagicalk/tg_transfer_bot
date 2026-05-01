// 已持有 job 运行锁后的核心执行逻辑。
// 该模块负责下载/准备、整批上传、终态写入和文件引用释放。

use std::collections::HashMap;

use crate::db;
use crate::tgbot::queue;

use super::super::file;
use super::super::file::PreparedUpload;
use super::super::store::{
    self, ITEM_STATUS_FAILED, ITEM_STATUS_PREPARED, ITEM_STATUS_PREPARING, ITEM_STATUS_SUCCESS,
    ITEM_STATUS_UPLOADING,
};
use super::TransferOutcome;
use super::control::{apply_job_control, finish_skipped_by_control};
use super::upload::{build_result_message_link, upload_prepared};

/// 已持有 job 运行锁后的核心执行逻辑：
/// 1. 准备所有上传内容（包括下载与缓存回填）
/// 2. 若全部成功，再进行上传（单条 send_message，多条 send_message_album）
/// 3. 结束后释放引用，进入延迟删除队列
pub(super) async fn run_job_inner(
    job: db::transfer_job::Model,
    messages: Vec<tdlib_rs::types::Message>,
    client_id: i32,
) -> anyhow::Result<TransferOutcome> {
    tracing::info!(
        job_id = job.id,
        target_chat_id = job.target_chat_id,
        message_count = messages.len(),
        "transfer job execution started"
    );

    if let Some(outcome) = apply_job_control(job.id).await? {
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
        if let Some(outcome) = apply_job_control(job.id).await? {
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
        if let Some(outcome) = apply_job_control(job.id).await? {
            tracing::info!(
                job_id = job.id,
                item_id = item.id,
                "transfer job stopped by control before item prepare"
            );
            return Ok(outcome);
        }

        let file_key = file::extract_file_key(msg);
        if let Some(seed) = file::extract_download_seed(msg) {
            store::mark_file_cache_downloading(&seed).await?;
            let download_result = queue::run_singleflight(seed.file_key.clone(), || async {
                file::ensure_media_downloaded(msg, client_id).await
            })
            .await;
            if let Err(err) = download_result {
                let err_str = format!("{:#}", err);
                tracing::warn!(
                    job_id = job.id,
                    item_id = item.id,
                    file_key = %seed.file_key,
                    error = %err_str,
                    "prepare item download failed"
                );
                store::set_item_status(item.id, ITEM_STATUS_FAILED, Some(err_str.clone())).await?;
                store::mark_file_cache_failed(&seed.file_key, err_str.clone()).await?;
                prepare_fail_count += 1;
                last_error = Some(err_str);
                continue;
            }
            if let Some(outcome) = apply_job_control(job.id).await? {
                tracing::info!(
                    job_id = job.id,
                    item_id = item.id,
                    "transfer job stopped by control after download"
                );
                return Ok(outcome);
            }
        }

        match file::prepare_upload_content(msg, client_id).await {
            Ok(prepared_one) => {
                if let Some(meta) = &prepared_one.cache_meta {
                    store::mark_file_cache_ready(meta).await?;
                }
                store::set_item_status(item.id, ITEM_STATUS_PREPARED, None).await?;
                prepared.push((item.id, prepared_one));
                if let Some(outcome) = apply_job_control(job.id).await? {
                    tracing::info!(
                        job_id = job.id,
                        item_id = item.id,
                        "transfer job stopped by control after item prepare"
                    );
                    return Ok(outcome);
                }
            }
            Err(err) => {
                let err_str = format!("{:#}", err);
                tracing::warn!(
                    job_id = job.id,
                    item_id = item.id,
                    file_key = file_key.as_deref().unwrap_or("none"),
                    error = %err_str,
                    "prepare upload content failed"
                );
                store::set_item_status(item.id, ITEM_STATUS_FAILED, Some(err_str.clone())).await?;
                if let Some(key) = file_key {
                    store::mark_file_cache_failed(&key, err_str.clone()).await?;
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
                delay_minutes: super::file_delete_delay_minutes(),
            },
            item_updates,
        )
        .await?
        {
            return finish_skipped_by_control(job.id).await;
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
            super::file_delete_delay_minutes(),
        )
        .await?
        {
            return finish_skipped_by_control(job.id).await;
        }
        return Ok(TransferOutcome::Completed { link });
    }

    // 第二阶段：所有项准备成功后再上传。
    if let Some(outcome) = apply_job_control(job.id).await? {
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
        "transfer job upload started"
    );

    let upload_result = upload_prepared(job.target_chat_id, &prepared, client_id).await;
    match upload_result {
        Ok(upload_result) => {
            // 上传成功后目标消息已经真实发出，不能再用 pause/stop 把数据库隐藏成未完成。
            let result_link = build_result_message_link(
                job.target_chat_id,
                upload_result.result_message_id,
                upload_result.is_album,
                client_id,
            )
            .await?;
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
                    result_message_id: Some(upload_result.result_message_id),
                    result_message_link: Some(result_link.clone()),
                    delay_minutes: super::file_delete_delay_minutes(),
                },
                item_updates,
            )
            .await?
            {
                return finish_skipped_by_control(job.id).await;
            }
            tracing::info!(
                job_id = job.id,
                target_chat_id = job.target_chat_id,
                result_message_id = upload_result.result_message_id,
                is_album = upload_result.is_album,
                "transfer job completed"
            );
            Ok(TransferOutcome::Completed { link: result_link })
        }
        Err(err) => {
            let err_str = format!("{:#}", err);
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
                    delay_minutes: super::file_delete_delay_minutes(),
                },
                item_updates,
            )
            .await?
            {
                return finish_skipped_by_control(job.id).await;
            }
            anyhow::bail!("transfer upload failed, job_id={}", job.id)
        }
    }
}
