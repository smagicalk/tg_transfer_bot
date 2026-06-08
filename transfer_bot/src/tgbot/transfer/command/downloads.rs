// `/downloads` 命令实现：
// - 参数解析
// - 任务筛选/分页
// - 文本格式化
// - inline keyboard 翻页

use crate::tgbot::send;
use crate::tgbot::transfer::store;

mod keyboard;
mod render;
mod types;

#[cfg(test)]
mod tests;

use keyboard::{DownloadsCallbackAction, build_downloads_keyboard, parse_downloads_callback_data};
use render::{compute_downloads_query_limit, compute_total_pages, format_downloads_text};
use types::{DownloadsArgs, DownloadsFilter, parse_downloads_args};

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

/// 给菜单页使用的下载筛选按钮数据。
///
/// 菜单只传英文筛选参数，不直接依赖 `/downloads` 内部枚举，保持命令模块之间低耦合。
pub(super) fn build_downloads_menu_callback_data(filter_value: &str, limit: u64) -> Option<String> {
    build_downloads_filter_value_callback_data(filter_value, limit)
}

/// `/downloads` 命令入口。
/// 命令格式：`/downloads [filter] [limit] [page]`
/// 示例：
/// - `/downloads`
/// - `/downloads 10`
/// - `/downloads dl`
/// - `/downloads done 5`
/// - `/downloads done 5 2`
pub async fn downloads_command(
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let args = parse_downloads_args(&text)?;
    tracing::info!(
        request_chat_id,
        filter = args.filter.command_value(),
        limit = args.limit,
        page = args.page,
        "downloads command started"
    );
    render_downloads_page(request_chat_id, args)
        .await?
        .panel
        .send(request_chat_id, client_id)
        .await
}

/// 处理 `/downloads` 的分页按钮回调。
pub async fn downloads_callback_query(
    update: tdlib_rs::enums::UpdateNewCallbackQuery,
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

    let rendered = render_downloads_page(update.chat_id, args).await?;
    let (text, keyboard) = rendered.panel.into_card_parts()?;
    let callback_tip = match action {
        DownloadsCallbackAction::Page => None,
        DownloadsCallbackAction::Refresh => Some("已刷新"),
        DownloadsCallbackAction::Filter => Some(args.filter.label()),
    };
    send::answer_callback_query(update.id, callback_tip, client_id).await?;
    send::edit_card_message_with_inline_keyboard(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
    )
    .await
}

/// `/downloads` 页面渲染结果。
struct DownloadsRenderedPage {
    panel: send::ReplyPanel,
}

/// 查询并渲染某一页下载列表。
async fn render_downloads_page(
    request_chat_id: i64,
    args: DownloadsArgs,
) -> anyhow::Result<DownloadsRenderedPage> {
    // 先拉取更大窗口，再按筛选条件裁剪，避免“最近几条碰巧不匹配”导致空结果。
    let query_limit = compute_downloads_query_limit(args.limit, args.page);
    let snapshots = store::list_recent_job_snapshots(request_chat_id, query_limit).await?;
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
        request_chat_id,
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
