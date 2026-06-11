// 用户账号和积分账本访问逻辑。
// 这里集中处理余额变更事务，避免命令层直接读写积分导致并发扣费不一致。

use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, TransactionTrait,
};

use crate::config::ActorRole;
use crate::db;

use super::now_utc8;

/// 用户账号展示信息。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct UserAccountSnapshot {
    /// Telegram 用户 ID。
    pub telegram_user_id: i64,
    /// 业务角色。
    pub role: String,
    /// 当前可用积分。
    pub points_balance: i64,
    /// 累计增加积分。
    pub total_points_added: i64,
    /// 累计消费积分。
    pub total_points_spent: i64,
}

/// 积分变更结果。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct PointsChangeResult {
    /// 变更后的账号快照。
    pub account: UserAccountSnapshot,
    /// 是否命中已有账本幂等记录。
    pub idempotent_replay: bool,
}

/// 单条积分流水展示字段。
///
/// 命令层只需要这些轻量字段，不读取 point_ledger 整行可以降低列表查询成本。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct PointLedgerEntry {
    /// 自增流水 ID。
    pub id: i64,
    /// 积分变化量，正数为增加，负数为扣减。
    pub delta: i64,
    /// 变更后的余额快照。
    pub balance_after: i64,
    /// 变更原因。
    pub reason: String,
    /// 关联任务 ID。
    pub job_id: Option<i64>,
    /// 关联请求 chat。
    pub request_chat_id: Option<i64>,
    /// 关联请求消息。
    pub request_message_id: Option<i64>,
    /// 操作者用户 ID。
    pub created_by: Option<i64>,
    /// 发生时间，统一使用 UTC+8。
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

/// 积分流水分页结果。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer) struct PointLedgerPage {
    /// 被查询的 Telegram 用户 ID。
    pub telegram_user_id: i64,
    /// 当前页记录。
    pub entries: Vec<PointLedgerEntry>,
    /// 总记录数。
    pub total: u64,
    /// 每页条数。
    pub limit: u64,
    /// 当前页码。
    pub page: u64,
    /// 总页数；无记录时固定为 1，方便按钮和文案展示。
    pub total_pages: u64,
}

/// 积分变更参数。
pub(in crate::tgbot::transfer) struct PointsChange {
    /// 需要变更的用户。
    pub telegram_user_id: i64,
    /// 积分变化量；扣费传负数。
    pub delta: i64,
    /// 变更原因。
    pub reason: String,
    /// 关联任务 ID。
    pub job_id: Option<i64>,
    /// 关联请求 chat。
    pub request_chat_id: Option<i64>,
    /// 关联请求消息。
    pub request_message_id: Option<i64>,
    /// 幂等键；扣费必须传，admin 手动调整可不传。
    pub idempotency_key: Option<String>,
    /// 操作者 ID。
    pub created_by: Option<i64>,
}

