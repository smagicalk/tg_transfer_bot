// `/downloads` 的单元测试集中放在这里，避免入口文件继续膨胀。

use super::super::common::CommandStyle;
use super::keyboard::{
    build_downloads_callback_data, build_downloads_keyboard, build_downloads_page_command,
    parse_downloads_callback_data,
};
use super::render::{format_bytes, format_downloads_text};
use super::types::{DownloadsArgs, DownloadsFilter, parse_downloads_args};
use crate::tgbot::transfer::store;

// `/downloads` 支持“纯 limit”和“filter + limit”两种模式。
#[test]
fn test_parse_downloads_args() {
    assert_eq!(
        parse_downloads_args(&["/downloads"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::All,
            limit: 8,
            page: 1,
        }
    );
    assert_eq!(
        parse_downloads_args(&["/downloads", "3"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::All,
            limit: 3,
            page: 1,
        }
    );
    assert_eq!(
        parse_downloads_args(&["/downloads", "dl"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Downloading,
            limit: 8,
            page: 1,
        }
    );
    assert_eq!(
        parse_downloads_args(&["/downloads", "done", "5"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Finished,
            limit: 5,
            page: 1,
        }
    );
    assert_eq!(
        parse_downloads_args(&["/downloads", "ok", "5"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Success,
            limit: 5,
            page: 1,
        }
    );
    assert_eq!(
        parse_downloads_args(&["/downloads", "done", "5", "2"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Finished,
            limit: 5,
            page: 2,
        }
    );
    assert_eq!(
        parse_downloads_args(&["/downloads", "pause"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Paused,
            limit: 8,
            page: 1,
        }
    );
    assert_eq!(
        parse_downloads_args(&["/downloads", "cancel"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Cancelled,
            limit: 8,
            page: 1,
        }
    );
    assert_eq!(
        parse_downloads_args(&["/downloads", "5", "2"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::All,
            limit: 5,
            page: 2,
        }
    );
    assert!(parse_downloads_args(&["/downloads", "abc"]).is_err());
}

// 新增的暂停/停止筛选应只命中对应任务状态。
#[test]
fn test_downloads_filter_matches_control_status() {
    let paused = snapshot_with_status("paused");
    let cancelling = snapshot_with_status("cancelling");
    let cancel_finalizing = snapshot_with_status("cancel_finalizing");
    let cancelled = snapshot_with_status("cancelled");
    let running = snapshot_with_status("running");

    assert!(DownloadsFilter::Paused.matches(&paused));
    assert!(!DownloadsFilter::Paused.matches(&running));
    assert!(DownloadsFilter::Cancelling.matches(&cancelling));
    assert!(DownloadsFilter::Cancelling.matches(&cancel_finalizing));
    assert!(DownloadsFilter::Cancelled.matches(&cancelled));
    assert!(DownloadsFilter::Finished.matches(&cancelled));
}

// 空列表时应该给出明确提示。
#[test]
fn test_format_downloads_text_for_empty() {
    let text = format_downloads_text(
        &[],
        &DownloadsArgs {
            filter: DownloadsFilter::All,
            limit: 8,
            page: 1,
        },
        0,
    );
    assert!(text.contains("下载列表为空"));
}

// 字节格式化应能覆盖整数和小数展示。
#[test]
fn test_format_bytes() {
    assert_eq!(format_bytes(100), "100 B");
    assert_eq!(format_bytes(1024), "1.0 KB");
    assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
}

// 翻页命令应可直接用于后续按钮回调。
#[test]
fn test_build_downloads_page_command() {
    assert_eq!(
        build_downloads_page_command(DownloadsFilter::All, 8, 2, CommandStyle::Long),
        "/downloads 8 2"
    );
    assert_eq!(
        build_downloads_page_command(DownloadsFilter::Downloading, 5, 3, CommandStyle::Long,),
        "/downloads dl 5 3"
    );
}

// 分页按钮回调应能往返解析。
#[test]
fn test_downloads_callback_data_roundtrip() {
    let data = build_downloads_callback_data(DownloadsFilter::Finished, 5, 3);
    assert_eq!(
        parse_downloads_callback_data(&data),
        Some(DownloadsArgs {
            filter: DownloadsFilter::Finished,
            limit: 5,
            page: 3,
        })
    );
    assert_eq!(parse_downloads_callback_data("x:done:5:3"), None);
}

// 当前页按钮应复制当前命令，避免点了之后无变化。
#[test]
fn test_build_downloads_keyboard_current_page_is_copy_button() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Downloading,
        limit: 5,
        page: 2,
    };
    let keyboard = build_downloads_keyboard(&args, 4);
    let current = &keyboard.rows[0][2];
    assert_eq!(current.text, "2/4");
    assert!(matches!(
        current.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::CopyText(_)
    ));
}

// 构造最小任务快照，专门用于筛选器测试。
fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
    let now = store::now_utc8();
    store::JobProgressSnapshot {
        job: crate::db::transfer_job::Model {
            id: 1,
            request_chat_id: 100,
            request_message_id: 200,
            source_link: "https://t.me/c/1/2".to_owned(),
            source_chat_id: 1,
            source_message_id: 2,
            source_album_id: 0,
            target_chat_id: 300,
            result_message_id: None,
            result_message_link: None,
            status: status.to_owned(),
            total_items: 1,
            done_items: 0,
            failed_items: 0,
            retry_count: 0,
            last_error: None,
            created_at: now,
            updated_at: now,
            finished_at: None,
        },
        pending_count: 0,
        preparing_count: 0,
        prepared_count: 0,
        uploading_count: 0,
        success_count: 0,
        failed_count: 0,
        cancelled_count: if status == "cancelled" { 1 } else { 0 },
        active_download_files: 0,
        active_downloaded_bytes: 0,
        active_download_total_bytes: 0,
        has_unknown_download_total: false,
    }
}
