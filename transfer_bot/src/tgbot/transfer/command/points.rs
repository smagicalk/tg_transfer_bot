// `/balance` 与 `/points` 命令：
// - `/balance` 查询当前用户积分。
// - `/points` 仅 admin 可用，用于给普通用户增减积分或查看余额。

use crate::tgbot::send;
use crate::tgbot::transfer::store;

mod callback;
mod render;

use super::build_menu_home_button_data;
#[cfg(test)]
use super::common::balance_history_command;
pub(in crate::tgbot::transfer::command) use callback::points_callback_query;
use render::{format_balance_text, format_ledger_page_text, ledger_button_rows};

/// 积分流水 callback 前缀。
const POINTS_CALLBACK_PREFIX: &str = "pt:";
const POINTS_ADJUST_CALLBACK_PREFIX: &str = "pta:";
/// 积分流水默认每页条数。
const DEFAULT_LEDGER_LIMIT: u64 = 10;
/// 积分流水单页最大条数，避免一次回复过长。
const MAX_LEDGER_LIMIT: u64 = 50;

/// 判断 callback payload 是否属于积分模块。
pub(super) fn is_points_callback_data(data: &str) -> bool {
    data.starts_with(POINTS_CALLBACK_PREFIX)
}

/// 给菜单首页生成“查看余额”回调。
pub(super) fn build_balance_home_callback_data() -> String {
    build_ledger_callback_data(
        LedgerCallbackAction::Refresh,
        LedgerCommandKind::Balance,
        0,
        DEFAULT_LEDGER_LIMIT,
        1,
    )
}

/// 给菜单首页生成“查看我的流水”回调。
pub(super) fn build_balance_history_home_callback_data(limit: u64, page: u64) -> String {
    build_ledger_callback_data(
        LedgerCallbackAction::Refresh,
        LedgerCommandKind::Balance,
        0,
        limit,
        page,
    )
}

/// 给管理员查看其他用户时生成“查看该用户流水”回调。
pub(super) fn build_points_history_home_callback_data(
    user_id: i64,
    limit: u64,
    page: u64,
) -> String {
    build_ledger_callback_data(
        LedgerCallbackAction::Refresh,
        LedgerCommandKind::Points,
        user_id,
        limit,
        page,
    )
}

