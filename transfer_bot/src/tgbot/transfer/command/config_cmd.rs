// `/config` 命令实现：
// - 仅开放安全可调的运行参数
// - 修改后同时写回 config.json 与内存运行配置

use super::common::{CommandStyle, config_set_command, config_show_command, short_and_long};
use crate::config;
use crate::tgbot::send;
use crate::tgbot::transfer::card;

/// `/config` callback 前缀。
const CONFIG_CALLBACK_PREFIX: &str = "cfg:";
/// 后台并发允许的最小值。
const JOB_CONCURRENCY_MIN: usize = 1;
/// 后台并发允许的最大值，避免误触把本机和 TDLib 压垮。
const JOB_CONCURRENCY_MAX: usize = 32;
/// 文件引用归零后最小删除延迟分钟数。
const FILE_DELETE_DELAY_MINUTES_MIN: i64 = 0;
/// 文件引用归零后最大删除延迟分钟数。
const FILE_DELETE_DELAY_MINUTES_MAX: i64 = 24 * 60;
/// 文件 GC 最小扫描间隔秒数。
const FILE_GC_INTERVAL_SECONDS_MIN: u64 = 5;
/// 文件 GC 最大扫描间隔秒数。
const FILE_GC_INTERVAL_SECONDS_MAX: u64 = 60 * 60;

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
            format_current_transfer_config_text("当前可调配置"),
            build_config_buttons(),
        ),
        Some("show") => (
            format_current_transfer_config_text("当前可调配置"),
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

    let mut panel = send::ReplyPanel::card(reply);
    for row in rows {
        panel = panel.row(row);
    }
    panel.send(request_chat_id, client_id).await
}

/// 判断 callback payload 是否属于 `/config`。
pub(super) fn is_config_callback_data(data: &str) -> bool {
    data.starts_with(CONFIG_CALLBACK_PREFIX)
}

/// `/config` inline keyboard 回调入口。
///
/// 配置按钮只开放小步增减和刷新，避免把复杂输入塞进 callback。
pub async fn config_callback_query(
    update: tdlib_rs::enums::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    let Some(action) = parse_config_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("配置按钮参数无效"), client_id).await?;
        return Ok(());
    };

    let tip = match action {
        ConfigCallbackAction::Refresh => "已刷新".to_owned(),
        ConfigCallbackAction::Adjust { field, delta } => adjust_transfer_config(field, delta)
            .await
            .map(|_| "配置已更新".to_owned())?,
    };
    let (text, keyboard) =
        send::ReplyPanel::card(format_current_transfer_config_text("当前可调配置"))
            .rows(build_config_buttons())
            .into_card_parts()?;
    send::edit_card_message_with_inline_keyboard(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
    )
    .await?;
    send::answer_callback_query(update.id, Some(&tip), client_id).await?;
    Ok(())
}

