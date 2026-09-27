// `/config` callback payload 与按钮布局。
// 这里只承载按钮协议和按钮生成，配置读写仍留在上层命令实现里。

use super::super::common::build_runtime_admin_help_menu_row;
use super::super::menu::AdminInputAction;
use crate::tgbot::send;

/// `/config` callback 统一协议前缀。
const CONFIG_CALLBACK_PREFIX: &str = "cfg:";

/// 配置内联回调动作类型枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConfigCallbackAction {
    /// 刷新配置视图
    Refresh,
    /// 全量重置所有动态运行配置为启动默认值
    Reset,
    /// 打开全量重置二次确认卡片
    ConfirmReset,
    /// 仅恢复特定单个字段到启动配置里的默认值
    ResetField {
        field: ConfigField,
    },
    /// 查看某个具体字段的数值详情与微调面板
    View {
        field: ConfigField,
    },
    /// 触发针对某个字段的 ForceReply 文本输入修改流
    Input {
        field: ConfigField,
    },
    /// 点击步进按钮微调某个字段的值
    Adjust {
        field: ConfigField,
        direction: i8,
    },
}

impl ConfigCallbackAction {
    /// 点击按钮后的即时轻量提示文案。
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

/// 允许内联按钮动态调整的配置字段枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot::transfer::command) enum ConfigField {
    /// 任务并发执行数
    JobConcurrency,
    /// 文件引用归零后的延迟删除时间（分钟）
    FileDeleteDelayMinutes,
    /// 定期扫描与垃圾回收清理文件的扫描间隔（秒）
    FileGcIntervalSeconds,
    /// 上传/下载进度卡片的原地更新通知频率（秒）
    ProgressEditIntervalSeconds,
    /// `/downloads` 列表每页默认呈现任务数量
    DownloadsDefaultPageSize,
    /// 用户菜单回复输入态的会话超时时间（秒）
    MenuInputTimeoutSeconds,
}

impl ConfigField {
    /// 获取字段的短字符串编码（压缩写入 callback payload）。
    fn code(self) -> &'static str {
        self.spec().code
    }

    /// 从 callback 短编码反向解析出配置字段枚举。
    fn parse(code: &str) -> Option<Self> {
        CONFIG_FIELD_SPECS
            .iter()
            .find(|spec| spec.code == code)
            .map(|spec| spec.field)
    }

    /// 获取该配置字段对应的完整静态规格元数据。
    pub(in crate::tgbot::transfer::command) fn spec(self) -> &'static ConfigFieldSpec {
        CONFIG_FIELD_SPECS
            .iter()
            .find(|spec| spec.field == self)
            .expect("config field spec must exist")
    }
}

/// 可动态修改的运行配置字段规格定义结构体。
///
/// 按钮、help 示例、输入流命令都从这里读取，避免新增字段时漏改某一处 UI。
#[derive(Debug, Clone, Copy)]
pub(in crate::tgbot::transfer::command) struct ConfigFieldSpec {
    /// 字段枚举标识
    pub field: ConfigField,
    /// 短编码标识（如 "jc", "dd" 等）
    pub code: &'static str,
    /// 数据库与命令行参数名（如 "job_concurrency"）
    pub key: &'static str,
    /// 首页按钮简短文案（如 "并发"）
    pub short_label: &'static str,
    /// 详情页修改按钮文案（如 "设并发"）
    pub input_label: &'static str,
    /// 输入提示卡片主标题
    pub input_title: &'static str,
    /// 输入提示卡片正文说明
    pub input_detail: &'static str,
    /// Telegram ForceReply 提示占位符
    pub input_placeholder: &'static str,
    /// 帮助示例中的示例推荐数值
    pub example_value: i64,
    /// 关联的菜单输入流动作枚举
    pub admin_input_action: AdminInputAction,
}

