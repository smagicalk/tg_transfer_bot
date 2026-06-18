// 命令层公共工具：
// - 目标 chat 解析
// - 统一构造命令字符串
// - 各命令共用的基础依赖

use crate::tgbot::transfer::card;

/// 命令输出风格：
/// - Short: 旧版兼容枚举，当前用户可见输出已统一收口到长命令
/// - Long: 帮助文档、按钮复制和显式展示使用的标准形式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommandStyle {
    Short,
    Long,
}

/// 解析目标 chat_id：
/// 1. 命令参数显式指定数字 chat_id 或 `targets.aliases` 别名
/// 2. 否则从 `targets.by_request_chat_id[request_chat_id]` 获取
/// 3. 再否则尝试 `targets.default_chat_id` 兜底
/// 4. 如果配置了 `allowed_target_chat_ids`，最终目标必须命中白名单
pub(crate) fn resolve_target_chat_id(text: &[&str], request_chat_id: i64) -> anyhow::Result<i64> {
    let targets_config = crate::tgbot::transfer::targets_runtime_config();
    let access_control = crate::tgbot::transfer::access_control_runtime_config();
    let target_chat_id = if text.len() >= 3 {
        parse_target_arg(text[2], &targets_config)?
    } else if let Some(chat_id) = targets_config.by_request_chat_id.get(&request_chat_id) {
        *chat_id
    } else if targets_config.default_chat_id != 0 {
        targets_config.default_chat_id
    } else {
        anyhow::bail!("not found transfer target")
    };

    ensure_target_allowed(target_chat_id, &access_control)?;
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
        .ok_or_else(|| anyhow::anyhow!("unknown target chat alias: {}", arg))
}

