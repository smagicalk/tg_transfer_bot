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
    let text = build_help_index_text(true);
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

// 普通用户 help 正文不展示管理命令，避免文本提示和实际权限不一致。
#[test]
fn test_build_help_index_text_for_user_hides_admin_commands() {
    let text = build_help_index_text(false);

    assert!(text.contains("/transfer"));
    assert!(text.contains("/downloads"));
    assert!(text.contains("/balance"));
    assert!(text.contains("/job"));
    assert!(!text.contains("/config"));
    assert!(!text.contains("/health"));
    assert!(!text.contains("/cache"));
    assert!(!text.contains("管理员示例"));
}

// 详细帮助应能分别展开不同命令。
#[test]
fn test_build_help_detail_text() {
    let transfer = build_help_detail_text("transfer").unwrap();
    assert!(transfer.contains("/transfer <link> [target]"));
    let transfer_slash = build_help_detail_text("/transfer").unwrap();
    assert!(transfer_slash.contains("/transfer <link> [target]"));

    let downloads = build_help_detail_text("downloads").unwrap();
    assert!(downloads.contains(
        "all | wait | dl | up | done | ok | fail | run | ready | pause | cancelling | cancel"
    ));
    let downloads_full = build_help_detail_text("downloads").unwrap();
    assert!(downloads_full.contains("/downloads [filter] [limit] [page]"));

    let health = build_help_detail_text("health").unwrap();
    assert!(health.contains("/health"));

    let cache = build_help_detail_text("cache").unwrap();
    assert!(cache.contains("/cache"));
    assert!(cache.contains("/cache page"));

    let points = build_help_detail_text("points").unwrap();
    assert!(points.contains("/balance"));
    assert!(points.contains("/points show 123456789"));
    assert!(points.contains("/points add 123456789 10 admin_adjust"));
    let balance = build_help_detail_text("balance").unwrap();
    assert!(balance.contains("/balance"));

    let job = build_help_detail_text("job").unwrap();
    assert!(job.contains("/job pause 123"));
    assert!(job.contains("/job status 123"));

    let config = build_help_detail_text("config").unwrap();
    assert!(config.contains("/config set job_concurrency 4"));
    assert!(config.contains("progress_edit_interval_seconds"));
    assert!(config.contains("downloads_default_page_size"));
    assert!(config.contains("menu_input_timeout_seconds"));

    let menu = build_help_detail_text("menu").unwrap();
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
    let buttons = build_help_index_buttons(true);
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

// help 目录页保留少量长命令复制入口，主要依赖 callback 导航切页。
#[test]
fn test_help_index_buttons_keep_shortcuts_compact() {
    let buttons = build_help_index_buttons(true);
    let labels = buttons
        .iter()
        .flatten()
        .map(|button| button.text.as_str())
        .collect::<Vec<_>>();

    assert!(labels.contains(&"复制流水"));
    assert!(labels.contains(&"复制 /config show"));
    assert!(labels.contains(&"复制 /cache"));
    assert!(!labels.contains(&"复制 /t"));
    assert!(!labels.contains(&"复制 /d"));
    assert!(!labels.contains(&"复制 /cfg"));
    assert!(labels.contains(&"转存"));
    assert!(labels.contains(&"下载列表"));
}

// help 目录页应遵循统一层级：主导航 -> 管理导航 -> 刷新/返回/菜单 -> 复制类。
#[test]
fn test_help_index_buttons_follow_row_hierarchy() {
    let admin = build_help_index_buttons(true);
    let user = build_help_index_buttons(false);

    assert_eq!(admin[0][0].text, "转存");
    assert_eq!(admin[1][0].text, "积分账户");
    assert_eq!(admin[3][0].text, "运行健康");
    let admin_labels = admin
        .iter()
        .flatten()
        .map(|button| button.text.as_str())
        .collect::<Vec<_>>();
    let admin_refresh_row = admin
        .iter()
        .find(|row| row.iter().any(|button| button.text == "刷新"))
        .expect("admin help should have refresh row");
    assert_eq!(admin_refresh_row[0].text, "刷新");
    assert_eq!(admin_refresh_row[1].text, "帮助说明");
    assert_eq!(admin_refresh_row[2].text, "菜单");
    assert!(admin_labels.contains(&"复制流水"));
    assert!(admin_labels.contains(&"复制 /health"));
    assert!(admin_labels.contains(&"复制 /cache"));

    assert_eq!(user[0][0].text, "转存");
    assert_eq!(user[1][0].text, "积分账户");
    let user_refresh_row = user
        .iter()
        .find(|row| row.iter().any(|button| button.text == "刷新"))
        .expect("user help should have refresh row");
    assert_eq!(user_refresh_row[0].text, "刷新");
    assert_eq!(user_refresh_row[1].text, "帮助说明");
    assert_eq!(user_refresh_row[2].text, "菜单");
    assert!(
        user.iter()
            .flatten()
            .any(|button| button.text == "复制流水")
    );
    assert!(
        !user
            .iter()
            .flatten()
            .any(|button| button.text == "复制帮助命令")
    );
}

// 普通用户帮助目录不展示 admin-only 命令入口，避免按钮提示和实际权限不一致。
#[test]
fn test_help_index_buttons_for_user_hide_admin_entries() {
    let buttons = build_help_index_buttons(false);
    let labels = buttons
        .iter()
        .flatten()
        .map(|button| button.text.as_str())
        .collect::<Vec<_>>();

    assert!(labels.contains(&"复制流水"));
    assert!(labels.contains(&"积分账户"));
    assert!(!labels.contains(&"复制 /config show"));
    assert!(!labels.contains(&"复制 /health"));
    assert!(!labels.contains(&"复制 /cache"));
    assert!(!labels.contains(&"运行配置"));
    assert!(!labels.contains(&"运行健康"));
    assert!(!labels.contains(&"文件缓存"));
}

// help 目录页的“帮助说明”只应保留一个入口，避免同页重复表达。
#[test]
fn test_help_index_buttons_only_one_help_entry() {
    let buttons = build_help_index_buttons(true);
    let help_count = buttons
        .iter()
        .flatten()
        .filter(|button| button.text == "帮助说明")
        .count();

    assert_eq!(help_count, 1);
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
