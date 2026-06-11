// 命令层公共工具：
// - 目标 chat 解析
// - 短命令 / 长命令构造
// - 各命令共用的基础依赖

use crate::config::BotConfig;
use crate::tgbot::transfer::card;

/// 命令输出风格：
/// - Short: 适合按钮复制、快速输入
/// - Long: 适合帮助文档、显式展示
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
pub(crate) fn resolve_target_chat_id(
    text: &[&str],
    config: &BotConfig,
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    let target_chat_id = if text.len() >= 3 {
        parse_target_arg(text[2], config)?
    } else if let Some(chat_id) = config.target_map.get(&request_chat_id) {
        *chat_id
    } else if let Some(chat_id) = config.target_map.get(&0) {
        *chat_id
    } else {
        anyhow::bail!("not found transfer target")
    };

    ensure_target_allowed(target_chat_id, config)?;
    Ok(target_chat_id)
}

/// 解析命令里的目标参数。
///
/// 目标可以是数字 chat_id，也可以是 `targets.aliases` 中配置的短名称。
fn parse_target_arg(arg: &str, config: &BotConfig) -> anyhow::Result<i64> {
    if let Ok(chat_id) = arg.parse::<i64>() {
        return Ok(chat_id);
    }
    config
        .target_aliases
        .get(arg)
        .copied()
        .ok_or_else(|| anyhow::anyhow!("unknown target chat alias: {}", arg))
}

/// 校验目标 chat 是否允许。
///
/// 空白名单表示不限制；一旦配置了白名单，默认目标、别名和显式数字都必须在列表内。
fn ensure_target_allowed(target_chat_id: i64, config: &BotConfig) -> anyhow::Result<()> {
    if config.allowed_target_chat_ids.is_empty()
        || config.allowed_target_chat_ids.contains(&target_chat_id)
    {
        return Ok(());
    }
    anyhow::bail!("target chat is not allowed: {}", target_chat_id)
}

/// 构造 `/transfer` 或 `/t` 命令。
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

/// 构造 `/lookup` 或 `/lk` 命令。
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