pub(super) fn build_points_adjust_home_callback_data(user_id: i64, add: bool) -> String {
    format!(
        "{}{}:{}",
        POINTS_ADJUST_CALLBACK_PREFIX,
        if add { "add" } else { "sub" },
        user_id
    )
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
                send::build_callback_button(
                    "查看流水",
                    &build_balance_history_home_callback_data(DEFAULT_LEDGER_LIMIT, 1),
                    tdlib_rs::enums::ButtonStyle::Primary,
                ),
                send::build_callback_button(
                    "菜单",
                    &build_menu_home_button_data(),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ],
            vec![send::build_callback_button(
                "帮助",
                &super::help::build_help_callback_data(Some("points")),
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
        "show" => store::get_user_account(user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user account not found: {}", user_id))?,
        "add" | "sub" => {
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
            send::build_callback_button(
                "查看流水",
                &build_points_history_home_callback_data(user_id, DEFAULT_LEDGER_LIMIT, 1),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .row(vec![
            send::build_callback_button(
                "加分",
                &build_points_adjust_home_callback_data(user_id, true),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "扣分",
                &build_points_adjust_home_callback_data(user_id, false),
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
    BalanceHome,
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
    matches!(action, Some("history" | "ledger"))
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
        "bh" => LedgerCallbackAction::BalanceHome,
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
        LedgerCallbackAction::BalanceHome => "bh",
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

    // 积分流水参数仅支持长命令，并限制单页最大数量。
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
            parse_balance_history_args(&["/balance", "history", "99", "2"]).unwrap(),
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

    // 流水页按钮全部使用 callback 原地翻页与刷新，避免再退回复制命令。
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
        assert_eq!(rows[1][1].text, "返回");
        assert_eq!(rows[1][2].text, "菜单");
        assert_eq!(rows.len(), 2);

        assert!(matches!(
            rows[0][0].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
        assert!(matches!(
            rows[0][3].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
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

    // admin 查看他人流水时，“返回”按钮也应回到该用户的余额卡片。
    #[test]
    fn test_ledger_button_rows_admin_return_is_balance_home_callback() {
        use base64::{Engine as _, engine::general_purpose};

        let page = store::PointLedgerPage {
            telegram_user_id: 9,
            entries: vec![],
            total: 0,
            limit: 10,
            page: 1,
            total_pages: 1,
        };

        let rows = ledger_button_rows(LedgerCommandKind::Points, 9, &page, true);
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[1][1].r#type
        else {
            panic!("points ledger return button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();

        assert_eq!(rows[1][1].text, "返回");
        assert_eq!(decoded, "pt:bh:pts:9:10:1");
        assert_eq!(rows.len(), 2);
    }

    // 流水页不应再出现复制型当前页入口。
    #[test]
    fn test_ledger_button_rows_drop_duplicate_current_command_copy() {
        let page = store::PointLedgerPage {
            telegram_user_id: 1,
            entries: vec![],
            total: 0,
            limit: 10,
            page: 1,
            total_pages: 1,
        };

        let rows = ledger_button_rows(LedgerCommandKind::Balance, 1, &page, false);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"1/1"));
        assert!(!labels.contains(&"复制当前命令"));
        assert!(!labels.contains(&"复制当前页"));
        assert!(!labels.contains(&"复制余额"));
    }

    // 普通用户流水页的“返回”按钮必须是独立返回动作，不能再复用刷新 payload。
    #[test]
    fn test_balance_ledger_return_button_uses_balance_home_action() {
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
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[1][1].r#type
        else {
            panic!("balance ledger return button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();

        assert_eq!(decoded, "pt:bh:b:1:10:1");
        assert_ne!(
            decoded,
            build_ledger_callback_data(
                LedgerCallbackAction::Refresh,
                LedgerCommandKind::Balance,
                1,
                10,
                2
            )
        );
    }

    // 帮助文案已统一成长命令。
    #[test]
    fn test_history_command_pairs() {
        assert_eq!(
            short_and_long(
                balance_history_command(10, 1, super::super::common::CommandStyle::Short),
                balance_history_command(10, 1, super::super::common::CommandStyle::Long)
            ),
            "‹/balance history 10 1›"
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
        assert_eq!(
            parse_points_callback_data("pt:bh:b:1:10:1"),
            Some((
                LedgerCallbackAction::BalanceHome,
                LedgerCallbackRequest {
                    kind: LedgerCommandKind::Balance,
                    user_id: 1,
                    limit: 10,
                    page: 1
                }
            ))
        );
        assert_eq!(parse_points_callback_data("pt:p:b:1:10:2:bad"), None);
        assert_eq!(parse_points_callback_data("x:p:b:1:10:2"), None);
    }

    // admin 查看用户余额时，应提供余额查询、流水和调分相关的直接入口。
    #[test]
    fn test_points_show_card_keeps_adjustment_entry_points() {
        let account = store::UserAccountSnapshot {
            telegram_user_id: 42,
            role: "user".to_owned(),
            points_balance: 10,
            total_points_added: 20,
            total_points_spent: 10,
        };

        let panel = send::ReplyPanel::card(format_balance_text(&account))
            .row(vec![
                send::build_callback_button(
                    "查看流水",
                    &build_points_history_home_callback_data(42, DEFAULT_LEDGER_LIMIT, 1),
                    tdlib_rs::enums::ButtonStyle::Primary,
                ),
                send::build_callback_button(
                    "菜单",
                    &build_menu_home_button_data(),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ])
            .row(vec![
                send::build_callback_button(
                    "加分",
                    &build_points_adjust_home_callback_data(42, true),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
                send::build_callback_button(
                    "扣分",
                    &build_points_adjust_home_callback_data(42, false),
                    tdlib_rs::enums::ButtonStyle::Default,
                ),
            ]);
        let (_text, keyboard) = panel.into_card_parts().expect("card parts");
        let rows = keyboard.rows;
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(rows[0][0].text, "查看流水");
        assert_eq!(rows[0][1].text, "菜单");
        assert_eq!(rows[1][0].text, "加分");
        assert_eq!(rows[1][1].text, "扣分");
        assert!(labels.contains(&"查看流水"));
        assert!(labels.contains(&"加分"));
        assert!(labels.contains(&"扣分"));
    }

    // 积分流水 callback 前缀必须保持独立，避免被其他命令路由误判。
    #[test]
    fn test_is_points_callback_data() {
        assert!(is_points_callback_data("pt:p:b:1:10:1"));
        assert!(!is_points_callback_data("p:p:b:1:10:1"));
    }
}
