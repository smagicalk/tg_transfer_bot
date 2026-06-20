// `/downloads` 的单元测试集中放在这里，避免入口文件继续膨胀。

use super::super::common::CommandStyle;
use super::super::common::format_bytes;
use super::keyboard::{
    DownloadsCallbackAction, build_downloads_filter_callback_data, build_downloads_keyboard,
    build_downloads_page_callback_data, build_downloads_page_command,
    parse_downloads_callback_data,
};
use super::render::format_downloads_text;
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
        false,
    );
    assert!(text.contains("下载列表为空"));
}

// 管理员和普通用户应显示不同的范围说明，避免把全局语义误展示给普通用户。
#[test]
fn test_format_downloads_text_scope_label_changes_by_role() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::All,
        limit: 8,
        page: 1,
    };
    let page_items = [snapshot_with_status("running")];

    let admin_text = format_downloads_text(&page_items, &args, 1, true);
    let user_text = format_downloads_text(&page_items, &args, 1, false);

    assert!(admin_text.contains("范围：管理员 / 全局"));
    assert!(user_text.contains("范围：当前用户 / 自己"));
}

// 当前页存在任务时，应为每个任务生成详情按钮。
#[test]
fn test_build_downloads_keyboard_has_job_detail_buttons() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::All,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[snapshot_with_status("running")]);

    assert_eq!(keyboard.rows[0][0].text, "详情 #1");
    assert!(matches!(
        keyboard.rows[0][0].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

// 运行中任务在列表页应能直接暂停/停止，减少进入详情后二次点击。
#[test]
fn test_build_downloads_keyboard_has_running_job_controls() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Running,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[snapshot_with_status("running")]);

    assert_eq!(keyboard.rows[0][0].text, "详情 #1");
    assert_eq!(keyboard.rows[0][1].text, "暂停");
    assert_eq!(keyboard.rows[0][2].text, "停止");
    assert!(matches!(
        keyboard.rows[0][1].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert!(matches!(
        keyboard.rows[0][2].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

// 暂停任务在列表页应能直接恢复或停止。
#[test]
fn test_build_downloads_keyboard_has_paused_job_controls() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Paused,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[snapshot_with_status("paused")]);

    assert_eq!(keyboard.rows[0][0].text, "详情 #1");
    assert_eq!(keyboard.rows[0][1].text, "恢复");
    assert_eq!(keyboard.rows[0][2].text, "停止");
    assert!(matches!(
        keyboard.rows[0][1].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

// 已完成任务只保留详情，避免列表里出现无效控制按钮。
#[test]
fn test_build_downloads_keyboard_hides_controls_for_finished_job() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Success,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[snapshot_with_status("success")]);

    assert_eq!(keyboard.rows[0][0].text, "详情 #1");
    assert_eq!(keyboard.rows[0].len(), 1);
}

// 任务详情按钮应使用短 callback payload，方便和 `/job` 统一路由。
#[test]
fn test_build_downloads_keyboard_job_detail_callback_data() {
    use base64::{Engine as _, engine::general_purpose};

    let args = DownloadsArgs {
        filter: DownloadsFilter::All,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[snapshot_with_status("running")]);

    let button = &keyboard.rows[0][0];
    let data = match &button.r#type {
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) => &callback.data,
        other => panic!("unexpected button type: {:?}", other),
    };
    let decoded = String::from_utf8(general_purpose::STANDARD.decode(data).unwrap()).unwrap();
    assert_eq!(decoded, "j:st:1");
}

// 空列表时不应生成任务详情按钮行。
#[test]
fn test_build_downloads_keyboard_empty_page_has_no_job_detail_row() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::All,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[]);

    assert_eq!(keyboard.rows[0][0].text, "全部");
    assert_eq!(keyboard.rows[4][0].text, "刷新");
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
    let data = build_downloads_page_callback_data(DownloadsFilter::Finished, 5, 3);
    assert_eq!(
        parse_downloads_callback_data(&data),
        Some((
            DownloadsCallbackAction::Page,
            DownloadsArgs {
                filter: DownloadsFilter::Finished,
                limit: 5,
                page: 3,
            }
        ))
    );
    assert_eq!(parse_downloads_callback_data("x:done:5:3"), None);
    assert_eq!(parse_downloads_callback_data("d:done:5:3"), None);

    assert_eq!(
        parse_downloads_callback_data("d:r:run:8:1"),
        Some((
            DownloadsCallbackAction::Refresh,
            DownloadsArgs {
                filter: DownloadsFilter::Running,
                limit: 8,
                page: 1,
            }
        ))
    );

    assert_eq!(
        parse_downloads_callback_data(&build_downloads_filter_callback_data(
            DownloadsFilter::Failed,
            8,
        )),
        Some((
            DownloadsCallbackAction::Filter,
            DownloadsArgs {
                filter: DownloadsFilter::Failed,
                limit: 8,
                page: 1,
            }
        ))
    );
}

