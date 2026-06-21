// 积分模块的流水渲染与按钮拼装。
// 这里只负责把数据渲染成卡片和按钮，命令入口仍留在 `points.rs`。

use crate::tgbot::send;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store;

use super::super::common::{
    CommandStyle, balance_history_command as build_balance_history_command,
    build_page_command_section, build_page_empty_note, build_ready_page_header,
    build_refresh_return_menu_row, points_history_command as build_points_history_command,
};

/// 渲染 `/balance` 或 `/points` 使用的积分流水面板。
pub(super) async fn render_ledger_panel(
    kind: super::LedgerCommandKind,
    user_id: i64,
    limit: u64,
    page: u64,
    admin_view: bool,
) -> anyhow::Result<send::ReplyPanel> {
    let page = store::list_point_ledger_page(user_id, limit, page).await?;
    Ok(
        send::ReplyPanel::card(format_ledger_page_text(&page, admin_view))
            .rows(ledger_button_rows(kind, user_id, &page, admin_view)),
    )
}

/// 积分余额卡片正文。
pub(super) fn format_balance_text(account: &store::UserAccountSnapshot) -> String {
    let mut lines = build_ready_page_header("积分账户");
    lines.extend([
        card::section("账户"),
        card::field("用户", account.telegram_user_id),
        card::field("角色", &account.role),
        card::field("余额", account.points_balance),
        card::field("累计增加", account.total_points_added),
        card::field("累计消费", account.total_points_spent),
        build_page_command_section(),
        card::command_line("余额", "/balance"),
        card::command_line("流水", "/balance history"),
        card::command_line("帮助", "/help points"),
    ]);
    lines.join("\n")
}

/// 积分流水卡片正文。
pub(super) fn format_ledger_page_text(page: &store::PointLedgerPage, admin_view: bool) -> String {
    let mut lines = build_ready_page_header(if admin_view {
        "积分流水 [admin]"
    } else {
        "积分流水"
    });
    lines.extend([
        if admin_view {
            card::field("模式", "admin")
        } else {
            card::field("模式", "self")
        },
        card::section("账户"),
        card::field("用户", page.telegram_user_id),
        format!(
            "页码：{}/{}  每页：{}  总数：{}",
            page.page, page.total_pages, page.limit, page.total
        ),
    ]);

    if page.entries.is_empty() {
        lines.push(card::section("记录"));
        lines.push(build_page_empty_note("暂无积分流水。"));
    } else {
        lines.push(card::section("记录"));
        for entry in &page.entries {
            lines.extend(format_ledger_entry_lines(entry));
        }
    }

    lines.push(build_page_command_section());
    if admin_view {
        lines.push(card::command_line(
            "当前页",
            points_history_command(page.telegram_user_id, page.limit, page.page, false),
        ));
    } else {
        lines.push(card::command_line(
            "当前页",
            balance_history_command(page.limit, page.page, false),
        ));
    }
    lines.join("\n")
}

/// 单条积分流水正文。
pub(super) fn format_ledger_entry_lines(entry: &store::PointLedgerEntry) -> Vec<String> {
    let mut lines = vec![
        format!(
            "#{} {} 余额 {}",
            entry.id,
            signed_delta(entry.delta),
            card::code(entry.balance_after)
        ),
        format!(
            "原因：{}  时间：{}",
            card::code(&entry.reason),
            entry.created_at
        ),
    ];
    if let Some(job_id) = entry.job_id {
        lines.push(format!("任务：{}", card::code(job_id)));
    }
    if let Some(request_chat_id) = entry.request_chat_id {
        let request_message = entry
            .request_message_id
            .map(|message_id| format!(" / {}", card::code(message_id)))
            .unwrap_or_default();
        lines.push(format!(
            "请求：{}{}",
            card::code(request_chat_id),
            request_message
        ));
    }
    if let Some(created_by) = entry.created_by {
        lines.push(format!("操作人：{}", card::code(created_by)));
    }
    lines
}

/// 积分变化量展示，正数显式加号。
pub(super) fn signed_delta(delta: i64) -> String {
    if delta > 0 {
        format!("+{}", card::code(delta))
    } else {
        card::code(delta)
    }
}

/// 构造积分流水按钮。
pub(super) fn ledger_button_rows(
    kind: super::LedgerCommandKind,
    user_id: i64,
    page: &store::PointLedgerPage,
    admin_view: bool,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = Vec::new();
    let prev_page = page.page.saturating_sub(1).max(1);
    let next_page = (page.page + 1).min(page.total_pages);
    rows.push(vec![
        ledger_nav_button("首页", kind, user_id, page.limit, 1, page.page),
        ledger_nav_button("上页", kind, user_id, page.limit, prev_page, page.page),
        send::build_callback_button(
            &format!("{}/{}", page.page, page.total_pages),
            &super::build_ledger_callback_data(
                super::LedgerCallbackAction::Refresh,
                kind,
                user_id,
                page.limit,
                page.page,
            ),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        ledger_nav_button("下页", kind, user_id, page.limit, next_page, page.page),
        ledger_nav_button(
            "末页",
            kind,
            user_id,
            page.limit,
            page.total_pages,
            page.page,
        ),
    ]);
    rows.push(build_refresh_return_menu_row(
        send::build_callback_button(
            "刷新",
            &super::build_ledger_callback_data(
                super::LedgerCallbackAction::Refresh,
                kind,
                user_id,
                page.limit,
                page.page,
            ),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        balance_return_button(kind, user_id),
        send::build_callback_button(
            "菜单",
            &super::super::build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ));
    let _ = admin_view;
    rows
}

/// 构造积分流水页的返回按钮。
///
/// 普通用户余额页直接回到自己的余额卡片；
/// admin 查看他人流水时也回到该用户的余额卡片，避免把“返回”误做成复制命令。
fn balance_return_button(
    kind: super::LedgerCommandKind,
    user_id: i64,
) -> tdlib_rs::types::InlineKeyboardButton {
    match kind {
        super::LedgerCommandKind::Balance => send::build_callback_button(
            "返回",
            &super::build_ledger_callback_data(
                super::LedgerCallbackAction::BalanceHome,
                super::LedgerCommandKind::Balance,
                user_id,
                10,
                1,
            ),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        super::LedgerCommandKind::Points => send::build_callback_button(
            "返回",
            &super::build_ledger_callback_data(
                super::LedgerCallbackAction::BalanceHome,
                super::LedgerCommandKind::Points,
                user_id,
                10,
                1,
            ),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    }
}

/// 构造积分流水翻页按钮。
fn ledger_nav_button(
    text: &str,
    kind: super::LedgerCommandKind,
    user_id: i64,
    limit: u64,
    page: u64,
    current_page: u64,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        text,
        &super::build_ledger_callback_data(
            if page == current_page {
                super::LedgerCallbackAction::Refresh
            } else {
                super::LedgerCallbackAction::Page
            },
            kind,
            user_id,
            limit,
            if page == current_page {
                current_page
            } else {
                page
            },
        ),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// 构造 `/balance history` 命令；测试仍使用这个 helper 校验长命令格式。
pub(super) fn balance_history_command(limit: u64, page: u64, short: bool) -> String {
    let style = if short {
        CommandStyle::Short
    } else {
        CommandStyle::Long
    };
    build_balance_history_command(limit, page, style)
}

/// 构造 `/points history` 命令；正文里继续展示统一的长命令格式。
fn points_history_command(user_id: i64, limit: u64, page: u64, short: bool) -> String {
    let style = if short {
        CommandStyle::Short
    } else {
        CommandStyle::Long
    };
    build_points_history_command(user_id, limit, page, style)
}
