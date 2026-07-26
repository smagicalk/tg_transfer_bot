// `/lookup` 命令实现。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store;
use crate::tgbot::transfer::{refresh_stored_result_link, refresh_stored_result_messages};

use super::common::{
    CommandStyle, lookup_command as build_lookup_command, resolve_target_chat_id_on,
};
use super::{
    build_downloads_status_button_data, build_job_pause_button_data, build_job_resume_button_data,
    build_job_status_button_data, build_job_stop_button_data, build_menu_home_button_data,
    build_view_commands_button,
};

const LOOKUP_CALLBACK_PREFIX: &str = "lk:";

pub(super) fn is_lookup_callback_data(data: &str) -> bool {
    data.starts_with(LOOKUP_CALLBACK_PREFIX)
}

pub(in crate::tgbot::transfer) fn build_lookup_retry_transfer_callback_data() -> String {
    format!("{LOOKUP_CALLBACK_PREFIX}rt")
}

/// `/help lookup` 共用的详细说明正文。
///
/// 查询命令的命中规则和目标解析属于 lookup 模块语义，
/// 文案留在本模块维护，避免 help 模块重复理解历史结果与进行中任务的查询规则。
pub(in crate::tgbot::transfer::command) fn build_lookup_help_detail_text() -> String {
    [
        "lookup".to_owned(),
        "用途：按源链接查询历史转存结果。".to_owned(),
        "说明：target 可填数字 chat_id 或配置里的别名；命中成功任务时会返回目标消息入口或定位信息。"
            .to_owned(),
        "说明：查询会命中当前数据库中的历史结果或进行中任务。".to_owned(),
        card::DIVIDER.to_owned(),
        card::section("命令"),
        build_lookup_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
        String::new(),
        card::section("示例"),
        "/lookup https://t.me/c/123/456".to_owned(),
        "/lookup https://t.me/c/123/456 -1001234567890".to_owned(),
        "/lookup https://t.me/c/123/456 archive".to_owned(),
    ]
    .join("\n")
}

/// `/help lookup` 共用的按钮入口。
///
/// help 详情页正文已经给出命令示例，这里只保留真实交互入口；
/// 返回目录和菜单由 help 模块统一追加。
pub(in crate::tgbot::transfer::command) fn build_lookup_help_entry_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![vec![
        send::build_callback_button(
            "指定目标",
            &super::menu::build_menu_new_lookup_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "快速查询",
            &super::menu::build_menu_quick_lookup_default_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]]
}

/// 在指定上下文上执行 `/lookup`。
pub async fn lookup_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    _config: Arc<BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    if text.len() < 2 {
        anyhow::bail!("usage: /lookup <link> [target]");
    }

    let source_link = text[1].to_string();
    let target_chat_id = resolve_target_chat_id_on(app, &text, actor.request_chat_id)?;
    // 源链接可能来自私有聊天，日志只记录请求 chat 与目标 chat。
    tracing::info!(
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        owner_user_id = actor.user_id,
        target_chat_id,
        "lookup command started"
    );
    if let Some(job) =
        store::find_success_job_by_source_target(&source_link, target_chat_id).await?
    {
        let link = refresh_stored_result_link(
            job.id,
            job.target_chat_id,
            job.result_message_id,
            &job.result_message_link,
            super::super::transfer_client_ids()?.upload,
        )
        .await?;
        let result_messages = store::list_result_messages_by_job(job.id).await?;
        let result_messages = crate::tgbot::transfer::outcome::normalize_result_messages(
            result_messages,
            &link,
            target_chat_id,
        );
        let result_messages = refresh_stored_result_messages(
            job.id,
            result_messages,
            super::super::transfer_client_ids()?.upload,
        )
        .await?;
        tracing::info!(
            request_chat_id = actor.request_chat_id,
            target_chat_id,
            job_id = job.id,
            result_count = result_messages.len(),
            "lookup command hit success job"
        );
        let text = crate::tgbot::transfer::outcome::format_result_card_text(
            "已找到历史转存结果",
            &source_link,
            target_chat_id,
            Some(job.id),
            &result_messages,
        );
        let mut rows = crate::tgbot::transfer::outcome::build_result_message_rows(&result_messages);
        rows.extend(build_lookup_success_navigation_buttons(job.id));
        return crate::tgbot::transfer::outcome::send_result_card(
            text,
            rows,
            &result_messages,
            actor.request_chat_id,
            client_id,
        )
        .await;
    }

    if let Some(job) = store::find_active_job_by_source_target(&source_link, target_chat_id).await?
    {
        tracing::info!(
            request_chat_id = actor.request_chat_id,
            target_chat_id,
            job_id = job.id,
            status = %job.status,
            "lookup command hit active job"
        );
        return send::ReplyPanel::card(format_lookup_active_text(
            &source_link,
            target_chat_id,
            job.id,
            &job.status,
        ))
        .rows(build_lookup_active_button_rows(job.id, &job.status))
        .send(actor.request_chat_id, client_id)
        .await;
    }

    tracing::info!(
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        owner_user_id = actor.user_id,
        target_chat_id,
        "lookup command missed"
    );
    let sent = send::send_card_message_with_buttons_returning(
        format_lookup_miss_text(&source_link, target_chat_id),
        actor.request_chat_id,
        build_lookup_miss_button_rows(),
        client_id,
    )
    .await?;
    app.lookup_retry.put_context(
        actor.request_chat_id,
        actor.user_id,
        sent.id,
        crate::app_context::LookupRetryContext {
            source_link,
            target_chat_id,
        },
    );
    Ok(())
}

