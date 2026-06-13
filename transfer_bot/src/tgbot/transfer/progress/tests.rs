// 进度面板的轻量单元测试。
// 这里只覆盖纯函数，避免测试依赖真实 TDLib 网络调用。

use super::keyboard::{build_transfer_progress_keyboard, build_transfer_result_keyboard};
use super::text::{
    format_progress_bytes, format_transfer_control_text, format_transfer_progress_text,
    format_transfer_waiting_text,
};
use crate::config::{ActorRole, BillingConfig, ClientRole, RequestActor};
use crate::tgbot::transfer::types::SourceKind;
use crate::tgbot::transfer::{store, types};

// 字节格式化用于实时下载进度面板，应保持和 `/downloads` 类似的展示风格。
#[test]
fn test_format_progress_bytes() {
    assert_eq!(format_progress_bytes(100), "100 B");
    assert_eq!(format_progress_bytes(1024), "1.0 KB");
    assert_eq!(format_progress_bytes(1024 * 1024), "1.0 MB");
}

// 等待面板应给出当前阶段和源链接，方便用户确认后台是否已经接收任务。
#[test]
fn test_format_transfer_waiting_text() {
    let text = format_transfer_waiting_text(&types::TransferPlan {
        actor: RequestActor {
            request_chat_id: 10,
            user_id: 10,
            role: ActorRole::Admin,
        },
        source_link: "https://t.me/c/1/2".to_owned(),
        source_kind: SourceKind::Link,
        preferred_source_client_role: ClientRole::Bot,
        allow_user_fallback: true,
        billing: BillingConfig::default(),
        source_message_chat_id: None,
        source_message_id: None,
        target_chat_id: -100,
        request_chat_id: 10,
        request_message_id: 20,
    });

    assert!(text.contains("转存进度 · 等待"));
    assert!(text.contains("状态：‹waiting›"));
    assert!(text.contains("■ 当前阶段"));
    assert!(text.contains("■ 源链接"));
}

// 运行中面板应展示聚合进度和 TDLib 实时下载进度。
#[test]
fn test_format_transfer_progress_text_card_layout() {
    let mut snapshot = snapshot_with_status(store::JOB_STATUS_RUNNING);
    snapshot.active_download_files = 1;
    snapshot.active_downloaded_bytes = 1024;
    snapshot.active_download_total_bytes = 2048;

    let text = format_transfer_progress_text(&snapshot, "https://t.me/c/1/2");

    assert!(text.contains("转存进度 ‹#42›"));
    assert!(text.contains("状态：‹running›  job：‹#42›  目标：‹-100›"));
    assert!(text.contains("总进度：‹1/3›"));
    assert!(text.contains("完成率：‹|||||||------------- 33%›"));
    assert!(text.contains("等待/下载：‹1/0›"));
    assert!(text.contains("成功/失败：‹1/0›"));
    assert!(text.contains("真实下载：1 个文件 1.0 KB/2.0 KB"));
    assert!(text.contains("||||||||||---------- 50%"));
    assert!(text.contains("■ 命令"));
    assert!(text.contains("详情：‹/j st 42›"));
    assert!(text.contains("暂停：‹/j p 42›"));
    assert!(text.contains("停止：‹/j s 42›"));
    assert!(text.contains("列表：‹/d run›"));
    assert!(text.contains("查询：‹/lk https://t.me/c/1/2 -100›"));
}

// 控制态正文也要包含命令，避免用户号模式隐藏按钮后无法操作。
#[test]
fn test_format_transfer_control_text_contains_commands() {
    let text = format_transfer_control_text(
        "相同链接正在转存中",
        "https://t.me/c/1/2",
        -100,
        42,
        "可以继续观察当前进度。",
    );

    assert!(text.contains("■ 命令"));
    assert!(text.contains("详情：‹/j st 42›"));
    assert!(text.contains("暂停：‹/j p 42›"));
    assert!(text.contains("恢复：‹/j r 42›"));
    assert!(text.contains("停止：‹/j s 42›"));
}

