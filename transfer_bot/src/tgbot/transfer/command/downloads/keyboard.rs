// `/downloads` 的 inline keyboard 和 callback 数据。
// 回调数据保持短格式，避免 Telegram callback payload 过长。

#[cfg(test)]
use super::super::common::downloads_command as build_command;
use super::super::job::build_job_status_callback_data;
use super::super::menu::{build_menu_home_callback_data, build_menu_tasks_hub_callback_data};
use super::types::{DownloadsArgs, DownloadsFilter};
use crate::tgbot::send;
use crate::tgbot::transfer::store;

/// `/downloads` 按钮回调协议统一前缀。
const DOWNLOADS_CALLBACK_PREFIX: &str = "d:";

/// 生成翻页命令字符串，供回归测试校验命令协议。
///
/// # 参数
/// - `filter`: 当前筛选枚举
/// - `limit`: 单页容量
/// - `page`: 目标页码
#[cfg(test)]
pub(super) fn build_downloads_page_command(
    filter: DownloadsFilter,
    limit: u64,
    page: u64,
) -> String {
    // 若筛选为 All 则省略参数中的 filter 标签
    let filter = if filter == DownloadsFilter::All {
        None
    } else {
        Some(filter.command_value())
    };
    build_command(
        filter,
        Some(limit),
        Some(page),
        super::super::common::CommandStyle::Long,
    )
}

/// 下载列表按钮动作类型枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DownloadsCallbackAction {
    /// 翻页动作（首页、上一页、下一页、末页）
    Page,
    /// 刷新当前视图动作
    Refresh,
    /// 切换状态筛选维度动作
    Filter,
}

/// 生成翻页按钮的回调数据字符串。
///
/// # 参数
/// - `filter`: 筛选条件
/// - `limit`: 单页条数
/// - `page`: 目标跳转页码
pub(super) fn build_downloads_page_callback_data(
    filter: DownloadsFilter,
    limit: u64,
    page: u64,
) -> String {
    build_downloads_callback_data(DownloadsCallbackAction::Page, filter, limit, page)
}

/// 生成刷新当前下载列表按钮的回调数据。
///
/// # 参数
/// - `args`: 当前页面参数结构体引用
pub(super) fn build_downloads_refresh_callback_data(args: &DownloadsArgs) -> String {
    build_downloads_callback_data(
        DownloadsCallbackAction::Refresh,
        args.filter,
        args.limit,
        args.page,
    )
}

/// 生成切换筛选条件按钮的回调数据（默认切到第 1 页）。
///
/// # 参数
/// - `filter`: 目标筛选条件
/// - `limit`: 单页容量
pub(super) fn build_downloads_filter_callback_data(filter: DownloadsFilter, limit: u64) -> String {
    build_downloads_callback_data(DownloadsCallbackAction::Filter, filter, limit, 1)
}

/// 组装短格式的下载列表按钮回调 payload 字符串。
/// 格式为：`d:{action}:{filter}:{limit}:{page}`
///
/// # 参数
/// - `action`: 按钮动作类型
/// - `filter`: 状态筛选器
/// - `limit`: 单页条目数（限制在 1~20 之间）
/// - `page`: 目标页码（至少为 1）
fn build_downloads_callback_data(
    action: DownloadsCallbackAction,
    filter: DownloadsFilter,
    limit: u64,
    page: u64,
) -> String {
    let action = match action {
        DownloadsCallbackAction::Page => "p",
        DownloadsCallbackAction::Refresh => "r",
        DownloadsCallbackAction::Filter => "f",
    };
    format!(
        "{}{}:{}:{}:{}",
        DOWNLOADS_CALLBACK_PREFIX,
        action,
        filter.command_value(),
        limit.clamp(1, 20),
        page.max(1)
    )
}

