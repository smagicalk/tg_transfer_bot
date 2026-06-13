// `/health` 命令实现。
// 只读查看运行状态，不修改任何任务、文件或配置。

use crate::tgbot::send;
use crate::tgbot::send::send_interaction_error_card;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store;

use super::common::{
    CommandStyle, downloads_command as downloads_command_text,
    health_command as health_command_text, help_command as help_command_text,
};

/// `/health` callback 前缀。
const HEALTH_CALLBACK_PREFIX: &str = "hl:";

/// 判断 callback payload 是否属于 `/health`。
pub(super) fn is_health_callback_data(data: &str) -> bool {
    data.starts_with(HEALTH_CALLBACK_PREFIX)
}

/// 给菜单页生成健康页 callback 数据。
pub(super) fn build_health_callback_data() -> String {
    format!("{}show", HEALTH_CALLBACK_PREFIX)
}

/// `/health` 命令入口。
///
/// 用于快速确认：
/// - bot/user client 是否已准备好
/// - 运行时并发和 GC 参数
/// - 当前任务、缓存、恢复队列的大致规模
pub async fn health_command(
    _text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let snapshot = build_health_snapshot().await?;
    send::ReplyPanel::card(format_health_text(&snapshot))
        .rows(build_health_buttons())
        .send(request_chat_id, client_id)
        .await
}

/// `/health` callback 入口。
pub async fn health_callback_query(
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
    let snapshot = match build_health_snapshot().await {
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
        "健康页已生成，但原消息编辑失败；请复制错误或重新发送 /health。",
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
            send::build_callback_button(
                "刷新",
                &build_health_callback_data(),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            downloads_run_button(),
            send::build_callback_button(
                "文件缓存",
                &super::build_cache_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button(
                "复制 /hl",
                &health_command_text(CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制运行列表",
                &downloads_command_text(Some("run"), None, None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制 /h",
                &help_command_text(None, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button(
                "复制 /help",
                &help_command_text(None, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &super::build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ]
}

/// 健康页进入运行列表按钮。
///
/// 正常情况下使用 callback 原地跳转；如果下载筛选协议未来变更，降级为复制短命令，
/// 避免健康页因为一个辅助按钮构造失败而整体不可用。
fn downloads_run_button() -> tdlib_rs::types::InlineKeyboardButton {
    if let Some(data) = super::build_downloads_filter_button_data("run", 8) {
        return send::build_callback_button(
            "下载列表",
            &data,
            tdlib_rs::enums::ButtonStyle::Default,
        );
    }

    tracing::warn!("downloads run callback data is unavailable, fallback to copy command");
    send::build_copy_button(
        "下载列表",
        &downloads_command_text(Some("run"), None, None, CommandStyle::Short),
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

/// 构造健康快照。
async fn build_health_snapshot() -> anyhow::Result<HealthSnapshot> {
    let app_context = crate::app_context::app_context();
    Ok(HealthSnapshot {
        transfer: store::list_transfer_health_snapshot(app_context.as_ref()).await?,
        clients: app_context.transfer_runtime.transfer_client_ids(),
        now: store::now_utc8(),
    })
}

/// 构造健康卡片正文。
fn format_health_text(snapshot: &HealthSnapshot) -> String {
    let transfer = &snapshot.transfer;
    [
        "运行健康".to_owned(),
        format!("状态：{}", card::code("ready")),
        card::DIVIDER.to_owned(),
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
        card::section("命令"),
        card::command_line("缓存", "/cache"),
        card::command_line("下载", "/downloads"),
        card::command_line("帮助", "/help"),
    ]
    .join("\n")
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

    // 健康页应像控制面板一样直接跳转/刷新，复制命令只作为兜底。
    #[test]
    fn test_build_health_buttons_prefer_callbacks() {
        let rows = build_health_buttons();
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in ["刷新", "下载列表", "文件缓存", "菜单"] {
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

    // 健康页仍保留短命令复制，方便 reply_markup 异常时手动排查。
    #[test]
    fn test_build_health_buttons_keep_copy_fallbacks() {
        let rows = build_health_buttons();
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"复制 /hl"));
        assert!(labels.contains(&"复制运行列表"));
        assert!(labels.contains(&"复制 /h"));
    }
}
