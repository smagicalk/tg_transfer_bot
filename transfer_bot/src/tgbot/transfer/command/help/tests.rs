// `/help` 文案测试。
// 这里覆盖公开命令目录和几个常用 topic 的展开结果。

use super::keyboard::{
    build_help_callback_data, build_help_detail_buttons, build_help_index_buttons,
    is_help_callback_data, parse_help_callback_data,
};
use super::text::{build_help_detail_text, build_help_index_text};

// help 目录页应包含所有公开命令入口。
#[test]
fn test_build_help_index_text_contains_commands() {
    let text = build_help_index_text();
    assert!(text.contains("/help"));
    assert!(text.contains("/transfer"));
    assert!(text.contains("/lookup"));
    assert!(text.contains("/config"));
    assert!(text.contains("/downloads"));
    assert!(text.contains("/job"));
}

// 详细帮助应能分别展开不同命令。
#[test]
fn test_build_help_detail_text() {
    let transfer = build_help_detail_text("transfer").unwrap();
    assert!(transfer.contains("/transfer <link> [target_chat_id]"));
    let transfer_short = build_help_detail_text("t").unwrap();
    assert!(transfer_short.contains("/transfer <link> [target_chat_id]"));
    let transfer_slash = build_help_detail_text("/t").unwrap();
    assert!(transfer_slash.contains("/transfer <link> [target_chat_id]"));

    let downloads = build_help_detail_text("downloads").unwrap();
    assert!(downloads.contains(
        "all | wait | dl | up | done | ok | fail | run | ready | pause | cancelling | cancel"
    ));
    let downloads_short = build_help_detail_text("d").unwrap();
    assert!(downloads_short.contains("/downloads [filter] [limit] [page]"));

    let job = build_help_detail_text("j").unwrap();
    assert!(job.contains("/job pause 123"));
    assert!(job.contains("/job status 123"));

    let config = build_help_detail_text("config").unwrap();
    assert!(config.contains("/config set job_concurrency 4"));

    assert!(build_help_detail_text("unknown").is_err());
}

// help callback 使用短 payload 原地切换页面。
#[test]
fn test_help_callback_data_roundtrip() {
    let transfer = build_help_callback_data(Some("transfer"));
    assert_eq!(transfer, "h:transfer");
    assert!(is_help_callback_data(&transfer));
    assert_eq!(parse_help_callback_data(&transfer), Some(Some("transfer")));

    let index = build_help_callback_data(None);
    assert_eq!(index, "h:index");
    assert_eq!(parse_help_callback_data(&index), Some(None));
    assert_eq!(parse_help_callback_data("h:bad"), None);
    assert_eq!(parse_help_callback_data("d:r:run:8:1"), None);
}

// help 目录页应提供 callback 导航按钮，而不是只能复制命令。
#[test]
fn test_help_index_buttons_have_navigation_callbacks() {
    let buttons = build_help_index_buttons();
    let transfer = &buttons[1][0];
    assert_eq!(transfer.text, "转存");
    assert!(matches!(
        transfer.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

// help 详情页应保留返回目录 callback。
#[test]
fn test_help_detail_buttons_have_back_callback() {
    let buttons = build_help_detail_buttons("job").expect("job help buttons should build");
    let back = buttons
        .first()
        .and_then(|row| row.last())
        .expect("detail help should have back button");
    assert_eq!(back.text, "返回目录");
    assert!(matches!(
        back.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}
