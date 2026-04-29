// `/help` 的 topic 归一化逻辑。
// 长命令、短命令、带斜杠写法最终都映射到同一组内部命令名。

/// 将帮助 topic 归一化为内部命令名。
///
/// 支持长命令、短命令、以及用户直接把 `/transfer` 这种带斜杠命令传进来。
pub(super) fn normalize_help_topic(command_name: &str) -> anyhow::Result<&'static str> {
    match command_name.trim_start_matches('/') {
        "help" | "h" => Ok("help"),
        "transfer" | "t" => Ok("transfer"),
        "lookup" | "lk" => Ok("lookup"),
        "config" | "cfg" => Ok("config"),
        "downloads" | "download" | "d" => Ok("downloads"),
        "job" | "j" => Ok("job"),
        other => anyhow::bail!("unknown help topic: {}", other),
    }
}
