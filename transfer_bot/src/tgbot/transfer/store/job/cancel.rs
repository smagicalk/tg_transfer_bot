// transfer_job 取消收尾逻辑。
// stop 命令先把任务标记为 cancelling，本模块负责最终 cancelled 和文件引用释放。

use std::collections::HashSet;
use std::sync::{LazyLock, Mutex, MutexGuard};

use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::TransactionTrait;
use sea_orm::sea_query::Expr;

use crate::db;

use super::super::file_cache::release_job_file_refs_on_conn;
use super::super::item::{list_items_by_job_on_conn, set_item_status_on_conn};
use super::super::{
    ITEM_STATUS_CANCELLED, ITEM_STATUS_FAILED, ITEM_STATUS_SUCCESS, JOB_STATUS_CANCEL_FINALIZING,
    JOB_STATUS_CANCELLED, JOB_STATUS_CANCELLING, JOB_STATUS_FAILED, JOB_STATUS_PARTIAL,
    JOB_STATUS_PAUSED, JOB_STATUS_PENDING, JOB_STATUS_RUNNING, JOB_STATUS_SUCCESS, now_utc8,
};

/// 取消收尾被其他执行者认领时，等待对方完成的轮数。
const CANCEL_FINALIZING_WAIT_LIMIT: usize = 40;
/// 取消收尾等待轮询间隔。
const CANCEL_FINALIZING_WAIT_DELAY_MS: u64 = 50;

// 进程内取消收尾互斥集合：同一个 job 的 stop/cancel 收尾只允许一个执行者进入数据库写事务。
static CANCEL_FINALIZING_JOB_IDS: LazyLock<Mutex<HashSet<i64>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// 获取指定 job 的取消收尾进程内排他锁。
///
/// 数据库里的 `cancel_finalizing` 负责跨进程/跨连接幂等；这里额外避免同进程内两个 stop
/// 协程同时进入 SQLite 写事务，降低 `database is locked` 锁竞争风险。
///
/// # 参数
/// - `job_id`: 任务主键 ID。
///
/// # 返回值
/// - `CancelFinalizingGuard` 互斥保护守卫。
async fn acquire_cancel_finalizing_guard(job_id: i64) -> CancelFinalizingGuard {
    loop {
        {
            let mut guard = lock_cancel_finalizing_job_ids();
            if guard.insert(job_id) {
                return CancelFinalizingGuard { job_id };
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
}

/// 取消收尾进程内锁 guard，在 drop 时从全局集合中移除该 job_id。
struct CancelFinalizingGuard {
    job_id: i64,
}

impl Drop for CancelFinalizingGuard {
    fn drop(&mut self) {
        let mut guard = lock_cancel_finalizing_job_ids();
        guard.remove(&self.job_id);
    }
}

/// 获取取消收尾互斥锁；当出现锁中毒（Mutex Poisoned）时自动重置集合，避免单个异常崩溃波及后续所有 stop 操作。
///
/// # 返回值
/// - 内部集合的 `MutexGuard`。
fn lock_cancel_finalizing_job_ids() -> MutexGuard<'static, HashSet<i64>> {
    match CANCEL_FINALIZING_JOB_IDS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!("recover poisoned cancel finalizing mutex");
            poisoned.into_inner()
        }
    }
}