/// `/config set` 当前允许动态调整的字段规格静态清单。
pub(in crate::tgbot::transfer::command) const CONFIG_FIELD_SPECS: &[ConfigFieldSpec] = &[
    ConfigFieldSpec {
        field: ConfigField::JobConcurrency,
        code: "jc",
        key: "job_concurrency",
        short_label: "并发",
        input_label: "设并发",
        input_title: "设置并发",
        input_detail: "请回复并发数，范围 1-32；回复“取消”可退出。",
        input_placeholder: "输入并发数（回复“取消”可退出）",
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
        input_detail: "请回复删除延迟分钟数，范围 0-1440；回复“取消”可退出。",
        input_placeholder: "输入分钟数（回复“取消”可退出）",
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
        input_detail: "请回复 GC 扫描间隔秒数，范围 5-3600；回复“取消”可退出。",
        input_placeholder: "输入秒数（回复“取消”可退出）",
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
        input_detail: "请回复进度刷新秒数，范围 1-60；回复“取消”可退出。",
        input_placeholder: "输入秒数（回复“取消”可退出）",
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
        input_detail: "请回复分页大小，范围 1-20；回复“取消”可退出。",
        input_placeholder: "输入分页大小（回复“取消”可退出）",
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
        input_detail: "请回复菜单超时秒数，范围 30-86400；回复“取消”可退出。",
        input_placeholder: "输入超时秒数（回复“取消”可退出）",
        example_value: 900,
        admin_input_action: AdminInputAction::ConfigSetMenuInputTimeoutSeconds,
    },
];

/// 判断 callback payload 是否属于 `/config` 协议。
pub(super) fn is_config_callback_data(data: &str) -> bool {
    data.starts_with(CONFIG_CALLBACK_PREFIX)
}

/// 解析配置内联按钮回调载荷字符串。
///
/// # 参数
/// - `data`: 原始回调字符串
pub(super) fn parse_config_callback_data(data: &str) -> Option<ConfigCallbackAction> {
    let payload = data.strip_prefix(CONFIG_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    match parts.next()? {
        // "r" -> 刷新
        "r" => {
            if parts.next().is_none() {
                Some(ConfigCallbackAction::Refresh)
            } else {
                None
            }
        }
        // "x" -> 执行重置全部
        "x" => {
            if parts.next().is_none() {
                Some(ConfigCallbackAction::Reset)
            } else {
                None
            }
        }
        // "xc" -> 确认重置全部卡片
        "xc" => {
            if parts.next().is_none() {
                Some(ConfigCallbackAction::ConfirmReset)
            } else {
                None
            }
        }
        // "xf:<field>" -> 重置指定单字段
        "xf" => {
            let field = ConfigField::parse(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::ResetField { field })
        }
        // "v:<field>" -> 查看字段详情
        "v" => {
            let field = ConfigField::parse(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::View { field })
        }
        // "i:<field>" -> 开启字段输入流
        "i" => {
            let field = ConfigField::parse(parts.next()?)?;
            if parts.next().is_some() {
                return None;
            }
            Some(ConfigCallbackAction::Input { field })
        }
        // "a:<field>:<dir>" -> 步进微调
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

/// config 页面快捷按钮（测试环境入口）。
#[cfg(test)]
pub(in crate::tgbot::transfer::command) fn build_config_buttons()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let app_context = crate::app_context::app_context();
    build_config_buttons_on(app_context.as_ref())
}

/// 在指定应用上下文上构建 config 首页按钮矩阵。
///
/// # 参数
/// - `app`: 全局应用上下文实例引用
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

    // 追加底部全局功能行（刷新、重置全部、查看命令、菜单）
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

/// 构造配置字段详情入口按钮单行（每行最多 3 个字段按钮）。
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

/// 构造配置动作对应的短回调 payload 字符串。
///
/// # 参数
/// - `action`: 配置回调动作
pub(in crate::tgbot::transfer::command) fn build_config_detail_callback_data(
    action: ConfigCallbackAction,
) -> String {
    match action {
        ConfigCallbackAction::Refresh => format!("{CONFIG_CALLBACK_PREFIX}r"),
        ConfigCallbackAction::Reset => format!("{CONFIG_CALLBACK_PREFIX}x"),
        ConfigCallbackAction::ConfirmReset => format!("{CONFIG_CALLBACK_PREFIX}xc"),
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
            format!("{CONFIG_CALLBACK_PREFIX}a:{}:{direction}", field.code())
        }
    }
}

/// 构造配置 callback payload 的快捷别名。
fn build_config_callback_data(action: ConfigCallbackAction) -> String {
    build_config_detail_callback_data(action)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 验证配置 callback 使用短 payload 序列化与反序列化的往返一致性。
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

    /// 验证点击配置按钮时立即返回对应的即时轻量提示。
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

    /// 验证配置交互按钮完整覆盖 `/config set` 支持的全部动态字段。
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

    /// 验证配置页按钮按“字段选择 / 刷新重置 / 帮助菜单”分层。
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
        assert_eq!(footer[0].text, "查看命令");
        assert_eq!(footer[1].text, "菜单");
    }
}
