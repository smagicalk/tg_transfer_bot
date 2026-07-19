// `/config` callback payload 与按钮布局。
// 这里只承载按钮协议和按钮生成，配置读写仍留在上层命令实现里。

use super::super::common::build_runtime_admin_help_menu_row;
use super::super::menu::AdminInputAction;
use crate::tgbot::send;

/// `/config` callback 前缀。
const CONFIG_CALLBACK_PREFIX: &str = "cfg:";

/// 配置 callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConfigCallbackAction {
    Refresh,
    Reset,
    ConfirmReset,
    /// 仅恢复当前字段到启动配置里的默认值。
    ResetField {
        field: ConfigField,
    },
    View {
        field: ConfigField,
    },
    Input {
        field: ConfigField,
    },
    Adjust {
        field: ConfigField,
        direction: i8,
    },
}

impl ConfigCallbackAction {
    /// 点击按钮后的即时提示。
    ///
    /// 这里的提示用于尽快 ACK callback，避免 Telegram 客户端按钮长时间转圈。
    pub(super) fn started_tip(self) -> &'static str {
        match self {
            Self::Refresh => "正在刷新",
            Self::Reset => "正在重置",
            Self::ConfirmReset => "请确认重置",
            Self::ResetField { .. } => "正在恢复默认值",
            Self::View { .. } => "正在打开字段详情",
            Self::Input { .. } => "请回复参数",
            Self::Adjust { .. } => "正在调整",
        }
    }
}

/// 允许按钮调整的配置字段。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot::transfer::command) enum ConfigField {
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
        self.spec().code
    }

    /// 从 callback 短编码解析字段。
    fn parse(code: &str) -> Option<Self> {
        CONFIG_FIELD_SPECS
            .iter()
            .find(|spec| spec.code == code)
            .map(|spec| spec.field)
    }

    /// 获取字段完整规格。
    pub(in crate::tgbot::transfer::command) fn spec(self) -> &'static ConfigFieldSpec {
        CONFIG_FIELD_SPECS
            .iter()
            .find(|spec| spec.field == self)
            .expect("config field spec must exist")
    }
}

/// 可动态修改的运行配置字段规格。
///
/// 按钮、help 示例、输入流命令都从这里读取，避免新增字段时漏改某一处 UI。
#[derive(Debug, Clone, Copy)]
pub(in crate::tgbot::transfer::command) struct ConfigFieldSpec {
    pub field: ConfigField,
    pub code: &'static str,
    pub key: &'static str,
    pub short_label: &'static str,
    pub input_label: &'static str,
    pub input_title: &'static str,
    pub input_detail: &'static str,
    pub input_placeholder: &'static str,
    pub example_value: i64,
    pub admin_input_action: AdminInputAction,
}

/// `/config set` 当前允许动态调整的字段清单。
pub(in crate::tgbot::transfer::command) const CONFIG_FIELD_SPECS: &[ConfigFieldSpec] = &[
    ConfigFieldSpec {
        field: ConfigField::JobConcurrency,
        code: "jc",
        key: "job_concurrency",
        short_label: "并发",
        input_label: "设并发",
        input_title: "设置并发",
        input_detail: "请回复并发数，范围 1-32；或发送 /cancel 取消。",
        input_placeholder: "输入并发数，或发送 /cancel",
        example_value: 4,
        admin_input_action: AdminInputAction::ConfigSetJobConcurrency,
    },
    ConfigFieldSpec {
        field: ConfigField::FileDeleteDelayMinutes,
        code: "dd",
        key: "file_delete_delay_minutes",
        short_label: "删除",
        input_label: "设删除",
        input_title: "设置删除延迟",
        input_detail: "请回复删除延迟分钟数，范围 0-1440；或发送 /cancel 取消。",
        input_placeholder: "输入分钟数，或发送 /cancel",
        example_value: 3,
        admin_input_action: AdminInputAction::ConfigSetFileDeleteDelayMinutes,
    },
    ConfigFieldSpec {
        field: ConfigField::FileGcIntervalSeconds,
        code: "gc",
        key: "file_gc_interval_seconds",
        short_label: "GC",
        input_label: "设GC",
        input_title: "设置 GC 间隔",
        input_detail: "请回复 GC 扫描间隔秒数，范围 5-3600；或发送 /cancel 取消。",
        input_placeholder: "输入秒数，或发送 /cancel",
        example_value: 30,
        admin_input_action: AdminInputAction::ConfigSetFileGcIntervalSeconds,
    },
    ConfigFieldSpec {
        field: ConfigField::ProgressEditIntervalSeconds,
        code: "pe",
        key: "progress_edit_interval_seconds",
        short_label: "进度",
        input_label: "设进度",
        input_title: "设置进度刷新间隔",
        input_detail: "请回复进度刷新秒数，范围 1-60；或发送 /cancel 取消。",
        input_placeholder: "输入秒数，或发送 /cancel",
        example_value: 3,
        admin_input_action: AdminInputAction::ConfigSetProgressEditIntervalSeconds,
    },
    ConfigFieldSpec {
        field: ConfigField::DownloadsDefaultPageSize,
        code: "ps",
        key: "downloads_default_page_size",
        short_label: "分页",
        input_label: "设分页",
        input_title: "设置分页大小",
        input_detail: "请回复分页大小，范围 1-20；或发送 /cancel 取消。",
        input_placeholder: "输入分页大小，或发送 /cancel",
        example_value: 10,
        admin_input_action: AdminInputAction::ConfigSetDownloadsDefaultPageSize,
    },
    ConfigFieldSpec {
        field: ConfigField::MenuInputTimeoutSeconds,
        code: "mt",
        key: "menu_input_timeout_seconds",
        short_label: "超时",
        input_label: "设超时",
        input_title: "设置菜单超时",
        input_detail: "请回复菜单超时秒数，范围 30-86400；或发送 /cancel 取消。",
        input_placeholder: "输入超时秒数，或发送 /cancel",
        example_value: 900,
        admin_input_action: AdminInputAction::ConfigSetMenuInputTimeoutSeconds,
    },
];

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
        "x" => {
            if parts.next().is_none() {
                Some(ConfigCallbackAction::Reset)
            } else {
                None
            }
        }
        "xc" => {
            if parts.next().is_none() {
                Some(ConfigCallbackAction::ConfirmReset)
            } else {
                None
            }
        }
        "xf" => {
            let field = ConfigField::parse(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::ResetField { field })
        }
        "v" => {
            let field = ConfigField::parse(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::View { field })
        }
        "i" => {
            let field = ConfigField::parse(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::Input { field })
        }
        "a" => {
            let field = ConfigField::parse(parts.next()?)?;
            let direction = parts.next()?.parse::<i8>().ok()?;
            if !matches!(direction, -1 | 1) || parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::Adjust { field, direction })
        }
        _ => None,
    }
}

