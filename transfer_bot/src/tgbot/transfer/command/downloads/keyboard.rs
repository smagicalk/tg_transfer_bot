// `/downloads` 的 inline keyboard 和 callback 数据。
// 回调数据保持短格式，避免 Telegram callback payload 过长。

use super::super::common::{CommandStyle, downloads_command as build_command};
use super::types::{DownloadsArgs, DownloadsFilter};
use crate::tgbot::send;

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

/// 生成按钮回调数据。
pub(super) fn build_downloads_callback_data(
    filter: DownloadsFilter,
    limit: u64,
    page: u64,
) -> String {
    format!(
        "{}{}:{}:{}",
        DOWNLOADS_CALLBACK_PREFIX,
        filter.command_value(),
        limit.clamp(1, 20),
        page.max(1)
    )
}

/// 解析按钮回调数据，还原为分页参数。
pub(super) fn parse_downloads_callback_data(data: &str) -> Option<DownloadsArgs> {
    let payload = data.strip_prefix(DOWNLOADS_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    let filter = DownloadsFilter::parse(parts.next()?)?;
    let limit = parts.next()?.parse::<u64>().ok()?.clamp(1, 20);
    let page = parts.next()?.parse::<u64>().ok()?.max(1);
    if parts.next().is_some() {
        return None;
    }
    Some(DownloadsArgs {
        filter,
        limit,
        page,
    })
}

/// 构建下载列表分页键盘。
///
/// 规则：
/// - 可翻页按钮使用 callback，点击后原地刷新
/// - 当前页按钮使用 copy-text，方便直接复制当前页命令
/// - 不能继续翻页时退化为 copy-text，避免触发“消息未修改”
pub(super) fn build_downloads_keyboard(
    args: &DownloadsArgs,
    total_pages: u64,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let current_command =
        build_downloads_page_command(args.filter, args.limit, args.page, CommandStyle::Short);
    let first_page = 1u64;
    let prev_page = args.page.saturating_sub(1).max(1);
    let next_page = (args.page + 1).min(total_pages);
    let last_page = total_pages.max(1);

    tdlib_rs::types::ReplyMarkupInlineKeyboard {
        rows: vec![vec![
            build_navigation_button("首页", args, first_page, current_command.clone()),
            build_navigation_button("上页", args, prev_page, current_command.clone()),
            build_copy_button(
                &format!("{}/{}", args.page, total_pages),
                &build_downloads_page_command(
                    args.filter,
                    args.limit,
                    args.page,
                    CommandStyle::Short,
                ),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            build_navigation_button(
                "下页",
                args,
                next_page,
                build_downloads_page_command(
                    args.filter,
                    args.limit,
                    args.page,
                    CommandStyle::Short,
                ),
            ),
            build_navigation_button(
                "末页",
                args,
                last_page,
                build_downloads_page_command(
                    args.filter,
                    args.limit,
                    args.page,
                    CommandStyle::Short,
                ),
            ),
        ]],
    }
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
                data: build_downloads_callback_data(args.filter, args.limit, target_page),
            },
        ),
    }
}

/// 构建一个复制文本按钮。
fn build_copy_button(
    text: &str,
    value: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_copy_button(text, value, style)
}
