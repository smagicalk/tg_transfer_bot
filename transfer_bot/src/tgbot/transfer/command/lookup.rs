// `/lookup` 命令实现。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;
use crate::tgbot::transfer::store;
use crate::tgbot::transfer::{refresh_stored_result_link, refresh_stored_result_messages};

use super::common::{
    CommandStyle, job_command as build_job_command, lookup_command as build_lookup_command,
    resolve_target_chat_id, transfer_command as build_transfer_command,
};
use super::{
    build_downloads_filter_button_data, build_downloads_status_button_data,
    build_job_pause_button_data, build_job_resume_button_data, build_job_status_button_data,
    build_job_stop_button_data, build_menu_home_button_data,
};

/// `/lookup` 命令入口。
/// 命令格式：`/lookup <link> [target]`
/// 用于按源链接查询历史转存结果。
pub async fn lookup_command(
    text: Vec<&str>,
    config: Arc<BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    if text.len() < 2 {
        anyhow::bail!("usage: /lookup <link> [target]");
    }

    let source_link = text[1].to_string();
    let target_chat_id = resolve_target_chat_id(&text, &config, actor.request_chat_id)?;
    // 源链接可能来自私有聊天，日志只记录请求 chat 与目标 chat。
    tracing::info!(
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        actor_role = actor.role.as_str(),
        target_chat_id,
        "lookup command started"
    );
    let transfer_command = build_transfer_command(&source_link, target_chat_id, CommandStyle::Long);

    if let Some(job) =
        store::find_success_job_by_source_target(&source_link, target_chat_id, actor.owner_scope())
            .await?
    {
        let link = refresh_stored_result_link(
            job.id,
            job.target_chat_id,
            job.result_message_id,
            &job.result_message_link,
            config.transfer_client_ids()?.upload,
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
            config.transfer_client_ids()?.upload,
        )
        .await?;
        tracing::info!(
            request_chat_id = actor.request_chat_id,
            target_chat_id,
            job_id = job.id,
            result_count = result_messages.len(),
            "lookup command hit success job"
        );
        let mut panel =
            send::ReplyPanel::card(crate::tgbot::transfer::outcome::format_result_card_text(
                "已找到历史转存结果",
                &source_link,
                target_chat_id,
                Some(job.id),
                &result_messages,
            ));
        for result in result_messages.iter().take(6) {
            let idx = result.result_index + 1;
            let mut row = Vec::new();
            if send::is_openable_url(&result.message_link) {
                row.push(send::build_url_button(
                    &format!("打开结果 {}", idx),
                    &result.message_link,
                    if idx == 1 {
                        tdlib_rs::enums::ButtonStyle::Primary
                    } else {
                        tdlib_rs::enums::ButtonStyle::Default
                    },
                ));
            }
            row.push(send::build_copy_button(
                &format!("复制结果 {}", idx),
                &result.message_link,
                if send::is_openable_url(&result.message_link) {
                    tdlib_rs::enums::ButtonStyle::Default
                } else {
                    tdlib_rs::enums::ButtonStyle::Primary
                },
            ));
            panel = panel.row(row);
        }
        return panel
            .row(build_lookup_success_navigation_buttons(job.id))
            .send(actor.request_chat_id, client_id)
            .await;
    }

    if let Some(job) =
        store::find_active_job_by_source_target(&source_link, target_chat_id, actor.owner_scope())
            .await?
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
        .row(vec![
            send::build_callback_button(
                "查看任务详情",
                &build_job_status_button_data(job.id),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "返回列表",
                &build_downloads_status_button_data(&job.status, 8),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ])
        .row(build_lookup_active_control_buttons(job.id, &job.status))
        .send(actor.request_chat_id, client_id)
        .await;
    }

    tracing::info!(
        request_chat_id = actor.request_chat_id,
        owner_user_id = actor.user_id,
        actor_role = actor.role.as_str(),
        target_chat_id,
        "lookup command missed"
    );
    send::ReplyPanel::card(format_lookup_miss_text(&source_link, target_chat_id))
        .rows(build_lookup_miss_button_rows(
            &transfer_command,
            &source_link,
        ))
        .send(actor.request_chat_id, client_id)
        .await
}

/// 构建 lookup 命中成功结果时的导航按钮。
///
/// 查询命令和重转命令已经在正文里保留；按钮区只放可以直接点击执行的动作和结果数据入口。
fn build_lookup_success_navigation_buttons(
    job_id: i64,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    let mut row = vec![send::build_callback_button(
        "查看任务详情",
        &build_job_status_button_data(job_id),
        tdlib_rs::enums::ButtonStyle::Default,
    )];
    if let Some(callback_data) = build_downloads_filter_button_data("done", 8) {
        row.push(send::build_callback_button(
            "查看完成列表",
            &callback_data,
            tdlib_rs::enums::ButtonStyle::Primary,
        ));
    }
    row.push(send::build_callback_button(
        "菜单",
        &build_menu_home_button_data(),
        tdlib_rs::enums::ButtonStyle::Default,
    ));
    row
}