/// 解析按钮回调数据字符串，还原为动作枚举与分页参数结构体。
///
/// # 参数
/// - `data`: 原始回调数据字符串
pub(super) fn parse_downloads_callback_data(
    data: &str,
) -> Option<(DownloadsCallbackAction, DownloadsArgs)> {
    // 检查并剥离 "d:" 前缀
    let payload = data.strip_prefix(DOWNLOADS_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    // 解析动作类型
    let action = match parts.next()? {
        "p" => DownloadsCallbackAction::Page,
        "r" => DownloadsCallbackAction::Refresh,
        "f" => DownloadsCallbackAction::Filter,
        _ => return None,
    };
    // 解析筛选条件
    let filter = DownloadsFilter::parse(parts.next()?)?;
    // 解析每页数量并钳制在合法范围
    let limit = parts.next()?.parse::<u64>().ok()?.clamp(1, 20);
    // 解析目标页码
    let page = parts.next()?.parse::<u64>().ok()?.max(1);
    // 校验不能有多余字段段
    if parts.next().is_some() {
        return None;
    }
    Some((
        action,
        DownloadsArgs {
            filter,
            limit,
            page,
        },
    ))
}

/// 构建下载列表内联交互键盘。
///
/// 规则：
/// - 主操作区优先放任务详情、控制和筛选
/// - “刷新 / 任务中心 / 查看命令 / 菜单”固定为单独一行
/// - 复制类按钮固定单独一行
/// - 分页固定单独一行，放在最末尾
/// - 当前页/当前筛选/边界页同样允许点击刷新；发送层会把“消息未修改”当成幂等成功处理
///
/// # 参数
/// - `args`: 当前下载参数状态
/// - `total_pages`: 总页码
/// - `page_items`: 当前页呈现的任务快照列表
pub(super) fn build_downloads_keyboard(
    args: &DownloadsArgs,
    total_pages: u64,
    page_items: &[store::JobProgressSnapshot],
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let prev_page = args.page.saturating_sub(1).max(1);
    let next_page = (args.page + 1).min(total_pages);

    let mut rows = Vec::new();

    // 1. 添加当前页具体任务的详情快捷入口按钮
    rows.extend(build_job_detail_buttons(page_items));
    // 2. 添加常用状态筛选快捷切换按钮行
    rows.extend(build_filter_button_rows(args));

    // 3. 全局功能导航行：刷新、任务中心、查看命令、主菜单
    rows.push(vec![
        build_callback_button(
            "刷新",
            &build_downloads_refresh_callback_data(args),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        build_callback_button(
            "任务中心",
            &build_menu_tasks_hub_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        build_callback_button(
            "查看命令",
            &super::super::build_help_button_data(Some("downloads")),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        build_callback_button(
            "菜单",
            &build_menu_home_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]);

    // 4. 分页控制行（置于底部）
    let mut navigation_row = Vec::new();
    if args.page > 1 {
        navigation_row.push(build_navigation_button("首页", args, 1));
        navigation_row.push(build_navigation_button("上页", args, prev_page));
    }
    // 当前页按钮（高亮展示，点击触发当前页刷新）
    navigation_row.push(build_callback_button(
        &format!("{}/{total_pages}", args.page),
        &build_downloads_refresh_callback_data(args),
        tdlib_rs::enums::ButtonStyle::Primary,
    ));
    if args.page < total_pages {
        navigation_row.push(build_navigation_button("下页", args, next_page));
        navigation_row.push(build_navigation_button("末页", args, total_pages));
    }
    rows.push(navigation_row);

    // 返回组装好的 ReplyMarkupInlineKeyboard
    tdlib_rs::types::ReplyMarkupInlineKeyboard {
        rows,
        force_reply: false,
    }
}

/// 构建当前页任务快捷操作按钮列表。
///
/// 每行放两个详情入口；暂停、恢复、停止统一进入详情页操作，避免列表按钮过密。
///
/// # 参数
/// - `page_items`: 当前页任务快照切片
fn build_job_detail_buttons(
    page_items: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let buttons = page_items
        .iter()
        .map(|snapshot| {
            let status = snapshot.job.status.as_str();
            // 运行中或排队中的任务使用 Primary 突出样式
            let style = if matches!(
                status,
                store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
            ) {
                tdlib_rs::enums::ButtonStyle::Primary
            } else {
                tdlib_rs::enums::ButtonStyle::Default
            };
            let job_id = snapshot.job.id;
            send::build_callback_button(
                &format!("详情 #{job_id}"),
                &build_job_status_callback_data(job_id),
                style,
            )
        })
        .collect::<Vec<_>>();
    // 每行最多容纳 2 个详情按钮
    buttons.chunks(2).map(<[_]>::to_vec).collect()
}

/// 构建常用筛选按钮行。
///
/// 只展示高频聚合状态；下载、上传、就绪等细分阶段仍保留命令筛选能力。
///
/// # 参数
/// - `args`: 当前下载参数状态引用
fn build_filter_button_rows(
    args: &DownloadsArgs,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    [
        // 第一行筛选维度：全部、运行中、暂停
        [
            ("全部", DownloadsFilter::All),
            ("运行", DownloadsFilter::Running),
            ("暂停", DownloadsFilter::Paused),
        ]
        .as_slice(),
        // 第二行筛选维度：成功、失败、已停止
        [
            ("成功", DownloadsFilter::Success),
            ("失败", DownloadsFilter::Failed),
            ("已停止", DownloadsFilter::Cancelled),
        ]
        .as_slice(),
    ]
    .into_iter()
    .map(|filters| {
        filters
            .iter()
            .copied()
            .map(|(label, filter)| build_filter_button(label, filter, args))
            .collect::<Vec<_>>()
    })
    .collect()
}

/// 构建单个筛选按钮。
///
/// 当前激活的筛选模式呈现高亮风格，再次点击视为刷新。
///
/// # 参数
/// - `label`: 按钮文案
/// - `filter`: 该按钮代表的筛选条件
/// - `args`: 当前列表的查询参数
fn build_filter_button(
    label: &str,
    filter: DownloadsFilter,
    args: &DownloadsArgs,
) -> tdlib_rs::types::InlineKeyboardButton {
    let callback_data = if filter == args.filter {
        build_downloads_refresh_callback_data(args)
    } else {
        build_downloads_filter_callback_data(filter, args.limit)
    };
    build_callback_button(
        label,
        &callback_data,
        if filter == args.filter {
            tdlib_rs::enums::ButtonStyle::Primary
        } else {
            tdlib_rs::enums::ButtonStyle::Default
        },
    )
}

/// 构建一个翻页导航按钮。
///
/// 若目标页与当前页相同，则点击后会触发同页刷新；发送层会把无变化编辑视为幂等成功。
///
/// # 参数
/// - `text`: 按钮显示文案（如 "首页"、"上页" 等）
/// - `args`: 当前查询参数
/// - `target_page`: 目标跳转页码
fn build_navigation_button(
    text: &str,
    args: &DownloadsArgs,
    target_page: u64,
) -> tdlib_rs::types::InlineKeyboardButton {
    let callback_data = if target_page == args.page {
        build_downloads_refresh_callback_data(args)
    } else {
        build_downloads_page_callback_data(args.filter, args.limit, target_page)
    };
    // TDLib JSON 协议里的 callback data 是 bytes，必须走统一入口做 base64 编码。
    // 手动塞入业务 payload 会触发 `Wrong padding length`。
    send::build_callback_button(text, &callback_data, tdlib_rs::enums::ButtonStyle::Default)
}

/// 构建一个标准 callback 内联按钮。
///
/// # 参数
/// - `text`: 按钮标题
/// - `data`: 回调业务载荷
/// - `style`: 按钮样式（默认或主高亮）
fn build_callback_button(
    text: &str,
    data: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(text, data, style)
}