// 最终结果按钮应按成功/失败切换列表筛选命令。
#[test]
fn test_build_transfer_result_keyboard_uses_result_state_filter() {
    let success_keyboard = build_transfer_result_keyboard(
        "https://t.me/c/1/2",
        -100,
        Some(42),
        Some("https://t.me/c/3/4"),
    );
    let fail_keyboard = build_transfer_result_keyboard("https://t.me/c/1/2", -100, None, None);

    assert_eq!(success_keyboard.rows[1][2].text, "复制列表命令");
    assert_eq!(fail_keyboard.rows[1][2].text, "复制列表命令");
    assert_eq!(success_keyboard.rows[1][1].text, "查看完成列表");
    assert_eq!(fail_keyboard.rows[1][1].text, "查看失败列表");
    assert_eq!(success_keyboard.rows[1][3].text, "菜单");
    assert_eq!(fail_keyboard.rows[1][3].text, "菜单");
    assert!(matches!(
        success_keyboard.rows[1][1].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert!(matches!(
        success_keyboard.rows[1][3].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

// 非 HTTP(S) 定位信息不能生成“打开转存消息”按钮，否则客户端点击可能无反应。
#[test]
fn test_build_transfer_result_keyboard_uses_copy_for_locator() {
    let keyboard = build_transfer_result_keyboard(
        "https://t.me/c/1/2",
        -5106953357,
        Some(42),
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
    let keyboard = build_transfer_result_keyboard(
        "https://t.me/c/1/2",
        -100,
        Some(42),
        Some("https://t.me/c/3/4"),
    );

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

// 运行中进度面板应支持直接跳任务详情和运行列表，减少复制命令操作。
#[test]
fn test_build_transfer_progress_keyboard_has_callback_buttons() {
    let keyboard = build_transfer_progress_keyboard(
        Some(42),
        Some(store::JOB_STATUS_RUNNING),
        "https://t.me/c/1/2",
        -100,
    );

    assert_eq!(keyboard.rows[0][0].text, "查看运行列表");
    assert_eq!(keyboard.rows[0][2].text, "菜单");
    assert_eq!(keyboard.rows[1][0].text, "查看任务详情");
    assert_eq!(keyboard.rows[1][1].text, "暂停");
    assert_eq!(keyboard.rows[1][2].text, "停止");
    assert_eq!(keyboard.rows[2][0].text, "复制暂停命令");
    assert!(matches!(
        keyboard.rows[0][0].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert!(matches!(
        keyboard.rows[1][0].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert!(matches!(
        keyboard.rows[1][1].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert!(matches!(
        keyboard.rows[0][2].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

// 暂停态进度面板应展示恢复按钮，避免用户只能复制命令恢复。
#[test]
fn test_build_transfer_progress_keyboard_for_paused_job() {
    let keyboard = build_transfer_progress_keyboard(
        Some(42),
        Some(store::JOB_STATUS_PAUSED),
        "https://t.me/c/1/2",
        -100,
    );

    assert_eq!(keyboard.rows[0][0].text, "查看暂停列表");
    assert_eq!(keyboard.rows[1][1].text, "恢复");
    assert_eq!(keyboard.rows[2][0].text, "复制恢复命令");
    assert!(matches!(
        keyboard.rows[1][1].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

// 停止态进度面板不能再给出暂停/停止动作，只保留定位和列表入口。
#[test]
fn test_build_transfer_progress_keyboard_for_cancelled_job() {
    let keyboard = build_transfer_progress_keyboard(
        Some(42),
        Some(store::JOB_STATUS_CANCELLED),
        "https://t.me/c/1/2",
        -100,
    );

    assert_eq!(keyboard.rows[0][0].text, "查看已停列表");
    assert_eq!(keyboard.rows[1][0].text, "查看任务详情");
    assert_eq!(keyboard.rows[2][0].text, "复制 job_id");
    assert_eq!(keyboard.rows[1].len(), 1);
}

// 构造最小进度快照，避免文本测试依赖数据库。
fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
    let now = store::now_utc8();
    store::JobProgressSnapshot {
        job: store::JobProgressJob {
            id: 42,
            target_chat_id: -100,
            status: status.to_owned(),
            total_items: 3,
            last_error: None,
            created_at: now,
            updated_at: now,
        },
        pending_count: 1,
        preparing_count: 0,
        prepared_count: 1,
        uploading_count: 0,
        success_count: 1,
        failed_count: 0,
        cancelled_count: 0,
        active_download_files: 0,
        active_downloaded_bytes: 0,
        active_download_total_bytes: 0,
        has_unknown_download_total: false,
    }
}
