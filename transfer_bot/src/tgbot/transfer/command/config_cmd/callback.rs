// `/config` callback payload 与按钮布局。
// 这里只承载按钮协议和按钮生成，配置读写仍留在上层命令实现里。

use super::super::common::{
    CommandStyle, build_copy_only_row, build_refresh_return_menu_row, config_set_command,
    config_show_command,
};
use super::super::help::build_help_callback_data;
use super::super::menu::build_menu_home_callback_data;
use crate::tgbot::send;

/// `/config` callback 前缀。
const CONFIG_CALLBACK_PREFIX: &str = "cfg:";

/// 配置 callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConfigCallbackAction {
    Refresh,
    Adjust { field: ConfigField, delta: i64 },
}

impl ConfigCallbackAction {
    /// 点击按钮后的即时提示。
    ///
    /// 这里的提示用于尽快 ACK callback，避免 Telegram 客户端按钮长时间转圈。
    pub(super) fn started_tip(self) -> &'static str {
        match self {
            Self::Refresh => "正在刷新",
            Self::Adjust { .. } => "正在更新配置",
        }
    }
}

/// 允许按钮调整的配置字段。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConfigField {
    JobConcurrency,
    FileDeleteDelayMinutes,
    FileGcIntervalSeconds,
    ProgressEditIntervalSeconds,
    DownloadsDefaultPageSize,
    MenuInputTimeoutSeconds,
}

impl ConfigField {
    /// 字段短编码，写入 callback payload。
    fn code(self) -> &'static str {
        match self {
            Self::JobConcurrency => "jc",
            Self::FileDeleteDelayMinutes => "dd",
            Self::FileGcIntervalSeconds => "gc",
            Self::ProgressEditIntervalSeconds => "pe",
            Self::DownloadsDefaultPageSize => "ps",
            Self::MenuInputTimeoutSeconds => "mt",
        }
    }

    /// 字段配置键，写入日志。
    pub(super) fn key(self) -> &'static str {
        match self {
            Self::JobConcurrency => "job_concurrency",
            Self::FileDeleteDelayMinutes => "file_delete_delay_minutes",
            Self::FileGcIntervalSeconds => "file_gc_interval_seconds",
            Self::ProgressEditIntervalSeconds => "progress_edit_interval_seconds",
            Self::DownloadsDefaultPageSize => "downloads_default_page_size",
            Self::MenuInputTimeoutSeconds => "menu_input_timeout_seconds",
        }
    }

    /// 从 callback 短编码解析字段。
    fn parse(code: &str) -> Option<Self> {
        match code {
            "jc" => Some(Self::JobConcurrency),
            "dd" => Some(Self::FileDeleteDelayMinutes),
            "gc" => Some(Self::FileGcIntervalSeconds),
            "pe" => Some(Self::ProgressEditIntervalSeconds),
            "ps" => Some(Self::DownloadsDefaultPageSize),
            "mt" => Some(Self::MenuInputTimeoutSeconds),
            _ => None,
        }
    }
}

/// 判断 callback payload 是否属于 `/config`。
pub(super) fn is_config_callback_data(data: &str) -> bool {
    data.starts_with(CONFIG_CALLBACK_PREFIX)
}

/// 解析配置 callback payload。
pub(super) fn parse_config_callback_data(data: &str) -> Option<ConfigCallbackAction> {
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

/// config 页面快捷按钮。
pub(in crate::tgbot::transfer::command) fn build_config_buttons()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
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
                tdlib_rs::enums::ButtonStyle::Primary,
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
            send::build_callback_button(
                "进度 -1s",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::ProgressEditIntervalSeconds,
                    delta: -1,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "进度 +1s",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::ProgressEditIntervalSeconds,
                    delta: 1,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_callback_button(
                "分页 -1",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::DownloadsDefaultPageSize,
                    delta: -1,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "分页 +1",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::DownloadsDefaultPageSize,
                    delta: 1,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_callback_button(
                "超时 -60s",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::MenuInputTimeoutSeconds,
                    delta: -60,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "超时 +60s",
                &build_config_callback_data(ConfigCallbackAction::Adjust {
                    field: ConfigField::MenuInputTimeoutSeconds,
                    delta: 60,
                }),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_refresh_return_menu_row(
            send::build_callback_button(
                "刷新",
                &build_config_callback_data(ConfigCallbackAction::Refresh),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "返回",
                &build_help_callback_data(Some("config")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ),
        build_copy_only_row(send::build_copy_button(
            "复制 /config show",
            &config_show_command(CommandStyle::Long),
            tdlib_rs::enums::ButtonStyle::Default,
        )),
        vec![
            send::build_copy_button(
                "复制并发=4",
                &config_set_command("job_concurrency", 4, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制删除=3m",
                &config_set_command("file_delete_delay_minutes", 3, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button(
                "复制GC=30s",
                &config_set_command("file_gc_interval_seconds", 30, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制进度=3s",
                &config_set_command("progress_edit_interval_seconds", 3, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_copy_button(
                "复制分页=10",
                &config_set_command("downloads_default_page_size", 10, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_copy_button(
                "复制超时=900s",
                &config_set_command("menu_input_timeout_seconds", 900, CommandStyle::Long),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ]
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

#[cfg(test)]
mod tests {
    use super::*;

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

        let progress = build_config_callback_data(ConfigCallbackAction::Adjust {
            field: ConfigField::ProgressEditIntervalSeconds,
            delta: -1,
        });
        assert_eq!(progress, "cfg:a:pe:-1");
        assert_eq!(
            parse_config_callback_data(&progress),
            Some(ConfigCallbackAction::Adjust {
                field: ConfigField::ProgressEditIntervalSeconds,
                delta: -1,
            })
        );
    }

    // 点击配置按钮时应先给即时提示，再执行可能较慢的 config.json 写入。
    #[test]
    fn test_config_callback_started_tip() {
        assert_eq!(ConfigCallbackAction::Refresh.started_tip(), "正在刷新");
        assert_eq!(
            ConfigCallbackAction::Adjust {
                field: ConfigField::JobConcurrency,
                delta: 1
            }
            .started_tip(),
            "正在更新配置"
        );
    }

    // 配置交互应覆盖 `/config set` 当前支持的全部动态字段。
    #[test]
    fn test_build_config_buttons_cover_runtime_fields() {
        let rows = build_config_buttons();
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        for expected in [
            "菜单",
            "并发 -1",
            "并发 +1",
            "删除 -1m",
            "删除 +1m",
            "GC -10s",
            "GC +10s",
            "进度 -1s",
            "进度 +1s",
            "分页 -1",
            "分页 +1",
            "超时 -60s",
            "超时 +60s",
        ] {
            assert!(
                labels.contains(&expected),
                "missing config button: {expected}"
            );
        }

        let menu = rows
            .iter()
            .flatten()
            .find(|button| button.text == "菜单")
            .expect("config buttons should include menu button");
        assert!(matches!(
            menu.r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    // 配置页按钮按“主操作 / 刷新返回菜单 / 复制”分层，避免高密度按钮混在一行。
    #[test]
    fn test_build_config_buttons_follow_row_hierarchy() {
        let rows = build_config_buttons();

        assert_eq!(rows[0][0].text, "并发 -1");
        assert_eq!(rows[0][1].text, "并发 +1");
        assert_eq!(rows[6][0].text, "刷新");
        assert_eq!(rows[6][1].text, "返回");
        assert_eq!(rows[6][2].text, "菜单");
        assert_eq!(rows[7][0].text, "复制 /config show");
    }
}
