// `/downloads` 的单元测试集中放在这里，避免入口文件继续膨胀。

use super::super::common::format_bytes;
use super::keyboard::{
    DownloadsCallbackAction, build_downloads_filter_callback_data, build_downloads_keyboard,
    build_downloads_page_callback_data, build_downloads_page_command,
    parse_downloads_callback_data,
};
use super::render::format_downloads_text;
use super::types::{DownloadsArgs, DownloadsFilter, parse_downloads_args};
use crate::tgbot::transfer::store;
use base64::{Engine as _, engine::general_purpose};

/// 测试解析 `/downloads` 命令的参数切片：
/// 支持“无参默认”、“纯 limit 数字”、“单个 filter 标签”以及“filter + limit + page”混合模式。
#[test]
fn test_parse_downloads_args() {
    // 1. 无参情况：默认筛选全部，每页 8 条，第 1 页
    assert_eq!(
        parse_downloads_args(&["/downloads"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::All,
            limit: 8,
            page: 1,
        }
    );
    // 2. 仅提供一个数字参数：视为 limit
    assert_eq!(
        parse_downloads_args(&["/downloads", "3"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::All,
            limit: 3,
            page: 1,
        }
    );
    // 3. 仅提供筛选标签 "dl"：筛选下载中任务
    assert_eq!(
        parse_downloads_args(&["/downloads", "dl"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Downloading,
            limit: 8,
            page: 1,
        }
    );
    // 4. 提供筛选标签 "done" 与数字 "5"：筛选已完成任务，每页 5 条
    assert_eq!(
        parse_downloads_args(&["/downloads", "done", "5"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Finished,
            limit: 5,
            page: 1,
        }
    );
    // 5. 提供筛选标签 "ok" 与数字 "5"：筛选成功任务
    assert_eq!(
        parse_downloads_args(&["/downloads", "ok", "5"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Success,
            limit: 5,
            page: 1,
        }
    );
    // 6. 提供完整三元组 "done" "5" "2"：已完成，每页 5 条，第 2 页
    assert_eq!(
        parse_downloads_args(&["/downloads", "done", "5", "2"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Finished,
            limit: 5,
            page: 2,
        }
    );
    // 7. 暂停筛选 "pause"
    assert_eq!(
        parse_downloads_args(&["/downloads", "pause"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Paused,
            limit: 8,
            page: 1,
        }
    );
    // 8. 取消筛选 "cancel"
    assert_eq!(
        parse_downloads_args(&["/downloads", "cancel"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::Cancelled,
            limit: 8,
            page: 1,
        }
    );
    // 9. 两个纯数字 "5" "2"：limit=5, page=2
    assert_eq!(
        parse_downloads_args(&["/downloads", "5", "2"]).unwrap(),
        DownloadsArgs {
            filter: DownloadsFilter::All,
            limit: 5,
            page: 2,
        }
    );
    // 10. 非法未知筛选字符，应当报错
    assert!(parse_downloads_args(&["/downloads", "abc"]).is_err());
}

/// 测试状态筛选器是否能准确匹配暂停、停止中、已取消等控制状态。
#[test]
fn test_downloads_filter_matches_control_status() {
    let paused = snapshot_with_status("paused");
    let cancelling = snapshot_with_status("cancelling");
    let cancel_finalizing = snapshot_with_status("cancel_finalizing");
    let cancelled = snapshot_with_status("cancelled");
    let running = snapshot_with_status("running");

    // 验证 Paused 匹配
    assert!(DownloadsFilter::Paused.matches(&paused));
    assert!(!DownloadsFilter::Paused.matches(&running));
    // 验证 Cancelling 匹配
    assert!(DownloadsFilter::Cancelling.matches(&cancelling));
    assert!(DownloadsFilter::Cancelling.matches(&cancel_finalizing));
    // 验证 Cancelled 匹配
    assert!(DownloadsFilter::Cancelled.matches(&cancelled));
    // 验证 Finished 包含已取消状态
    assert!(DownloadsFilter::Finished.matches(&cancelled));
}

/// 测试当任务列表为空时，富文本卡片能够给出友好的空态提示。
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
    // 必须包含空提示
    assert!(text.contains("下载列表为空"));
    // 不应出现多余的原始命令提示
    assert!(!text.contains("■ 命令"));
    assert!(!text.contains("/downloads"));
}

/// 测试普通授权用户查看列表时展示全局任务范围说明。
#[test]
fn test_format_downloads_text_uses_global_scope() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::All,
        limit: 8,
        page: 1,
    };
    let page_items = [snapshot_with_status("running")];

    let text = format_downloads_text(&page_items, &args, 1);
    assert!(text.contains("范围：所有任务"));
    assert!(!text.contains("■ 命令"));
    assert!(!text.contains("/downloads"));
}

/// 测试当前页包含任务快照时，键盘中应为每个任务生成对应的详情跳转按钮。
#[test]
fn test_build_downloads_keyboard_has_job_detail_buttons() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::All,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[snapshot_with_status("running")]);

    // 检查第 0 行第 0 个按钮是否为详情按钮
    assert_eq!(keyboard.rows[0][0].text, "详情 #1");
    assert!(matches!(
        keyboard.rows[0][0].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

/// 测试任务详情按钮每行最多分组 2 个，并且不在列表中塞入直接控制按钮（暂停/恢复统一在详情页做）。
#[test]
fn test_build_downloads_keyboard_groups_job_details_without_inline_controls() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::All,
        limit: 8,
        page: 1,
    };
    let first = snapshot_with_status("running");
    let mut second = snapshot_with_status("paused");
    second.job.id = 2;

    let keyboard = build_downloads_keyboard(&args, 1, &[first, second]);
    let first_row_labels = keyboard.rows[0]
        .iter()
        .map(|button| button.text.as_str())
        .collect::<Vec<_>>();
    assert_eq!(first_row_labels, vec!["详情 #1", "详情 #2"]);
    assert!(
        keyboard.rows[0]
            .iter()
            .all(|button| button.text.starts_with("详情 #"))
    );
}

/// 测试运行中任务在列表中只放详情按钮，控制统一由详情页承接。
#[test]
fn test_build_downloads_keyboard_routes_running_controls_through_detail() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Running,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[snapshot_with_status("running")]);

    assert_eq!(keyboard.rows[0][0].text, "详情 #1");
    assert_eq!(keyboard.rows[0].len(), 1);
    assert_eq!(decoded_callback_data(&keyboard.rows[0][0]), "j:st:1");
}

/// 测试暂停中的任务在列表中同样只保留详情入口。
#[test]
fn test_build_downloads_keyboard_routes_paused_controls_through_detail() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Paused,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[snapshot_with_status("paused")]);

    assert_eq!(keyboard.rows[0][0].text, "详情 #1");
    assert_eq!(keyboard.rows[0].len(), 1);
    assert_eq!(decoded_callback_data(&keyboard.rows[0][0]), "j:st:1");
}