/// 构建 lookup 命中成功结果时的导航按钮。
///
/// lookup 成功页复用统一结果导航层级，保持结果页布局一致。
fn build_lookup_success_navigation_buttons(
    job_id: i64,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    crate::tgbot::transfer::outcome::build_result_navigation_rows(
        Some(job_id),
        "查看完成列表",
        "done",
    )
}

/// 构建 lookup 未命中时的按钮。
///
/// “重新转存”通过短 callback + 进程内上下文触发，避免把长链接塞进 callback_data。
fn build_lookup_miss_button_rows() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![vec![
        send::build_callback_button(
            "重新转存",
            &build_lookup_retry_transfer_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        build_view_commands_button(Some("lookup")),
        send::build_callback_button(
            "菜单",
            &build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]]
}

pub async fn lookup_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: Arc<BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };
    if payload != build_lookup_retry_transfer_callback_data() {
        send::answer_callback_query(update.id, Some("查询按钮参数无效"), client_id).await?;
        return Ok(());
    }

    let Some(context) =
        app.lookup_retry
            .take_context(update.chat_id, update.sender_user_id, update.message_id)
    else {
        send::answer_callback_query(update.id, Some("重试入口已失效"), client_id).await?;
        send::ReplyPanel::card(super::menu::build_menu_recovery_text_for_outer(
            "重新转存入口已失效",
            "expired",
            "请点击“重新开始”重新选择来源和目标。",
        ))
        .rows(build_expired_retry_button_rows())
        .send(update.chat_id, client_id)
        .await?;
        return Ok(());
    };

    send::answer_callback_query(update.id, Some("开始重新转存"), client_id).await?;
    super::menu::discard_menu_input_for_command(update.chat_id, update.sender_user_id, client_id)
        .await?;
    let target = context.target_chat_id.to_string();
    super::transfer_cmd::transfer_link_command_on(
        Arc::new(app.clone()),
        vec!["/transfer", context.source_link.as_str(), target.as_str()],
        config,
        update.chat_id,
        super::transfer_cmd::TransferCommandContext {
            request_message_id: update.message_id,
            interaction_message_id: Some(update.message_id),
            actor,
            client_id,
        },
    )
    .await
}

/// 重新转存上下文失效后的恢复入口。
fn build_expired_retry_button_rows() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![vec![
        send::build_callback_button(
            "重新开始",
            &super::menu::build_menu_new_transfer_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "菜单",
            &build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]]
}

