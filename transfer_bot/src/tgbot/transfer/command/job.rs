// `/job` 命令入口。
// 参数解析和具体控制动作拆到子模块，入口只负责分发。

mod actions;
mod args;
mod callback;
mod keyboard;
mod render;
mod status_meta;

use super::common::{CommandStyle, job_command as build_job_command};
use crate::tgbot::send;
use crate::tgbot::send::send_interaction_error_card;
use crate::tgbot::transfer::card;
use args::{
    JobAction, JobCallbackAction, is_job_callback_data, parse_job_args, parse_job_callback_data,
};
use callback::{JobCallbackResult, handle_job_callback};

/// 判断 callback payload 是否属于 `/job`。
pub(super) fn is_job_callback_payload(data: &str) -> bool {
    is_job_callback_data(data)
}

/// 生成单任务详情按钮所需的 `/job status` callback 数据。
///
/// 这个包装函数给 `/downloads` 复用，避免它直接依赖 `/job` 的内部参数枚举。
pub(super) fn build_job_status_callback_data(job_id: i64) -> String {
    args::build_job_callback_data(args::JobCallbackAction::Status, job_id)
}

/// 生成单任务暂停按钮所需的 callback 数据。
pub(super) fn build_job_pause_callback_data(job_id: i64) -> String {
    args::build_job_callback_data(args::JobCallbackAction::Pause, job_id)
}

/// 生成单任务恢复按钮所需的 callback 数据。
pub(super) fn build_job_resume_callback_data(job_id: i64) -> String {
    args::build_job_callback_data(args::JobCallbackAction::Resume, job_id)
}

/// 生成单任务停止确认按钮所需的 callback 数据。
///
/// 所有普通 UI 入口都先进入确认页，避免列表页或最近任务页误触后直接停止任务。
pub(super) fn build_job_stop_callback_data(job_id: i64) -> String {
    args::build_job_callback_data(args::JobCallbackAction::StopConfirm, job_id)
}

/// 生成确认页里“真正停止”按钮所需的 callback 数据。
///
/// 旧消息上的 `j:s:<job_id>` 仍然兼容真正停止，因此这个函数只在确认页内部使用。
pub(super) fn build_job_stop_execute_callback_data(job_id: i64) -> String {
    args::build_job_callback_data(args::JobCallbackAction::Stop, job_id)
}

/// 给非 `/job` 模块使用的任务列表入口信息。
///
/// 外部卡片只需要知道“跳到哪个列表”和“按钮显示什么”，不需要依赖
/// `/job` 内部的完整状态元信息结构。
pub(super) fn job_list_button_meta(status: &str) -> (&'static str, &'static str) {
    let meta = status_meta::job_status_meta(status);
    (meta.list_filter, meta.list_button_label)
}

/// `/job` 帮助和目录页共用的用途描述。
pub(in crate::tgbot::transfer::command) fn job_help_purpose() -> &'static str {
    "手动控制转存任务。"
}

/// `/job` 帮助和目录页共用的一句话摘要。
pub(in crate::tgbot::transfer::command) fn job_help_summary() -> &'static str {
    "手动控制转存任务；支持暂停、恢复、停止和单任务详情。"
}

/// `/job` 帮助详情和菜单页共用的说明。
pub(in crate::tgbot::transfer::command) fn job_help_intro_lines() -> Vec<String> {
    vec![
        "先选择任务状态，再点任务详情进行暂停、恢复或停止，无需手动输入 job_id。".to_owned(),
        "单所有者模式可直接控制任意任务。".to_owned(),
        "已知 job_id 时仍可直接使用 /job 命令。".to_owned(),
    ]
}

/// `/job` 帮助详情里统一展示的动作说明。
pub(in crate::tgbot::transfer::command) fn job_help_action_lines() -> Vec<String> {
    vec![
        format!(
            "{}：暂停任务，当前单次 TDLib 调用会在安全点停止。",
            card::code("pause")
        ),
        format!(
            "{}：唤醒 paused/pending/running 任务；若当前进程已有执行器则不会重复派发。",
            card::code("resume")
        ),
        format!(
            "{}：停止任务并释放文件引用，文件按删除队列延迟清理。",
            card::code("stop")
        ),
        format!(
            "{}：查看单任务详情、阶段计数和真实下载进度。",
            card::code("status")
        ),
    ]
}

