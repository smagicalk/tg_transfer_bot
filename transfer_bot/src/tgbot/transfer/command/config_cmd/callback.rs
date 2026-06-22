// `/config` callback payload 与按钮布局。
// 这里只承载按钮协议和按钮生成，配置读写仍留在上层命令实现里。

use super::super::common::build_help_menu_row;
use super::super::help::build_help_callback_data;
use super::super::menu::AdminInputAction;
use super::super::menu::build_menu_home_callback_data;
use crate::tgbot::send;

/// `/config` callback 前缀。
const CONFIG_CALLBACK_PREFIX: &str = "cfg:";

/// 配置 callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConfigCallbackAction {
    Refresh,
    Reset,
    View { field: ConfigField },
    Adjust { field: ConfigField, delta: i64 },
    Input { field: ConfigField },
}

impl ConfigCallbackAction {
    /// 点击按钮后的即时提示。
    ///
    /// 这里的提示用于尽快 ACK callback，避免 Telegram 客户端按钮长时间转圈。
    pub(super) fn started_tip(self) -> &'static str {
        match self {
            Self::Refresh => "正在刷新",
            Self::Reset => "正在重置",
            Self::View { .. } => "正在打开字段详情",
            Self::Adjust { .. } => "正在更新配置",
            Self::Input { .. } => "请回复参数",
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

    /// 字段配置键，写入日志。
    pub(super) fn key(self) -> &'static str {
        self.spec().key
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
    pub adjust_step: i64,
    pub adjust_unit: &'static str,
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
        adjust_step: 1,
        adjust_unit: "",
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
        adjust_step: 1,
        adjust_unit: "m",
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
        adjust_step: 10,
        adjust_unit: "s",
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
        adjust_step: 1,
        adjust_unit: "s",
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
        adjust_step: 1,
        adjust_unit: "",
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
        adjust_step: 60,
        adjust_unit: "s",
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
        "v" => {
            let field = ConfigField::parse(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::View { field })
        }
        "a" => {
            let field = ConfigField::parse(parts.next()?)?;
            let delta = parts.next()?.parse::<i64>().ok()?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::Adjust { field, delta })
        }
        "i" => {
            let field = ConfigField::parse(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::Input { field })
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
    let mut rows = CONFIG_FIELD_SPECS
        .iter()
        .map(build_config_adjust_row)
        .collect::<Vec<_>>();

    rows.splice(
        0..0,
        CONFIG_FIELD_SPECS
            .chunks(3)
            .map(build_config_view_row)
            .collect::<Vec<_>>(),
    );

    rows.extend(
        CONFIG_FIELD_SPECS
            .chunks(3)
            .map(build_config_input_row)
            .collect::<Vec<_>>(),
    );

    rows.extend([
        vec![
            send::build_callback_button(
                "刷新",
                &build_config_callback_data(ConfigCallbackAction::Refresh),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "重置默认",
                &build_config_callback_data(ConfigCallbackAction::Reset),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_help_menu_row(
            send::build_callback_button(
                "帮助",
                &build_help_callback_data(Some("config")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &build_menu_home_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ),
    ]);

    let _ = app;
    rows
}

/// 构造单个字段的小步增减按钮行。
fn build_config_adjust_row(spec: &ConfigFieldSpec) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    let minus_delta = -spec.adjust_step;
    let plus_delta = spec.adjust_step;
    vec![
        send::build_callback_button(
            &format!("{} {}{}", spec.short_label, minus_delta, spec.adjust_unit),
            &build_config_callback_data(ConfigCallbackAction::Adjust {
                field: spec.field,
                delta: minus_delta,
            }),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            &format!("{} +{}{}", spec.short_label, plus_delta, spec.adjust_unit),
            &build_config_callback_data(ConfigCallbackAction::Adjust {
                field: spec.field,
                delta: plus_delta,
            }),
            if spec.field == ConfigField::JobConcurrency {
                tdlib_rs::enums::ButtonStyle::Primary
            } else {
                tdlib_rs::enums::ButtonStyle::Default
            },
        ),
    ]
}

/// 构造配置字段详情入口按钮行。
fn build_config_view_row(specs: &[ConfigFieldSpec]) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
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

/// 构造配置输入按钮行。
fn build_config_input_row(specs: &[ConfigFieldSpec]) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    specs
        .iter()
        .map(|spec| {
            send::build_callback_button(
                spec.input_label,
                &build_config_callback_data(ConfigCallbackAction::Input { field: spec.field }),
                tdlib_rs::enums::ButtonStyle::Default,
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
        ConfigCallbackAction::View { field } => {
            format!("{}v:{}", CONFIG_CALLBACK_PREFIX, field.code())
        }
        ConfigCallbackAction::Adjust { field, delta } => {
            format!("{}a:{}:{}", CONFIG_CALLBACK_PREFIX, field.code(), delta)
        }
        ConfigCallbackAction::Input { field } => {
            format!("{}i:{}", CONFIG_CALLBACK_PREFIX, field.code())
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
        assert_eq!(ConfigCallbackAction::Reset.started_tip(), "正在重置");
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
            "重置默认",
            "并发",
            "删除",
            "GC",
            "进度",
            "分页",
            "超时",
            "设并发",
            "设删除",
            "设GC",
            "设进度",
            "设分页",
            "设超时",
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

        assert_eq!(rows[0][0].text, "并发");
        assert_eq!(rows[0][1].text, "删除");
        assert_eq!(rows[0][2].text, "GC");
        assert_eq!(rows[1][0].text, "进度");
        assert_eq!(rows[1][1].text, "分页");
        assert_eq!(rows[1][2].text, "超时");
        assert!(rows.iter().flatten().any(|button| button.text == "并发 -1"));
        assert!(rows.iter().flatten().any(|button| button.text == "并发 +1"));
        assert!(rows.iter().flatten().any(|button| button.text == "设并发"));
        let footer = rows.last().expect("config page should have footer");
        assert_eq!(footer[0].text, "帮助");
        assert_eq!(footer[1].text, "菜单");
    }
}
