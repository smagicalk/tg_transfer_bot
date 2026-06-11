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
    assert!(text.contains("/balance"));
    assert!(text.contains("/points show"));
    assert!(text.contains("/health"));
    assert!(text.contains("/cache"));
    assert!(text.contains("/job"));
    assert!(text.contains("/menu"));
}

// 详细帮助应能分别展开不同命令。
#[test]
fn test_build_help_detail_text() {
    let transfer = build_help_detail_text("transfer").unwrap();
    assert!(transfer.contains("/transfer <link> [target]"));
    let transfer_short = build_help_detail_text("t").unwrap();
    assert!(transfer_short.contains("/transfer <link> [target]"));
    let transfer_slash = build_help_detail_text("/t").unwrap();
    assert!(transfer_slash.contains("/transfer <link> [target]"));

    let downloads = build_help_detail_text("downloads").unwrap();
    assert!(downloads.contains(
        "all | wait | dl | up | done | ok | fail | run | ready | pause | cancelling | cancel"
    ));
    let downloads_short = build_help_detail_text("d").unwrap();
    assert!(downloads_short.contains("/downloads [filter] [limit] [page]"));

    let health = build_help_detail_text("hl").unwrap();
    assert!(health.contains("/health"));

    let cache = build_help_detail_text("fc").unwrap();
    assert!(cache.contains("/cache"));
    assert!(cache.contains("/cache page"));

    let points = build_help_detail_text("points").unwrap();
    assert!(points.contains("/balance"));
    assert!(points.contains("/points show 123456789"));
    assert!(points.contains("/points add 123456789 10 admin_adjust"));
    let balance = build_help_detail_text("bal").unwrap();
    assert!(balance.contains("/balance"));

    let job = build_help_detail_text("j").unwrap();
    assert!(job.contains("/job pause 123"));
    assert!(job.contains("/job status 123"));

    let config = build_help_detail_text("config").unwrap();
    assert!(config.contains("/config set job_concurrency 4"));
    assert!(config.contains("progress_edit_interval_seconds"));
    assert!(config.contains("downloads_default_page_size"));
    assert!(config.contains("menu_input_timeout_seconds"));

    let menu = build_help_detail_text("m").unwrap();
    assert!(menu.contains("/menu"));
    assert!(menu.contains("/cancel"));

    assert!(build_help_detail_text("unknown").is_err());
}

// help callback 使用短 payload 原地切换页面。
#[test]
fn test_help_callback_data_roundtrip() {
    let transfer = build_help_callback_data(Some("transfer"));
    assert_eq!(transfer, "h:transfer");
    assert!(is_help_callback_data(&transfer));
    assert_eq!(parse_help_callback_data(&transfer), Some(Some("transfer")));
    assert_eq!(
        parse_help_callback_data(&build_help_callback_data(Some("health"))),
        Some(Some("health"))
    );
    assert_eq!(
        parse_help_callback_data(&build_help_callback_data(Some("cache"))),
        Some(Some("cache"))
    );
    assert_eq!(
        parse_help_callback_data(&build_help_callback_data(Some("points"))),
        Some(Some("points"))
    );

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
    let transfer = buttons
        .iter()
        .flatten()
        .find(|button| button.text == "转存")
        .expect("help index should have transfer navigation");
    let menu = buttons
        .iter()
        .flatten()
        .find(|button| button.text == "菜单")
        .expect("help index should have menu button");
    let points = buttons
        .iter()
        .flatten()
        .find(|button| button.text == "积分账户")
        .expect("help index should have points navigation");
    assert_eq!(transfer.text, "转存");
    assert!(matches!(
        transfer.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert!(matches!(
        menu.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert!(matches!(
        points.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

// help 详情页应保留返回目录 callback。
#[test]
fn test_help_detail_buttons_have_back_callback() {
    let buttons = build_help_detail_buttons("points").expect("points help buttons should build");
    let back = buttons
        .iter()
        .flatten()
        .find(|button| button.text == "返回目录")
        .expect("detail help should have back button");
    let menu = buttons
        .iter()
        .flatten()
        .find(|button| button.text == "菜单")
        .expect("detail help should have menu button");
    assert_eq!(back.text, "返回目录");
    assert!(matches!(
        back.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert!(matches!(
        menu.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}
