// `/downloads` 的 inline keyboard 和 callback 数据。
// 回调数据保持短格式，避免 Telegram callback payload 过长。

use super::super::common::{CommandStyle, downloads_command as build_command};
use super::super::job::build_job_status_callback_data;
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
/// - 可翻页按钮使用 callback，点击后原地刷新
/// - 当前页任务使用 callback，点击后直接进入单任务详情
/// - 当前页按钮使用 copy-text，方便直接复制当前页命令
/// - 不能继续翻页时退化为 copy-text，避免触发“消息未修改”
pub(super) fn build_downloads_keyboard(
    args: &DownloadsArgs,
    total_pages: u64,
    page_items: &[store::JobProgressSnapshot],
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let current_command =
        build_downloads_page_command(args.filter, args.limit, args.page, CommandStyle::Short);
    let first_page = 1u64;
    let prev_page = args.page.saturating_sub(1).max(1);
    let next_page = (args.page + 1).min(total_pages);
    let last_page = total_pages.max(1);

    let mut rows = vec![vec![
        build_navigation_button("首页", args, first_page, current_command.clone()),
        build_navigation_button("上页", args, prev_page, current_command.clone()),
        build_copy_button(
            &format!("{}/{}", args.page, total_pages),
            &build_downloads_page_command(args.filter, args.limit, args.page, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        build_navigation_button(
            "下页",
            args,
            next_page,
            build_downloads_page_command(args.filter, args.limit, args.page, CommandStyle::Short),
        ),
        build_navigation_button(
            "末页",
            args,
            last_page,
            build_downloads_page_command(args.filter, args.limit, args.page, CommandStyle::Short),
        ),
    ]];

    rows.extend(build_job_detail_buttons(page_items));

    rows.push(vec![
        build_callback_button(
            "刷新",
            &build_downloads_refresh_callback_data(args),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_copy_button(
            "复制当前命令",
            &current_command,
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]);
    rows.extend(build_filter_button_rows(args));

    tdlib_rs::types::ReplyMarkupInlineKeyboard { rows }
}

/// 构建当前页任务详情按钮。
///
/// 每个按钮只携带 job_id，让用户从下载列表直接跳到对应任务卡片。
/// 按两列排版可以减少一页任务较多时的键盘高度。
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

            send::build_callback_button(
                &format!("详情 #{}", snapshot.job.id),
                &build_job_status_callback_data(snapshot.job.id),
                style,
            )
        })
        .collect::<Vec<_>>()
        .chunks(2)
        .map(<[_]>::to_vec)
        .collect::<Vec<_>>()
}

/// 构建常用筛选按钮行。
///
/// 拆成两行是为了把任务控制相关状态也露出来，同时避免单行按钮过密。
fn build_filter_button_rows(
    args: &DownloadsArgs,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    [
        [
            ("全部", DownloadsFilter::All),
            ("运行", DownloadsFilter::Running),
            ("下载", DownloadsFilter::Downloading),
            ("上传", DownloadsFilter::Uploading),
            ("完成", DownloadsFilter::Finished),
            ("失败", DownloadsFilter::Failed),
        ]
        .as_slice(),
        [
            ("暂停", DownloadsFilter::Paused),
            ("停止中", DownloadsFilter::Cancelling),
            ("已停止", DownloadsFilter::Cancelled),
            ("就绪", DownloadsFilter::Ready),
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
    if filter == args.filter {
        return send::build_copy_button(
            label,
            &build_downloads_page_command(filter, args.limit, 1, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Primary,
        );
    }

    build_callback_button(
        label,
        &build_downloads_filter_callback_data(filter, args.limit),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// 构建一个导航按钮。
///
/// 若目标页与当前页相同，则退化为复制当前命令按钮，避免无效编辑。
fn build_navigation_button(
    text: &str,
    args: &DownloadsArgs,
    target_page: u64,
    fallback_command: String,
) -> tdlib_rs::types::InlineKeyboardButton {
    if target_page == args.page {
        return send::build_copy_button(
            text,
            &fallback_command,
            tdlib_rs::enums::ButtonStyle::Default,
        );
    }

    tdlib_rs::types::InlineKeyboardButton {
        text: text.to_owned(),
        icon_custom_emoji_id: 0,
        style: tdlib_rs::enums::ButtonStyle::Default,
        r#type: tdlib_rs::enums::InlineKeyboardButtonType::Callback(
            tdlib_rs::types::InlineKeyboardButtonTypeCallback {
                data: build_downloads_page_callback_data(args.filter, args.limit, target_page),
            },
        ),
    }
}

/// 构建一个 callback 按钮。
fn build_callback_button(
    text: &str,
    data: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(text, data, style)
}

/// 构建一个复制文本按钮。
fn build_copy_button(
    text: &str,
    value: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_copy_button(text, value, style)
}