/// config 页面快捷按钮。
#[cfg(test)]
pub(in crate::tgbot::transfer::command) fn build_config_buttons()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let app_context = crate::app_context::app_context();
    build_config_buttons_on(app_context.as_ref())
}

/// config 页面快捷按钮的上下文版本。
pub(in crate::tgbot::transfer::command) fn build_config_buttons_on(
    app: &crate::app_context::AppContext,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    // 配置页主列表只展示字段入口，当前值留在正文和字段详情页，避免按钮随着数值变化变得拥挤。
    // 具体修改统一下沉到字段详情页里的输入流，移动端更清晰。
    let config = crate::tgbot::transfer::runtime_config_on(app);
    let mut rows = CONFIG_FIELD_SPECS
        .chunks(3)
        .map(|specs| build_config_view_row(specs, &config))
        .collect::<Vec<_>>();

    rows.extend([
        vec![
            send::build_callback_button(
                "刷新",
                &build_config_callback_data(ConfigCallbackAction::Refresh),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "重置全部",
                &build_config_callback_data(ConfigCallbackAction::ConfirmReset),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ],
        build_runtime_admin_help_menu_row("config"),
    ]);
    rows
}

/// 构造配置字段详情入口按钮行。
fn build_config_view_row(
    specs: &[ConfigFieldSpec],
    _config: &crate::config::TransferConfig,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    specs
        .iter()
        .map(|spec| {
            send::build_callback_button(
                spec.short_label,
                &build_config_callback_data(ConfigCallbackAction::View { field: spec.field }),
                if spec.field == ConfigField::JobConcurrency {
                    tdlib_rs::enums::ButtonStyle::Primary
                } else {
                    tdlib_rs::enums::ButtonStyle::Default
                },
            )
        })
        .collect()
}

/// 构造配置 callback payload。
pub(in crate::tgbot::transfer::command) fn build_config_detail_callback_data(
    action: ConfigCallbackAction,
) -> String {
    match action {
        ConfigCallbackAction::Refresh => format!("{}r", CONFIG_CALLBACK_PREFIX),
        ConfigCallbackAction::Reset => format!("{}x", CONFIG_CALLBACK_PREFIX),
        ConfigCallbackAction::ConfirmReset => format!("{}xc", CONFIG_CALLBACK_PREFIX),
        ConfigCallbackAction::ResetField { field } => {
            format!("{}xf:{}", CONFIG_CALLBACK_PREFIX, field.code())
        }
        ConfigCallbackAction::View { field } => {
            format!("{}v:{}", CONFIG_CALLBACK_PREFIX, field.code())
        }
        ConfigCallbackAction::Input { field } => {
            format!("{}i:{}", CONFIG_CALLBACK_PREFIX, field.code())
        }
        ConfigCallbackAction::Adjust { field, direction } => {
            format!("{}a:{}:{}", CONFIG_CALLBACK_PREFIX, field.code(), direction)
        }
    }
}

fn build_config_callback_data(action: ConfigCallbackAction) -> String {
    build_config_detail_callback_data(action)
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

        let reset = build_config_callback_data(ConfigCallbackAction::Reset);
        assert_eq!(reset, "cfg:x");
        assert_eq!(
            parse_config_callback_data(&reset),
            Some(ConfigCallbackAction::Reset)
        );
        let confirm_reset = build_config_callback_data(ConfigCallbackAction::ConfirmReset);
        assert_eq!(confirm_reset, "cfg:xc");
        assert_eq!(
            parse_config_callback_data(&confirm_reset),
            Some(ConfigCallbackAction::ConfirmReset)
        );

        let reset_field = build_config_callback_data(ConfigCallbackAction::ResetField {
            field: ConfigField::JobConcurrency,
        });
        assert_eq!(reset_field, "cfg:xf:jc");
        assert_eq!(
            parse_config_callback_data(&reset_field),
            Some(ConfigCallbackAction::ResetField {
                field: ConfigField::JobConcurrency,
            })
        );

        assert_eq!(parse_config_callback_data("cfg:a:bad:1"), None);
        assert_eq!(parse_config_callback_data("cfg:a:gc:x"), None);
        assert_eq!(parse_config_callback_data("cfg:a:gc:10"), None);

        let view = build_config_callback_data(ConfigCallbackAction::View {
            field: ConfigField::JobConcurrency,
        });
        assert_eq!(view, "cfg:v:jc");
        assert_eq!(
            parse_config_callback_data(&view),
            Some(ConfigCallbackAction::View {
                field: ConfigField::JobConcurrency
            })
        );

        let input = build_config_callback_data(ConfigCallbackAction::Input {
            field: ConfigField::ProgressEditIntervalSeconds,
        });
        assert_eq!(input, "cfg:i:pe");
        assert_eq!(
            parse_config_callback_data(&input),
            Some(ConfigCallbackAction::Input {
                field: ConfigField::ProgressEditIntervalSeconds,
            })
        );

        let decrease = build_config_callback_data(ConfigCallbackAction::Adjust {
            field: ConfigField::JobConcurrency,
            direction: -1,
        });
        assert_eq!(decrease, "cfg:a:jc:-1");
        assert_eq!(
            parse_config_callback_data(&decrease),
            Some(ConfigCallbackAction::Adjust {
                field: ConfigField::JobConcurrency,
                direction: -1,
            })
        );
        assert_eq!(parse_config_callback_data("cfg:a:jc:0"), None);
    }

    // 点击配置按钮时应先给即时提示，再执行可能较慢的数据库写入。
    #[test]
    fn test_config_callback_started_tip() {
        assert_eq!(ConfigCallbackAction::Refresh.started_tip(), "正在刷新");
        assert_eq!(ConfigCallbackAction::Reset.started_tip(), "正在重置");
        assert_eq!(
            ConfigCallbackAction::ConfirmReset.started_tip(),
            "请确认重置"
        );
        assert_eq!(
            ConfigCallbackAction::ResetField {
                field: ConfigField::JobConcurrency,
            }
            .started_tip(),
            "正在恢复默认值"
        );
        assert_eq!(
            ConfigCallbackAction::View {
                field: ConfigField::JobConcurrency
            }
            .started_tip(),
            "正在打开字段详情"
        );
        assert_eq!(
            ConfigCallbackAction::Input {
                field: ConfigField::JobConcurrency
            }
            .started_tip(),
            "请回复参数"
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
            "重置全部",
            "并发",
            "删除",
            "GC",
            "进度",
            "分页",
            "超时",
        ] {
            assert!(
                labels.iter().any(|label| label == &expected),
                "missing config button: {expected}"
            );
        }
        assert!(!labels.contains(&"重置默认"));

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

    // 配置页按钮按“主操作 / 刷新返回菜单 / 详情”分层，避免高密度按钮混在一行。
    #[test]
    fn test_build_config_buttons_follow_row_hierarchy() {
        let rows = build_config_buttons();

        assert_eq!(rows[0][0].text, "并发");
        assert_eq!(rows[0][1].text, "删除");
        assert_eq!(rows[0][2].text, "GC");
        assert_eq!(rows[1][0].text, "进度");
        assert_eq!(rows[1][1].text, "分页");
        assert_eq!(rows[1][2].text, "超时");
        assert!(
            !rows
                .iter()
                .flatten()
                .any(|button| button.text.contains(' '))
        );
        assert!(!rows.iter().flatten().any(|button| button.text == "并发 +1"));
        assert!(!rows.iter().flatten().any(|button| button.text == "设并发"));
        let footer = rows.last().expect("config page should have footer");
        assert_eq!(footer[0].text, "帮助");
        assert_eq!(footer[1].text, "菜单");
    }
}
