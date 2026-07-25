// 命令层公共工具：
// - 目标 chat 解析
// - 统一构造命令字符串
// - 各命令共用的基础依赖

use crate::tgbot::transfer::card;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommandStyle {
    Long,
}

/// 解析目标 chat_id：
/// 1. 命令参数显式指定数字 chat_id 或 `targets.aliases` 别名
/// 2. 否则尝试 `targets.default_chat_id` 兜底
/// 3. 仍未配置时，默认回退到当前请求 chat 本身
#[cfg(test)]
pub(crate) fn resolve_target_chat_id(text: &[&str], request_chat_id: i64) -> anyhow::Result<i64> {
    resolve_target_chat_id_on(
        crate::app_context::app_context().as_ref(),
        text,
        request_chat_id,
    )
}

/// 在指定上下文上解析目标 chat_id。
///
/// 高层命令入口如果已经拿到 `AppContext`，优先走这个版本，避免再次抓全局运行态。
pub(crate) fn resolve_target_chat_id_on(
    app: &crate::app_context::AppContext,
    text: &[&str],
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    let targets_config = crate::tgbot::transfer::targets_runtime_config_on(app);
    let target_chat_id = if text.len() >= 3 {
        parse_target_arg(text[2], &targets_config)?
    } else if targets_config.default_chat_id != 0 {
        targets_config.default_chat_id
    } else {
        request_chat_id
    };

    Ok(target_chat_id)
}

/// 解析命令里的目标参数。
///
/// 目标可以是数字 chat_id，也可以是 `targets.aliases` 中配置的短名称。
fn parse_target_arg(arg: &str, config: &crate::config::TargetsConfig) -> anyhow::Result<i64> {
    if let Ok(chat_id) = arg.parse::<i64>() {
        return Ok(chat_id);
    }
    config
        .aliases
        .get(arg)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("unknown target chat alias: {arg}"))
}

/// 构造 `/transfer` 命令。
pub(crate) fn transfer_command(
    source_link: &str,
    target_chat_id: i64,
    style: CommandStyle,
) -> String {
    format!(
        "{} {} {}",
        command_name("transfer", style),
        source_link,
        target_chat_id
    )
}

/// 构造 `/lookup` 命令。
pub(crate) fn lookup_command(
    source_link: &str,
    target_chat_id: i64,
    style: CommandStyle,
) -> String {
    format!(
        "{} {} {}",
        command_name("lookup", style),
        source_link,
        target_chat_id
    )
}

/// 构造 `/downloads` 命令。
pub(crate) fn downloads_command(
    filter: Option<&str>,
    limit: Option<u64>,
    page: Option<u64>,
    style: CommandStyle,
) -> String {
    let mut parts = vec![command_name("downloads", style)];
    if let Some(filter) = filter {
        parts.push(filter.to_owned());
    }
    if let Some(limit) = limit {
        parts.push(limit.to_string());
    }
    if let Some(page) = page {
        parts.push(page.to_string());
    }
    parts.join(" ")
}

/// 构造 `/job ...` 命令。
pub(crate) fn job_command(action: &str, job_id: i64, style: CommandStyle) -> String {
    format!(
        "{} {} {}",
        command_name("job", style),
        normalize_job_action(action),
        job_id
    )
}

/// 构造 `/config show` 命令。
pub(crate) fn config_show_command(style: CommandStyle) -> String {
    format!("{} show", command_name("config", style))
}

/// 构造 `/targets show` 命令。
pub(crate) fn targets_show_command(style: CommandStyle) -> String {
    format!("{} show", command_name("targets", style))
}

/// 构造 `/config set ...` 命令。
pub(crate) fn config_set_command(key: &str, value: impl ToString, style: CommandStyle) -> String {
    format!(
        "{} set {} {}",
        command_name("config", style),
        key,
        value.to_string()
    )
}

/// 构造 `/help <topic>` 命令。
pub(crate) fn help_command(topic: Option<&str>, style: CommandStyle) -> String {
    match topic {
        Some(topic) => format!("{} {topic}", command_name("help", style)),
        None => command_name("help", style),
    }
}

