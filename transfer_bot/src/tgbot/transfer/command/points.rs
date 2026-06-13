// `/balance` 与 `/points` 命令：
// - `/balance` 查询当前用户积分。
// - `/points` 仅 admin 可用，用于给普通用户增减积分或查看余额。

use crate::tgbot::send;
use crate::tgbot::transfer::store;

mod callback;
mod render;

pub(in crate::tgbot::transfer::command) use callback::points_callback_query;
use render::{format_balance_text, format_ledger_page_text, ledger_button_rows};

#[cfg(test)]
use render::balance_history_command;

/// 积分流水 callback 前缀。
const POINTS_CALLBACK_PREFIX: &str = "pt:";
/// 积分流水默认每页条数。
const DEFAULT_LEDGER_LIMIT: u64 = 10;
/// 积分流水单页最大条数，避免一次回复过长。
const MAX_LEDGER_LIMIT: u64 = 50;

/// 判断 callback payload 是否属于积分模块。
pub(super) fn is_points_callback_data(data: &str) -> bool {
    data.starts_with(POINTS_CALLBACK_PREFIX)
}

/// `/balance` 命令入口。
pub async fn balance_command(
    text: Vec<&str>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    if is_history_action(text.get(1).copied()) {
        let args = parse_balance_history_args(&text)?;
        let page = store::list_point_ledger_page(actor.user_id, args.limit, args.page).await?;
        return send::ReplyPanel::card(format_ledger_page_text(&page, false))
            .rows(ledger_button_rows(
                LedgerCommandKind::Balance,
                actor.user_id,
                &page,
                false,
            ))
            .send(actor.request_chat_id, client_id)
            .await;
    }

    let account = store::get_user_account(actor.user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user account not found: {}", actor.user_id))?;
    send::ReplyPanel::card(format_balance_text(&account))
        .rows(vec![
            vec![
                send::build_copy_button(
                    "复制 /balance",
                    "/balance",
                    tdlib_rs::enums::ButtonStyle::Primary,
                ),
                send::build_copy_button(
                    "复制流水",
                    "/balance history",
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ],
            vec![send::build_copy_button(
                "复制短命令流水",
                "/bal h",
                tdlib_rs::enums::ButtonStyle::Default,
            )],
        ])
        .send(actor.request_chat_id, client_id)
        .await
}

/// admin `/points` 命令入口。
///
/// 支持：
/// - `/points show <user_id>`
/// - `/points history <user_id> [limit] [page]`
/// - `/points add <user_id> <amount> [reason]`
/// - `/points sub <user_id> <amount> [reason]`
pub async fn points_command(
    text: Vec<&str>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    if !actor.is_admin() {
        anyhow::bail!("permission denied: points command requires admin");
    }

    let action = text
        .get(1)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("usage: /points <show|add|sub> <user_id> [amount]"))?;
    let user_id = text
        .get(2)
        .ok_or_else(|| anyhow::anyhow!("usage: /points <show|history|add|sub> <user_id> [amount]"))?
        .parse::<i64>()?;
    if is_history_action(Some(action)) {
        let args = parse_points_history_args(&text)?;
        let page = store::list_point_ledger_page(user_id, args.limit, args.page).await?;
        return send::ReplyPanel::card(format_ledger_page_text(&page, true))
            .rows(ledger_button_rows(
                LedgerCommandKind::Points,
                user_id,
                &page,
                true,
            ))
            .send(actor.request_chat_id, client_id)
            .await;
    }

    let account = match action {
        "show" | "s" => store::get_user_account(user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user account not found: {}", user_id))?,
        "add" | "a" | "sub" => {
            let amount = text
                .get(3)
                .ok_or_else(|| anyhow::anyhow!("usage: /points {} <user_id> <amount>", action))?
                .parse::<i64>()?;
            if amount <= 0 {
                anyhow::bail!("amount must be positive");
            }
            store::ensure_user_account(user_id, crate::config::ActorRole::User, 0).await?;
            let delta = if action == "sub" { -amount } else { amount };
            let reason = text.get(4).copied().unwrap_or("admin_adjust").to_owned();
            let changed = store::change_points(store::PointsChange {
                telegram_user_id: user_id,
                delta,
                reason,
                job_id: None,
                request_chat_id: Some(actor.request_chat_id),
                request_message_id: None,
                idempotency_key: None,
                created_by: Some(actor.user_id),
            })
            .await?;
            tracing::info!(
                admin_user_id = actor.user_id,
                target_user_id = user_id,
                delta,
                balance_after = changed.account.points_balance,
                "points adjusted by admin command"
            );
            changed.account
        }
        other => anyhow::bail!("unknown points action: {}", other),
    };

    send::ReplyPanel::card(format_balance_text(&account))
        .row(vec![
            send::build_copy_button(
                "复制查看",
                &format!("/points show {}", user_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制流水",
                &format!("/points history {}", user_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .row(vec![
            send::build_copy_button(
                "复制加分",
                &format!("/points add {} 10 admin_adjust", user_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制扣分",
                &format!("/points sub {} 10 admin_adjust", user_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .send(actor.request_chat_id, client_id)
        .await
}

/// 积分余额卡片正文。
/// 积分流水命令类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LedgerCommandKind {
    Balance,
    Points,
}

/// 积分流水 callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LedgerCallbackAction {
    Page,
    Refresh,
}

/// 积分流水分页参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LedgerArgs {
    limit: u64,
    page: u64,
}

/// 积分流水 callback 请求。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LedgerCallbackRequest {
    kind: LedgerCommandKind,
    user_id: i64,
    limit: u64,
    page: u64,
}

/// 判断是否为流水查询动作。
fn is_history_action(action: Option<&str>) -> bool {
    matches!(action, Some("history" | "hist" | "h" | "ledger" | "l"))
}

/// 解析 `/balance history [limit] [page]`。
fn parse_balance_history_args(text: &[&str]) -> anyhow::Result<LedgerArgs> {
    parse_ledger_args(text, 2)
}

/// 解析 `/points history <user_id> [limit] [page]`。
fn parse_points_history_args(text: &[&str]) -> anyhow::Result<LedgerArgs> {
    parse_ledger_args(text, 3)
}

/// 解析通用流水分页参数。
fn parse_ledger_args(text: &[&str], offset: usize) -> anyhow::Result<LedgerArgs> {
    let limit = text
        .get(offset)
        .map(|value| value.parse::<u64>())
        .transpose()?
        .unwrap_or(DEFAULT_LEDGER_LIMIT)
        .clamp(1, MAX_LEDGER_LIMIT);
    let page = text
        .get(offset + 1)
        .map(|value| value.parse::<u64>())
        .transpose()?
        .unwrap_or(1)
        .max(1);
    Ok(LedgerArgs { limit, page })
}

/// 积分流水 callback 解析。
fn parse_points_callback_data(data: &str) -> Option<(LedgerCallbackAction, LedgerCallbackRequest)> {
    let payload = data.strip_prefix(POINTS_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    let action = match parts.next()? {
        "p" => LedgerCallbackAction::Page,
        "r" => LedgerCallbackAction::Refresh,
        _ => return None,
    };
    let kind = match parts.next()? {
        "b" => LedgerCommandKind::Balance,
        "pts" => LedgerCommandKind::Points,
        _ => return None,
    };
    let user_id = parts.next()?.parse::<i64>().ok()?;
    let limit = parts
        .next()?
        .parse::<u64>()
        .ok()?
        .clamp(1, MAX_LEDGER_LIMIT);
    let page = parts.next()?.parse::<u64>().ok()?.max(1);
    if parts.next().is_some() {
        return None;
    }
    Some((
        action,
        LedgerCallbackRequest {
            kind,
            user_id,
            limit,
            page,
        },
    ))
}

/// 构造积分流水分页 callback。
fn build_ledger_callback_data(
    action: LedgerCallbackAction,
    kind: LedgerCommandKind,
    user_id: i64,
    limit: u64,
    page: u64,
) -> String {
    let action = match action {
        LedgerCallbackAction::Page => "p",
        LedgerCallbackAction::Refresh => "r",
    };
    let kind = match kind {
        LedgerCommandKind::Balance => "b",
        LedgerCommandKind::Points => "pts",
    };
    format!(
        "{}{}:{}:{}:{}:{}",
        POINTS_CALLBACK_PREFIX,
        action,
        kind,
        user_id,
        limit.clamp(1, MAX_LEDGER_LIMIT),
        page.max(1)
    )
}

#[cfg(test)]
mod tests {
    use super::super::common::short_and_long;
    use super::*;

    // 余额卡片必须展示当前余额和累计消费，便于用户判断是否还能继续转存。
    #[test]
    fn test_format_balance_text() {
        let text = format_balance_text(&store::UserAccountSnapshot {
            telegram_user_id: 1,
            role: "user".to_owned(),
            points_balance: 10,
            total_points_added: 20,
            total_points_spent: 10,
        });

        assert!(text.contains("积分账户"));
        assert!(text.contains("用户：‹1›"));
        assert!(text.contains("余额：‹10›"));
        assert!(text.contains("累计消费：‹10›"));
        assert!(text.contains("‹/balance history›"));
    }

    // 积分流水参数应兼容长命令和短命令，并限制单页最大数量。
    #[test]
    fn test_parse_ledger_args() {
        assert_eq!(
            parse_balance_history_args(&["/balance", "history"]).unwrap(),
            LedgerArgs {
                limit: DEFAULT_LEDGER_LIMIT,
                page: 1
            }
        );
        assert_eq!(
            parse_balance_history_args(&["/bal", "h", "99", "2"]).unwrap(),
            LedgerArgs {
                limit: MAX_LEDGER_LIMIT,
                page: 2
            }
        );
        assert_eq!(
            parse_points_history_args(&["/points", "history", "123", "5", "3"]).unwrap(),
            LedgerArgs { limit: 5, page: 3 }
        );
    }

    // 流水卡片应展示分页、正负变化、原因和任务定位。
    #[test]
    fn test_format_ledger_page_text() {
        let page = store::PointLedgerPage {
            telegram_user_id: 1,
            entries: vec![store::PointLedgerEntry {
                id: 9,
                delta: -2,
                balance_after: 8,
                reason: "transfer_charge".to_owned(),
                job_id: Some(42),
                request_chat_id: Some(100),
                request_message_id: Some(200),
                created_by: Some(1),
                created_at: chrono::DateTime::parse_from_rfc3339("2026-01-01T08:00:00+08:00")
                    .unwrap(),
            }],
            total: 1,
            limit: 10,
            page: 1,
            total_pages: 1,
        };

        let text = format_ledger_page_text(&page, false);

        assert!(text.contains("积分流水"));
        assert!(text.contains("页码：1/1"));
        assert!(text.contains("#9 ‹-2› 余额 ‹8›"));
        assert!(text.contains("原因：‹transfer_charge›"));
        assert!(text.contains("任务：‹42›"));
        assert!(text.contains("请求：‹100› / ‹200›"));
    }

    // 流水页按钮使用 callback 原地翻页，当前页按钮保留 copy-text 方便复制命令。
    #[test]
    fn test_ledger_button_rows() {
        let page = store::PointLedgerPage {
            telegram_user_id: 1,
            entries: vec![],
            total: 0,
            limit: 10,
            page: 1,
            total_pages: 1,
        };

        let rows = ledger_button_rows(LedgerCommandKind::Balance, 1, &page, false);

        assert_eq!(rows[0][0].text, "首页");
        assert_eq!(rows[0][2].text, "1/1");
        assert_eq!(rows[0][3].text, "下页");
        assert_eq!(rows[1][0].text, "刷新");

        assert!(matches!(
            rows[0][0].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::CopyText(_)
        ));
        assert!(matches!(
            rows[0][3].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::CopyText(_)
        ));
    }

    // 非边界页仍应使用 callback 原地翻页。
    #[test]
    fn test_ledger_button_rows_middle_page_uses_callback_navigation() {
        use base64::{Engine as _, engine::general_purpose};

        let page = store::PointLedgerPage {
            telegram_user_id: 1,
            entries: vec![],
            total: 30,
            limit: 10,
            page: 2,
            total_pages: 3,
        };

        let rows = ledger_button_rows(LedgerCommandKind::Balance, 1, &page, false);

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("ledger navigation must be callback outside boundary page");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "pt:p:b:1:10:1");
    }

    // 帮助文案中短命令和长命令应同时可见，减少日常输入成本。
    #[test]
    fn test_history_command_pairs() {
        assert_eq!(
            short_and_long(
                balance_history_command(10, 1, true),
                balance_history_command(10, 1, false)
            ),
            "‹/balance history 10 1› | ‹/bal h 10 1›"
        );
    }

    // callback payload 应能还原入口类型、用户和分页参数，并拒绝尾随垃圾。
    #[test]
    fn test_parse_points_callback_data() {
        assert_eq!(
            parse_points_callback_data("pt:p:b:1:10:2"),
            Some((
                LedgerCallbackAction::Page,
                LedgerCallbackRequest {
                    kind: LedgerCommandKind::Balance,
                    user_id: 1,
                    limit: 10,
                    page: 2
                }
            ))
        );
        assert_eq!(
            parse_points_callback_data("pt:r:pts:9:99:0"),
            Some((
                LedgerCallbackAction::Refresh,
                LedgerCallbackRequest {
                    kind: LedgerCommandKind::Points,
                    user_id: 9,
                    limit: MAX_LEDGER_LIMIT,
                    page: 1
                }
            ))
        );
        assert_eq!(parse_points_callback_data("pt:p:b:1:10:2:bad"), None);
        assert_eq!(parse_points_callback_data("x:p:b:1:10:2"), None);
    }

    // 积分流水 callback 前缀必须保持独立，避免被其他命令路由误判。
    #[test]
    fn test_is_points_callback_data() {
        assert!(is_points_callback_data("pt:p:b:1:10:1"));
        assert!(!is_points_callback_data("p:p:b:1:10:1"));
    }
}