/// `/job` 菜单页复用的命令示例。
pub(in crate::tgbot::transfer::command) fn job_menu_command_lines() -> Vec<String> {
    vec![
        card::command_line(
            "详情",
            format!(
                "{} status <job_id>",
                super::common::command_root("job", CommandStyle::Long)
            ),
        ),
        card::command_line(
            "暂停",
            format!(
                "{} pause <job_id>",
                super::common::command_root("job", CommandStyle::Long)
            ),
        ),
        card::command_line(
            "恢复",
            format!(
                "{} resume <job_id>",
                super::common::command_root("job", CommandStyle::Long)
            ),
        ),
        card::command_line(
            "停止",
            format!(
                "{} stop <job_id>",
                super::common::command_root("job", CommandStyle::Long)
            ),
        ),
    ]
}

/// `/job` 帮助详情复用的示例命令。
pub(in crate::tgbot::transfer::command) fn job_help_example_commands() -> Vec<String> {
    vec![
        build_job_command("pause", 123, CommandStyle::Long),
        build_job_command("resume", 123, CommandStyle::Long),
        build_job_command("stop", 123, CommandStyle::Long),
        build_job_command("status", 123, CommandStyle::Long),
    ]
}

/// `/help job` 和其他外层入口共用的详细说明正文。
pub(in crate::tgbot::transfer::command) fn build_job_help_detail_text() -> String {
    let mut lines = vec!["job".to_owned(), format!("用途：{}", job_help_purpose())];
    lines.extend(
        job_help_intro_lines()
            .into_iter()
            .map(|line| format!("说明：{}", line)),
    );
    lines.extend([
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        format!(
            "{} <pause|resume|stop|status> <job_id>",
            super::common::command_root("job", CommandStyle::Long)
        ),
        String::new(),
        "动作：".to_owned(),
    ]);
    lines.extend(job_help_action_lines());
    lines.extend([String::new(), "示例：".to_owned()]);
    lines.extend(job_help_example_commands());
    lines.join("\n")
}

/// `/help job` 详情页共用的按钮入口。
///
/// 这里把任务控制和任务详情统一收在 `/job` 模块里，避免 help 层重复维护。
///
/// 详情页已经提供输入式 callback，继续复制模板会和真实交互重复；
/// 命令示例仍保留在正文里，方便需要命令模式时查看。
pub(in crate::tgbot::transfer::command) fn build_job_help_entry_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    build_job_menu_filter_rows()
}

/// `/menu` 任务页复用的筛选按钮行。
pub(in crate::tgbot::transfer::command) fn build_job_menu_filter_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    [
        [
            ("最近任务", "all", tdlib_rs::enums::ButtonStyle::Primary),
            ("运行任务", "run", tdlib_rs::enums::ButtonStyle::Default),
            ("暂停任务", "pause", tdlib_rs::enums::ButtonStyle::Default),
        ]
        .as_slice(),
        [
            (
                "停止中",
                "cancelling",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            ("已停止", "cancel", tdlib_rs::enums::ButtonStyle::Default),
            ("失败任务", "fail", tdlib_rs::enums::ButtonStyle::Default),
        ]
        .as_slice(),
        [
            ("成功任务", "ok", tdlib_rs::enums::ButtonStyle::Default),
            ("就绪任务", "ready", tdlib_rs::enums::ButtonStyle::Default),
        ]
        .as_slice(),
    ]
    .into_iter()
    .map(|row| {
        row.iter()
            .map(|(text, filter, style)| {
                send::build_callback_button(
                    text,
                    &super::require_downloads_filter_button_data(filter, 8),
                    style.clone(),
                )
            })
            .collect::<Vec<_>>()
    })
    .collect()
}

/// `/job` 命令入口。
/// 命令格式：`/job <pause|resume|stop|status> <job_id>`
pub async fn job_command(
    text: Vec<&str>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let app_context = crate::app_context::app_context();
    job_command_on(app_context.as_ref(), text, actor, client_id).await
}