/// 更新 `transfer_config` 中允许动态调整的字段。
async fn update_transfer_config(key: &str, value: &str) -> anyhow::Result<String> {
    let mut bot_config = config::load_runtime_bot_config().await?;
    match key {
        "job_concurrency" => {
            let parsed = value.parse::<usize>()?;
            if !(JOB_CONCURRENCY_MIN..=JOB_CONCURRENCY_MAX).contains(&parsed) {
                anyhow::bail!(
                    "job_concurrency must be between {} and {}",
                    JOB_CONCURRENCY_MIN,
                    JOB_CONCURRENCY_MAX
                );
            }
            bot_config.transfer_config.job_concurrency = parsed;
        }
        "file_delete_delay_minutes" | "file_delete_delay_hours" => {
            let parsed = value.parse::<i64>()?;
            if !(FILE_DELETE_DELAY_MINUTES_MIN..=FILE_DELETE_DELAY_MINUTES_MAX).contains(&parsed) {
                anyhow::bail!(
                    "file_delete_delay_minutes must be between {} and {}",
                    FILE_DELETE_DELAY_MINUTES_MIN,
                    FILE_DELETE_DELAY_MINUTES_MAX
                );
            }
            bot_config.transfer_config.file_delete_delay_minutes = parsed;
        }
        "file_gc_interval_seconds" => {
            let parsed = value.parse::<u64>()?;
            if !(FILE_GC_INTERVAL_SECONDS_MIN..=FILE_GC_INTERVAL_SECONDS_MAX).contains(&parsed) {
                anyhow::bail!(
                    "file_gc_interval_seconds must be between {} and {}",
                    FILE_GC_INTERVAL_SECONDS_MIN,
                    FILE_GC_INTERVAL_SECONDS_MAX
                );
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

/// 按按钮小步调整运行配置。
async fn adjust_transfer_config(field: ConfigField, delta: i64) -> anyhow::Result<()> {
    let mut bot_config = config::load_runtime_bot_config().await?;
    match field {
        ConfigField::JobConcurrency => {
            let current = i64::try_from(bot_config.transfer_config.job_concurrency)?;
            bot_config.transfer_config.job_concurrency = clamp_i64(
                current + delta,
                JOB_CONCURRENCY_MIN as i64,
                JOB_CONCURRENCY_MAX as i64,
            ) as usize;
        }
        ConfigField::FileDeleteDelayMinutes => {
            let current = bot_config.transfer_config.file_delete_delay_minutes;
            bot_config.transfer_config.file_delete_delay_minutes = clamp_i64(
                current + delta,
                FILE_DELETE_DELAY_MINUTES_MIN,
                FILE_DELETE_DELAY_MINUTES_MAX,
            );
        }
        ConfigField::FileGcIntervalSeconds => {
            let current = i64::try_from(bot_config.transfer_config.file_gc_interval_seconds)?;
            bot_config.transfer_config.file_gc_interval_seconds = clamp_i64(
                current + delta,
                FILE_GC_INTERVAL_SECONDS_MIN as i64,
                FILE_GC_INTERVAL_SECONDS_MAX as i64,
            ) as u64;
        }
    }

    config::save_runtime_bot_config(&bot_config).await?;
    crate::tgbot::transfer::update_runtime_config(bot_config.transfer_config.clone());
    tracing::info!(
        field = field.key(),
        delta,
        "transfer runtime config adjusted by callback"
    );
    Ok(())
}

/// 把运行时配置格式化成当前卡片文本。
pub(super) fn format_current_transfer_config_text(title: &str) -> String {
    format_transfer_config_text(title, &crate::tgbot::transfer::runtime_config())
}

/// 格式化当前可调配置。
fn format_transfer_config_text(title: &str, config: &config::TransferConfig) -> String {
    [
        title.to_owned(),
        format!("状态：{}", card::code("ready")),
        card::DIVIDER.to_owned(),
        card::section("运行参数"),
        card::field("job_concurrency", config.job_concurrency),
        card::field(
            "file_delete_delay_minutes",
            config.file_delete_delay_minutes,
        ),
        card::field("file_gc_interval_seconds", config.file_gc_interval_seconds),
        "".to_owned(),
        card::section("命令"),
        short_and_long(
            config_show_command(CommandStyle::Short),
            config_show_command(CommandStyle::Long),
        ),
        short_and_long(
            config_set_command("job_concurrency", 4, CommandStyle::Short),
            config_set_command("job_concurrency", 4, CommandStyle::Long),
        ),
        short_and_long(
            config_set_command("file_delete_delay_minutes", 3, CommandStyle::Short),
            config_set_command("file_delete_delay_minutes", 3, CommandStyle::Long),
        ),
        short_and_long(
            config_set_command("file_gc_interval_seconds", 30, CommandStyle::Short),
            config_set_command("file_gc_interval_seconds", 30, CommandStyle::Long),
        ),
    ]
    .join("\n")
}

/// config 页面快捷按钮。
pub(super) fn build_config_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "刷新",
                &build_config_callback_data(ConfigCallbackAction::Refresh),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_copy_button(
                "复制 /cfg show",
                &config_show_command(CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_callback_button(
                "并发 -1",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::JobConcurrency,
                    delta: -1,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "并发 +1",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::JobConcurrency,
                    delta: 1,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_callback_button(
                "删除 -1m",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::FileDeleteDelayMinutes,
                    delta: -1,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "删除 +1m",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::FileDeleteDelayMinutes,
                    delta: 1,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_callback_button(
                "GC -10s",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::FileGcIntervalSeconds,
                    delta: -10,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "GC +10s",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::FileGcIntervalSeconds,
                    delta: 10,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button(
                "复制并发=4",
                &config_set_command("job_concurrency", 4, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制删除=3m",
                &config_set_command("file_delete_delay_minutes", 3, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制GC=30s",
                &config_set_command("file_gc_interval_seconds", 30, CommandStyle::Short),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ]
}

/// 配置 callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConfigCallbackAction {
    Refresh,
    Adjust { field: ConfigField, delta: i64 },
}

/// 允许按钮调整的配置字段。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConfigField {
    JobConcurrency,
    FileDeleteDelayMinutes,
    FileGcIntervalSeconds,
}

impl ConfigField {
    /// 字段短编码，写入 callback payload。
    fn code(self) -> &'static str {
        match self {
            Self::JobConcurrency => "jc",
            Self::FileDeleteDelayMinutes => "dd",
            Self::FileGcIntervalSeconds => "gc",
        }
    }

    /// 字段配置键，写入日志。
    fn key(self) -> &'static str {
        match self {
            Self::JobConcurrency => "job_concurrency",
            Self::FileDeleteDelayMinutes => "file_delete_delay_minutes",
            Self::FileGcIntervalSeconds => "file_gc_interval_seconds",
        }
    }

    /// 从 callback 短编码解析字段。
    fn parse(code: &str) -> Option<Self> {
        match code {
            "jc" => Some(Self::JobConcurrency),
            "dd" => Some(Self::FileDeleteDelayMinutes),
            "gc" => Some(Self::FileGcIntervalSeconds),
            _ => None,
        }
    }
}

/// 构造配置 callback payload。
fn build_config_callback_data(action: ConfigCallbackAction) -> String {
    match action {
        ConfigCallbackAction::Refresh => format!("{}r", CONFIG_CALLBACK_PREFIX),
        ConfigCallbackAction::Adjust { field, delta } => {
            format!("{}a:{}:{}", CONFIG_CALLBACK_PREFIX, field.code(), delta)
        }
    }
}

/// 解析配置 callback payload。
fn parse_config_callback_data(data: &str) -> Option<ConfigCallbackAction> {
    let payload = data.strip_prefix(CONFIG_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    match parts.next()? {
        "r" => {
            if parts.next().is_none() {
                Some(ConfigCallbackAction::Refresh)
            } else {
                None
            }
        }
        "a" => {
            let field = ConfigField::parse(parts.next()?)?;
            let delta = parts.next()?.parse::<i64>().ok()?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::Adjust { field, delta })
        }
        _ => None,
    }
}

/// 把整数限制在安全区间内。
fn clamp_i64(value: i64, min: i64, max: i64) -> i64 {
    value.clamp(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 文本输出应包含三个可调字段。
    #[test]
    fn test_format_transfer_config_text() {
        let cfg = config::TransferConfig {
            job_concurrency: 2,
            file_delete_delay_minutes: 2,
            file_gc_interval_seconds: 60,
        };
        let text = format_transfer_config_text("当前可调配置", &cfg);
        assert!(text.contains("job_concurrency：‹2›"));
        assert!(text.contains("file_delete_delay_minutes：‹2›"));
        assert!(text.contains("file_gc_interval_seconds：‹60›"));
        assert!(text.contains("‹/cfg show›"));
    }

    // 配置 callback 使用短 payload，避免 Telegram callback data 过长。
    #[test]
    fn test_config_callback_data_roundtrip() {
        let refresh = build_config_callback_data(ConfigCallbackAction::Refresh);
        assert_eq!(refresh, "cfg:r");
        assert!(is_config_callback_data(&refresh));
        assert_eq!(
            parse_config_callback_data(&refresh),
            Some(ConfigCallbackAction::Refresh)
        );

        let adjust = build_config_callback_data(ConfigCallbackAction::Adjust {
            field: ConfigField::FileGcIntervalSeconds,
            delta: 10,
        });
        assert_eq!(adjust, "cfg:a:gc:10");
        assert_eq!(
            parse_config_callback_data(&adjust),
            Some(ConfigCallbackAction::Adjust {
                field: ConfigField::FileGcIntervalSeconds,
                delta: 10,
            })
        );
        assert_eq!(parse_config_callback_data("cfg:a:bad:1"), None);
        assert_eq!(parse_config_callback_data("cfg:a:gc:x"), None);
    }

    // 按钮调整必须做边界限制，避免误触后出现 0 并发或过短 GC。
    #[test]
    fn test_clamp_i64() {
        assert_eq!(clamp_i64(0, 1, 32), 1);
        assert_eq!(clamp_i64(33, 1, 32), 32);
        assert_eq!(clamp_i64(10, 1, 32), 10);
    }
}
