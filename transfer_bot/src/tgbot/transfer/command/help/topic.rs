// `/help` 的 topic 归一化逻辑。
// 当前仅支持长命令和带斜杠写法，不再公开短别名。

/// 将帮助 topic 归一化为内部命令名。
///
/// 支持长命令，以及用户直接把 `/transfer` 这种带斜杠命令传进来。
pub(super) fn normalize_help_topic(command_name: &str) -> anyhow::Result<&'static str> {
    match command_name.trim_start_matches('/') {
        "help" => Ok("help"),
        "health" => Ok("health"),
        "transfer" => Ok("transfer"),
        "lookup" => Ok("lookup"),
        "cache" | "file" | "files" => Ok("cache"),
        "balance" | "points" => Ok("points"),
        "config" => Ok("config"),
        "targets" => Ok("targets"),
        "acl" => Ok("acl"),
        "billing" => Ok("billing"),
        "downloads" | "download" => Ok("downloads"),
        "job" => Ok("job"),
        "menu" => Ok("menu"),
        other => anyhow::bail!("unknown help topic: {}", other),
    }
}
