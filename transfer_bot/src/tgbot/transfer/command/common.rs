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
/// 1. 命令参数显式指定
/// 2. 否则从 `target_map[request_chat_id]` 获取
/// 3. 再否则尝试 `target_map[0]` 兜底
pub(crate) fn resolve_target_chat_id(
    text: &[&str],
    config: &BotConfig,
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    if text.len() >= 3 {
        return Ok(text[2].parse::<i64>()?);
    }

    if let Some(chat_id) = config.target_map.get(&request_chat_id) {
        return Ok(*chat_id);
    }

    if let Some(chat_id) = config.target_map.get(&0) {
        return Ok(*chat_id);
    }

    anyhow::bail!("not found transfer target")
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
        ("transfer", CommandStyle::Short) => "/t",
        ("transfer", CommandStyle::Long) => "/transfer",
        ("lookup", CommandStyle::Short) => "/lk",
        ("lookup", CommandStyle::Long) => "/lookup",
        ("config", CommandStyle::Short) => "/cfg",
        ("config", CommandStyle::Long) => "/config",
        ("downloads", CommandStyle::Short) => "/d",
        ("downloads", CommandStyle::Long) => "/downloads",
        ("job", CommandStyle::Short) => "/j",
        ("job", CommandStyle::Long) => "/job",
        _ => unreachable!("unknown command kind"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(
            config_set_command("job_concurrency", 2, CommandStyle::Long),
            "/config set job_concurrency 2"
        );
        assert_eq!(help_command(Some("job"), CommandStyle::Short), "/h job");
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
}
