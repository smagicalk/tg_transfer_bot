// 转存启动阶段：
// - source_link + target_chat_id 只用于复用已经成功的历史结果。
// - request_chat_id + request_message_id 是请求幂等维度，只兜底处理 TDLib/网络重复投递同一条命令。
// - 不同请求即使源和目标相同也必须各自执行，不能合并成同一个活跃任务。

use crate::db;

use super::super::types::{SourceKind, TransferPlan};
use super::super::{spider, store};
use super::TransferOutcome;
use super::control::apply_job_control;
use super::guard::{acquire_job_guard, acquire_source_target_create_guard};
use super::result_link::refresh_stored_result_link;

/// 转存入口完成创建阶段后的下一步动作。
pub(super) enum TransferStart {
    /// 已经可以直接返回结果，不需要执行后台流程（例如复用历史成功结果、或任务已被暂停/取消）。
    Outcome(TransferOutcome),
    /// 命中同一请求的未完成任务，释放创建锁后再恢复执行。
    Resume(db::transfer_job::Model),
    /// 新建任务完成，释放创建锁后执行下载与上传流程。
    /// 包含任务模型、源消息列表及进程内独占的任务运行守卫。
    Run(
        db::transfer_job::Model,
        Vec<tdlib_rs::types::Message>,
        crate::app_context::TransferJobGuard,
    ),
}

