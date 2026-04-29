// `/config` 命令实现：
// - 仅开放安全可调的运行参数
// - 修改后同时写回 config.json 与内存运行配置

use super::common::{CommandStyle, config_set_command, config_show_command, short_and_long};
use crate::config;
use crate::tgbot::send;

/// `/config` 命令入口。
/// 支持：
/// - `/config`
/// - `/config show`
/// - `/config set <key> <value>`
pub async fn config_command(
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (reply, rows) = match text.get(1).copied() {
        None => (
            format_transfer_config_text("当前可调配置", &crate::tgbot::transfer::runtime_config()),
            build_config_buttons(),
        ),
        Some("show") => (
            format_transfer_config_text("当前可调配置", &crate::tgbot::transfer::runtime_config()),
            build_config_buttons(),
        ),
        Some("set") => {
            let key = text
                .get(2)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("usage: /config set <key> <value>"))?;
            let value = text
                .get(3)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("usage: /config set <key> <value>"))?;
            (
                update_transfer_config(key, value).await?,
                build_config_buttons(),
            )
        }
        Some(other) => anyhow::bail!("unknown config subcommand: {}", other),
    };

    let mut panel = send::ReplyPanel::markdown(reply);
    for row in rows {
        panel = panel.row(row);
    }
    panel.send(request_chat_id, client_id).await
}

/// 更新 `transfer_config` 中允许动态调整的字段。
async fn update_transfer_config(key: &str, value: &str) -> anyhow::Result<String> {
    let mut bot_config = config::load_runtime_bot_config().await?;
    match key {
        "job_concurrency" => {
            let parsed = value.parse::<usize>()?;
            if parsed == 0 {
                anyhow::bail!("job_concurrency must be >= 1");
            }
            bot_config.transfer_config.job_concurrency = parsed;
        }
        "file_delete_delay_hours" => {
            let parsed = value.parse::<i64>()?;
            if parsed < 0 {
                anyhow::bail!("file_delete_delay_hours must be >= 0");
            }
            bot_config.transfer_config.file_delete_delay_hours = parsed;
        }
        "file_gc_interval_seconds" => {
            let parsed = value.parse::<u64>()?;
            if parsed == 0 {
                anyhow::bail!("file_gc_interval_seconds must be >= 1");
            }
            bot_config.transfer_config.file_gc_interval_seconds = parsed;
        }
        _ => anyhow::bail!("unsupported config key: {}", key),
    }

    config::save_runtime_bot_config(&bot_config).await?;
    crate::tgbot::transfer::update_runtime_config(bot_config.transfer_config.clone());
    // 这里只允许修改非敏感运行参数，因此 key/value 可以安全记录，便于追踪运行时变更。
    tracing::info!(key, value, "transfer runtime config updated");

    Ok(format_transfer_config_text(
        &format!("配置已更新：{} = {}", key, value),
        &bot_config.transfer_config,
    ))
}

/// 格式化当前可调配置。
fn format_transfer_config_text(title: &str, config: &config::TransferConfig) -> String {
    [
        format!("*{}*", title),
        format!("`job_concurrency = {}`", config.job_concurrency),
        format!(
            "`file_delete_delay_hours = {}`",
            config.file_delete_delay_hours
        ),
        format!(
            "`file_gc_interval_seconds = {}`",
            config.file_gc_interval_seconds
        ),
        "".to_owned(),
        "示例：".to_owned(),
        short_and_long(
            config_show_command(CommandStyle::Short),
            config_show_command(CommandStyle::Long),
        ),
        short_and_long(
            config_set_command("job_concurrency", 4, CommandStyle::Short),
            config_set_command("job_concurrency", 4, CommandStyle::Long),
        ),
        short_and_long(
            config_set_command("file_delete_delay_hours", 3, CommandStyle::Short),
            config_set_command("file_delete_delay_hours", 3, CommandStyle::Long),
        ),
        short_and_long(
            config_set_command("file_gc_interval_seconds", 30, CommandStyle::Short),
            config_set_command("file_gc_interval_seconds", 30, CommandStyle::Long),
        ),
    ]
    .join("\n")
}

/// config 页面快捷按钮。
fn build_config_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_copy_button(
                "复制 /cfg show",
                &config_show_command(CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "并发=4",
                &config_set_command("job_concurrency", 4, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button(
                "删除延迟=3h",
                &config_set_command("file_delete_delay_hours", 3, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "GC=30s",
                &config_set_command("file_gc_interval_seconds", 30, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // 文本输出应包含三个可调字段。
    #[test]
    fn test_format_transfer_config_text() {
        let cfg = config::TransferConfig {
            job_concurrency: 2,
            file_delete_delay_hours: 2,
            file_gc_interval_seconds: 60,
        };
        let text = format_transfer_config_text("当前可调配置", &cfg);
        assert!(text.contains("job_concurrency = 2"));
        assert!(text.contains("file_delete_delay_hours = 2"));
        assert!(text.contains("file_gc_interval_seconds = 60"));
    }
}