/// 构建 lookup 命中进行中任务时的控制按钮。
///
/// 控制按钮复用 `/job` callback；停止按钮会先进入确认页。
/// 这里不再额外复制 `job_id`，避免和“查看任务详情”形成重复入口。
fn build_lookup_active_control_buttons(
    job_id: i64,
    status: &str,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    let mut row = Vec::new();
    if status == store::JOB_STATUS_PAUSED {
        row.push(send::build_callback_button(
            "恢复",
            &build_job_resume_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        ));
    } else if matches!(
        status,
        store::JOB_STATUS_PENDING | store::JOB_STATUS_RUNNING
    ) {
        row.push(send::build_callback_button(
            "暂停",
            &build_job_pause_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        ));
    }

    if !matches!(
        status,
        store::JOB_STATUS_CANCELLED
            | store::JOB_STATUS_CANCELLING
            | store::JOB_STATUS_CANCEL_FINALIZING
    ) {
        row.push(send::build_callback_button(
            "停止",
            &build_job_stop_button_data(job_id),
            tdlib_rs::enums::ButtonStyle::Danger,
        ));
    }
    row
}

/// 构建 lookup 命中进行中任务时的完整按钮层级。
fn build_lookup_active_button_rows(
    job_id: i64,
    status: &str,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut action_row = vec![send::build_callback_button(
        "查看任务详情",
        &build_job_status_button_data(job_id),
        tdlib_rs::enums::ButtonStyle::Primary,
    )];
    action_row.extend(build_lookup_active_control_buttons(job_id, status));
    let mut rows = vec![action_row];
    rows.push(vec![
        send::build_callback_button(
            "返回列表",
            &build_downloads_status_button_data(status, 8),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        build_view_commands_button(Some("job")),
        send::build_callback_button(
            "菜单",
            &build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]);
    rows
}

/// 构造命中进行中任务时的查询卡片。
fn format_lookup_active_text(
    source_link: &str,
    target_chat_id: i64,
    job_id: i64,
    status: &str,
) -> String {
    let mut lines = vec![
        "找到进行中的转存任务".to_owned(),
        card::status_job_target(status, job_id, target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("下一步"),
        "可直接用按钮控制任务或查看对应列表；需要命令时点击“查看命令”。".to_owned(),
        String::new(),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

/// 构造未命中历史结果时的查询卡片。
fn format_lookup_miss_text(source_link: &str, target_chat_id: i64) -> String {
    let mut lines = vec![
        "未找到转存结果".to_owned(),
        card::status_target("miss", target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("下一步"),
        "可直接点击下方“重新转存”；需要命令时点击“查看命令”。".to_owned(),
        String::new(),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{
        build_expired_retry_button_rows, build_lookup_active_button_rows,
        build_lookup_active_control_buttons, build_lookup_miss_button_rows,
        build_lookup_success_navigation_buttons, format_lookup_active_text,
        format_lookup_miss_text, is_lookup_callback_data,
    };

    // lookup 命中运行中任务时应使用 card 标记，避免 Markdown 原文泄露到消息里。
    #[test]
    fn test_format_lookup_active_text() {
        let text = format_lookup_active_text("https://t.me/c/1/2", -100, 42, "running");

        assert!(text.contains("状态：‹running›"));
        assert!(text.contains("job：‹#42›"));
        assert!(!text.contains("/downloads run"));
        assert!(text.contains("需要命令时点击“查看命令”"));
        assert!(text.contains("可直接用按钮控制任务"));
    }

    // lookup 未命中时应保留源链接并给出 miss 状态。
    #[test]
    fn test_format_lookup_miss_text() {
        let text = format_lookup_miss_text("https://t.me/c/1/2", -100);

        assert!(text.contains("状态：‹miss›"));
        assert!(text.contains("‹https://t.me/c/1/2›"));
        assert!(text.contains("可直接点击下方“重新转存”"));
    }

    // lookup 命中运行中任务时，应给暂停 callback 和一次点击停止 callback，而不是只能复制命令。
    #[test]
    fn test_build_lookup_active_control_buttons_for_running() {
        let buttons = build_lookup_active_control_buttons(42, "running");
        use base64::{Engine as _, engine::general_purpose};

        assert_eq!(buttons[0].text, "暂停");
        assert_eq!(buttons[1].text, "停止");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &buttons[1].r#type
        else {
            panic!("stop button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "j:sc:42");
        assert!(matches!(
            buttons[0].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
        assert!(matches!(
            buttons[1].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    // paused 任务应给恢复 callback；停止中任务不再展示停止按钮。
    #[test]
    fn test_build_lookup_active_control_buttons_by_status() {
        let paused = build_lookup_active_control_buttons(42, "paused");
        let cancelling = build_lookup_active_control_buttons(42, "cancelling");

        assert_eq!(paused[0].text, "恢复");
        assert!(paused.iter().any(|button| button.text == "停止"));
        assert!(!cancelling.iter().any(|button| button.text == "停止"));
        assert!(!paused.iter().any(|button| button.text == "复制 job_id"));
        assert!(!cancelling.iter().any(|button| button.text == "复制 job_id"));
    }

    // 查询命中卡应先展示任务详情和控制，最后再放列表、命令和菜单导航。
    #[test]
    fn test_build_lookup_active_button_rows_prioritizes_job_actions() {
        let rows = build_lookup_active_button_rows(42, "running");

        assert_eq!(rows[0][0].text, "查看任务详情");
        assert_eq!(rows[0][1].text, "暂停");
        assert_eq!(rows[0][2].text, "停止");
        assert_eq!(rows[0][2].style, tdlib_rs::enums::ButtonStyle::Danger);
        assert_eq!(rows[1][0].text, "返回列表");
        assert_eq!(rows[1][1].text, "查看命令");
        assert_eq!(rows[1][2].text, "菜单");
        assert_eq!(rows.len(), 2);
    }

    // 停止中任务没有可执行控制时，详情和导航之间不能插入空行。
    #[test]
    fn test_build_lookup_active_button_rows_omits_empty_controls() {
        let rows = build_lookup_active_button_rows(42, "cancelling");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0].text, "查看任务详情");
        assert_eq!(rows[1][0].text, "返回列表");
        assert_eq!(rows[1][1].text, "查看命令");
        assert_eq!(rows[1][2].text, "菜单");
    }

    // lookup 成功命中已有结果时，按钮区应使用真实 callback 导航，不再重复复制查询/重转命令。
    #[test]
    fn test_build_lookup_success_navigation_buttons_drop_command_copy_buttons() {
        let rows = build_lookup_success_navigation_buttons(42);
        let buttons = rows.iter().flatten().collect::<Vec<_>>();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 1);
        assert_eq!(rows[0][0].text, "查看任务详情");
        assert_eq!(rows[1].len(), 3);
        assert_eq!(rows[1][0].text, "查看完成列表");
        assert_eq!(rows[1][1].text, "查看命令");
        assert_eq!(rows[1][2].text, "菜单");
        assert!(!buttons.iter().any(|button| button.text == "复制查询命令"));
        assert!(!buttons.iter().any(|button| button.text == "复制重新转存"));
    }

    // lookup 未命中时应优先给真实 callback，只保留重新转存和菜单，不再重复复制源链接。
    #[test]
    fn test_build_lookup_miss_button_rows_keep_only_retry_buttons() {
        let rows = build_lookup_miss_button_rows();
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"重新转存"));
        assert!(labels.contains(&"菜单"));
        assert!(!labels.contains(&"复制源链接"));
        assert!(!labels.contains(&"复制查询命令"));
    }

    #[test]
    fn test_lookup_retry_callback_prefix() {
        assert!(is_lookup_callback_data("lk:rt"));
        assert!(!is_lookup_callback_data("l:rt"));
    }

    // 重试上下文过期后应能直接重新开始转存，不必先返回菜单再找入口。
    #[test]
    fn test_build_expired_retry_button_rows_restart_transfer_directly() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = build_expired_retry_button_rows();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0].text, "重新开始");
        assert_eq!(rows[0][0].style, tdlib_rs::enums::ButtonStyle::Primary);
        assert_eq!(rows[0][1].text, "菜单");

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("restart button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "m:new");
    }
}
