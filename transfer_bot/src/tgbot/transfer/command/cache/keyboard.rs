// `/cache` 的 callback 数据和按钮布局。

use crate::tgbot::send;

use super::super::common::build_refresh_return_menu_row;
use super::types::{CacheArgs, CacheView};

/// `/cache` callback 统一协议前缀。
const CACHE_CALLBACK_PREFIX: &str = "c:";

/// 判断 callback payload 是否属于 `/cache` 模块。
///
/// # 参数
/// - `data`: 原始回调文本
pub(super) fn is_cache_callback_data(data: &str) -> bool {
    data.starts_with(CACHE_CALLBACK_PREFIX)
}

/// 生成缓存视图切换和分页的回调数据字符串。
///
/// 格式为：`c:v:{view}:{limit}:{page}`
///
/// # 参数
/// - `view`: 目标缓存视图枚举（Summary 或 Page）
/// - `limit`: 单页容量
/// - `page`: 目标页码
pub(super) fn build_cache_view_callback_data(view: CacheView, limit: u64, page: u64) -> String {
    format!(
        "{}v:{}:{}:{}",
        CACHE_CALLBACK_PREFIX,
        view.as_str(),
        limit.max(1),
        page.max(1)
    )
}

/// 解析缓存按钮的 callback 回调数据字符串。
///
/// # 参数
/// - `data`: 原始 payload 文本
pub(super) fn parse_cache_callback_data(data: &str) -> Option<CacheArgs> {
    // 剥离 "c:" 前缀
    let payload = data.strip_prefix(CACHE_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    // 校验动作标识段
    match parts.next()? {
        "v" => {}
        _ => return None,
    }
    // 解析视图名称
    let view = match parts.next()? {
        "summary" => CacheView::Summary,
        "page" => CacheView::Page,
        _ => return None,
    };
    // 解析单页容量与页码
    let limit = parts.next()?.parse::<u64>().ok()?.max(1);
    let page = parts.next()?.parse::<u64>().ok()?.max(1);
    // 校验不能有多余字段段
    if parts.next().is_some() {
        return None;
    }
    Some(CacheArgs { view, limit, page })
}

/// 构建缓存页面底部的内联交互键盘。
///
/// # 参数
/// - `args`: 当前缓存参数
/// - `total_pages`: 总页数
pub(super) fn build_cache_keyboard(
    args: &CacheArgs,
    total_pages: u64,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let current_callback = build_cache_view_callback_data(args.view, args.limit, args.page);
    let mut pagination_row = None;
    let mut rows = Vec::new();

    // 1. 视图切换行：概览切换、查看命令（若当前处于概览页，则额外提供切回列表的入口）
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

    // 2. 若当前为分页明细视图，则构造分页导航行（首页、上页、当前页、下页、末页）
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

    // 3. 通用功能行：刷新、健康体检、主菜单
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

    // 4. 将分页行推入键盘末尾
    if let Some(row) = pagination_row {
        rows.push(row);
    }
    tdlib_rs::types::ReplyMarkupInlineKeyboard {
        rows,
        force_reply: false,
    }
}

/// 构建单个分页导航按钮。
///
/// # 参数
/// - `text`: 按钮文字
/// - `args`: 当前参数
/// - `page`: 目标页码
/// - `total_pages`: 总页码
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
