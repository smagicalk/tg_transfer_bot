// `/cache` 文案与参数测试。

use super::keyboard::{
    build_cache_keyboard, build_cache_view_callback_data, parse_cache_callback_data,
};
use super::render::{compute_cache_page_count, format_cache_page_text, format_cache_summary_text};
use super::types::{CacheArgs, CacheView, parse_cache_args};
use crate::tgbot::transfer::store;

#[test]
fn test_parse_cache_args() {
    assert_eq!(parse_cache_args(&["/cache"]).unwrap(), CacheArgs::default());
    assert_eq!(
        parse_cache_args(&["/cache", "summary"]).unwrap(),
        CacheArgs {
            view: CacheView::Summary,
            limit: 10,
            page: 1,
        }
    );
    assert_eq!(
        parse_cache_args(&["/cache", "page", "20", "2"]).unwrap(),
        CacheArgs {
            view: CacheView::Page,
            limit: 20,
            page: 2,
        }
    );
}

#[test]
fn test_compute_cache_page_count() {
    assert_eq!(compute_cache_page_count(0, 10), 1);
    assert_eq!(compute_cache_page_count(1, 10), 1);
    assert_eq!(compute_cache_page_count(11, 10), 2);
}

#[test]
fn test_cache_callback_roundtrip() {
    let data = build_cache_view_callback_data(CacheView::Page, 10, 2);
    assert_eq!(
        parse_cache_callback_data(&data),
        Some(CacheArgs {
            view: CacheView::Page,
            limit: 10,
            page: 2,
        })
    );
    assert_eq!(parse_cache_callback_data("d:r:run:8:1"), None);
}

#[test]
fn test_cache_keyboard_boundary_navigation_is_callback_button() {
    let keyboard = build_cache_keyboard(
        &CacheArgs {
            view: CacheView::Page,
            limit: 10,
            page: 1,
        },
        1,
    );

    assert_eq!(keyboard.rows[3][0].text, "首页");
    assert_eq!(keyboard.rows[3][1].text, "上页");
    assert_eq!(keyboard.rows[3][3].text, "下页");
    assert_eq!(keyboard.rows[3][4].text, "末页");
    for index in [0, 1, 3, 4] {
        assert!(
            matches!(
                keyboard.rows[3][index].r#type,
                tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
            ),
            "cache boundary button should keep callback refresh behavior"
        );
    }
}

#[test]
fn test_cache_keyboard_follow_row_hierarchy() {
    let keyboard = build_cache_keyboard(
        &CacheArgs {
            view: CacheView::Page,
            limit: 10,
            page: 1,
        },
        2,
    );

    assert_eq!(keyboard.rows[0][0].text, "概览");
    assert_eq!(keyboard.rows[1][0].text, "刷新");
    assert_eq!(keyboard.rows[1][1].text, "返回");
    assert_eq!(keyboard.rows[1][2].text, "菜单");
    assert_eq!(keyboard.rows[2][0].text, "复制当前命令");
}

#[test]
fn test_format_cache_summary_text() {
    let health = store::TransferHealthSnapshot {
        total_jobs: 3,
        active_jobs: 1,
        success_jobs: 1,
        failed_jobs: 1,
        cancelled_jobs: 0,
        total_items: 10,
        preparing_items: 2,
        uploading_items: 1,
        file_cache_rows: 5,
        file_cache_active_rows: 4,
        file_cache_due_rows: 1,
        file_cache_failed_rows: 0,
        recoverable_jobs: 1,
        cancelling_jobs: 0,
        job_concurrency: 2,
        active_transfer_jobs: 1,
        progress_edit_interval_seconds: 3,
        file_delete_delay_minutes: 2,
        file_gc_interval_seconds: 60,
    };
    let text = format_cache_summary_text(
        &health,
        &[store::FileCacheStatusSummary {
            status: "ready".to_owned(),
            count: 2,
            active_refs: 2,
        }],
    );
    assert!(text.contains("文件缓存概览"));
    assert!(text.contains("ready"));
    assert!(text.contains("待删除"));
}

#[test]
fn test_format_cache_page_text() {
    let health = store::TransferHealthSnapshot {
        total_jobs: 3,
        active_jobs: 1,
        success_jobs: 1,
        failed_jobs: 1,
        cancelled_jobs: 0,
        total_items: 10,
        preparing_items: 2,
        uploading_items: 1,
        file_cache_rows: 5,
        file_cache_active_rows: 4,
        file_cache_due_rows: 1,
        file_cache_failed_rows: 0,
        recoverable_jobs: 1,
        cancelling_jobs: 0,
        job_concurrency: 2,
        active_transfer_jobs: 1,
        progress_edit_interval_seconds: 3,
        file_delete_delay_minutes: 2,
        file_gc_interval_seconds: 60,
    };
    let text = format_cache_page_text(
        &health,
        &[store::FileCacheSnapshot {
            owner_client_role: "user".to_owned(),
            file_key: "fk1".to_owned(),
            status: "ready".to_owned(),
            active_refs: 1,
            size_bytes: Some(1024),
            td_file_id: Some(1),
            local_path: Some("tg/user/a.bin".to_owned()),
            delete_after: None,
            last_used_at: store::now_utc8(),
            updated_at: store::now_utc8(),
            last_error: None,
        }],
        &CacheArgs {
            view: CacheView::Page,
            limit: 10,
            page: 1,
        },
        1,
    );
    assert!(text.contains("文件缓存列表"));
    assert!(text.contains("user · fk1"));
    assert!(text.contains("td_file_id"));
}
