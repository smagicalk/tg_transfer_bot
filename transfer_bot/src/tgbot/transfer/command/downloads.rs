// `/downloads` 命令实现：
// - 参数解析
// - 任务筛选/分页
// - 文本格式化
// - inline keyboard 翻页

use crate::tgbot::send;
use crate::tgbot::transfer::{card, store};

mod keyboard;
mod render;
mod types;

#[cfg(test)]
mod tests;

use keyboard::{DownloadsCallbackAction, build_downloads_keyboard, parse_downloads_callback_data};
use render::{compute_downloads_query_limit, compute_total_pages, format_downloads_text};
use types::{DownloadsArgs, DownloadsFilter, parse_downloads_args_on};

use super::common::{CommandStyle, downloads_command as build_downloads_command};
use crate::tgbot::send::send_interaction_error_card;

/// 判断 callback payload 是否属于 `/downloads`。
///
/// 统一回调分发器只看前缀，具体参数是否合法仍由 `/downloads` 自己解析和回复。
pub(super) fn is_downloads_callback_data(data: &str) -> bool {
    data.starts_with("d:")
}

/// 给任务详情卡片使用的“返回列表”回调数据。
///
/// 这里不向外暴露 `DownloadsFilter`，只让 `job` 模块拿到适合当前任务状态的列表入口。
pub(super) fn build_downloads_return_list_callback_data(status: &str, limit: u64) -> String {
    keyboard::build_downloads_filter_callback_data(job_status_downloads_filter(status), limit)
}

/// 给结果/错误卡片使用的“进入某个列表筛选”回调数据。
///
/// 参数仍使用 `/downloads` 的英文筛选值，方便按钮和手输命令保持同一套语义。
pub(super) fn build_downloads_filter_value_callback_data(
    filter_value: &str,
    limit: u64,
) -> Option<String> {
    let filter = DownloadsFilter::parse(filter_value)?;
    Some(keyboard::build_downloads_filter_callback_data(
        filter, limit,
    ))
}

/// `/downloads` 帮助和目录页共用的用途描述。
pub(in crate::tgbot::transfer::command) fn downloads_help_purpose() -> &'static str {
    "查看任务列表、状态和真实下载进度。"
}

/// `/downloads` 帮助和目录页共用的一句话摘要。
pub(in crate::tgbot::transfer::command) fn downloads_help_summary() -> &'static str {
    "查看任务列表、状态和真实下载进度；支持筛选、分页和任务详情入口。"
}

/// `/downloads` 详情页和菜单页共用的说明。
pub(in crate::tgbot::transfer::command) fn downloads_help_intro_lines() -> Vec<String> {
    vec![
        "直接点筛选按钮查看列表；列表页内可继续翻页、刷新和进入任务详情。".to_owned(),
        "已授权用户会看到全部任务。".to_owned(),
    ]
}

/// `/downloads` 帮助详情里统一展示的筛选参数列表。
pub(in crate::tgbot::transfer::command) fn downloads_help_filter_values() -> &'static str {
    "all | wait | dl | up | done | ok | fail | run | ready | pause | cancelling | cancel"
}

/// `/downloads` 帮助详情复用的示例命令。
pub(in crate::tgbot::transfer::command) fn downloads_help_example_commands() -> Vec<String> {
    vec![
        build_downloads_command(None, None, None, CommandStyle::Long),
        build_downloads_command(None, Some(10), None, CommandStyle::Long),
        build_downloads_command(Some("dl"), None, None, CommandStyle::Long),
        build_downloads_command(Some("done"), Some(5), None, CommandStyle::Long),
        build_downloads_command(Some("done"), Some(5), Some(2), CommandStyle::Long),
    ]
}

/// `/help downloads` 和其他外层入口共用的详细说明正文。
pub(in crate::tgbot::transfer::command) fn build_downloads_help_detail_text() -> String {
    let mut lines = vec![
        "downloads".to_owned(),
        format!("用途：{}", downloads_help_purpose()),
    ];
    lines.extend(
        downloads_help_intro_lines()
            .into_iter()
            .map(|line| format!("说明：{}", line)),
    );
    lines.extend([
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        format!(
            "{} [filter] [limit] [page]",
            build_downloads_command(None, None, None, CommandStyle::Long)
        ),
        String::new(),
        "筛选参数：".to_owned(),
        card::code(downloads_help_filter_values()),
        String::new(),
        "示例：".to_owned(),
    ]);
    lines.extend(downloads_help_example_commands());
    lines.join("\n")
}

/// `/help downloads` 详情页共用的按钮入口。
///
/// 这里把常用筛选统一收在 `/downloads` 模块里，避免 help 层重复维护。
pub(in crate::tgbot::transfer::command) fn build_downloads_help_entry_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![vec![
        send::build_callback_button(
            "全部列表",
            &build_downloads_filter_value_callback_data("all", 8)
                .expect("all downloads filter should exist"),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "运行列表",
            &build_downloads_filter_value_callback_data("run", 8)
                .expect("run downloads filter should exist"),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            "失败列表",
            &build_downloads_filter_value_callback_data("fail", 8)
                .expect("fail downloads filter should exist"),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]]
}

