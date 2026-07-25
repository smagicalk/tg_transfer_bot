// `/health` 命令实现。
// 只读查看运行状态，不修改任何任务、文件或配置。

use crate::tgbot::send;
use crate::tgbot::send::send_interaction_error_card;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store;

use super::common::{
    CommandStyle, build_ready_page_header, build_refresh_return_menu_row,
    health_command as build_health_command,
};

/// `health` 帮助页和目录页共用的用途描述。
pub(in crate::tgbot::transfer::command) fn health_help_purpose() -> &'static str {
    "只读查看运行健康状态。"
}

/// `health` 帮助页和目录页共用的一句话摘要。
pub(in crate::tgbot::transfer::command) fn health_help_summary() -> &'static str {
    "查看运行配置、并发、恢复队列、任务和缓存总体状态。"
}

/// `health` 菜单页和帮助详情页共用的开场说明。
pub(in crate::tgbot::transfer::command) fn health_intro_lines() -> Vec<String> {
    vec!["展示任务规模、恢复队列、缓存队列、并发和运行时配置，不修改任何状态。".to_owned()]
}

/// `/help health` 共用的详细说明正文。
///
/// 健康页的指标口径和只读约束由 health 模块维护，help 模块只负责路由展示。
pub(in crate::tgbot::transfer::command) fn build_health_help_detail_text() -> String {
    let mut lines = vec![
        "health".to_owned(),
        format!("用途：{}", health_help_purpose()),
    ];
    lines.extend(
        health_intro_lines()
            .into_iter()
            .map(|line| format!("说明：{line}")),
    );
    lines.extend([
        card::DIVIDER.to_owned(),
        card::section("命令"),
        build_health_command(CommandStyle::Long),
        String::new(),
        card::section("示例"),
        build_health_command(CommandStyle::Long),
    ]);
    lines.join("\n")
}

/// `health` 帮助页入口按钮行。
pub(in crate::tgbot::transfer::command) fn build_health_help_entry_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![vec![
        send::build_callback_button(
            "打开健康页",
            &build_health_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "文件缓存",
            &super::build_cache_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]]
}

/// `/health` callback 前缀。
const HEALTH_CALLBACK_PREFIX: &str = "hl:";

/// 判断 callback payload 是否属于 `/health`。
pub(super) fn is_health_callback_data(data: &str) -> bool {
    data.starts_with(HEALTH_CALLBACK_PREFIX)
}

/// 给菜单页生成健康页 callback 数据。
pub(super) fn build_health_callback_data() -> String {
    format!("{HEALTH_CALLBACK_PREFIX}show")
}

/// 在指定上下文上执行 `/health` 命令。
pub async fn health_command_on(
    app: &crate::app_context::AppContext,
    _text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let snapshot = build_health_snapshot_on(app).await?;
    send::ReplyPanel::card(format_health_text(&snapshot))
        .rows(build_health_buttons())
        .send(request_chat_id, client_id)
        .await
}

/// 在指定上下文上处理 `/health` callback。
pub async fn health_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };
    if payload != build_health_callback_data() {
        send::answer_callback_query(update.id, Some("健康页参数无效"), client_id).await?;
        return Ok(());
    }

    send::answer_callback_query(update.id, Some("已刷新"), client_id).await?;
    let snapshot = match build_health_snapshot_on(app).await {
        Ok(snapshot) => snapshot,
        Err(err) => {
            send_health_callback_error(update.chat_id, client_id, &err).await?;
            return Err(err);
        }
    };
    let (text, keyboard) = send::ReplyPanel::card(format_health_text(&snapshot))
        .rows(build_health_buttons())
        .into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
        "健康页刷新失败",
        "健康页已生成，但原消息编辑失败；请使用错误卡片上的“菜单”按钮重新进入。",
    )
    .await
}

/// 健康页刷新失败提示。
async fn send_health_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_interaction_error_card(
        request_chat_id,
        client_id,
        "健康页刷新失败",
        "健康数据未刷新，请检查日志或复制错误信息。",
        err,
    )
    .await
}

