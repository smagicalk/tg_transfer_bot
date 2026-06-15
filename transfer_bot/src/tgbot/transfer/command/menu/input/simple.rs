// `/menu` 中的单步输入逻辑。
// 这里集中处理不需要“源链接 -> 目标 -> 确认”三段向导的输入项，例如 job_id 和 user_id。

use crate::tgbot::send;
use crate::tgbot::transfer::command::{job, points};

use super::super::text::build_menu_status_text;
use super::state::MenuJobAction;

/// 调用已有 `/job` 命令入口，避免菜单输入流复制任务状态迁移逻辑。
pub(super) async fn run_existing_job_command(
    action: MenuJobAction,
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_owned = [
        "/job".to_owned(),
        action.command_action().to_owned(),
        job_id.to_string(),
    ];
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    job::job_command(command_refs, actor, client_id).await
}

/// 调用已有 `/points history` 命令，避免菜单输入流复制积分流水查询逻辑。
pub(super) async fn run_existing_points_history_command(
    user_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let command_owned = [
        "/points".to_owned(),
        "history".to_owned(),
        user_id.to_string(),
    ];
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    points::points_command(command_refs, actor, client_id).await
}

/// 解析用户回复的任务编号。
///
/// `job_id` 来自数据库自增主键，必须是正整数；这里先过滤空白、符号和混合文本，避免把错误输入传到命令层。
pub(super) fn parse_job_id_input(input: &str) -> Option<i64> {
    let trimmed = input.trim();
    if trimmed.is_empty() || !trimmed.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    let job_id = trimmed.parse::<i64>().ok()?;
    (job_id > 0).then_some(job_id)
}

/// 解析用户回复的 Telegram 用户 ID。
///
/// `user_id` 同样只接受纯数字，避免把 `@username` 误当成可解析目标；如果后续支持 username，再单独接 TDLib 查询。
pub(super) fn parse_user_id_input(input: &str) -> Option<i64> {
    parse_job_id_input(input)
}

/// 发送取消提示；如果选群键盘可能残留，则顺带移除 reply keyboard。
pub(super) async fn send_cancelled_notice(
    request_chat_id: i64,
    client_id: i32,
    needs_reply_keyboard_cleanup: bool,
) -> anyhow::Result<()> {
    let text = build_menu_status_text(
        "已取消",
        "cancelled",
        "当前输入流程已取消，可重新打开 /menu。",
    );
    if needs_reply_keyboard_cleanup {
        return send::send_card_message_with_remove_keyboard(text, request_chat_id, client_id)
            .await;
    }

    send::ReplyPanel::card(text)
        .row(vec![send::build_callback_button(
            "返回菜单",
            &super::super::build_menu_home_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        )])
        .send(request_chat_id, client_id)
        .await
}

/// 发送“键盘已清理”提示。
///
/// TDLib 不能同时在同一条消息上携带 inline keyboard 和 remove keyboard，
/// 因此清理 reply keyboard 必须单独发一条短卡片，再发送确认/目标选择卡片。
pub(super) async fn send_keyboard_cleanup_notice(
    request_chat_id: i64,
    client_id: i32,
    title: &str,
    detail: &str,
) -> anyhow::Result<()> {
    send::send_card_message_with_remove_keyboard(
        build_menu_status_text(title, "keyboard-cleared", detail),
        request_chat_id,
        client_id,
    )
    .await
}

/// 菜单输入过期提示，跟随运行时配置展示实际超时时间。
pub(super) fn expired_input_detail() -> String {
    format!(
        "上一次菜单输入已超过 {}，请重新打开 /menu。",
        format_duration_hint(
            crate::tgbot::transfer::runtime_config()
                .menu_input_timeout_seconds
                .max(1)
        )
    )
}

/// 把秒数压缩成适合卡片展示的短文案。
pub(super) fn format_duration_hint(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{} 秒", seconds);
    }
    if seconds.is_multiple_of(3600) {
        return format!("{} 小时", seconds / 3600);
    }
    if seconds.is_multiple_of(60) {
        return format!("{} 分钟", seconds / 60);
    }
    format!("{} 秒", seconds)
}

/// 判断普通文本是否表示取消。
///
/// `取消` 来自 reply keyboard 的文本按钮；`cancel` 作为英文兜底，`/cancel` 由上层命令路由优先处理。
pub(super) fn is_cancel_text(input: &str) -> bool {
    input == "取消" || input.eq_ignore_ascii_case("cancel") || input.eq_ignore_ascii_case("/cancel")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tgbot::transfer::command::menu::build_step_prompt_text;

    // reply keyboard 的“取消”按钮会发回普通文本，状态机必须能直接识别。
    #[test]
    fn test_is_cancel_text() {
        assert!(is_cancel_text("取消"));
        assert!(is_cancel_text("cancel"));
        assert!(is_cancel_text("/cancel"));
        assert!(!is_cancel_text("继续"));
    }

    // 任务控制输入只接受正整数 job_id，避免把说明文字或负数传给 `/job`。
    #[test]
    fn test_parse_job_id_input() {
        assert_eq!(parse_job_id_input("42"), Some(42));
        assert_eq!(parse_job_id_input(" 42 "), Some(42));
        assert_eq!(parse_job_id_input("0"), None);
        assert_eq!(parse_job_id_input("-1"), None);
        assert_eq!(parse_job_id_input("job 42"), None);
        assert_eq!(parse_job_id_input(""), None);
    }

    // 用户积分流水输入同样只接受 Telegram 数字 user_id，username 解析后续单独接 TDLib。
    #[test]
    fn test_parse_user_id_input() {
        assert_eq!(parse_user_id_input("123456789"), Some(123456789));
        assert_eq!(parse_user_id_input("@alice"), None);
        assert_eq!(parse_user_id_input("-123"), None);
    }

    // 过期提示应跟随可配置秒数展示，不再写死默认 10 分钟。
    #[test]
    fn test_format_duration_hint() {
        assert_eq!(format_duration_hint(45), "45 秒");
        assert_eq!(format_duration_hint(600), "10 分钟");
        assert_eq!(format_duration_hint(7200), "2 小时");
        assert_eq!(format_duration_hint(95), "95 秒");
    }

    // 调度 job 命令时应使用公开长动作参数，不再泄露旧短动作。
    #[test]
    fn test_job_action_commands_use_public_names() {
        assert_eq!(MenuJobAction::Status.command_action(), "status");
        assert_eq!(MenuJobAction::Pause.command_action(), "pause");
        assert_eq!(MenuJobAction::Resume.command_action(), "resume");
        assert_eq!(MenuJobAction::Stop.command_action(), "stop");
    }

    // 单步输入提示仍统一使用 `1/1` 风格，和多步向导区分开。
    #[test]
    fn test_single_step_prompt_format() {
        let text = build_step_prompt_text("1/1", "任务详情", "请输入 job_id。");

        assert!(text.contains("步骤：‹1/1›"));
        assert!(text.contains("‹/cancel›"));
    }
}