/// 构造 `/health` 命令。
pub(crate) fn health_command(style: CommandStyle) -> String {
    command_name("health", style)
}

/// 构造 `/cache` 命令。
pub(crate) fn cache_command(
    view: Option<&str>,
    limit: Option<u64>,
    page: Option<u64>,
    style: CommandStyle,
) -> String {
    let mut parts = vec![command_name("cache", style)];
    if let Some(view) = view {
        parts.push(view.to_owned());
    }
    if let Some(limit) = limit {
        parts.push(limit.to_string());
    }
    if let Some(page) = page {
        parts.push(page.to_string());
    }
    parts.join(" ")
}

/// 构造 `/menu` 命令。
pub(crate) fn menu_command(style: CommandStyle) -> String {
    command_name("menu", style)
}

/// 以人类可读形式展示字节数。
///
/// `/downloads` 和 `/job status` 都展示 TDLib 实时下载进度，统一放在命令公共层，
/// 避免同一个文件大小被不同命令渲染成不同样式。
pub(crate) fn format_bytes(bytes: i64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes.max(0) as f64;
    let mut unit_idx = 0usize;
    while value >= 1024.0 && unit_idx < units.len() - 1 {
        value /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", value as i64, units[unit_idx])
    } else {
        format!("{:.1} {}", value, units[unit_idx])
    }
}

/// 返回命令根名称。
///
/// 帮助页需要展示 `/config [show|set ...]` 这类不完全等同于具体命令的用法，
/// 所以这里提供一个统一入口，避免各模块重复手写命令根字符串。
pub(crate) fn command_root(kind: &str, style: CommandStyle) -> String {
    command_name(kind, style)
}

/// 构造交互页统一页头。
pub(crate) fn build_ready_page_header(title: &str) -> Vec<String> {
    vec![
        title.to_owned(),
        format!("状态：{}", card::code("ready")),
        card::DIVIDER.to_owned(),
    ]
}

/// 构造运行态管理页统一开头：
/// - 标题
/// - ready 状态
/// - 分割线
/// - 简要说明
pub(crate) fn build_runtime_admin_page_intro(title: &str, detail: &str) -> Vec<String> {
    let mut lines = build_ready_page_header(title);
    lines.push(card::note(detail));
    lines.push(String::new());
    lines
}

/// 运行态管理页统一的分区拼接块。
///
/// 适合“空行 + 分区标题 + 若干正文行”这种最常见的页面结构。
pub(crate) fn build_runtime_admin_section_block(
    section_title: &str,
    body_lines: impl IntoIterator<Item = String>,
) -> Vec<String> {
    let mut lines = vec![String::new(), card::section(section_title)];
    lines.extend(body_lines);
    lines
}

/// 构造交互页统一命令分区标题。
pub(crate) fn build_page_command_section() -> String {
    card::section("命令")
}

/// 运行态管理页帮助用的命令说明 descriptor。
///
/// 当前先只覆盖：
/// - 命令用法说明
/// - 交互说明
/// - 示例命令
///
/// 后续如果继续做“同一份元数据驱动”，可以再把 callback/action 文案补进来。
#[derive(Debug, Clone)]
pub(crate) struct RuntimeAdminHelpDescriptor {
    /// help 详情页里的“用途”概述。
    pub purpose: &'static str,
    /// help 目录页里的“一句话摘要”。
    pub summary: &'static str,
    /// `命令：` 下方的总用法，例如 `/config [show|reset|set <key> <value>]`。
    pub synopsis: String,
    /// 逐条命令说明，例如 `/config show` 对应一句解释。
    pub usage_items: Vec<RuntimeAdminUsageItem>,
    /// `交互：` 分区里的逐条说明。
    pub interaction_items: Vec<String>,
    /// `示例：` 分区里的示例命令。
    pub example_commands: Vec<String>,
}