/// `health` 卡片按钮。
fn build_health_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            downloads_run_button(),
            send::build_callback_button(
                "文件缓存",
                &super::build_cache_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_refresh_return_menu_row(
            send::build_callback_button(
                "刷新",
                &build_health_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "查看命令",
                &super::build_help_button_data(Some("health")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &super::build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ),
    ]
}

/// 健康页进入运行列表按钮。
///
/// 这里使用固定合法筛选值，协议漂移应在开发阶段暴露，而不是运行时退化成复制按钮。
fn downloads_run_button() -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        "下载列表",
        &super::require_downloads_filter_button_data("run", 8),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// 运行状态快照。
#[derive(Debug, Clone)]
struct HealthSnapshot {
    /// 当前运行时统计。
    transfer: store::TransferHealthSnapshot,
    /// 转存执行所需的 client id；如果后台服务尚未完全 ready，会显示为缺失。
    clients: Option<crate::config::TransferClientIds>,
    /// 当前时间。
    now: chrono::DateTime<chrono::FixedOffset>,
}

/// 在指定上下文上构造健康快照。
async fn build_health_snapshot_on(
    app: &crate::app_context::AppContext,
) -> anyhow::Result<HealthSnapshot> {
    Ok(HealthSnapshot {
        transfer: store::list_transfer_health_snapshot(app).await?,
        clients: app.transfer_runtime.transfer_client_ids(),
        now: store::now_utc8(),
    })
}

/// 构造健康卡片正文。
fn format_health_text(snapshot: &HealthSnapshot) -> String {
    let transfer = &snapshot.transfer;
    let mut lines = build_ready_page_header("运行健康");
    lines.extend([
        card::section("客户端"),
        format_client_line(snapshot.clients),
        card::field("ready_at", snapshot.now.format("%Y-%m-%d %H:%M:%S")),
        card::section("任务"),
        card::field("总任务", transfer.total_jobs),
        card::field("总子项", transfer.total_items),
        card::field("活跃任务", transfer.active_jobs),
        card::field_pair("成功", transfer.success_jobs, "失败", transfer.failed_jobs),
        card::field_pair(
            "待恢复",
            transfer.recoverable_jobs,
            "收敛中",
            transfer.cancelling_jobs,
        ),
        card::field_pair(
            "已停",
            transfer.cancelled_jobs,
            "准备项",
            transfer.preparing_items,
        ),
        card::field("上传项", transfer.uploading_items),
        card::section("文件"),
        card::field_pair(
            "活跃缓存",
            transfer.file_cache_active_rows,
            "待删",
            transfer.file_cache_due_rows,
        ),
        card::field_pair(
            "删除失败",
            transfer.file_cache_failed_rows,
            "缓存总数",
            transfer.file_cache_rows,
        ),
        card::section("配置"),
        card::field_pair(
            "执行中",
            transfer.active_transfer_jobs,
            "并发上限",
            transfer.job_concurrency,
        ),
        card::field(
            "progress_edit_interval_seconds",
            transfer.progress_edit_interval_seconds,
        ),
        card::field(
            "file_delete_delay_minutes",
            transfer.file_delete_delay_minutes,
        ),
        card::field(
            "file_gc_interval_seconds",
            transfer.file_gc_interval_seconds,
        ),
    ]);
    lines.join("\n")
}

/// 格式化当前执行 client 组合。
fn format_client_line(clients: Option<crate::config::TransferClientIds>) -> String {
    let Some(clients) = clients else {
        return card::field("transfer_clients", "not-ready");
    };
    format!(
        "interaction：{}  download：{}  upload：{}",
        card::code(clients.interaction),
        card::code(clients.download),
        card::code(clients.upload)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // 健康页应像控制面板一样直接跳转/刷新，命令说明通过“查看命令”打开。
    #[test]
    fn test_build_health_buttons_prefer_callbacks() {
        let rows = build_health_buttons();
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in ["刷新", "下载列表", "文件缓存", "查看命令", "菜单"] {
            assert!(
                labels.contains(&expected),
                "missing health button: {expected}"
            );
        }

        for expected in ["刷新", "下载列表", "文件缓存"] {
            let button = rows
                .iter()
                .flatten()
                .find(|button| button.text == expected)
                .expect("health control button should exist");
            assert!(matches!(
                button.r#type,
                tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
            ));
        }
    }

    #[test]
    fn test_build_health_buttons_primary_row_hierarchy() {
        let rows = build_health_buttons();

        assert_eq!(rows[0][0].text, "下载列表");
        assert_eq!(rows[0][1].text, "文件缓存");
        assert_eq!(rows[1][0].text, "刷新");
        assert_eq!(rows[1][1].text, "查看命令");
        assert_eq!(rows[1][2].text, "菜单");
    }

    #[test]
    fn test_format_health_text_hides_command_section_by_default() {
        let now = store::now_utc8();
        let snapshot = HealthSnapshot {
            transfer: store::TransferHealthSnapshot {
                total_jobs: 1,
                total_items: 2,
                active_jobs: 1,
                success_jobs: 0,
                failed_jobs: 0,
                recoverable_jobs: 0,
                cancelling_jobs: 0,
                cancelled_jobs: 0,
                preparing_items: 0,
                uploading_items: 0,
                file_cache_rows: 3,
                file_cache_active_rows: 2,
                file_cache_due_rows: 1,
                file_cache_failed_rows: 0,
                active_transfer_jobs: 1,
                job_concurrency: 2,
                progress_edit_interval_seconds: 5,
                file_delete_delay_minutes: 10,
                file_gc_interval_seconds: 30,
            },
            clients: None,
            now,
        };

        let text = format_health_text(&snapshot);

        assert!(text.contains("运行健康"));
        assert!(text.contains("状态：‹ready›"));
        assert!(!text.contains("■ 命令"));
        assert!(!text.contains("/cache"));
    }
}