/// 判断本次 `/transfer` 应复用、恢复还是创建新任务。
///
/// 核心执行逻辑：
/// 1. 获取 `source_link + target_chat_id` 的防并发穿透创建锁；
/// 2. 优先检查请求幂等性：同一 `(request_chat_id, request_message_id)` 不可重复创建；
/// 3. 检查历史成功结果复用：同源同目标且未强制重新转存时直接复用结果；
/// 4. 否则抓取源消息并创建新任务记录，进入运行态。
pub(super) async fn build_transfer_start(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    plan: TransferPlan,
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<TransferStart> {
    // 记录开始解析转存启动状态的调试日志
    tracing::debug!(
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        target_chat_id = plan.target_chat_id,
        "transfer start resolving"
    );
    // 获取源链接与目标会话的创建锁，防止并发相同请求同时穿透到数据库创建重复任务
    let _guard = acquire_source_target_create_guard(
        app_context.as_ref(),
        plan.source_link.clone(),
        plan.target_chat_id,
    )
    .await;
    // 记录成功获取创建互斥锁日志
    tracing::debug!(
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        target_chat_id = plan.target_chat_id,
        "transfer source-target create guard acquired"
    );

    // 请求级幂等必须最先判断。同一条命令 update 被 TDLib/网络重复投递时，
    // 无论当前任务处于什么状态，都不能创建第二个 job。
    if let Some(old) =
        store::find_job_by_request(plan.request_chat_id, plan.request_message_id).await?
    {
        // 记录命中请求级幂等的日志
        tracing::info!(
            job_id = old.id,
            status = %old.status,
            request_chat_id = plan.request_chat_id,
            request_message_id = plan.request_message_id,
            "matched idempotent transfer request"
        );
        // 根据已有任务的状态转换为下一步动作
        return request_job_start(old, client_ids.upload).await;
    }

    // 历史结果复用：
    // 同一个源链接转到同一个目标 chat，如果已经成功完成，直接返回历史结果。
    // 这里不看 request_message_id，因为不同命令重复转存同一链接时也应复用成功结果。
    if should_reuse_success(plan.force_retransfer)
        && let Some(old) =
            store::find_success_job_by_source_target(&plan.source_link, plan.target_chat_id).await?
    {
        // 刷新或确保存储的结果链接有效性
        let link = refresh_stored_result_link(
            old.id,
            old.target_chat_id,
            old.result_message_id,
            &old.result_message_link,
            client_ids.upload,
        )
        .await?;
        // 记录复用成功历史结果的日志
        tracing::info!(
            job_id = old.id,
            target_chat_id = plan.target_chat_id,
            "reuse successful transfer result"
        );
        // 返回复用历史结果的出口状态
        return Ok(TransferStart::Outcome(TransferOutcome::Reused {
            job_id: old.id,
            link,
        }));
    }

    // 无可复用结果时，开始抓取消息并创建全新转存任务
    create_new_job_start(app_context, plan, client_ids).await
}

/// 只有尚未得到用户明确确认（非 force_retransfer）时才复用历史成功结果。
fn should_reuse_success(force_retransfer: bool) -> bool {
    !force_retransfer
}

/// 将同一请求已存在的任务转换为下一步动作。
///
/// 状态分支判定：
/// - pending/running：恢复执行，避免上次后台任务已丢失；
/// - paused/cancelling/cancelled：返回相应终止/控制状态，不重新创建；
/// - 成功且有结果链接：刷新并返回复用链接；
/// - failed/partial 且无结果链接：报错，避免重复命令重试成新 job。
async fn request_job_start(
    old: db::transfer_job::Model,
    upload_client_id: i32,
) -> anyhow::Result<TransferStart> {
    // 同一条命令重复投递时按已有任务状态返回确定结果：
    // - pending/running：恢复执行，避免上次后台任务已丢失。
    // - paused/cancelling/cancelled：返回状态，不重新创建。
    // - success 且有结果链接：返回结果。
    // - failed/partial 且无结果链接：报错，不让同一条命令自动重试成新 job。
    if matches!(
        old.status.as_str(),
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
    ) {
        // 未完成任务返回恢复动作
        Ok(TransferStart::Resume(old))
    } else if old.status == store::JOB_STATUS_PAUSED {
        // 暂停状态返回暂停结果
        Ok(TransferStart::Outcome(TransferOutcome::Paused {
            job_id: old.id,
        }))
    } else if matches!(
        old.status.as_str(),
        store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING
    ) {
        // 取消中状态返回取消中结果
        Ok(TransferStart::Outcome(TransferOutcome::Cancelling {
            job_id: old.id,
        }))
    } else if old.status == store::JOB_STATUS_CANCELLED {
        // 已取消状态返回已取消结果
        Ok(TransferStart::Outcome(TransferOutcome::Cancelled {
            job_id: old.id,
        }))
    } else if let Some(link) = old.result_message_link.as_deref() {
        // 存在历史结果链接时，刷新链接并返回已复用结果
        let link = refresh_stored_result_link(
            old.id,
            old.target_chat_id,
            old.result_message_id,
            link,
            upload_client_id,
        )
        .await?;
        Ok(TransferStart::Outcome(TransferOutcome::Reused {
            job_id: old.id,
            link,
        }))
    } else {
        // 其他无可用结果的重复请求直接报错拒绝
        anyhow::bail!("duplicated request without reusable result link");
    }
}

/// 抓取源消息并创建新的转存任务。
///
/// 包含步骤：
/// 1. 爬取源消息（根据链接或 Bot 消息进行单条或相册解析，支持 Bot 优先与 User 回退）；
/// 2. 在数据库中持久化创建主转存任务记录；
/// 3. 获取任务级独占运行锁，避免并发重复运行或控制竞态；
/// 4. 检查创建期间是否有控制指令；无控制指令则写入子项并返回 Run 执行状态。
async fn create_new_job_start(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    plan: TransferPlan,
    client_ids: crate::config::TransferClientIds,
) -> anyhow::Result<TransferStart> {
    // 抓取源消息（单条或相册）。
    tracing::info!(
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        target_chat_id = plan.target_chat_id,
        "spider source messages started"
    );
    // 根据转存计划中的来源类型进行消息爬取
    let bundle = match plan.source_kind {
        SourceKind::Link => {
            // 源类型为链接
            if plan.preferred_source_client_role == crate::config::ClientRole::Bot {
                // 首选 Bot 客户端
                let bot_client_id = client_ids.get(crate::config::ClientRole::Bot)?;
                if plan.allow_user_fallback {
                    // 允许 Bot 失败后回退到 User 客户端抓取
                    let user_client_id = client_ids.get(crate::config::ClientRole::User)?;
                    spider::spider_link_bot_first(
                        plan.source_link.clone(),
                        bot_client_id,
                        user_client_id,
                    )
                    .await?
                } else {
                    // 仅使用 Bot 客户端抓取
                    spider::spider_message(
                        plan.source_link.clone(),
                        bot_client_id,
                        crate::config::ClientRole::Bot,
                    )
                    .await?
                }
            } else {
                // 首选其他客户端（如 User 客户端）
                let client_id = client_ids.get(plan.preferred_source_client_role)?;
                spider::spider_message(
                    plan.source_link.clone(),
                    client_id,
                    plan.preferred_source_client_role,
                )
                .await?
            }
        }
        SourceKind::BotMessage => {
            // 源类型为直接转发给 Bot 的消息
            let bot_client_id = client_ids.get(crate::config::ClientRole::Bot)?;
            let source_chat_id = plan
                .source_message_chat_id
                .ok_or_else(|| anyhow::anyhow!("bot message source chat_id missing"))?;
            let source_message_id = plan
                .source_message_id
                .ok_or_else(|| anyhow::anyhow!("bot message source message_id missing"))?;
            // 通过 Bot 可见消息解析器抓取单条或相册
            spider::spider_bot_visible_message(source_chat_id, source_message_id, bot_client_id)
                .await?
        }
    };
    // 记录爬取源消息完成日志
    tracing::info!(
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        source_chat_id = bundle.source_chat_id,
        source_message_id = bundle.source_message_id,
        source_album_id = bundle.source_album_id,
        message_count = bundle.messages.len(),
        "spider source messages completed"
    );

    // 创建主任务并对齐子项；创建完成后释放 source-target 锁，实际执行由 job_id 锁保护。
    let job = store::create_job(&plan, &bundle).await?;
    tracing::info!(
        job_id = job.id,
        request_chat_id = plan.request_chat_id,
        request_message_id = plan.request_message_id,
        source_chat_id = bundle.source_chat_id,
        source_message_id = bundle.source_message_id,
        source_album_id = bundle.source_album_id,
        target_chat_id = plan.target_chat_id,
        total_items = bundle.messages.len(),
        "created transfer job"
    );
    // 新任务从创建子项前就持有 job 锁，避免 `/j stop` 在子项写入前误判“无执行器”。
    match acquire_job_guard(app_context.as_ref(), job.id).await {
        Some(job_guard) => {
            // 成功获得任务执行锁，检查在建表期间是否已收到停止/暂停控制指令
            if let Some(outcome) = apply_job_control(app_context.as_ref(), job.id).await? {
                tracing::info!(
                    job_id = job.id,
                    "transfer job control applied before item creation"
                );
                // 收到控制指令，直接返回控制出口
                Ok(TransferStart::Outcome(outcome))
            } else {
                // 确保并初始化所有媒体子项记录
                let _ = store::ensure_items_for_bundle(job.id, &bundle).await?;
                tracing::debug!(
                    job_id = job.id,
                    total_items = bundle.messages.len(),
                    "transfer job items ensured"
                );
                // 返回可开始运行的任务、消息列表与执行守卫
                Ok(TransferStart::Run(job, bundle.messages, job_guard))
            }
        }
        None => {
            // 该任务已由其他并发分支持有运行守卫
            tracing::info!(
                job_id = job.id,
                "transfer job guard already held after creation"
            );
            Ok(TransferStart::Outcome(TransferOutcome::Running {
                job_id: job.id,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::{ClientRole, RequestActor, TransferClientIds};
    use crate::db;
    use rand::RngExt;
    use sea_orm::ActiveModelTrait;
    use sea_orm::EntityTrait;

    /// 获取全局测试应用上下文
    fn test_app_context() -> std::sync::Arc<crate::app_context::AppContext> {
        crate::app_context::app_context()
    }

    /// 测试同源同目标的成功任务应优先复用历史结果，不再重新 spider。
    #[tokio::test]
    async fn test_build_transfer_start_reuses_success_job_without_spider() -> anyhow::Result<()> {
        let _guard = db::TEST_DB_LOCK.lock().await;
        let db_conn = prepare_test_schema().await?;
        let source_link = unique_source_link();
        let request_chat_id = unique_id();
        let owner_user_id = unique_id();
        let target_chat_id = unique_id();
        // 预先插入一条已成功的历史转存任务
        let job = insert_job(
            store::JOB_STATUS_SUCCESS,
            owner_user_id,
            request_chat_id + 1,
            request_chat_id + 2,
            &source_link,
            target_chat_id,
            Some("https://example.com/result/1"),
        )
        .await?;

        // 构造具有相同 source_link 和 target_chat_id 的新请求
        let plan = build_test_plan(
            request_chat_id,
            owner_user_id,
            source_link.clone(),
            target_chat_id,
            request_chat_id,
            request_chat_id + 10,
        );
        let client_ids = test_client_ids();
        let app_context = test_app_context();

        // 执行转存启动决策
        let start = build_transfer_start(app_context, plan, client_ids).await?;

        // 验证命中历史复用分支并返回对应的任务 ID 与链接
        match start {
            TransferStart::Outcome(TransferOutcome::Reused { job_id, link }) => {
                assert_eq!(job_id, job.id);
                assert_eq!(link, "https://example.com/result/1");
            }
            _ => panic!("unexpected transfer start"),
        }

        // 验证原任务状态未被篡改
        let job = db::transfer_job::Entity::find_by_id(job.id)
            .one(db_conn)
            .await?
            .expect("job must exist");
        assert_eq!(job.status, store::JOB_STATUS_SUCCESS);
        Ok(())
    }

    /// 测试强制重新转存开关会跳过历史成功结果复用
    #[test]
    fn test_force_retransfer_skips_success_reuse() {
        assert!(should_reuse_success(false));
        assert!(!should_reuse_success(true));
    }

    /// 测试同源同目标但请求消息不同的运行中任务必须允许并存。
    #[tokio::test]
    async fn test_distinct_requests_allow_duplicate_active_source_target() -> anyhow::Result<()> {
        let _guard = db::TEST_DB_LOCK.lock().await;
        let source_link = unique_source_link();
        let request_chat_id = unique_id();
        let owner_user_id = unique_id();
        let target_chat_id = unique_id();
        // 插入第一个运行中任务
        let first = insert_job(
            store::JOB_STATUS_RUNNING,
            owner_user_id,
            request_chat_id + 1,
            request_chat_id + 2,
            &source_link,
            target_chat_id,
            None,
        )
        .await?;

        // 插入第二个运行中任务（不同请求消息）
        let second = insert_job(
            store::JOB_STATUS_RUNNING,
            owner_user_id,
            request_chat_id,
            request_chat_id + 10,
            &source_link,
            target_chat_id,
            None,
        )
        .await?;

        // 两个任务 ID 应该互不相同且共存
        assert_ne!(first.id, second.id);
        assert_eq!(first.source_link, second.source_link);
        assert_eq!(first.target_chat_id, second.target_chat_id);
        assert_ne!(first.request_message_id, second.request_message_id);
        Ok(())
    }

    /// 测试同一条请求若已经进入取消态，重复投递应走 request 幂等分支，而不是重新建任务。
    #[tokio::test]
    async fn test_build_transfer_start_returns_cancelled_for_same_request_job() -> anyhow::Result<()>
    {
        let _guard = db::TEST_DB_LOCK.lock().await;
        let source_link = unique_source_link();
        let request_chat_id = unique_id();
        let owner_user_id = unique_id();
        let target_chat_id = unique_id();
        // 插入已取消的任务
        let job = insert_job(
            store::JOB_STATUS_CANCELLED,
            owner_user_id,
            request_chat_id,
            request_chat_id + 1,
            &source_link,
            target_chat_id,
            None,
        )
        .await?;

        // 构造相同请求参数
        let plan = build_test_plan(
            request_chat_id,
            owner_user_id,
            source_link,
            target_chat_id,
            request_chat_id,
            request_chat_id + 1,
        );
        let client_ids = test_client_ids();
        let app_context = test_app_context();

        let start = build_transfer_start(app_context, plan, client_ids).await?;

        // 验证返回已取消的直接结果，而非创建新任务
        match start {
            TransferStart::Outcome(TransferOutcome::Cancelled { job_id }) => {
                assert_eq!(job_id, job.id);
            }
            _ => panic!("unexpected transfer start"),
        }
        Ok(())
    }

    /// 准备测试数据库 schema
    async fn prepare_test_schema() -> anyhow::Result<&'static sea_orm::DatabaseConnection> {
        let db_conn = db::get_db().await?;
        db::ensure_test_schema_current(db_conn).await?;
        Ok(db_conn)
    }

    /// 插入测试用 transfer_job 记录
    async fn insert_job(
        status: &str,
        owner_user_id: i64,
        request_chat_id: i64,
        request_message_id: i64,
        source_link: &str,
        target_chat_id: i64,
        result_message_link: Option<&str>,
    ) -> anyhow::Result<db::transfer_job::Model> {
        let db_conn = prepare_test_schema().await?;
        let now = store::now_utc8();
        let mut active = db::transfer_job::ActiveModel {
            request_chat_id: sea_orm::ActiveValue::Set(request_chat_id),
            request_message_id: sea_orm::ActiveValue::Set(request_message_id),
            owner_user_id: sea_orm::ActiveValue::Set(owner_user_id),
            source_link: sea_orm::ActiveValue::Set(source_link.to_owned()),
            source_kind: sea_orm::ActiveValue::Set("link".to_owned()),
            source_client_role: sea_orm::ActiveValue::Set("user".to_owned()),
            allow_user_fallback: sea_orm::ActiveValue::Set(false),
            source_chat_id: sea_orm::ActiveValue::Set(unique_id()),
            source_message_id: sea_orm::ActiveValue::Set(unique_id()),
            source_album_id: sea_orm::ActiveValue::Set(0),
            target_chat_id: sea_orm::ActiveValue::Set(target_chat_id),
            result_message_id: sea_orm::ActiveValue::Set(result_message_link.map(|_| unique_id())),
            result_message_link: sea_orm::ActiveValue::Set(
                result_message_link.map(|s| s.to_owned()),
            ),
            status: sea_orm::ActiveValue::Set(status.to_owned()),
            total_items: sea_orm::ActiveValue::Set(1),
            done_items: sea_orm::ActiveValue::Set(0),
            failed_items: sea_orm::ActiveValue::Set(0),
            retry_count: sea_orm::ActiveValue::Set(0),
            last_error: sea_orm::ActiveValue::Set(None),
            created_at: sea_orm::ActiveValue::Set(now),
            updated_at: sea_orm::ActiveValue::Set(now),
            finished_at: sea_orm::ActiveValue::Set(
                if matches!(
                    status,
                    store::JOB_STATUS_SUCCESS | store::JOB_STATUS_CANCELLED
                ) {
                    Some(now)
                } else {
                    None
                },
            ),
            ..Default::default()
        };
        if result_message_link.is_none() {
            active.result_message_id = sea_orm::ActiveValue::Set(None);
        }

        active.insert(db_conn).await.map_err(Into::into)
    }

    /// 构造测试用转存计划
    fn build_test_plan(
        request_chat_id: i64,
        owner_user_id: i64,
        source_link: String,
        target_chat_id: i64,
        plan_request_chat_id: i64,
        plan_request_message_id: i64,
    ) -> TransferPlan {
        TransferPlan {
            actor: RequestActor {
                request_chat_id,
                user_id: owner_user_id,
            },
            source_link,
            source_kind: SourceKind::Link,
            preferred_source_client_role: ClientRole::Bot,
            allow_user_fallback: false,
            source_message_chat_id: None,
            source_message_id: None,
            target_chat_id,
            request_chat_id: plan_request_chat_id,
            request_message_id: plan_request_message_id,
            force_retransfer: false,
        }
    }

    /// 构造测试用客户端分配映射
    fn test_client_ids() -> TransferClientIds {
        TransferClientIds {
            interaction: 1,
            download: 2,
            upload: 3,
            user: Some(4),
            bot: Some(5),
        }
    }

    /// 生成随机测试 ID
    fn unique_id() -> i64 {
        rand::rng().random_range(1_000_000..=9_999_999)
    }

    /// 生成唯一测试源链接
    fn unique_source_link() -> String {
        format!("https://t.me/c/{}/{}", unique_id(), unique_id())
    }
}
