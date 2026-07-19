// 转存成功或命中历史结果时的回复卡片。
// 成功卡片提供结果入口和任务/列表导航；查询、重转命令保留在正文里作为降级入口。

use super::super::card;
use super::super::command::build_menu_home_button_data;
use super::super::command::common::{
    CommandStyle, downloads_command as build_downloads_command,
    lookup_command as build_lookup_command, transfer_command as build_transfer_command,
};
use super::super::command::{build_job_status_button_data, require_downloads_filter_button_data};
use super::super::store::ResultMessageRecord;

/// 发送“命中历史结果 / 已完成”的结果卡片。
pub(in crate::tgbot::transfer) async fn send_history_hit_message(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    result_link: &str,
    notify_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let result_messages = super::super::store::list_result_messages_by_job(job_id)
        .await
        .unwrap_or_else(|err| {
            tracing::warn!(
                job_id,
                error = %err,
                "load result messages failed, fallback to primary result link"
            );
            Vec::new()
        });
    let result_messages = normalize_result_messages(result_messages, result_link, target_chat_id);
    let mut rows = build_result_message_rows(&result_messages);
    rows.extend(build_result_navigation_rows(
        Some(job_id),
        "查看完成列表",
        "done",
    ));

    let mut panel = crate::tgbot::send::ReplyPanel::card(format_result_card_text(
        title,
        source_link,
        target_chat_id,
        Some(job_id),
        &result_messages,
    ));
    for row in rows {
        panel = panel.row(row);
    }
    panel.send(notify_chat_id, client_id).await
}

/// 构造结果卡片任务导航行。
///
/// 结果页第一条导航行只保留任务详情入口，列表和菜单统一落到下一行，避免同页重复出现菜单按钮。
fn build_result_job_row(job_id: i64) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![crate::tgbot::send::build_callback_button(
        "查看任务详情",
        &build_job_status_button_data(job_id),
        tdlib_rs::enums::ButtonStyle::Default,
    )]
}