/// 立即取消任务并释放其占用的文件引用。
///
/// 内部流程：
/// 1. 获取进程内互斥 Guard；
/// 2. 两阶段认领：先原子将状态从 `pending/running/paused/cancelling` 更新为 `cancel_finalizing`；
/// 3. 若并发冲突且对方正在收尾，则轮询等待最终结果，超时则接管；
/// 4. 开启写事务：统计已完成/失败项，将未完成项全部置为 `cancelled`；
/// 5. 将任务主表状态更新为 `cancelled` 并记录终止原因；
/// 6. 批量释放本任务持有的文件缓存引用（`release_job_file_refs_on_conn`），设定延迟删除期。
///
/// # 参数
/// - `job_id`: 待取消的任务 ID。
/// - `reason`: 终止原因（如用户指令输入或超时退出）。
/// - `delay_minutes`: 文件引用的安全删除延迟（分钟）。
///
/// # 返回值
/// - 终止后的 `transfer_job::Model` 数据库模型。
pub(in crate::tgbot::transfer) async fn cancel_job_now(
    job_id: i64,
    reason: impl Into<String>,
    delay_minutes: i64,
) -> anyhow::Result<db::transfer_job::Model> {
    let _cancel_guard = acquire_cancel_finalizing_guard(job_id).await;
    let db_conn = db::get_db().await?;
    let reason = reason.into();

    // 先原子认领取消收尾权，避免并发 stop 对同一个 job 重复扣减 file_cache.active_refs。
    let rs = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::Status,
            Expr::value(JOB_STATUS_CANCEL_FINALIZING),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now_utc8()))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .filter(db::transfer_job::Column::Status.is_in([
            JOB_STATUS_PENDING.to_owned(),
            JOB_STATUS_RUNNING.to_owned(),
            JOB_STATUS_PAUSED.to_owned(),
            JOB_STATUS_CANCELLING.to_owned(),
        ]))
        .exec(db_conn)
        .await?;

    if rs.rows_affected == 0 {
        let current = db::transfer_job::Entity::find_by_id(job_id)
            .one(db_conn)
            .await?
            .ok_or_else(|| anyhow::anyhow!("job not found: {job_id}"))?;

        if current.status == JOB_STATUS_CANCELLED {
            return Ok(current);
        }

        if matches!(
            current.status.as_str(),
            JOB_STATUS_SUCCESS | JOB_STATUS_FAILED | JOB_STATUS_PARTIAL
        ) {
            anyhow::bail!("job already finished: {}", current.status);
        }

        if current.status == JOB_STATUS_CANCEL_FINALIZING {
            // 其他执行者正在释放引用；通常很快完成，当前调用等待最终 cancelled 结果。
            for _ in 0..CANCEL_FINALIZING_WAIT_LIMIT {
                tokio::time::sleep(std::time::Duration::from_millis(
                    CANCEL_FINALIZING_WAIT_DELAY_MS,
                ))
                .await;
                let latest = db::transfer_job::Entity::find_by_id(job_id)
                    .one(db_conn)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("job not found: {job_id}"))?;
                if latest.status == JOB_STATUS_CANCELLED {
                    return Ok(latest);
                }
                if latest.status != JOB_STATUS_CANCEL_FINALIZING {
                    anyhow::bail!("job status changed during cancel: {}", latest.status);
                }
            }
            // 超时说明可能是上次进程在收尾中退出；当前调用接管后续释放流程。
        } else {
            anyhow::bail!("job status doesn't support cancel now: {}", current.status);
        }
    }

    let job = db::transfer_job::Entity::find_by_id(job_id)
        .one(db_conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found after cancel claim: {job_id}"))?;
    if job.status == JOB_STATUS_CANCELLED {
        return Ok(job);
    }
    if job.status != JOB_STATUS_CANCEL_FINALIZING {
        anyhow::bail!("job status doesn't own cancel finalizing: {}", job.status);
    }

    let txn = db_conn.begin().await?;
    let items = list_items_by_job_on_conn(&txn, job_id).await?;
    let mut done_count = 0i32;
    let mut failed_count = 0i32;
    let mut item_updates = Vec::new();
    for item in items {
        match item.status.as_str() {
            ITEM_STATUS_SUCCESS => done_count += 1,
            ITEM_STATUS_FAILED => failed_count += 1,
            ITEM_STATUS_CANCELLED => {}
            _ => {
                item_updates.push((
                    item.id,
                    ITEM_STATUS_CANCELLED.to_owned(),
                    Some("cancelled by user".to_owned()),
                ));
            }
        }
    }

    let now = now_utc8();
    let rs = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::Status,
            Expr::value(JOB_STATUS_CANCELLED),
        )
        .col_expr(db::transfer_job::Column::DoneItems, Expr::value(done_count))
        .col_expr(
            db::transfer_job::Column::FailedItems,
            Expr::value(failed_count),
        )
        .col_expr(
            db::transfer_job::Column::LastError,
            Expr::value(Some(reason)),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now))
        .col_expr(db::transfer_job::Column::FinishedAt, Expr::value(Some(now)))
        .filter(db::transfer_job::Column::Id.eq(job_id))
        .filter(db::transfer_job::Column::Status.eq(JOB_STATUS_CANCEL_FINALIZING))
        .exec(&txn)
        .await?;

    if rs.rows_affected == 0 {
        txn.rollback().await?;
        let current = db::transfer_job::Entity::find_by_id(job_id)
            .one(db_conn)
            .await?
            .ok_or_else(|| anyhow::anyhow!("job not found during cancel finish: {job_id}"))?;
        if current.status == JOB_STATUS_CANCELLED {
            return Ok(current);
        }
        anyhow::bail!(
            "job status changed during cancel finish: {}",
            current.status
        );
    }

    for (item_id, status, error_message) in item_updates {
        set_item_status_on_conn(&txn, item_id, &status, error_message).await?;
    }
    release_job_file_refs_on_conn(&txn, job_id, delay_minutes).await?;
    txn.commit().await?;

    db::transfer_job::Entity::find_by_id(job_id)
        .one(db_conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("job not found after cancel finish: {job_id}"))
}