/// 校验目标 chat 是否允许。
///
/// 空白名单表示不限制；一旦配置了白名单，默认目标、别名和显式数字都必须在列表内。
fn ensure_target_allowed(
    target_chat_id: i64,
    access_control: &crate::config::AccessControlConfig,
) -> anyhow::Result<()> {
    if access_control.allowed_target_chat_ids.is_empty()
        || access_control
            .allowed_target_chat_ids
            .contains(&target_chat_id)
    {
        return Ok(());
    }
    anyhow::bail!("target chat is not allowed: {}", target_chat_id)
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

/// 构造 `/balance` 命令。
pub(crate) fn balance_command(style: CommandStyle) -> String {
    command_name("balance", style)
}

/// 构造 `/balance history ...` 命令。
pub(crate) fn balance_history_command(limit: u64, page: u64, style: CommandStyle) -> String {
    let _ = style;
    let action = "history";
    format!(
        "{} {} {} {}",
        command_name("balance", style),
        action,
        limit,
        page
    )
}

/// 构造 `/points show ...` 命令。
pub(crate) fn points_show_command(user_id: i64, style: CommandStyle) -> String {
    let _ = style;
    let action = "show";
    format!("{} {} {}", command_name("points", style), action, user_id)
}

/// 构造 `/points history ...` 命令。
pub(crate) fn points_history_command(
    user_id: i64,
    limit: u64,
    page: u64,
    style: CommandStyle,
) -> String {
    let _ = style;
    let action = "history";
    format!(
        "{} {} {} {} {}",
        command_name("points", style),
        action,
        user_id,
        limit,
        page
    )
}

/// 构造 `/points add/sub ...` 命令。
///
/// `reason` 会进入积分账本，命令帮助中统一给出可复制模板，避免手动输入时漏掉原因。
pub(crate) fn points_change_command(
    action: &str,
    user_id: i64,
    amount: i64,
    reason: &str,
    style: CommandStyle,
) -> String {
    let action = match action {
        "a" => "add",
        "s" | "sub" => "sub",
        "show" => "show",
        "history" | "h" => "history",
        other => other,
    };
    format!(
        "{} {} {} {} {}",
        command_name("points", style),
        action,
        user_id,
        amount,
        reason
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

/// 构造 `/acl show` 命令。
pub(crate) fn acl_show_command(style: CommandStyle) -> String {
    format!("{} show", command_name("acl", style))
}

/// 构造 `/billing show` 命令。
pub(crate) fn billing_show_command(style: CommandStyle) -> String {
    format!("{} show", command_name("billing", style))
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
        Some(topic) => format!("{} {}", command_name("help", style), topic),
        None => command_name("help", style),
    }
}

/// 构造 `/health` 或 `/hl` 命令。
pub(crate) fn health_command(style: CommandStyle) -> String {
    command_name("health", style)
}

/// 构造 `/cache` 或 `/fc` 命令。
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

/// 构造 `/menu` 或 `/m` 命令。
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

/// 同时展示兼容命令和标准命令。
pub(crate) fn short_and_long(short: String, long: String) -> String {
    // 帮助和列表回复统一使用 card 格式；发送层会把 `‹...›` 转成 TDLib code 实体。
    if short == long {
        card::code(long)
    } else {
        format!("{} | {}", card::code(long), card::code(short))
    }
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

/// 构造交互页统一命令分区标题。
pub(crate) fn build_page_command_section() -> String {
    card::section("命令")
}

/// 运行态管理页统一命令示例区。
///
/// 只负责拼接：
/// - 空行
/// - `命令` 标题
/// - 若干示例命令
///
/// 每页仍自己决定展示哪些示例，不把业务字段硬抽成一套。
pub(crate) fn build_command_examples<I, S>(examples: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut lines = vec![String::new(), build_page_command_section()];
    lines.extend(examples.into_iter().map(Into::into));
    lines
}

/// 运行态管理页帮助用的命令说明 descriptor。
///
/// 当前先只覆盖：
/// - 命令用法说明
/// - 交互说明
/// - 示例命令
/// - help 页复制按钮
///
/// 后续如果继续做“同一份元数据驱动”，可以再把 callback/action 文案补进来。
#[derive(Debug, Clone)]
pub(crate) struct RuntimeAdminHelpDescriptor {
    /// `命令：` 下方的总用法，例如 `/config [show|reset|set <key> <value>]`。
    pub synopsis: String,
    /// 逐条命令说明，例如 `/config show` 对应一句解释。
    pub usage_items: Vec<RuntimeAdminUsageItem>,
    /// `交互：` 分区里的逐条说明。
    pub interaction_items: Vec<String>,
    /// `示例：` 分区里的示例命令。
    pub example_commands: Vec<String>,
    /// help 详情页里的复制按钮定义。
    pub help_copy_buttons: Vec<RuntimeAdminHelpCopyButton>,
}

/// 管理页帮助中的单条命令说明。
#[derive(Debug, Clone)]
pub(crate) struct RuntimeAdminUsageItem {
    pub command: String,
    pub detail: String,
}

/// 管理页帮助中的复制按钮定义。
#[derive(Debug, Clone)]
pub(crate) struct RuntimeAdminHelpCopyButton {
    pub label: String,
    pub command: String,
    pub style: tdlib_rs::enums::ButtonStyle,
}

impl RuntimeAdminHelpCopyButton {
    /// 构造一条复制按钮定义。
    pub(crate) fn new(
        label: impl Into<String>,
        command: impl Into<String>,
        style: tdlib_rs::enums::ButtonStyle,
    ) -> Self {
        Self {
            label: label.into(),
            command: command.into(),
            style,
        }
    }
}

/// 渲染管理页帮助中的“命令：”分区。
pub(crate) fn build_runtime_admin_usage_block(
    descriptor: &RuntimeAdminHelpDescriptor,
) -> Vec<String> {
    let mut lines = vec!["命令：".to_owned(), descriptor.synopsis.clone()];
    for item in &descriptor.usage_items {
        lines.push(String::new());
        lines.push(item.command.clone());
        lines.push(item.detail.clone());
    }
    lines
}

/// 渲染管理页帮助中的“交互：”分区。
pub(crate) fn build_runtime_admin_interaction_block(
    descriptor: &RuntimeAdminHelpDescriptor,
) -> Vec<String> {
    let mut lines = vec!["交互：".to_owned()];
    lines.extend(descriptor.interaction_items.iter().cloned());
    lines.push(format!("取消：{}", card::code("/cancel")));
    lines
}

/// 渲染管理页帮助中的“示例：”分区。
pub(crate) fn build_runtime_admin_examples_block(
    descriptor: &RuntimeAdminHelpDescriptor,
) -> Vec<String> {
    let mut lines = vec![String::new(), "示例：".to_owned()];
    lines.extend(descriptor.example_commands.iter().cloned());
    lines
}

/// 把 descriptor 中的复制按钮定义渲染成 help 详情页按钮行。
pub(crate) fn build_runtime_admin_help_copy_rows(
    descriptor: &RuntimeAdminHelpDescriptor,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    descriptor
        .help_copy_buttons
        .iter()
        .map(|button| {
            build_copy_only_row(crate::tgbot::send::build_copy_button(
                &button.label,
                &button.command,
                button.style.clone(),
            ))
        })
        .collect()
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

/// 交互页常用的单独复制按钮行。
pub(crate) fn build_copy_only_row(
    button: tdlib_rs::types::InlineKeyboardButton,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![button]
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

/// 运行态管理页统一“帮助 / 菜单”导航行。
pub(crate) fn build_help_menu_row(
    help: tdlib_rs::types::InlineKeyboardButton,
    menu: tdlib_rs::types::InlineKeyboardButton,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    vec![help, menu]
}

/// 统一“已更新...”成功标题。
pub(crate) fn updated_action_title(subject: &str) -> String {
    format!("已更新{}", subject)
}

/// 统一“已清空...”成功标题。
pub(crate) fn cleared_action_title(subject: &str) -> String {
    format!("已清空{}", subject)
}

/// 统一“已删除...”成功标题。
pub(crate) fn deleted_action_title(subject: &str) -> String {
    format!("已删除{}", subject)
}

/// 统一“已添加...”成功标题。
pub(crate) fn added_action_title(subject: &str) -> String {
    format!("已添加{}", subject)
}

/// 统一“已解除...”成功标题。
pub(crate) fn released_action_title(subject: &str) -> String {
    format!("已解除{}", subject)
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
pub(crate) fn runtime_admin_edit_error_detail(command: &str) -> String {
    format!("配置已处理，但原消息编辑失败；请复制错误或重新发送 {command}。")
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
    retry_command: &str,
) -> anyhow::Result<()> {
    crate::tgbot::send::edit_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        &runtime_admin_edit_error_title(subject),
        &runtime_admin_edit_error_detail(retry_command),
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
        "acl" => "/acl",
        "billing" => "/billing",
        "downloads" => "/downloads",
        "job" => "/job",
        "balance" => "/balance",
        "points" => "/points",
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

    fn install_target_runtime(
        targets: crate::config::TargetsConfig,
        access_control: crate::config::AccessControlConfig,
    ) {
        let app = app_context();
        app.targets_runtime.update_runtime_config(targets);
        app.access_control_runtime
            .update_runtime_config(access_control);
    }

    // 用户可见输出统一使用长命令；Short 枚举目前只保留兼容调用形状。
    #[test]
    fn test_command_builders_use_long_commands() {
        assert_eq!(
            transfer_command("https://t.me/c/1/2", -100, CommandStyle::Long),
            "/transfer https://t.me/c/1/2 -100"
        );
        assert_eq!(
            transfer_command("https://t.me/c/1/2", -100, CommandStyle::Short),
            "/transfer https://t.me/c/1/2 -100"
        );
        assert_eq!(
            lookup_command("https://t.me/c/1/2", -100, CommandStyle::Short),
            "/lookup https://t.me/c/1/2 -100"
        );
        assert_eq!(
            downloads_command(Some("run"), Some(8), Some(2), CommandStyle::Short),
            "/downloads run 8 2"
        );
        assert_eq!(job_command("p", 42, CommandStyle::Short), "/job pause 42");
        assert_eq!(job_command("st", 42, CommandStyle::Short), "/job status 42");
        assert_eq!(balance_command(CommandStyle::Short), "/balance");
        assert_eq!(balance_command(CommandStyle::Long), "/balance");
        assert_eq!(
            balance_history_command(10, 2, CommandStyle::Short),
            "/balance history 10 2"
        );
        assert_eq!(
            points_show_command(7, CommandStyle::Short),
            "/points show 7"
        );
        assert_eq!(
            points_history_command(7, 10, 2, CommandStyle::Long),
            "/points history 7 10 2"
        );
        assert_eq!(
            points_change_command("add", 7, 10, "admin_adjust", CommandStyle::Long),
            "/points add 7 10 admin_adjust"
        );
        assert_eq!(
            config_set_command("job_concurrency", 2, CommandStyle::Long),
            "/config set job_concurrency 2"
        );
        assert_eq!(help_command(Some("job"), CommandStyle::Short), "/help job");
        assert_eq!(health_command(CommandStyle::Short), "/health");
        assert_eq!(health_command(CommandStyle::Long), "/health");
        assert_eq!(
            cache_command(Some("page"), Some(10), Some(2), CommandStyle::Short),
            "/cache page 10 2"
        );
        assert_eq!(
            cache_command(None, None, None, CommandStyle::Long),
            "/cache"
        );
        assert_eq!(menu_command(CommandStyle::Short), "/menu");
        assert_eq!(menu_command(CommandStyle::Long), "/menu");
    }

    // 帮助页拼命令根时未知 kind 不应触发 panic，返回原始名称便于上层给出可读错误。
    #[test]
    fn test_command_root_unknown_kind_is_safe() {
        assert_eq!(command_root("unknown", CommandStyle::Short), "unknown");
        assert_eq!(command_root("unknown", CommandStyle::Long), "unknown");
    }

    // 帮助页同时展示长命令和短命令，保持这个格式方便用户直接复制。
    #[test]
    fn test_short_and_long_formats_copyable_pair() {
        assert_eq!(
            short_and_long("/d run".to_owned(), "/downloads run".to_owned()),
            "‹/downloads run› | ‹/d run›"
        );
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
        install_target_runtime(
            crate::config::TargetsConfig {
                default_chat_id: 0,
                by_request_chat_id: HashMap::new(),
                aliases: HashMap::from([("archive".to_owned(), -100)]),
            },
            crate::config::AccessControlConfig {
                allowed_target_chat_ids: vec![-100],
                ..Default::default()
            },
        );

        let target = resolve_target_chat_id(&["/t", "https://t.me/c/1/2", "archive"], 1)
            .expect("alias should resolve to target chat");

        assert_eq!(target, -100);
    }

    // 默认目标同样要受目标白名单保护，避免配置了 allowed_target_chat_ids 但实际未生效。
    #[test]
    fn test_resolve_target_chat_id_rejects_disallowed_default_target() {
        install_target_runtime(
            crate::config::TargetsConfig {
                default_chat_id: -200,
                by_request_chat_id: HashMap::new(),
                aliases: HashMap::new(),
            },
            crate::config::AccessControlConfig {
                allowed_target_chat_ids: vec![-100],
                ..Default::default()
            },
        );

        let err = resolve_target_chat_id(&["/t", "https://t.me/c/1/2"], 1).unwrap_err();

        assert!(err.to_string().contains("target chat is not allowed"));
    }

    // 显式数字目标也必须命中白名单。
    #[test]
    fn test_resolve_target_chat_id_rejects_disallowed_explicit_target() {
        install_target_runtime(
            crate::config::TargetsConfig::default(),
            crate::config::AccessControlConfig {
                allowed_target_chat_ids: vec![-100],
                ..Default::default()
            },
        );

        let err = resolve_target_chat_id(&["/t", "https://t.me/c/1/2", "-200"], 1).unwrap_err();

        assert!(err.to_string().contains("target chat is not allowed"));
    }
}