/// 构造结果卡片第二行：进入列表、菜单。
pub(in crate::tgbot::transfer) fn build_list_menu_row(
    list_label: &str,
    list_filter: &str,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![
        crate::tgbot::send::build_callback_button(
            list_label,
            &require_downloads_filter_button_data(list_filter, 8),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        crate::tgbot::send::build_callback_button(
            "菜单",
            &build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]
}

/// 构造结果页的统一导航行。
///
/// 结果入口在最上面，随后固定是“任务详情”一行，再是“列表 + 菜单”一行；
/// 这样 `progress`、`success`、`lookup` 三类结果页能保持同一层级。
pub(in crate::tgbot::transfer) fn build_result_navigation_rows(
    job_id: Option<i64>,
    list_label: &str,
    list_filter: &str,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = Vec::new();
    if let Some(job_id) = job_id {
        rows.push(build_result_job_row(job_id));
    }
    rows.push(build_list_menu_row(list_label, list_filter));
    rows
}

/// 构造结果卡片正文。
///
/// 正文只对 HTTP(S) 结果使用 TDLib 原生文本链接；旧的 `tg://openmessage`
/// 或纯定位字符串只作为代码字段展示，避免客户端显示成不可用链接。
pub(in crate::tgbot::transfer) fn format_result_card_text(
    title: &str,
    source_link: &str,
    target_chat_id: i64,
    job_id: Option<i64>,
    result_messages: &[ResultMessageRecord],
) -> String {
    let mut lines = vec![
        title.to_owned(),
        card::summary_line("success", job_id, target_chat_id),
        card::DIVIDER.to_owned(),
        format_result_messages_block(result_messages),
        card::section("命令"),
        card::command_line(
            "查询",
            build_lookup_command(source_link, target_chat_id, CommandStyle::Long),
        ),
        card::command_line(
            "重转",
            build_transfer_command(source_link, target_chat_id, CommandStyle::Long),
        ),
        card::command_line(
            "列表",
            build_downloads_command(Some("done"), None, None, CommandStyle::Long),
        ),
        String::new(),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

/// 将结果表缺失的旧数据补成单条结果，保证旧任务仍能正常显示。
pub(in crate::tgbot::transfer) fn normalize_result_messages(
    mut result_messages: Vec<ResultMessageRecord>,
    fallback_link: &str,
    target_chat_id: i64,
) -> Vec<ResultMessageRecord> {
    if !result_messages.is_empty() {
        return result_messages;
    }

    result_messages.push(ResultMessageRecord {
        result_index: 0,
        target_chat_id,
        message_id: 0,
        message_link: fallback_link.to_owned(),
        is_album: false,
        item_count: 1,
    });
    result_messages
}

/// 构造多结果正文块。
fn format_result_messages_block(result_messages: &[ResultMessageRecord]) -> String {
    if result_messages.len() == 1 {
        return card::result_block(&result_messages[0].message_link);
    }

    let mut lines = vec![
        card::section("结果"),
        format!("共 {} 个结果入口", card::code(result_messages.len())),
    ];
    for result in result_messages {
        let label = format!(
            "#{} · {} 条{}",
            result.result_index + 1,
            result.item_count,
            if result.is_album { " · album" } else { "" }
        );
        if crate::tgbot::send::is_openable_url(&result.message_link) {
            lines.push(format!(
                "{}：{}",
                label,
                card::link("打开", &result.message_link)
            ));
            lines.push(format!("链接：{}", card::code(&result.message_link)));
        } else {
            lines.push(format!("{}：{}", label, card::code(&result.message_link)));
        }
    }
    lines.join("\n")
}

/// 构造结果入口按钮。
///
/// 开放 URL 已经同时出现在正文里，按钮区只保留“打开”动作；
/// 不可点击的定位字符串已经在正文里完整展示，这里不再重复给复制按钮。
pub(in crate::tgbot::transfer) fn build_result_message_rows(
    result_messages: &[ResultMessageRecord],
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = Vec::new();
    for result in result_messages.iter().take(6) {
        let idx = result.result_index + 1;
        if crate::tgbot::send::is_openable_url(&result.message_link) {
            rows.push(vec![crate::tgbot::send::build_url_button(
                &format!("打开结果 {}", idx),
                &result.message_link,
                if idx == 1 {
                    tdlib_rs::enums::ButtonStyle::Primary
                } else {
                    tdlib_rs::enums::ButtonStyle::Default
                },
            )]);
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::{
        ResultMessageRecord, build_list_menu_row, build_result_job_row, build_result_message_rows,
        build_result_navigation_rows, format_result_card_text,
    };

    // HTTP(S) 结果应在正文中渲染为 Telegram 原生文本链接，按钮之外也能点击。
    #[test]
    fn test_format_result_card_text_uses_card_link_for_openable_result() {
        let text = format_result_card_text(
            "转存完成",
            "https://t.me/c/1/2",
            -5106953357,
            Some(42),
            &[ResultMessageRecord {
                result_index: 0,
                target_chat_id: -5106953357,
                message_id: 734,
                message_link: "https://t.me/c/5106953357/734".to_owned(),
                is_album: true,
                item_count: 10,
            }],
        );

        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("【打开转存消息】(https://t.me/c/5106953357/734)"));
        assert!(text.contains("链接：‹https://t.me/c/5106953357/734›"));
        assert!(text.contains("查询：‹/lookup https://t.me/c/1/2 -5106953357›"));
        assert!(text.contains("重转：‹/transfer https://t.me/c/1/2 -5106953357›"));
        assert!(text.contains("列表：‹/downloads done›"));
    }

    // 不可打开的定位信息只能作为代码展示，不能伪装成可点击链接。
    #[test]
    fn test_format_result_card_text_keeps_locator_as_code() {
        let text = format_result_card_text(
            "转存完成",
            "https://t.me/c/1/2",
            -5106953357,
            Some(42),
            &[ResultMessageRecord {
                result_index: 0,
                target_chat_id: -5106953357,
                message_id: 769654784,
                message_link: "chat_id=-5106953357 message_id=769654784".to_owned(),
                is_album: false,
                item_count: 1,
            }],
        );

        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("无可跳转消息链接"));
        assert!(text.contains("定位：‹chat_id=-5106953357 message_id=769654784›"));
        assert!(!text.contains("【打开转存消息】("));
    }

    // 超过 10 条拆成多个 album 时，正文必须列出所有结果入口。
    #[test]
    fn test_format_result_card_text_lists_multiple_results() {
        let text = format_result_card_text(
            "转存完成",
            "https://t.me/c/1/2",
            -5106953357,
            Some(42),
            &[
                ResultMessageRecord {
                    result_index: 0,
                    target_chat_id: -5106953357,
                    message_id: 734,
                    message_link: "https://t.me/c/1/734".to_owned(),
                    is_album: true,
                    item_count: 9,
                },
                ResultMessageRecord {
                    result_index: 1,
                    target_chat_id: -5106953357,
                    message_id: 735,
                    message_link: "https://t.me/c/1/735".to_owned(),
                    is_album: true,
                    item_count: 2,
                },
            ],
        );

        assert!(text.contains("共 ‹2› 个结果入口"));
        assert!(text.contains("#1 · 9 条 · album"));
        assert!(text.contains("#2 · 2 条 · album"));
        assert!(text.contains("https://t.me/c/1/734"));
        assert!(text.contains("https://t.me/c/1/735"));
    }

    // 结果卡片的列表行应提供完成列表和主菜单入口，便于继续操作。
    #[test]
    fn test_build_result_list_row_has_menu_button() {
        let row = build_list_menu_row("查看完成列表", "done");

        assert!(row.iter().any(|button| button.text == "查看完成列表"));
        let menu = row
            .iter()
            .find(|button| button.text == "菜单")
            .expect("result list row should have menu button");
        assert!(matches!(
            menu.r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    // 成功卡片的任务行只保留可直接点击的 callback，不再重复复制查询或重转命令。
    #[test]
    fn test_build_result_job_row_uses_callback_buttons_only() {
        let row = build_result_job_row(42);

        assert!(row.iter().any(|button| button.text == "查看任务详情"));
        assert!(!row.iter().any(|button| button.text == "菜单"));
        assert!(!row.iter().any(|button| button.text == "复制查询命令"));
        assert!(!row.iter().any(|button| button.text == "复制重新转存"));
    }

    // 成功结果页应固定为“详情”一行 + “列表/菜单”一行，避免按钮层级漂移。
    #[test]
    fn test_build_result_navigation_rows_follow_result_page_hierarchy() {
        let rows = build_result_navigation_rows(Some(42), "查看完成列表", "done");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 1);
        assert_eq!(rows[0][0].text, "查看任务详情");
        assert_eq!(rows[1].len(), 2);
        assert_eq!(rows[1][0].text, "查看完成列表");
        assert_eq!(rows[1][1].text, "菜单");
    }

    // 可点击的结果链接已经在正文可见，按钮区只保留直接打开动作，避免重复堆叠复制按钮。
    #[test]
    fn test_build_result_message_rows_for_openable_link_uses_open_only() {
        let rows = build_result_message_rows(&[ResultMessageRecord {
            result_index: 0,
            target_chat_id: -5106953357,
            message_id: 734,
            message_link: "https://t.me/c/5106953357/734".to_owned(),
            is_album: true,
            item_count: 10,
        }]);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].len(), 1);
        assert_eq!(rows[0][0].text, "打开结果 1");
        assert!(matches!(
            rows[0][0].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Url(_)
        ));
    }

    // 不可点击的定位字符串已经在正文里完整展示，按钮区不再重复给复制入口。
    #[test]
    fn test_build_result_message_rows_for_locator_has_no_extra_button() {
        let rows = build_result_message_rows(&[ResultMessageRecord {
            result_index: 0,
            target_chat_id: -5106953357,
            message_id: 769654784,
            message_link: "chat_id=-5106953357 message_id=769654784".to_owned(),
            is_album: false,
            item_count: 1,
        }]);

        assert!(rows.is_empty());
    }
}