/// 管理页帮助中的单条命令说明。
#[derive(Debug, Clone)]
pub(crate) struct RuntimeAdminUsageItem {
    pub command: String,
    pub detail: String,
}

/// 渲染管理页帮助中的“命令”分区。
pub(crate) fn build_runtime_admin_usage_block(
    descriptor: &RuntimeAdminHelpDescriptor,
) -> Vec<String> {
    let mut lines = vec![card::section("命令"), descriptor.synopsis.clone()];
    for item in &descriptor.usage_items {
        lines.push(String::new());
        lines.push(item.command.clone());
        lines.push(item.detail.clone());
    }
    lines
}

/// 渲染管理页帮助中的“交互”分区。
pub(crate) fn build_runtime_admin_interaction_block(
    descriptor: &RuntimeAdminHelpDescriptor,
) -> Vec<String> {
    let mut lines = vec![card::section("交互")];
    lines.extend(descriptor.interaction_items.iter().cloned());
    lines.push(format!("取消：{}", card::code("/cancel")));
    lines
}

/// 渲染管理页帮助中的“示例”分区。
pub(crate) fn build_runtime_admin_examples_block(
    descriptor: &RuntimeAdminHelpDescriptor,
) -> Vec<String> {
    let mut lines = vec![String::new(), card::section("示例")];
    lines.extend(descriptor.example_commands.iter().cloned());
    lines
}

/// 构造运行态管理页的统一落地页文本。
///
/// 菜单落地页默认只展示字段和交互说明；完整命令通过页面上的“查看命令”按钮打开。
pub(crate) fn build_runtime_admin_landing_text(
    title: &str,
    intro_lines: impl IntoIterator<Item = String>,
    _descriptor: &RuntimeAdminHelpDescriptor,
) -> String {
    let mut lines = build_ready_page_header(title);
    lines.extend(build_runtime_admin_section_block("说明", intro_lines));
    lines.push("需要命令时点击“查看命令”。".to_owned());
    lines.join("\n")
}

/// 构造运行态管理命令的 help 详情页正文。
///
/// 适合 `/config`、`/targets` 这类：
/// - 有统一“用途 / 说明”
/// - 中间可能穿插额外字段/入口摘要
/// - 后面再接统一“命令 / 交互 / 示例”
pub(crate) fn build_runtime_admin_help_detail_text(
    title: &str,
    detail_lines: impl IntoIterator<Item = String>,
    extra_lines: impl IntoIterator<Item = String>,
    descriptor: &RuntimeAdminHelpDescriptor,
) -> String {
    let mut lines = vec![title.to_owned(), format!("用途：{}", descriptor.purpose)];
    lines.extend(detail_lines.into_iter().map(|line| format!("说明：{line}")));
    lines.push(card::DIVIDER.to_owned());
    lines.extend(extra_lines);
    lines.extend(build_runtime_admin_usage_block(descriptor));
    lines.extend(build_runtime_admin_interaction_block(descriptor));
    lines.extend(build_runtime_admin_examples_block(descriptor));
    lines.join("\n")
}

/// 构造交互页统一空态说明。
pub(crate) fn build_page_empty_note(detail: &str) -> String {
    card::note(detail)
}

/// 交互页常用的“刷新 / 返回 / 菜单”操作行。
pub(crate) fn build_refresh_return_menu_row(
    refresh: tdlib_rs::types::InlineKeyboardButton,
    back: tdlib_rs::types::InlineKeyboardButton,
    menu: tdlib_rs::types::InlineKeyboardButton,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![refresh, back, menu]
}

/// 交互页常用的“返回 / 菜单”导航行。
///
/// 某些页面没有合适的刷新语义，单独提供二按钮导航行，避免为了套统一格式硬塞无意义按钮。
pub(crate) fn build_return_menu_row(
    back: tdlib_rs::types::InlineKeyboardButton,
    menu: tdlib_rs::types::InlineKeyboardButton,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![back, menu]
}