/// 确保用户账号存在，并按配置角色同步 role。
pub(in crate::tgbot::transfer) async fn ensure_user_account(
    telegram_user_id: i64,
    role: ActorRole,
    initial_points: i64,
) -> anyhow::Result<UserAccountSnapshot> {
    let db_conn = db::get_db().await?;
    let now = now_utc8();
    let role_text = role.as_str().to_owned();
    let initial_points = if role.is_admin() {
        0
    } else {
        std::cmp::max(initial_points, 0)
    };

    if let Some(account) = db::user_account::Entity::find_by_id(telegram_user_id)
        .one(db_conn)
        .await?
    {
        return sync_account_role(account, &role_text, now).await;
    }

    let insert_result = db::user_account::Entity::insert(db::user_account::ActiveModel {
        telegram_user_id: sea_orm::ActiveValue::Set(telegram_user_id),
        role: sea_orm::ActiveValue::Set(role_text.clone()),
        points_balance: sea_orm::ActiveValue::Set(initial_points),
        total_points_added: sea_orm::ActiveValue::Set(initial_points),
        total_points_spent: sea_orm::ActiveValue::Set(0),
        created_at: sea_orm::ActiveValue::Set(now),
        updated_at: sea_orm::ActiveValue::Set(now),
    })
    .exec(db_conn)
    .await;

    if let Err(err) = insert_result
        && !is_unique_constraint_error(&err)
    {
        return Err(err.into());
    }

    let account = db::user_account::Entity::find_by_id(telegram_user_id)
        .one(db_conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user account not found after ensure"))?;
    sync_account_role(account, &role_text, now).await
}

/// 确保账号 role 与最新权限配置一致。
///
/// 管理员和普通用户可能在配置里切换身份，账号存在时只同步 role，不重复发放初始积分。
async fn sync_account_role(
    mut account: db::user_account::Model,
    role_text: &str,
    now: chrono::DateTime<chrono::FixedOffset>,
) -> anyhow::Result<UserAccountSnapshot> {
    if account.role != role_text {
        let db_conn = db::get_db().await?;
        let mut active: db::user_account::ActiveModel = account.clone().into();
        active.role = sea_orm::ActiveValue::Set(role_text.to_owned());
        active.updated_at = sea_orm::ActiveValue::Set(now);
        account = active.update(db_conn).await?;
    }
    Ok(snapshot(account))
}

/// 查询用户账号；不存在时返回 None。
pub(in crate::tgbot::transfer) async fn get_user_account(
    telegram_user_id: i64,
) -> anyhow::Result<Option<UserAccountSnapshot>> {
    let db_conn = db::get_db().await?;
    Ok(db::user_account::Entity::find_by_id(telegram_user_id)
        .one(db_conn)
        .await?
        .map(snapshot))
}

/// 分页查询某个用户的积分流水。
pub(in crate::tgbot::transfer) async fn list_point_ledger_page(
    telegram_user_id: i64,
    limit: u64,
    page: u64,
) -> anyhow::Result<PointLedgerPage> {
    let db_conn = db::get_db().await?;
    let limit = limit.clamp(1, 50);
    let total = db::point_ledger::Entity::find()
        .filter(db::point_ledger::Column::TelegramUserId.eq(telegram_user_id))
        .count(db_conn)
        .await?;
    let total_pages = std::cmp::max(total.div_ceil(limit), 1);
    let page = page.clamp(1, total_pages);
    let offset = page.saturating_sub(1).saturating_mul(limit);

    let rows = db::point_ledger::Entity::find()
        .select_only()
        .column(db::point_ledger::Column::Id)
        .column(db::point_ledger::Column::Delta)
        .column(db::point_ledger::Column::BalanceAfter)
        .column(db::point_ledger::Column::Reason)
        .column(db::point_ledger::Column::JobId)
        .column(db::point_ledger::Column::RequestChatId)
        .column(db::point_ledger::Column::RequestMessageId)
        .column(db::point_ledger::Column::CreatedBy)
        .column(db::point_ledger::Column::CreatedAt)
        .filter(db::point_ledger::Column::TelegramUserId.eq(telegram_user_id))
        .order_by_desc(db::point_ledger::Column::CreatedAt)
        .order_by_desc(db::point_ledger::Column::Id)
        .limit(limit)
        .offset(offset)
        .into_tuple::<(
            i64,
            i64,
            i64,
            String,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            chrono::DateTime<chrono::FixedOffset>,
        )>()
        .all(db_conn)
        .await?;

    Ok(PointLedgerPage {
        telegram_user_id,
        entries: rows
            .into_iter()
            .map(
                |(
                    id,
                    delta,
                    balance_after,
                    reason,
                    job_id,
                    request_chat_id,
                    request_message_id,
                    created_by,
                    created_at,
                )| PointLedgerEntry {
                    id,
                    delta,
                    balance_after,
                    reason,
                    job_id,
                    request_chat_id,
                    request_message_id,
                    created_by,
                    created_at,
                },
            )
            .collect(),
        total,
        limit,
        page,
        total_pages,
    })
}

/// 在事务中变更积分并写入账本。
pub(in crate::tgbot::transfer) async fn change_points(
    change: PointsChange,
) -> anyhow::Result<PointsChangeResult> {
    if change.delta == i64::MIN {
        anyhow::bail!("points delta is too small");
    }

    let db_conn = db::get_db().await?;
    let txn = db_conn.begin().await?;
    let idempotency_key = change.idempotency_key.clone();

    if let Some(idempotency_key) = change.idempotency_key.as_deref()
        && let Some(_existing) = db::point_ledger::Entity::find()
            .filter(db::point_ledger::Column::IdempotencyKey.eq(idempotency_key.to_owned()))
            .one(&txn)
            .await?
    {
        let account = db::user_account::Entity::find_by_id(change.telegram_user_id)
            .one(&txn)
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("user account not found: {}", change.telegram_user_id)
            })?;
        txn.commit().await?;
        return Ok(PointsChangeResult {
            account: snapshot(account),
            idempotent_replay: true,
        });
    }

    let now = now_utc8();
    let mut update = db::user_account::Entity::update_many()
        .col_expr(
            db::user_account::Column::PointsBalance,
            Expr::col(db::user_account::Column::PointsBalance).add(change.delta),
        )
        .col_expr(db::user_account::Column::UpdatedAt, Expr::value(now))
        .filter(db::user_account::Column::TelegramUserId.eq(change.telegram_user_id));
    if change.delta > 0 {
        update = update.col_expr(
            db::user_account::Column::TotalPointsAdded,
            Expr::col(db::user_account::Column::TotalPointsAdded).add(change.delta),
        );
    } else if change.delta < 0 {
        // 扣费用数据库条件保护余额，避免两个并发请求读到同一旧余额后覆盖更新。
        update = update
            .col_expr(
                db::user_account::Column::TotalPointsSpent,
                Expr::col(db::user_account::Column::TotalPointsSpent).add(-change.delta),
            )
            .filter(db::user_account::Column::PointsBalance.gte(-change.delta));
    }
    let updated = update.exec(&txn).await?;
    if updated.rows_affected == 0 {
        let account = db::user_account::Entity::find_by_id(change.telegram_user_id)
            .one(&txn)
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("user account not found: {}", change.telegram_user_id)
            })?;
        if change.delta < 0 {
            anyhow::bail!(
                "insufficient points: user={}, balance={}, required={}",
                change.telegram_user_id,
                account.points_balance,
                -change.delta
            );
        }
        anyhow::bail!(
            "points update affected no rows: {}",
            change.telegram_user_id
        );
    }

    let account = db::user_account::Entity::find_by_id(change.telegram_user_id)
        .one(&txn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user account not found: {}", change.telegram_user_id))?;

    let ledger_insert = db::point_ledger::ActiveModel {
        telegram_user_id: sea_orm::ActiveValue::Set(change.telegram_user_id),
        delta: sea_orm::ActiveValue::Set(change.delta),
        balance_after: sea_orm::ActiveValue::Set(account.points_balance),
        reason: sea_orm::ActiveValue::Set(change.reason),
        job_id: sea_orm::ActiveValue::Set(change.job_id),
        request_chat_id: sea_orm::ActiveValue::Set(change.request_chat_id),
        request_message_id: sea_orm::ActiveValue::Set(change.request_message_id),
        idempotency_key: sea_orm::ActiveValue::Set(idempotency_key.clone()),
        created_by: sea_orm::ActiveValue::Set(change.created_by),
        created_at: sea_orm::ActiveValue::Set(now),
        ..Default::default()
    }
    .insert(&txn)
    .await;

    if let Err(err) = ledger_insert {
        let is_idempotent_conflict = idempotency_key.is_some() && is_unique_constraint_error(&err);
        txn.rollback().await?;
        if is_idempotent_conflict {
            let account = db::user_account::Entity::find_by_id(change.telegram_user_id)
                .one(db_conn)
                .await?
                .ok_or_else(|| {
                    anyhow::anyhow!("user account not found: {}", change.telegram_user_id)
                })?;
            return Ok(PointsChangeResult {
                account: snapshot(account),
                idempotent_replay: true,
            });
        }
        return Err(err.into());
    }

    txn.commit().await?;
    Ok(PointsChangeResult {
        account: snapshot(account),
        idempotent_replay: false,
    })
}

