// `/downloads` 的 inline keyboard 和 callback 数据。
// 回调数据保持短格式，避免 Telegram callback payload 过长。

use super::super::common::{
    CommandStyle, build_refresh_return_menu_row, downloads_command as build_command,
};
use super::super::job::{
    build_job_pause_callback_data, build_job_resume_callback_data, build_job_status_callback_data,
    build_job_stop_callback_data,
};
use super::super::menu::{build_menu_downloads_callback_data, build_menu_home_callback_data};
use super::types::{DownloadsArgs, DownloadsFilter};
use crate::tgbot::send;
use crate::tgbot::transfer::store;

/// `/downloads` 按钮回调前缀。
const DOWNLOADS_CALLBACK_PREFIX: &str = "d:";

/// 生成翻页命令，供当前文本页脚直接复用。
pub(super) fn build_downloads_page_command(
    filter: DownloadsFilter,
    limit: u64,
    page: u64,
    style: CommandStyle,
) -> String {
    let filter = if filter == DownloadsFilter::All {
        None
    } else {
        Some(filter.command_value())
    };
    build_command(filter, Some(limit), Some(page), style)
}

/// 下载列表按钮动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DownloadsCallbackAction {
    Page,
    Refresh,
    Filter,
}

/// 生成翻页按钮回调数据。
pub(super) fn build_downloads_page_callback_data(
    filter: DownloadsFilter,
    limit: u64,
    page: u64,
) -> String {
    build_downloads_callback_data(DownloadsCallbackAction::Page, filter, limit, page)
}

/// 生成刷新按钮回调数据。
pub(super) fn build_downloads_refresh_callback_data(args: &DownloadsArgs) -> String {
    build_downloads_callback_data(
        DownloadsCallbackAction::Refresh,
        args.filter,
        args.limit,
        args.page,
    )
}

/// 生成筛选按钮回调数据。
pub(super) fn build_downloads_filter_callback_data(filter: DownloadsFilter, limit: u64) -> String {
    build_downloads_callback_data(DownloadsCallbackAction::Filter, filter, limit, 1)
}

/// 生成按钮回调数据。
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

/// 解析按钮回调数据，还原为分页参数。
pub(super) fn parse_downloads_callback_data(
    data: &str,
) -> Option<(DownloadsCallbackAction, DownloadsArgs)> {
    let payload = data.strip_prefix(DOWNLOADS_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    let action = match parts.next()? {
        "p" => DownloadsCallbackAction::Page,
        "r" => DownloadsCallbackAction::Refresh,
        "f" => DownloadsCallbackAction::Filter,
        _ => return None,
    };
    let filter = DownloadsFilter::parse(parts.next()?)?;
    let limit = parts.next()?.parse::<u64>().ok()?.clamp(1, 20);
    let page = parts.next()?.parse::<u64>().ok()?.max(1);
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

/// 构建下载列表分页键盘。
///
/// 规则：
/// - 主操作区优先放任务详情、控制和筛选
/// - “刷新 / 返回 / 菜单”固定为单独一行
/// - 复制类按钮固定单独一行
/// - 分页固定单独一行，放在最末尾
/// - 当前页/当前筛选/边界页同样允许点击刷新；发送层会把“消息未修改”当成幂等成功处理
pub(super) fn build_downloads_keyboard(
    args: &DownloadsArgs,
    total_pages: u64,
    page_items: &[store::JobProgressSnapshot],
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let first_page = 1u64;
    let prev_page = args.page.saturating_sub(1).max(1);
    let next_page = (args.page + 1).min(total_pages);
    let last_page = total_pages.max(1);

    let mut rows = Vec::new();

    rows.extend(build_job_detail_buttons(page_items));
    rows.extend(build_filter_button_rows(args));

    rows.push(build_refresh_return_menu_row(
        build_callback_button(
            "刷新",
            &build_downloads_refresh_callback_data(args),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        build_callback_button(
            "返回",
            &build_menu_downloads_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        build_callback_button(
            "菜单",
            &build_menu_home_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ));
    rows.push(vec![
        build_navigation_button("首页", args, first_page),
        build_navigation_button("上页", args, prev_page),
        build_callback_button(
            &format!("{}/{}", args.page, total_pages),
            &build_downloads_refresh_callback_data(args),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        build_navigation_button("下页", args, next_page),
        build_navigation_button("末页", args, last_page),
    ]);

    tdlib_rs::types::ReplyMarkupInlineKeyboard { rows }
}

/// 构建当前页任务快捷操作按钮。
///
/// 每个任务独占一行：左侧始终是详情，右侧根据状态给出暂停、恢复、停止等操作。
/// 这样用户在列表页就能直接控制任务，不需要先点详情再点控制按钮。
fn build_job_detail_buttons(
    page_items: &[store::JobProgressSnapshot],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    page_items
        .iter()
        .map(|snapshot| {
            let status = snapshot.job.status.as_str();
            let style = if matches!(
                status,
                store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
            ) {
                tdlib_rs::enums::ButtonStyle::Primary
            } else {
                tdlib_rs::enums::ButtonStyle::Default
            };
            let job_id = snapshot.job.id;

            let mut row = vec![send::build_callback_button(
                &format!("详情 #{}", job_id),
                &build_job_status_callback_data(job_id),
                style,
            )];
            row.extend(build_inline_job_control_buttons(job_id, status));
            row
        })
        .collect::<Vec<_>>()
}

/// 构建列表页中的任务控制按钮。
///
/// 控制按钮仍复用 `/job` callback，因此这里不直接修改任务状态。
fn build_inline_job_control_buttons(
    job_id: i64,
    status: &str,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    if matches!(
        status,
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
    ) {
        return vec![
            send::build_callback_button(
                "暂停",
                &build_job_pause_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ];
    }

    if status == store::JOB_STATUS_PAUSED {
        return vec![
            send::build_callback_button(
                "恢复",
                &build_job_resume_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "停止",
                &build_job_stop_callback_data(job_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ];
    }

    Vec::new()
}

/// 构建常用筛选按钮行。
///
/// 每行最多 3 个按钮，移动端比 5-6 个按钮挤在一行更稳定，也方便后续扩展状态。
fn build_filter_button_rows(
    args: &DownloadsArgs,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    [
        [
            ("全部", DownloadsFilter::All),
            ("运行", DownloadsFilter::Running),
            ("等待", DownloadsFilter::Waiting),
        ]
        .as_slice(),
        [
            ("下载", DownloadsFilter::Downloading),
            ("上传", DownloadsFilter::Uploading),
            ("就绪", DownloadsFilter::Ready),
        ]
        .as_slice(),
        [
            ("完成", DownloadsFilter::Finished),
            ("成功", DownloadsFilter::Success),
            ("失败", DownloadsFilter::Failed),
        ]
        .as_slice(),
        [
            ("暂停", DownloadsFilter::Paused),
            ("停止中", DownloadsFilter::Cancelling),
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

/// 构建一个导航按钮。
///
/// 若目标页与当前页相同，则点击后会触发同页刷新；发送层会把无变化编辑视为幂等成功。
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

/// 构建一个 callback 按钮。
fn build_callback_button(
    text: &str,
    data: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(text, data, style)
}