/// 运行态管理页统一“查看命令 / 菜单”导航行，直接按 topic 生成帮助按钮。
///
/// 这类页面的 footer 文案结构固定，直接复用统一 helper 可以减少每个模块重复手写按钮创建。
pub(crate) fn build_runtime_admin_help_menu_row(
    help_topic: &str,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![
        crate::tgbot::send::build_callback_button(
            "查看命令",
            &super::help::build_help_callback_data(Some(help_topic)),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        crate::tgbot::send::build_callback_button(
            "菜单",
            &super::menu::build_menu_home_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]
}

/// 运行态管理页统一“返回 / 菜单”导航行。
///
/// 详情页和子页只需要提供“返回”按钮本身，菜单按钮保持一致即可。
pub(crate) fn build_runtime_admin_back_menu_row(
    back: tdlib_rs::types::InlineKeyboardButton,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![
        back,
        crate::tgbot::send::build_callback_button(
            "菜单",
            &super::menu::build_menu_home_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]
}

/// 运行态管理页统一详情卡片文本模板。
///
/// 适合“标题 + 若干字段 + 说明”这种稳定结构的字段详情页。
pub(crate) fn build_runtime_admin_detail_text(
    title: &str,
    field_lines: impl IntoIterator<Item = String>,
    detail_title: &str,
    detail_lines: impl IntoIterator<Item = String>,
) -> String {
    let mut lines = vec![title.to_owned()];
    lines.extend(field_lines);
    lines.push(String::new());
    lines.push(card::section(detail_title));
    lines.extend(detail_lines);
    lines.join("\n")
}

/// 统一“已更新...”成功标题。
pub(crate) fn updated_action_title(subject: &str) -> String {
    format!("已更新{subject}")
}

/// 统一“已清空...”成功标题。
pub(crate) fn cleared_action_title(subject: &str) -> String {
    format!("已清空{subject}")
}

/// 统一“已删除...”成功标题。
pub(crate) fn deleted_action_title(subject: &str) -> String {
    format!("已删除{subject}")
}

/// 统一“XXX已重置为启动默认值”成功标题。
pub(crate) fn reset_action_title(subject: &str) -> String {
    format!("{subject}已重置为启动默认值")
}

/// 统一运行态管理页 callback 错误卡片标题。
pub(crate) fn runtime_admin_error_title(subject: &str) -> String {
    format!("{subject}操作失败")
}

/// 统一运行态管理页 callback 错误卡片副文案。
pub(crate) fn runtime_admin_error_detail(subject: &str) -> String {
    format!("{subject}未更新，请检查日志或复制错误信息。")
}

/// 统一运行态管理页编辑原消息失败时的标题。
pub(crate) fn runtime_admin_edit_error_title(subject: &str) -> String {
    format!("{subject}刷新失败")
}

/// 统一运行态管理页编辑原消息失败时的副文案。
pub(crate) fn runtime_admin_edit_error_detail() -> String {
    "配置已处理，但原消息编辑失败；请使用错误卡片上的“菜单”按钮重新进入。".to_owned()
}

/// 统一发送运行态管理页的 callback 错误卡片。
pub(crate) async fn send_runtime_admin_callback_error(
    request_chat_id: i64,
    client_id: i32,
    subject: &str,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    crate::tgbot::send::send_interaction_error_card(
        request_chat_id,
        client_id,
        &runtime_admin_error_title(subject),
        &runtime_admin_error_detail(subject),
        err,
    )
    .await
}

/// 统一编辑运行态管理页交互卡片，并在失败时输出一致的提示。
pub(crate) async fn edit_runtime_admin_interaction_card_or_error(
    text: String,
    chat_id: i64,
    message_id: i64,
    keyboard: tdlib_rs::types::ReplyMarkupInlineKeyboard,
    client_id: i32,
    subject: &str,
) -> anyhow::Result<()> {
    crate::tgbot::send::edit_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        &runtime_admin_edit_error_title(subject),
        &runtime_admin_edit_error_detail(),
    )
    .await
}

/// 返回命令名称。
fn command_name(kind: &str, style: CommandStyle) -> String {
    let _ = style;
    match kind {
        "help" => "/help",
        "health" => "/health",
        "transfer" => "/transfer",
        "lookup" => "/lookup",
        "cache" => "/cache",
        "config" => "/config",
        "targets" => "/targets",
        "downloads" => "/downloads",
        "job" => "/job",
        "menu" => "/menu",
        _ => kind,
    }
    .to_owned()
}

/// 统一 `/job` 的公开动作名称。
///
/// 代码里仍有少量旧调用会传入短动作别名；这里统一折叠成长动作，避免这些旧调用继续泄露到用户可见回复里。
fn normalize_job_action(action: &str) -> &str {
    match action {
        "p" => "pause",
        "r" => "resume",
        "s" | "cancel" => "stop",
        "st" => "status",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_context::app_context;
    use std::collections::HashMap;

    fn test_app_context() -> std::sync::Arc<crate::app_context::AppContext> {
        app_context()
    }

    fn install_target_runtime(targets: crate::config::TargetsConfig) {
        let app = test_app_context();
        app.targets_runtime.update_runtime_config(targets);
    }

    // 用户可见输出统一使用完整命令。
    #[test]
    fn test_command_builders_use_long_commands() {
        assert_eq!(
            transfer_command("https://t.me/c/1/2", -100, CommandStyle::Long),
            "/transfer https://t.me/c/1/2 -100"
        );
        assert_eq!(
            lookup_command("https://t.me/c/1/2", -100, CommandStyle::Long),
            "/lookup https://t.me/c/1/2 -100"
        );
        assert_eq!(
            downloads_command(Some("run"), Some(8), Some(2), CommandStyle::Long),
            "/downloads run 8 2"
        );
        assert_eq!(job_command("p", 42, CommandStyle::Long), "/job pause 42");
        assert_eq!(job_command("st", 42, CommandStyle::Long), "/job status 42");
        assert_eq!(
            config_set_command("job_concurrency", 2, CommandStyle::Long),
            "/config set job_concurrency 2"
        );
        assert_eq!(help_command(Some("job"), CommandStyle::Long), "/help job");
        assert_eq!(health_command(CommandStyle::Long), "/health");
        assert_eq!(
            cache_command(Some("page"), Some(10), Some(2), CommandStyle::Long),
            "/cache page 10 2"
        );
        assert_eq!(
            cache_command(None, None, None, CommandStyle::Long),
            "/cache"
        );
        assert_eq!(menu_command(CommandStyle::Long), "/menu");
    }

    // 帮助页拼命令根时未知 kind 不应触发 panic，返回原始名称便于上层给出可读错误。
    #[test]
    fn test_command_root_unknown_kind_is_safe() {
        assert_eq!(command_root("unknown", CommandStyle::Long), "unknown");
    }

    // 文件大小格式要在不同命令之间保持一致，避免排查下载进度时出现两套展示。
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(100), "100 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
    }

    // 目标解析应支持配置别名，避免每次都手动输入长 chat_id。
    #[test]
    fn test_resolve_target_chat_id_supports_alias() {
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: 0,
            aliases: HashMap::from([("archive".to_owned(), -100)]),
        });

        let target = resolve_target_chat_id(&["/t", "https://t.me/c/1/2", "archive"], 1)
            .expect("alias should resolve to target chat");

        assert_eq!(target, -100);
    }

    // 未显式指定目标时使用默认目标。
    #[test]
    fn test_resolve_target_chat_id_uses_default_target() {
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -200,
            aliases: HashMap::new(),
        });

        assert_eq!(
            resolve_target_chat_id(&["/t", "https://t.me/c/1/2"], 1).unwrap(),
            -200
        );
    }

    // 显式数字目标不依赖默认配置。
    #[test]
    fn test_resolve_target_chat_id_accepts_explicit_target() {
        install_target_runtime(crate::config::TargetsConfig::default());
        assert_eq!(
            resolve_target_chat_id(&["/t", "https://t.me/c/1/2", "-200"], 1).unwrap(),
            -200
        );
    }
}