/// 按任务幂等退回已扣积分。
///
/// 退款只处理 `billing_status = charged` 且 `refund_points > 0` 的普通用户任务。
/// 同一事务里先把任务计费状态切到 `refunded`，再返还余额并写入账本；
/// 如果另一个流程已经退款，状态切换会影响 0 行，本函数直接返回 false。
pub(in crate::tgbot::transfer) async fn refund_job_points_on_conn<C>(
    db_conn: &C,
    job: &db::transfer_job::Model,
    refund_points: i64,
    reason: &str,
) -> anyhow::Result<bool>
where
    C: ConnectionTrait,
{
    let refund_points = refund_points.clamp(0, job.charged_points);
    if refund_points <= 0 || job.billing_status != "charged" {
        return Ok(false);
    }

    let now = now_utc8();
    let rs = db::transfer_job::Entity::update_many()
        .col_expr(
            db::transfer_job::Column::BillingStatus,
            Expr::value("refunded"),
        )
        .col_expr(db::transfer_job::Column::UpdatedAt, Expr::value(now))
        .filter(db::transfer_job::Column::Id.eq(job.id))
        .filter(db::transfer_job::Column::BillingStatus.eq("charged"))
        .filter(db::transfer_job::Column::ChargedPoints.gt(0))
        .exec(db_conn)
        .await?;
    if rs.rows_affected == 0 {
        return Ok(false);
    }

    let account_update = db::user_account::Entity::update_many()
        .col_expr(
            db::user_account::Column::PointsBalance,
            Expr::col(db::user_account::Column::PointsBalance).add(refund_points),
        )
        .col_expr(
            db::user_account::Column::TotalPointsSpent,
            Expr::col(db::user_account::Column::TotalPointsSpent).sub(refund_points),
        )
        .col_expr(db::user_account::Column::UpdatedAt, Expr::value(now))
        .filter(db::user_account::Column::TelegramUserId.eq(job.owner_user_id))
        .filter(db::user_account::Column::TotalPointsSpent.gte(refund_points))
        .exec(db_conn)
        .await?;
    if account_update.rows_affected == 0 {
        anyhow::bail!(
            "points refund account update affected no rows: job_id={}, owner_user_id={}",
            job.id,
            job.owner_user_id
        );
    }

    let account = db::user_account::Entity::find_by_id(job.owner_user_id)
        .one(db_conn)
        .await?
        .ok_or_else(|| {
            anyhow::anyhow!("user account not found for refund: {}", job.owner_user_id)
        })?;

    db::point_ledger::ActiveModel {
        telegram_user_id: sea_orm::ActiveValue::Set(job.owner_user_id),
        delta: sea_orm::ActiveValue::Set(refund_points),
        balance_after: sea_orm::ActiveValue::Set(account.points_balance),
        reason: sea_orm::ActiveValue::Set(reason.to_owned()),
        job_id: sea_orm::ActiveValue::Set(Some(job.id)),
        request_chat_id: sea_orm::ActiveValue::Set(Some(job.request_chat_id)),
        request_message_id: sea_orm::ActiveValue::Set(Some(job.request_message_id)),
        idempotency_key: sea_orm::ActiveValue::Set(Some(format!("refund:job:{}", job.id))),
        created_by: sea_orm::ActiveValue::Set(None),
        created_at: sea_orm::ActiveValue::Set(now),
        ..Default::default()
    }
    .insert(db_conn)
    .await?;

    tracing::info!(
        job_id = job.id,
        owner_user_id = job.owner_user_id,
        refunded_points = refund_points,
        balance_after = account.points_balance,
        reason,
        "transfer points refunded"
    );

    Ok(true)
}