// 当前页按钮应直接刷新当前页，避免列表面板出现“只有这一格不能点”的割裂感。
#[test]
fn test_build_downloads_keyboard_current_page_is_refresh_callback() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Downloading,
        limit: 5,
        page: 2,
    };
    let keyboard = build_downloads_keyboard(&args, 4, &[]);
    let current = &keyboard.rows[6][2];
    assert_eq!(current.text, "2/4");
    assert!(matches!(
        current.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

// 翻页 callback 也必须通过统一按钮入口编码，否则 TDLib 会把裸 payload 当成非法 bytes。
#[test]
fn test_build_downloads_keyboard_navigation_callback_data_is_encoded() {
    use base64::{Engine as _, engine::general_purpose};

    let args = DownloadsArgs {
        filter: DownloadsFilter::Running,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 3, &[]);
    let next = &keyboard.rows[6][3];

    let data = match &next.r#type {
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) => &callback.data,
        other => panic!("unexpected button type: {:?}", other),
    };
    let decoded = String::from_utf8(general_purpose::STANDARD.decode(data).unwrap()).unwrap();
    assert_eq!(decoded, "d:p:run:8:2");
}

// 列表页把“刷新 / 返回 / 菜单”单独放一行，复制命令再单独一行。
#[test]
fn test_build_downloads_keyboard_has_refresh_row() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Running,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 2, &[]);

    assert_eq!(keyboard.rows.len(), 7);
    assert_eq!(keyboard.rows[4][0].text, "刷新");
    assert!(matches!(
        keyboard.rows[4][0].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert_eq!(keyboard.rows[4][1].text, "返回");
    assert_eq!(keyboard.rows[4][2].text, "菜单");
    assert!(matches!(
        keyboard.rows[4][2].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert_eq!(keyboard.rows[5][0].text, "复制当前命令");
}

// 主操作区中的筛选按钮仍保留多行，分页单独放在最后一行。
#[test]
fn test_build_downloads_keyboard_has_filter_row() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Running,
        limit: 8,
        page: 2,
    };
    let keyboard = build_downloads_keyboard(&args, 4, &[]);

    assert_eq!(keyboard.rows[0][0].text, "全部");
    assert!(matches!(
        keyboard.rows[0][0].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert_eq!(keyboard.rows[0][1].text, "运行");
    assert!(matches!(
        keyboard.rows[0][1].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert_eq!(keyboard.rows[0][2].text, "等待");
    assert_eq!(keyboard.rows[1][0].text, "下载");
    assert_eq!(keyboard.rows[1][1].text, "上传");
    assert_eq!(keyboard.rows[1][2].text, "就绪");
    assert_eq!(keyboard.rows[2][0].text, "完成");
    assert_eq!(keyboard.rows[2][1].text, "成功");
    assert_eq!(keyboard.rows[3][0].text, "暂停");
    assert_eq!(keyboard.rows[3][1].text, "停止中");
    assert_eq!(keyboard.rows[3][2].text, "已停止");
    assert_eq!(keyboard.rows[6][0].text, "首页");
    assert_eq!(keyboard.rows[6][4].text, "末页");
}

// 构造最小任务快照，专门用于筛选器测试。
fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
    let now = store::now_utc8();
    store::JobProgressSnapshot {
        job: store::JobProgressJob {
            id: 1,
            target_chat_id: 300,
            status: status.to_owned(),
            total_items: 1,
            last_error: None,
            created_at: now,
            updated_at: now,
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
