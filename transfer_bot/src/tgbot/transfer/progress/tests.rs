// 进度面板的轻量单元测试。
// 这里只覆盖纯函数，避免测试依赖真实 TDLib 网络调用。

use super::keyboard::build_transfer_result_keyboard;
use super::text::format_progress_bytes;

// 字节格式化用于实时下载进度面板，应保持和 `/downloads` 类似的展示风格。
#[test]
fn test_format_progress_bytes() {
    assert_eq!(format_progress_bytes(100), "100 B");
    assert_eq!(format_progress_bytes(1024), "1.0 KB");
    assert_eq!(format_progress_bytes(1024 * 1024), "1.0 MB");
}

// 最终结果按钮应按成功/失败切换列表筛选命令。
#[test]
fn test_build_transfer_result_keyboard_uses_result_state_filter() {
    let success_keyboard =
        build_transfer_result_keyboard("https://t.me/c/1/2", -100, Some("https://t.me/c/3/4"));
    let fail_keyboard = build_transfer_result_keyboard("https://t.me/c/1/2", -100, None);

    let success_last = success_keyboard
        .rows
        .last()
        .and_then(|row| row.last())
        .expect("success keyboard must have last button");
    let fail_last = fail_keyboard
        .rows
        .last()
        .and_then(|row| row.last())
        .expect("fail keyboard must have last button");

    assert_eq!(success_last.text, "复制完成列表");
    assert_eq!(fail_last.text, "复制失败列表");
}

// 非 HTTP(S) 定位信息不能生成“打开转存消息”按钮，否则客户端点击可能无反应。
#[test]
fn test_build_transfer_result_keyboard_uses_copy_for_locator() {
    let keyboard = build_transfer_result_keyboard(
        "https://t.me/c/1/2",
        -5106953357,
        Some("chat_id=-5106953357 message_id=769654784"),
    );

    let first = keyboard
        .rows
        .first()
        .and_then(|row| row.first())
        .expect("keyboard must have first button");

    assert_eq!(first.text, "复制结果定位");
    assert!(matches!(
        first.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::CopyText(_)
    ));
}

// HTTP(S) 结果链接保留“打开转存消息”按钮，由 Telegram 客户端负责跳转。
#[test]
fn test_build_transfer_result_keyboard_uses_url_for_http_link() {
    let keyboard =
        build_transfer_result_keyboard("https://t.me/c/1/2", -100, Some("https://t.me/c/3/4"));

    let first = keyboard
        .rows
        .first()
        .and_then(|row| row.first())
        .expect("keyboard must have first button");

    assert_eq!(first.text, "打开转存消息");
    assert!(matches!(
        first.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Url(_)
    ));
}
