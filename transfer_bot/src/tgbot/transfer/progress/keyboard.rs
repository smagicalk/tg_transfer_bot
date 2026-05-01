// 转存进度面板的按钮构造。
// 这里只生成 Telegram inline keyboard，具体发送/编辑由上层 progress 模块负责。

use crate::tgbot::transfer::command::common::{
    CommandStyle, downloads_command as build_downloads_command, job_command as build_job_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};

/// 构造进度面板按钮。
pub(super) fn build_transfer_progress_keyboard(
    job_id: Option<i64>,
    source_link: &str,
    target_chat_id: i64,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let lookup_command = build_lookup_command(source_link, target_chat_id, CommandStyle::Short);
    let mut rows = vec![vec![
        crate::tgbot::send::build_copy_button(
            "复制查询命令",
            &lookup_command,
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_copy_button(
            "复制运行列表",
            &build_downloads_command(Some("run"), None, None, CommandStyle::Short),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]];

    if let Some(job_id) = job_id {
        rows.push(vec![
            crate::tgbot::send::build_copy_button(
                "复制暂停",
                &build_job_command("p", job_id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            crate::tgbot::send::build_copy_button(
                "复制停止",
                &build_job_command("s", job_id, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ]);
    }

    crate::tgbot::send::build_inline_keyboard(rows)
}

/// 构造最终结果按钮。
pub(super) fn build_transfer_result_keyboard(
    source_link: &str,
    target_chat_id: i64,
    result_link: Option<&str>,
) -> tdlib_rs::types::ReplyMarkupInlineKeyboard {
    let lookup_command = build_lookup_command(source_link, target_chat_id, CommandStyle::Short);
    let retry_command = build_transfer_command(source_link, target_chat_id, CommandStyle::Short);
    let mut first_row = Vec::new();
    if let Some(result_link) = result_link {
        // 只有 TDLib 返回或本模块兜底生成的 HTTP(S) 链接才放“打开转存消息”按钮；客户端 deeplink 不稳定，
        // 放到 URL 按钮里会造成点击无反应。
        if crate::tgbot::send::is_openable_url(result_link) {
            first_row.push(crate::tgbot::send::build_url_button(
                "打开转存消息",
                result_link,
                tdlib_rs::enums::ButtonStyle::Primary,
            ));
        }
        first_row.push(crate::tgbot::send::build_copy_button(
            if crate::tgbot::send::is_openable_url(result_link) {
                "复制结果链接"
            } else {
                "复制结果定位"
            },
            result_link,
            if crate::tgbot::send::is_openable_url(result_link) {
                tdlib_rs::enums::ButtonStyle::Default
            } else {
                tdlib_rs::enums::ButtonStyle::Primary
            },
        ));
    }
    first_row.push(crate::tgbot::send::build_copy_button(
        "复制查询命令",
        &lookup_command,
        tdlib_rs::enums::ButtonStyle::Default,
    ));

    let list_filter = if result_link.is_some() {
        "done"
    } else {
        "fail"
    };
    crate::tgbot::send::build_inline_keyboard(vec![
        first_row,
        vec![
            crate::tgbot::send::build_copy_button(
                "复制重新转存",
                &retry_command,
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            crate::tgbot::send::build_copy_button(
                if result_link.is_some() {
                    "复制完成列表"
                } else {
                    "复制失败列表"
                },
                &build_downloads_command(Some(list_filter), None, None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ])
}