/// 测试已完成任务只保留详情按钮，避免出现无效控制按钮。
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

/// 测试详情按钮的回调 payload 格式，确认生成短协议 `j:st:1`。
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
        other => panic!("unexpected button type: {other:?}"),
    };
    let decoded = String::from_utf8(general_purpose::STANDARD.decode(data).unwrap()).unwrap();
    assert_eq!(decoded, "j:st:1");
}

/// 测试空列表时第一行不生成任何任务详情按钮，直接呈现筛选行。
#[test]
fn test_build_downloads_keyboard_empty_page_has_no_job_detail_row() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::All,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 1, &[]);

    assert_eq!(keyboard.rows[0][0].text, "全部");
    assert_eq!(keyboard.rows[2][0].text, "刷新");
}

/// 测试字节大小格式化工具：涵盖 B、KB、MB 等阶梯展示。
#[test]
fn test_format_bytes() {
    assert_eq!(format_bytes(100), "100 B");
    assert_eq!(format_bytes(1024), "1.0 KB");
    assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
}

/// 测试反向构造翻页命令文本的正确性。
#[test]
fn test_build_downloads_page_command() {
    assert_eq!(
        build_downloads_page_command(DownloadsFilter::All, 8, 2),
        "/downloads 8 2"
    );
    assert_eq!(
        build_downloads_page_command(DownloadsFilter::Downloading, 5, 3),
        "/downloads dl 5 3"
    );
}

/// 测试下载列表 callback 数据字符串的编解码往返一致性。
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
    // 前缀错误或段数错误时返回 None
    assert_eq!(parse_downloads_callback_data("x:done:5:3"), None);
    assert_eq!(parse_downloads_callback_data("d:done:5:3"), None);

    // 刷新动作往返解析
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

    // 筛选动作往返解析
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