/// 构建 lookup 未命中时的按钮。
///
/// 这里没有已存在的任务可 callback 执行，因此只保留真正需要用户复制的数据和重发命令。
fn build_lookup_miss_button_rows(
    transfer_command: &str,
    source_link: &str,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_copy_button(
                "复制转存命令",
                transfer_command,
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![send::build_copy_button(
            "复制源链接",
            source_link,
            tdlib_rs::enums::ButtonStyle::Default,
        )],
    ]
}

/// 构建 lookup 命中进行中任务时的直接控制按钮。
///
/// 控制按钮复用 `/job` callback；正文里已经保留完整命令，因此按钮区只保留真正的交互操作和 `job_id`。
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
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    row.push(send::build_copy_button(
        "复制 job_id",
        &job_id.to_string(),
        tdlib_rs::enums::ButtonStyle::Default,
    ));
    row
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
        format!(
            "可直接用按钮控制任务，或使用 {} 查看运行列表。",
            card::code("/downloads run")
        ),
        card::section("命令"),
        card::command_line(
            "详情",
            build_job_command("status", job_id, CommandStyle::Long),
        ),
        card::command_line(
            "暂停",
            build_job_command("pause", job_id, CommandStyle::Long),
        ),
        card::command_line(
            "停止",
            build_job_command("stop", job_id, CommandStyle::Long),
        ),
        card::command_line("列表", "/downloads run"),
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
        "复制转存命令后重新发起任务。".to_owned(),
        card::section("命令"),
        card::command_line(
            "转存",
            build_transfer_command(source_link, target_chat_id, CommandStyle::Long),
        ),
        card::command_line(
            "查询",
            build_lookup_command(source_link, target_chat_id, CommandStyle::Long),
        ),
        String::new(),
    ];
    lines.extend(card::source_block(source_link));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{
        build_lookup_active_control_buttons, build_lookup_miss_button_rows,
        build_lookup_success_navigation_buttons, format_lookup_active_text,
        format_lookup_miss_text,
    };

    // lookup 命中运行中任务时应使用 card 标记，避免 Markdown 原文泄露到消息里。
    #[test]
    fn test_format_lookup_active_text() {
        let text = format_lookup_active_text("https://t.me/c/1/2", -100, 42, "running");

        assert!(text.contains("状态：‹running›"));
        assert!(text.contains("job：‹#42›"));
        assert!(text.contains("‹/downloads run›"));
        assert!(text.contains("可直接用按钮控制任务"));
    }

    // lookup 未命中时应保留源链接并给出 miss 状态。
    #[test]
    fn test_format_lookup_miss_text() {
        let text = format_lookup_miss_text("https://t.me/c/1/2", -100);

        assert!(text.contains("状态：‹miss›"));
        assert!(text.contains("‹https://t.me/c/1/2›"));
    }

    // lookup 命中运行中任务时，应直接给暂停/停止 callback，而不是只能复制命令。
    #[test]
    fn test_build_lookup_active_control_buttons_for_running() {
        let buttons = build_lookup_active_control_buttons(42, "running");

        assert_eq!(buttons[0].text, "暂停");
        assert_eq!(buttons[1].text, "停止");
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
        assert!(cancelling.iter().any(|button| button.text == "复制 job_id"));
    }

    // lookup 成功命中已有结果时，按钮区应使用真实 callback 导航，不再重复复制查询/重转命令。
    #[test]
    fn test_build_lookup_success_navigation_buttons_drop_command_copy_buttons() {
        let buttons = build_lookup_success_navigation_buttons(42);

        assert!(buttons.iter().any(|button| button.text == "查看任务详情"));
        assert!(buttons.iter().any(|button| button.text == "查看完成列表"));
        assert!(buttons.iter().any(|button| button.text == "菜单"));
        assert!(!buttons.iter().any(|button| button.text == "复制查询命令"));
        assert!(!buttons.iter().any(|button| button.text == "复制重新转存"));
    }

    // lookup 未命中时没有可执行 callback，只保留发起转存和源链接这两个必要复制入口。
    #[test]
    fn test_build_lookup_miss_button_rows_keep_only_needed_copy_buttons() {
        let rows = build_lookup_miss_button_rows(
            "/transfer https://t.me/c/1/2 -100",
            "https://t.me/c/1/2",
        );
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"复制转存命令"));
        assert!(labels.contains(&"复制源链接"));
        assert!(labels.contains(&"菜单"));
        assert!(!labels.contains(&"复制查询命令"));
    }
}
