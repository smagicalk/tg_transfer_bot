// `/cache` 的 callback 数据和按钮布局。

use crate::tgbot::send;

use super::super::common::build_refresh_return_menu_row;
use super::types::{CacheArgs, CacheView};

/// `/cache` callback 前缀。
const CACHE_CALLBACK_PREFIX: &str = "c:";

/// 判断 callback payload 是否属于 `/cache`。
pub(super) fn is_cache_callback_data(data: &str) -> bool {
    data.starts_with(CACHE_CALLBACK_PREFIX)
}

/// 生成视图 callback 数据。
pub(super) fn build_cache_view_callback_data(view: CacheView, limit: u64, page: u64) -> String {
    format!(
        "{}v:{}:{}:{}",
        CACHE_CALLBACK_PREFIX,
        view.as_str(),
        limit.max(1),
        page.max(1)
    )
}

/// 解析 callback 数据。
pub(super) fn parse_cache_callback_data(data: &str) -> Option<CacheArgs> {
    let payload = data.strip_prefix(CACHE_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    match parts.next()? {
        "v" => {}
        _ => return None,
    }
    let view = match parts.next()? {
        "summary" => CacheView::Summary,
        "page" => CacheView::Page,
        _ => return None,
    };
    let limit = parts.next()?.parse::<u64>().ok()?.max(1);
    let page = parts.next()?.parse::<u64>().ok()?.max(1);
    if parts.next().is_some() {
        return None;
    }
    Some(CacheArgs { view, limit, page })
}

/// 构建缓存页键盘。
pub(super) fn build_cache_keyboard(
    args: &CacheArgs,
    total_pages: u64,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let current_callback = build_cache_view_callback_data(args.view, args.limit, args.page);
    let mut pagination_row = None;
    let mut rows = Vec::new();
    // 明细列表是默认入口，不再重复展示“分页”切换；概览页仅补一个返回列表的入口。
    let mut view_row = vec![send::build_callback_button(
        "概览",
        &build_cache_view_callback_data(CacheView::Summary, args.limit, 1),
        if matches!(args.view, CacheView::Summary) {
            tdlib_rs::enums::ButtonStyle::Primary
        } else {
            tdlib_rs::enums::ButtonStyle::Default
        },
    )];
    if matches!(args.view, CacheView::Summary) {
        view_row.push(send::build_callback_button(
            "缓存列表",
            &build_cache_view_callback_data(CacheView::Page, args.limit, 1),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    view_row.push(send::build_callback_button(
        "查看命令",
        &super::super::build_help_button_data(Some("cache")),
        tdlib_rs::enums::ButtonStyle::Default,
    ));
    rows.push(view_row);
    if matches!(args.view, CacheView::Page) {
        let first_page = 1u64;
        let prev_page = args.page.saturating_sub(1).max(1);
        let next_page = (args.page + 1).min(total_pages.max(1));
        let last_page = total_pages.max(1);
        pagination_row = Some(vec![
            cache_nav_button("首页", args, first_page, last_page),
            cache_nav_button("上页", args, prev_page, last_page),
            send::build_callback_button(
                &format!("{}/{}", args.page, total_pages.max(1)),
                &current_callback,
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            cache_nav_button("下页", args, next_page, last_page),
            cache_nav_button("末页", args, last_page, last_page),
        ]);
    }
    rows.push(build_refresh_return_menu_row(
        send::build_callback_button(
            "刷新",
            &current_callback,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "健康",
            &super::super::build_health_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            "菜单",
            &super::super::build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ));
    if let Some(row) = pagination_row {
        rows.push(row);
    }
    tdlib_rs::types::ReplyMarkupInlineKeyboard { rows }
}

/// 构建分页导航按钮。
fn cache_nav_button(
    text: &str,
    args: &CacheArgs,
    page: u64,
    total_pages: u64,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        text,
        &build_cache_view_callback_data(
            args.view,
            args.limit,
            if page == args.page {
                args.page.min(total_pages)
            } else {
                page
            },
        ),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}