/// 测试“当前页”按钮被渲染为点击刷新的 callback，保持界面所有格子均可点。
#[test]
fn test_build_downloads_keyboard_current_page_is_refresh_callback() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Downloading,
        limit: 5,
        page: 2,
    };
    let keyboard = build_downloads_keyboard(&args, 4, &[]);
    let current = &keyboard.rows[3][2];
    assert_eq!(current.text, "2/4");
    assert!(matches!(
        current.r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
}

/// 测试翻页 callback 数据在组装为 TDLib 按钮结构体时已进行标准 base64 编码。
#[test]
fn test_build_downloads_keyboard_navigation_callback_data_is_encoded() {
    use base64::{Engine as _, engine::general_purpose};

    let args = DownloadsArgs {
        filter: DownloadsFilter::Running,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 3, &[]);
    let next = &keyboard.rows[3][1];

    let data = match &next.r#type {
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) => &callback.data,
        other => panic!("unexpected button type: {other:?}"),
    };
    let decoded = String::from_utf8(general_purpose::STANDARD.decode(data).unwrap()).unwrap();
    assert_eq!(decoded, "d:p:run:8:2");
}

/// 测试下载列表键盘的全局功能操作行（刷新、任务中心、查看命令、菜单）。
#[test]
fn test_build_downloads_keyboard_has_refresh_row() {
    let args = DownloadsArgs {
        filter: DownloadsFilter::Running,
        limit: 8,
        page: 1,
    };
    let keyboard = build_downloads_keyboard(&args, 2, &[]);

    assert_eq!(keyboard.rows.len(), 4);
    assert_eq!(keyboard.rows[2][0].text, "刷新");
    assert!(matches!(
        keyboard.rows[2][0].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert_eq!(keyboard.rows[2][1].text, "任务中心");
    assert_eq!(decoded_callback_data(&keyboard.rows[2][1]), "m:th");
    assert_eq!(keyboard.rows[2][2].text, "查看命令");
    assert!(matches!(
        keyboard.rows[2][2].r#type,
        tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
    ));
    assert_eq!(keyboard.rows[2][3].text, "菜单");
    assert_eq!(keyboard.rows[3][0].text, "1/2");
    assert_eq!(keyboard.rows[3][1].text, "下页");
    assert_eq!(keyboard.rows[3][2].text, "末页");
}

/// 测试筛选按钮的分行紧凑聚合展示。
#[test]
fn test_build_downloads_keyboard_uses_compact_filter_groups() {
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
    assert_eq!(keyboard.rows[0][2].text, "暂停");
    assert_eq!(keyboard.rows[1][0].text, "成功");
    assert_eq!(keyboard.rows[1][1].text, "失败");
    assert_eq!(keyboard.rows[1][2].text, "已停止");
    assert_eq!(keyboard.rows[3][0].text, "首页");
    assert_eq!(keyboard.rows[3][4].text, "末页");
}

/// 测试位于首页、末页或单页等边界情况下，隐藏无效的跳转按钮。
#[test]
fn test_build_downloads_keyboard_hides_unavailable_navigation() {
    // 1. 处于第一页：不展示首页和上页，只展示 [1/4, 下页, 末页]
    let first_page = DownloadsArgs {
        filter: DownloadsFilter::All,
        limit: 8,
        page: 1,
    };
    let first_keyboard = build_downloads_keyboard(&first_page, 4, &[]);
    assert_eq!(
        first_keyboard.rows[3]
            .iter()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>(),
        vec!["1/4", "下页", "末页"]
    );

    // 2. 处于最后一页：展示 [首页, 上页, 4/4]，不展示下页和末页
    let last_page = DownloadsArgs {
        page: 4,
        ..first_page
    };
    let last_keyboard = build_downloads_keyboard(&last_page, 4, &[]);
    assert_eq!(
        last_keyboard.rows[3]
            .iter()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>(),
        vec!["首页", "上页", "4/4"]
    );

    // 3. 仅有单页：仅保留当前页按钮 [1/1]
    let single_page = DownloadsArgs {
        page: 1,
        ..first_page
    };
    let single_keyboard = build_downloads_keyboard(&single_page, 1, &[]);
    assert_eq!(single_keyboard.rows[3].len(), 1);
    assert_eq!(single_keyboard.rows[3][0].text, "1/1");
}

/// 测试辅助工具：构造特定状态的最小化任务快照实例。
fn snapshot_with_status(status: &str) -> store::JobProgressSnapshot {
    let now = store::now_utc8();
    store::JobProgressSnapshot {
        job: store::JobProgressJob {
            id: 1,
            target_chat_id: 300,
            result_message_link: None,
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
        active_upload_files: 0,
        active_uploaded_bytes: 0,
        active_upload_total_bytes: 0,
        has_unknown_upload_total: false,
    }
}

/// 测试辅助工具：解码按钮中的 base64 回调数据为原始字符串。
fn decoded_callback_data(button: &tdlib_rs::types::InlineKeyboardButton) -> String {
    let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &button.r#type else {
        panic!("button must be callback");
    };
    String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap()
}