/// `/menu` 下载页复用的筛选按钮行。
pub(in crate::tgbot::transfer::command) fn build_downloads_menu_filter_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    [
        [
            ("全部", "all", tdlib_rs::enums::ButtonStyle::Primary),
            ("运行", "run", tdlib_rs::enums::ButtonStyle::Default),
            ("等待", "wait", tdlib_rs::enums::ButtonStyle::Default),
        ]
        .as_slice(),
        [
            ("下载", "dl", tdlib_rs::enums::ButtonStyle::Default),
            ("上传", "up", tdlib_rs::enums::ButtonStyle::Default),
            ("就绪", "ready", tdlib_rs::enums::ButtonStyle::Default),
        ]
        .as_slice(),
        [
            ("完成", "done", tdlib_rs::enums::ButtonStyle::Default),
            ("成功", "ok", tdlib_rs::enums::ButtonStyle::Default),
            ("失败", "fail", tdlib_rs::enums::ButtonStyle::Default),
        ]
        .as_slice(),
        [
            ("暂停", "pause", tdlib_rs::enums::ButtonStyle::Default),
            (
                "停止中",
                "cancelling",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            ("已停止", "cancel", tdlib_rs::enums::ButtonStyle::Default),
        ]
        .as_slice(),
    ]
    .into_iter()
    .map(|row| {
        row.iter()
            .map(|(text, filter, style)| {
                send::build_callback_button(
                    text,
                    &super::require_downloads_filter_button_data(filter, 8),
                    style.clone(),
                )
            })
            .collect::<Vec<_>>()
    })
    .collect()
}

/// 在指定上下文上执行 `/downloads` 命令。
pub async fn downloads_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let args = parse_downloads_args_on(app, &text)?;
    tracing::info!(
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        filter = args.filter.command_value(),
        limit = args.limit,
        page = args.page,
        "downloads command started"
    );
    render_downloads_page_on(app, actor, args)
        .await?
        .panel
        .send(actor.request_chat_id, client_id)
        .await
}

/// 在指定上下文上处理 `/downloads` callback。
pub async fn downloads_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    let Some((action, args)) = parse_downloads_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("分页参数无效"), client_id).await?;
        return Ok(());
    };
    tracing::debug!(
        chat_id = update.chat_id,
        message_id = update.message_id,
        filter = args.filter.command_value(),
        limit = args.limit,
        page = args.page,
        action = ?action,
        "downloads callback page requested"
    );

    let callback_tip = match action {
        DownloadsCallbackAction::Page => None,
        DownloadsCallbackAction::Refresh => Some("已刷新"),
        DownloadsCallbackAction::Filter => Some(args.filter.label()),
    };
    send::answer_callback_query(update.id, callback_tip, client_id).await?;

    let rendered = match render_downloads_page_on(app, actor, args).await {
        Ok(rendered) => rendered,
        Err(err) => {
            send_downloads_callback_error(update.chat_id, client_id, &err).await?;
            return Err(err);
        }
    };
    let (text, keyboard) = rendered.panel.into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
        "下载列表刷新失败",
        "列表已生成，但原消息编辑失败；请使用错误卡片上的“菜单”按钮重新进入。",
    )
    .await
}

/// 下载列表按钮失败提示。
///
/// callback 已经先 ACK，失败时不能再 answer 同一个 callback，因此发送一条短卡片说明错误。
async fn send_downloads_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_interaction_error_card(
        request_chat_id,
        client_id,
        "下载列表刷新失败",
        "列表未刷新，请检查日志或复制错误信息。",
        err,
    )
    .await
}

/// `/downloads` 页面渲染结果。
struct DownloadsRenderedPage {
    panel: send::ReplyPanel,
}

/// 在指定上下文上查询并渲染某一页下载列表。
async fn render_downloads_page_on(
    app: &crate::app_context::AppContext,
    actor: crate::config::RequestActor,
    args: DownloadsArgs,
) -> anyhow::Result<DownloadsRenderedPage> {
    // 先拉取更大窗口，再按筛选条件裁剪，避免“最近几条碰巧不匹配”导致空结果。
    let query_limit = compute_downloads_query_limit(args.limit, args.page);
    let snapshots = store::list_recent_job_snapshots(app, query_limit).await?;
    let filtered = snapshots
        .into_iter()
        .filter(|snapshot| args.filter.matches(snapshot))
        .collect::<Vec<_>>();
    let total = filtered.len();
    let total_pages = compute_total_pages(total, args.limit);
    let page = args.page.min(total_pages).max(1);
    let normalized_args = DownloadsArgs { page, ..args };
    let start = ((normalized_args.page - 1) * normalized_args.limit) as usize;
    let end = start.saturating_add(args.limit as usize).min(total);
    let page_items = if start >= total {
        vec![]
    } else {
        filtered[start..end].to_vec()
    };
    tracing::info!(
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        filter = normalized_args.filter.command_value(),
        limit = normalized_args.limit,
        page = normalized_args.page,
        total,
        total_pages,
        page_items = page_items.len(),
        "downloads page rendered"
    );
    let text = format_downloads_text(&page_items, &normalized_args, total);
    let keyboard = build_downloads_keyboard(&normalized_args, total_pages, &page_items);

    Ok(DownloadsRenderedPage {
        panel: send::ReplyPanel::card(text).rows(keyboard.rows),
    })
}

/// 根据任务状态选择最接近的 `/downloads` 筛选器。
fn job_status_downloads_filter(status: &str) -> DownloadsFilter {
    match status {
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING => DownloadsFilter::Running,
        store::JOB_STATUS_PAUSED => DownloadsFilter::Paused,
        store::JOB_STATUS_CANCELLING | store::JOB_STATUS_CANCEL_FINALIZING => {
            DownloadsFilter::Cancelling
        }
        store::JOB_STATUS_CANCELLED => DownloadsFilter::Cancelled,
        store::JOB_STATUS_SUCCESS => DownloadsFilter::Finished,
        store::JOB_STATUS_FAILED | store::JOB_STATUS_PARTIAL => DownloadsFilter::Failed,
        _ => DownloadsFilter::All,
    }
}
