//! `/menu` 交互菜单中的单步简单输入与基础解析辅助模块。
//!
//! 集中处理不需要“源链接 -> 目标 -> 确认”三段流式向导的单步输入逻辑（如指定任务 ID 进行控制），
//! 以及用户取消操作判定、输入超时提示文案格式化等公共基础能力。

use crate::tgbot::send;
use crate::tgbot::transfer::command::job;

use super::super::text::build_menu_status_text;
use super::state::MenuJobAction;

/// 调用已有的 `/job` 模块命令入口执行任务状态迁移。
///
/// 避免在菜单输入流中复制繁琐的任务暂停、恢复、停止等底层业务状态校验逻辑。
///
/// # 参数说明
/// - `app`: 全局应用程序上下文句柄
/// - `action`: 任务操作动作类型（Status, Pause, Resume, Stop）
/// - `job_id`: 目标任务的唯一正整数 ID
/// - `actor`: 请求操作者身份上下文
/// - `client_id`: TDLib 客户端实例标识
pub(super) async fn run_existing_job_command(
    app: &crate::app_context::AppContext,
    action: MenuJobAction,
    job_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    // 构造标准的 `/job <action> <job_id>` 命令行参数数组
    let command_owned = [
        "/job".to_owned(),
        action.command_action().to_owned(),
        job_id.to_string(),
    ];
    let command_refs = command_owned.iter().map(String::as_str).collect::<Vec<_>>();
    // 转发给底层 job 命令分发器执行
    job::job_command_on(app, command_refs, actor, client_id).await
}

/// 解析用户回复的文本为有效的正整数任务编号（`job_id`）。
///
/// 校验规则：
/// - 剔除首尾空白；
/// - 必须非空且全部由 ASCII 数字构成；
/// - 解析后的数值必须大于 0（数据库主键约束）；
/// - 任何符号、负数或混杂字符均返回 `None`。
///
/// # 参数说明
/// - `input`: 用户回复的原始文本切片
///
/// # 返回值
/// 若合法返回 `Some(i64)`，否则返回 `None`。
pub(super) fn parse_job_id_input(input: &str) -> Option<i64> {
    let trimmed = input.trim();
    if trimmed.is_empty() || !trimmed.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    let job_id = trimmed.parse::<i64>().ok()?;
    (job_id > 0).then_some(job_id)
}

/// 向用户发送流程已取消的提示通知卡片。
///
/// # 参数说明
/// - `request_chat_id`: 目标接收聊天会话 ID
/// - `client_id`: TDLib 客户端实例标识符
/// - `remove_reply_keyboard`: 若为 true 则发送强制清除底部原生回复键盘的提示，否则发送带有“返回菜单”按钮的标准内联卡片
pub(super) async fn send_cancelled_notice(
    request_chat_id: i64,
    client_id: i32,
    remove_reply_keyboard: bool,
) -> anyhow::Result<()> {
    let text = build_menu_status_text(
        "已取消",
        "cancelled",
        "当前输入流程已取消，可从菜单重新开始。",
    );
    // 需要清理自定义回复键盘（如原生选聊按钮）
    if remove_reply_keyboard {
        return send::send_card_message_with_remove_keyboard(text, request_chat_id, client_id)
            .await;
    }

    // 默认展示附带“返回菜单”内联按钮的提示卡片
    send::ReplyPanel::card(text)
        .row(vec![send::build_callback_button(
            "返回菜单",
            &super::super::build_menu_home_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        )])
        .send(request_chat_id, client_id)
        .await
}

/// 构造菜单输入超时的详细错误提示文案。
///
/// 自动从运行时配置动态获取 `menu_input_timeout_seconds` 并转换为易读的时长文本。
///
/// # 参数说明
/// - `app`: 应用程序全局上下文引用
pub(super) fn expired_input_detail_on(app: &crate::app_context::AppContext) -> String {
    format!(
        "上一次菜单输入已超过 {}，请返回菜单重新开始。",
        format_duration_hint(
            crate::tgbot::transfer::runtime_config_on(app)
                .menu_input_timeout_seconds
                .max(1)
        )
    )
}

/// 将秒数压缩转换为适合在 Telegram 卡片中紧凑展示的人类易读时长短文案。
///
/// 转换规则：
/// - 小于 60 秒：直接展示 "x 秒"；
/// - 整小时（能被 3600 整除）：展示 "x 小时"；
/// - 整分钟（能被 60 整除）：展示 "x 分钟"；
/// - 其他情况降级展示 "x 秒"。
///
/// # 参数说明
/// - `seconds`: 秒数
pub(super) fn format_duration_hint(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{seconds} 秒");
    }
    if seconds.is_multiple_of(3600) {
        return format!("{} 小时", seconds / 3600);
    }
    if seconds.is_multiple_of(60) {
        return format!("{} 分钟", seconds / 60);
    }
    format!("{seconds} 秒")
}

/// 判断普通用户文本是否表达主动取消意图。
///
/// 匹配项：
/// - `"取消"`: 常见 ReplyKeyboard 按钮输出文本
/// - `"cancel"` (忽略大小写): 常见英文快捷输入
/// - `"/cancel"` (忽略大小写): 标准 Slash 命令兜底
pub(super) fn is_cancel_text(input: &str) -> bool {
    input == "取消" || input.eq_ignore_ascii_case("cancel") || input.eq_ignore_ascii_case("/cancel")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tgbot::transfer::command::menu::build_step_prompt_text;

    /// 测试取消文本的判断：必须能够精准识别中英文及带斜杠的取消指令。
    #[test]
    fn test_is_cancel_text() {
        assert!(is_cancel_text("取消"));
        assert!(is_cancel_text("cancel"));
        assert!(is_cancel_text("/cancel"));
        assert!(!is_cancel_text("继续"));
    }

    /// 测试任务 ID 的解析校验：只接受正整数，严格拒绝负数、0、混合字符串及空输入。
    #[test]
    fn test_parse_job_id_input() {
        assert_eq!(parse_job_id_input("42"), Some(42));
        assert_eq!(parse_job_id_input(" 42 "), Some(42));
        assert_eq!(parse_job_id_input("0"), None);
        assert_eq!(parse_job_id_input("-1"), None);
        assert_eq!(parse_job_id_input("job 42"), None);
        assert_eq!(parse_job_id_input(""), None);
    }

    /// 测试时长提示文案格式化输出。
    #[test]
    fn test_format_duration_hint() {
        assert_eq!(format_duration_hint(45), "45 秒");
        assert_eq!(format_duration_hint(600), "10 分钟");
        assert_eq!(format_duration_hint(7200), "2 小时");
        assert_eq!(format_duration_hint(95), "95 秒");
    }

    /// 测试任务动作枚举对应的底层命令动作标识（长格式名称）。
    #[test]
    fn test_job_action_commands_use_public_names() {
        assert_eq!(MenuJobAction::Status.command_action(), "status");
        assert_eq!(MenuJobAction::Pause.command_action(), "pause");
        assert_eq!(MenuJobAction::Resume.command_action(), "resume");
        assert_eq!(MenuJobAction::Stop.command_action(), "stop");
    }

    /// 测试单步输入提示格式统一采用 "1/1" 标记。
    #[test]
    fn test_single_step_prompt_format() {
        let text = build_step_prompt_text("1/1", "任务详情", "请输入 job_id。");

        assert!(text.contains("步骤：‹1/1›"));
        assert!(text.contains("回复“取消”结束当前流程"));
    }
}