/// 在指定上下文上执行 `/job` 命令。
pub(in crate::tgbot) async fn job_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let args = parse_job_args(&text)?;

    match args.action {
        JobAction::Pause => actions::pause_job_on(app, args.job_id, actor, client_id).await,
        JobAction::Resume => actions::resume_job_on(app, args.job_id, actor, client_id).await,
        JobAction::Stop => actions::stop_job_on(app, args.job_id, actor, client_id).await,
        JobAction::Status => actions::show_job_status_on(app, args.job_id, actor, client_id).await,
    }
}

/// 在指定上下文上处理 `/job` inline keyboard 回调。
///
/// 这样外层统一 callback 路由在已经拿到 `AppContext` 时，可以整条链继续显式传递。
pub(in crate::tgbot) async fn job_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            crate::tgbot::send::answer_callback_query(
                update.id,
                Some("暂不支持这种按钮类型"),
                client_id,
            )
            .await?;
            return Ok(());
        }
    };

    let Some(args) = parse_job_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("任务按钮参数无效"), client_id).await?;
        return Ok(());
    };

    send::answer_callback_query(
        update.id,
        Some(job_callback_started_tip(args.action)),
        client_id,
    )
    .await?;

    match handle_job_callback(
        app,
        args.action,
        args.job_id,
        actor,
        update.message_id,
        client_id,
    )
    .await
    {
        Ok(JobCallbackResult::Updated) => Ok(()),
        Ok(JobCallbackResult::ConfirmFailed(err)) => {
            send_job_confirm_error(update.chat_id, client_id, args.job_id, &err).await?;
            Err(err)
        }
        Ok(JobCallbackResult::RefreshFailed(err)) => {
            send_job_refresh_error(update.chat_id, client_id, args.job_id, &err).await?;
            Err(err)
        }
        Err(err) => {
            send_job_callback_error(update.chat_id, client_id, args.job_id, &err).await?;
            Err(err)
        }
    }
}

/// 任务按钮点击后的即时提示。
fn job_callback_started_tip(action: JobCallbackAction) -> &'static str {
    match action {
        JobCallbackAction::Pause => "正在暂停",
        JobCallbackAction::Resume => "正在恢复",
        JobCallbackAction::StopConfirm => "请确认停止",
        JobCallbackAction::Stop => "正在停止",
        JobCallbackAction::Status => "正在刷新",
    }
}

/// 任务按钮失败提示。
///
/// callback 已经先 ACK，失败时不能再 answer 同一个 callback，因此发送一条短卡片说明错误。
async fn send_job_callback_error(
    request_chat_id: i64,
    client_id: i32,
    job_id: i64,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    let title = format!("任务操作失败 #{}", job_id);
    send_interaction_error_card(
        request_chat_id,
        client_id,
        &title,
        "任务状态未更新，请检查日志或复制错误信息。",
        err,
    )
    .await
}

/// 任务已处理但详情卡片编辑失败时的提示。
async fn send_job_refresh_error(
    request_chat_id: i64,
    client_id: i32,
    job_id: i64,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    let title = format!("任务详情刷新失败 #{}", job_id);
    send_interaction_error_card(
        request_chat_id,
        client_id,
        &title,
        "任务操作已处理，但原详情卡片未刷新；可重新打开任务详情确认状态。",
        err,
    )
    .await
}

/// 停止确认页编辑失败时的提示。
async fn send_job_confirm_error(
    request_chat_id: i64,
    client_id: i32,
    job_id: i64,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    let title = format!("停止确认失败 #{}", job_id);
    send_interaction_error_card(
        request_chat_id,
        client_id,
        &title,
        "停止确认页未能打开；请重新打开任务详情再试。",
        err,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    // callback 应先给即时提示，避免 Telegram 客户端按钮持续转圈。
    #[test]
    fn test_job_callback_started_tip() {
        assert_eq!(
            job_callback_started_tip(JobCallbackAction::Pause),
            "正在暂停"
        );
        assert_eq!(
            job_callback_started_tip(JobCallbackAction::Resume),
            "正在恢复"
        );
        assert_eq!(
            job_callback_started_tip(JobCallbackAction::StopConfirm),
            "请确认停止"
        );
        assert_eq!(
            job_callback_started_tip(JobCallbackAction::Stop),
            "正在停止"
        );
        assert_eq!(
            job_callback_started_tip(JobCallbackAction::Status),
            "正在刷新"
        );
    }
}
