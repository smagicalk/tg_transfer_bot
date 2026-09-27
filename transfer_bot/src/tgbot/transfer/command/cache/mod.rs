// `/cache` 命令实现。
// 只读查看 file_cache 的汇总和明细，不执行清理。

use crate::tgbot::send;
use crate::tgbot::transfer::{card, store};

mod keyboard;
mod render;
mod types;

#[cfg(test)]
mod tests;

use keyboard::{build_cache_keyboard, parse_cache_callback_data};
use render::{compute_cache_page_count, format_cache_page_text, format_cache_summary_text};
use types::{CacheArgs, CacheView, parse_cache_args};

use crate::tgbot::send::send_interaction_error_card;

use super::common::{CommandStyle, cache_command as build_cache_command};

/// `cache` 帮助页和目录页共用的用途描述。
pub(in crate::tgbot::transfer::command) fn cache_help_purpose() -> &'static str {
    "只读查看 file_cache 状态。"
}

/// `cache` 帮助页和目录页共用的一句话摘要。
pub(in crate::tgbot::transfer::command) fn cache_help_summary() -> &'static str {
    "查看 file_cache 概览和最近缓存记录；只读，不执行清理。"
}

/// `cache` 菜单页和帮助详情页共用的开场说明文本行。
pub(in crate::tgbot::transfer::command) fn cache_intro_lines() -> Vec<String> {
    vec!["默认展示最近更新的缓存记录并直接分页；概览可查看状态汇总，不执行删除。".to_owned()]
}

/// `/help cache` 共用的详细说明正文。
///
/// 缓存页只展示 file_cache 状态，不负责删除动作；正文留在 cache 模块维护，避免 help 模块重复理解缓存视图。
pub(in crate::tgbot::transfer::command) fn build_cache_help_detail_text() -> String {
    let mut lines = vec![
        "cache".to_owned(),
        format!("用途：{}", cache_help_purpose()),
    ];
    lines.extend(
        cache_intro_lines()
            .into_iter()
            .map(|line| format!("说明：{line}")),
    );
    lines.extend([
        card::DIVIDER.to_owned(),
        card::section("命令"),
        build_cache_command(None, None, None, CommandStyle::Long),
        build_cache_command(Some("page"), None, None, CommandStyle::Long),
        String::new(),
        card::section("示例"),
        build_cache_command(None, None, None, CommandStyle::Long),
        build_cache_command(Some("page"), Some(10), Some(1), CommandStyle::Long),
    ]);
    lines.join("\n")
}

/// `cache` 帮助页入口按钮行。
pub(in crate::tgbot::transfer::command) fn build_cache_help_entry_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![vec![
        send::build_callback_button(
            "打开缓存页",
            &build_cache_default_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "运行健康",
            &super::build_health_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]]
}

/// 判断 callback payload 是否属于 `/cache` 模块。
///
/// # 参数
/// - `data`: 回调原始载荷
pub(super) fn is_cache_callback_data(data: &str) -> bool {
    keyboard::is_cache_callback_data(data)
}

/// 给菜单页生成缓存默认入口 callback 数据（默认第 1 页列表）。
pub(super) fn build_cache_default_callback_data() -> String {
    keyboard::build_cache_view_callback_data(CacheView::Page, CacheArgs::default().limit, 1)
}

/// 在指定应用上下文上执行 `/cache` 命令。
///
/// # 参数
/// - `app`: 全局应用上下文实例引用
/// - `text`: 命令行切片
/// - `request_chat_id`: 请求发起的 Telegram 会话 ID
/// - `client_id`: TDLib 客户端实例 ID
pub async fn cache_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    // 1. 解析命令参数
    let args = parse_cache_args(&text)?;
    tracing::info!(
        request_chat_id,
        view = args.view.as_str(),
        limit = args.limit,
        page = args.page,
        "cache command started"
    );

    // 2. 渲染卡片内容与配套 inline 键盘
    let rendered = render_cache_page_on(app, args).await?;
    // 3. 发送回复卡片
    rendered.panel.send(request_chat_id, client_id).await
}

/// 在指定应用上下文上处理 `/cache` 内联键盘点击回调。
///
/// # 参数
/// - `app`: 全局应用上下文实例引用
/// - `update`: 回调更新事件
/// - `client_id`: TDLib 客户端实例 ID
pub async fn cache_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    // 提取载荷
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    // 解析缓存参数
    let Some(args) = parse_cache_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("缓存参数无效"), client_id).await?;
        return Ok(());
    };

    // 立即响应 ACK 消除加载圈
    send::answer_callback_query(update.id, Some("已刷新"), client_id).await?;

    // 重新拉取并渲染最新数据
    let rendered = match render_cache_page_on(app, args).await {
        Ok(rendered) => rendered,
        Err(err) => {
            send_cache_callback_error(update.chat_id, client_id, &err).await?;
            return Err(err);
        }
    };
    // 原地编辑更新原消息卡片
    let (text, keyboard) = rendered.panel.into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
        "缓存刷新失败",
        "缓存页已生成，但原消息编辑失败；请使用错误卡片上的“菜单”按钮重新进入。",
    )
    .await
}

/// 缓存命令渲染结果包装体。
struct CacheRenderedPage {
    /// 包含富文本卡片与按钮矩阵的面板实例
    panel: send::ReplyPanel,
}

/// 在指定应用上下文上渲染缓存页面。
///
/// # 参数
/// - `app`: 应用上下文引用
/// - `args`: 缓存分页与视图参数
async fn render_cache_page_on(
    app: &crate::app_context::AppContext,
    args: CacheArgs,
) -> anyhow::Result<CacheRenderedPage> {
    // 1. 查询缓存状态统计与整体传输健康
    let summary_rows = store::list_file_cache_status_summaries().await?;
    let health = store::list_transfer_health_snapshot(app).await?;
    // 2. 计算总页码并钳制当前页码在有效区间内
    let total_pages = compute_cache_page_count(health.file_cache_rows as usize, args.limit);
    let page = args.page.min(total_pages).max(1);
    let normalized = CacheArgs { page, ..args };
    // 3. 构建内联键盘
    let keyboard = build_cache_keyboard(&normalized, total_pages);

    // 4. 根据视图模式分别渲染概览卡片或分页明细卡片
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

/// 发送缓存回调操作异常卡片。
///
/// # 参数
/// - `request_chat_id`: 会话 ID
/// - `client_id`: 客户端 ID
/// - `err`: 错误原因
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