/// 构造 `/downloads` 或 `/d` 命令。
pub(crate) fn downloads_command(
    filter: Option<&str>,
    limit: Option<u64>,
    page: Option<u64>,
    style: CommandStyle,
) -> String {
    let mut parts = vec![command_name("downloads", style).to_owned()];
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

/// 构造 `/job ...` 或 `/j ...` 命令。
pub(crate) fn job_command(action: &str, job_id: i64, style: CommandStyle) -> String {
    format!("{} {} {}", command_name("job", style), action, job_id)
}

/// 构造 `/balance` 或 `/bal` 命令。
pub(crate) fn balance_command(style: CommandStyle) -> String {
    command_name("balance", style).to_owned()
}

/// 构造 `/balance history ...` 或 `/bal h ...` 命令。
pub(crate) fn balance_history_command(limit: u64, page: u64, style: CommandStyle) -> String {
    let action = match style {
        CommandStyle::Short => "h",
        CommandStyle::Long => "history",
    };
    format!(
        "{} {} {} {}",
        command_name("balance", style),
        action,
        limit,
        page
    )
}

/// 构造 `/points show ...` 或 `/pts s ...` 命令。
pub(crate) fn points_show_command(user_id: i64, style: CommandStyle) -> String {
    let action = match style {
        CommandStyle::Short => "s",
        CommandStyle::Long => "show",
    };
    format!("{} {} {}", command_name("points", style), action, user_id)
}

/// 构造 `/points history ...` 或 `/pts h ...` 命令。
pub(crate) fn points_history_command(
    user_id: i64,
    limit: u64,
    page: u64,
    style: CommandStyle,
) -> String {
    let action = match style {
        CommandStyle::Short => "h",
        CommandStyle::Long => "history",
    };
    format!(
        "{} {} {} {} {}",
        command_name("points", style),
        action,
        user_id,
        limit,
        page
    )
}

/// 构造 `/points add/sub ...` 或 `/pts a/sub ...` 命令。
///
/// `reason` 会进入积分账本，命令帮助中统一给出可复制模板，避免手动输入时漏掉原因。
pub(crate) fn points_change_command(
    action: &str,
    user_id: i64,
    amount: i64,
    reason: &str,
    style: CommandStyle,
) -> String {
    format!(
        "{} {} {} {} {}",
        command_name("points", style),
        action,
        user_id,
        amount,
        reason
    )
}

/// 构造 `/config show` 或 `/cfg show` 命令。
pub(crate) fn config_show_command(style: CommandStyle) -> String {
    format!("{} show", command_name("config", style))
}

/// 构造 `/config set ...` 或 `/cfg set ...` 命令。
pub(crate) fn config_set_command(key: &str, value: impl ToString, style: CommandStyle) -> String {
    format!(
        "{} set {} {}",
        command_name("config", style),
        key,
        value.to_string()
    )
}

/// 构造 `/help <topic>` 或 `/h <topic>` 命令。
pub(crate) fn help_command(topic: Option<&str>, style: CommandStyle) -> String {
    match topic {
        Some(topic) => format!("{} {}", command_name("help", style), topic),
        None => command_name("help", style).to_owned(),
    }
}

/// 构造 `/health` 或 `/hl` 命令。
pub(crate) fn health_command(style: CommandStyle) -> String {
    command_name("health", style).to_owned()
}

/// 构造 `/cache` 或 `/fc` 命令。
pub(crate) fn cache_command(
    view: Option<&str>,
    limit: Option<u64>,
    page: Option<u64>,
    style: CommandStyle,
) -> String {
    let mut parts = vec![command_name("cache", style).to_owned()];
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
    command_name("menu", style).to_owned()
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
/// 所以这里提供一个统一入口，避免各模块手写 `/cfg`、`/config` 字符串。
pub(crate) fn command_root(kind: &str, style: CommandStyle) -> &'static str {
    command_name(kind, style)
}

/// 同时展示短命令和长命令。
pub(crate) fn short_and_long(short: String, long: String) -> String {
    // 帮助和列表回复统一使用 card 格式；发送层会把 `‹...›` 转成 TDLib code 实体。
    format!("{} | {}", card::code(long), card::code(short))
}

/// 返回命令名称。
fn command_name(kind: &str, style: CommandStyle) -> &'static str {
    match (kind, style) {
        ("help", CommandStyle::Short) => "/h",
        ("help", CommandStyle::Long) => "/help",
        ("health", CommandStyle::Short) => "/hl",
        ("health", CommandStyle::Long) => "/health",
        ("transfer", CommandStyle::Short) => "/t",
        ("transfer", CommandStyle::Long) => "/transfer",
        ("lookup", CommandStyle::Short) => "/lk",
        ("lookup", CommandStyle::Long) => "/lookup",
        ("cache", CommandStyle::Short) => "/fc",
        ("cache", CommandStyle::Long) => "/cache",
        ("config", CommandStyle::Short) => "/cfg",
        ("config", CommandStyle::Long) => "/config",
        ("downloads", CommandStyle::Short) => "/d",
        ("downloads", CommandStyle::Long) => "/downloads",
        ("job", CommandStyle::Short) => "/j",
        ("job", CommandStyle::Long) => "/job",
        ("balance", CommandStyle::Short) => "/bal",
        ("balance", CommandStyle::Long) => "/balance",
        ("points", CommandStyle::Short) => "/pts",
        ("points", CommandStyle::Long) => "/points",
        ("menu", CommandStyle::Short) => "/m",
        ("menu", CommandStyle::Long) => "/menu",
        _ => unreachable!("unknown command kind"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // 短命令用于按钮复制，长命令用于帮助文档；这里固定两套格式避免后续误改。
    #[test]
    fn test_command_builders_keep_short_and_long_forms() {
        assert_eq!(
            transfer_command("https://t.me/c/1/2", -100, CommandStyle::Long),
            "/transfer https://t.me/c/1/2 -100"
        );
        assert_eq!(
            transfer_command("https://t.me/c/1/2", -100, CommandStyle::Short),
            "/t https://t.me/c/1/2 -100"
        );
        assert_eq!(
            lookup_command("https://t.me/c/1/2", -100, CommandStyle::Short),
            "/lk https://t.me/c/1/2 -100"
        );
        assert_eq!(
            downloads_command(Some("run"), Some(8), Some(2), CommandStyle::Short),
            "/d run 8 2"
        );
        assert_eq!(job_command("p", 42, CommandStyle::Short), "/j p 42");
        assert_eq!(job_command("st", 42, CommandStyle::Short), "/j st 42");
        assert_eq!(balance_command(CommandStyle::Short), "/bal");
        assert_eq!(balance_command(CommandStyle::Long), "/balance");
        assert_eq!(
            balance_history_command(10, 2, CommandStyle::Short),
            "/bal h 10 2"
        );
        assert_eq!(points_show_command(7, CommandStyle::Short), "/pts s 7");
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
        assert_eq!(help_command(Some("job"), CommandStyle::Short), "/h job");
        assert_eq!(health_command(CommandStyle::Short), "/hl");
        assert_eq!(health_command(CommandStyle::Long), "/health");
        assert_eq!(
            cache_command(Some("page"), Some(10), Some(2), CommandStyle::Short),
            "/fc page 10 2"
        );
        assert_eq!(
            cache_command(None, None, None, CommandStyle::Long),
            "/cache"
        );
        assert_eq!(menu_command(CommandStyle::Short), "/m");
        assert_eq!(menu_command(CommandStyle::Long), "/menu");
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
        let config = BotConfig {
            target_aliases: HashMap::from([("archive".to_owned(), -100)]),
            allowed_target_chat_ids: vec![-100],
            ..Default::default()
        };

        let target = resolve_target_chat_id(&["/t", "https://t.me/c/1/2", "archive"], &config, 1)
            .expect("alias should resolve to target chat");

        assert_eq!(target, -100);
    }

    // 默认目标同样要受目标白名单保护，避免配置了 allowed_target_chat_ids 但实际未生效。
    #[test]
    fn test_resolve_target_chat_id_rejects_disallowed_default_target() {
        let config = BotConfig {
            target_map: HashMap::from([(0, -200)]),
            allowed_target_chat_ids: vec![-100],
            ..Default::default()
        };

        let err = resolve_target_chat_id(&["/t", "https://t.me/c/1/2"], &config, 1).unwrap_err();

        assert!(err.to_string().contains("target chat is not allowed"));
    }

    // 显式数字目标也必须命中白名单。
    #[test]
    fn test_resolve_target_chat_id_rejects_disallowed_explicit_target() {
        let config = BotConfig {
            allowed_target_chat_ids: vec![-100],
            ..Default::default()
        };

        let err =
            resolve_target_chat_id(&["/t", "https://t.me/c/1/2", "-200"], &config, 1).unwrap_err();

        assert!(err.to_string().contains("target chat is not allowed"));
    }
}
