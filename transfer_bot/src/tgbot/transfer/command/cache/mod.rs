// `/cache` 命令实现。
// 只读查看 file_cache 的汇总和明细，不执行清理。

use crate::tgbot::send;
use crate::tgbot::transfer::store;

mod keyboard;
mod render;
mod types;

#[cfg(test)]
mod tests;

use keyboard::{build_cache_keyboard, parse_cache_callback_data};
use render::{compute_cache_page_count, format_cache_page_text, format_cache_summary_text};
use types::{CacheArgs, CacheView, parse_cache_args};

use crate::tgbot::send::send_interaction_error_card;

/// 判断 callback payload 是否属于 `/cache`。
pub(super) fn is_cache_callback_data(data: &str) -> bool {
    keyboard::is_cache_callback_data(data)
}

/// 给菜单页生成缓存概览 callback 数据。
pub(super) fn build_cache_summary_callback_data() -> String {
    keyboard::build_cache_view_callback_data(CacheView::Summary, CacheArgs::default().limit, 1)
}

/// `/cache` 命令入口。
///
/// 支持：
/// - `/cache`
/// - `/cache summary`
/// - `/cache page [limit] [page]`
pub async fn cache_command(
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let args = parse_cache_args(&text)?;
    tracing::info!(
        request_chat_id,
        view = args.view.as_str(),
        limit = args.limit,
        page = args.page,
        "cache command started"
    );

    let rendered = render_cache_page(args).await?;
    rendered.panel.send(request_chat_id, client_id).await
}

/// `/cache` callback 入口。
pub async fn cache_callback_query(
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    let Some(args) = parse_cache_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("缓存参数无效"), client_id).await?;
        return Ok(());
    };

    send::answer_callback_query(update.id, Some("已刷新"), client_id).await?;

    let rendered = match render_cache_page(args).await {
        Ok(rendered) => rendered,
        Err(err) => {
            send_cache_callback_error(update.chat_id, client_id, &err).await?;
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
        "缓存刷新失败",
        "缓存页已生成，但原消息编辑失败；请复制错误或重新发送 /cache。",
    )
    .await
}

/// 缓存命令渲染结果。
struct CacheRenderedPage {
    panel: send::ReplyPanel,
}

/// 渲染缓存页面。
async fn render_cache_page(args: CacheArgs) -> anyhow::Result<CacheRenderedPage> {
    let summary_rows = store::list_file_cache_status_summaries().await?;
    let health =
        store::list_transfer_health_snapshot(crate::app_context::app_context().as_ref()).await?;
    let total_pages = compute_cache_page_count(health.file_cache_rows as usize, args.limit);
    let page = args.page.min(total_pages).max(1);
    let normalized = CacheArgs { page, ..args };
    let keyboard = build_cache_keyboard(&normalized, total_pages);

    let text = match normalized.view {
        CacheView::Summary => format_cache_summary_text(&health, &summary_rows),
        CacheView::Page => {
            let rows =
                store::list_recent_file_cache_snapshots(normalized.limit, normalized.page).await?;
            format_cache_page_text(&health, &rows, &normalized, total_pages)
        }
    };

    Ok(CacheRenderedPage {
        panel: send::ReplyPanel::card(text).rows(keyboard.rows),
    })
}

/// 缓存命令失败提示。
async fn send_cache_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_interaction_error_card(
        request_chat_id,
        client_id,
        "缓存刷新失败",
        "缓存页未更新，请检查日志或复制错误信息。",
        err,
    )
    .await
}