/// 失败或取消时按任务全额退款。
pub(in crate::tgbot::transfer) async fn refund_job_points_if_needed_on_conn<C>(
    db_conn: &C,
    job: &db::transfer_job::Model,
    reason: &str,
) -> anyhow::Result<bool>
where
    C: ConnectionTrait,
{
    refund_job_points_on_conn(db_conn, job, job.charged_points, reason).await
}

/// 部分成功时按失败条目占比退还已扣积分。
///
/// 规则：
/// - 没有失败或没有有效总数时不退。
/// - 按 `charged_points * failed_items / total_items` 向下取整。
/// - 只要存在失败且本次任务确实扣费，至少退 1 分。
pub(in crate::tgbot::transfer) fn partial_refund_points(
    charged_points: i64,
    total_items: i32,
    failed_items: i32,
) -> i64 {
    if charged_points <= 0 || total_items <= 0 || failed_items <= 0 {
        return 0;
    }
    let total_items = i64::from(total_items);
    let failed_items = i64::from(failed_items).clamp(0, total_items);
    let proportional = charged_points.saturating_mul(failed_items) / total_items;
    std::cmp::min(std::cmp::max(proportional, 1), charged_points)
}

/// 判断账本插入是否撞到唯一约束。
///
/// SeaORM 不同后端的错误枚举不完全一致，这里只在已有 idempotency_key 时兜底识别唯一冲突；
/// 识别成功后把本次事务回滚，并按“幂等重放”读取已提交余额。
fn is_unique_constraint_error(err: &sea_orm::DbErr) -> bool {
    let text = err.to_string().to_ascii_lowercase();
    text.contains("unique") || text.contains("duplicate")
}

/// 把实体转换为命令层展示快照。
fn snapshot(account: db::user_account::Model) -> UserAccountSnapshot {
    UserAccountSnapshot {
        telegram_user_id: account.telegram_user_id,
        role: account.role,
        points_balance: account.points_balance,
        total_points_added: account.total_points_added,
        total_points_spent: account.total_points_spent,
    }
}
