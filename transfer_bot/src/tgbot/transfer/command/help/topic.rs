// `/help` 的 topic 归一化逻辑。
// 当前仅支持长命令和带斜杠写法，不再公开短别名。

/// help 目录页按钮元数据。
///
/// topic 归一化、目录按钮和目录示例命令都依赖同一份最小定义，
/// 避免新增命令时漏改其中一处。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct HelpTopicButtonSpec {
    pub label: &'static str,
    pub topic: &'static str,
    pub primary: bool,
}

/// 运行态管理类 help topic。
///
/// 这四页都遵循“统一帮助正文 + 统一 footer + 模块自带入口按钮”的结构，
/// 单独标记后，`detail.rs` 和 `keyboard.rs` 可以复用同一套路由逻辑。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RuntimeAdminHelpTopic {
    Config,
    Targets,
}

/// help 详情页最小 topic 规格。
///
/// 当前先统一：
/// - admin-only 权限判断
/// - 是否属于运行态管理页
///
/// 后续如果继续推进“同源化”，可以在这里继续补充更高层的详情元数据。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct HelpDetailTopicSpec {
    pub topic: &'static str,
    pub runtime_admin: Option<RuntimeAdminHelpTopic>,
}

const HELP_DETAIL_TOPIC_SPECS: &[HelpDetailTopicSpec] = &[
    HelpDetailTopicSpec {
        topic: "help",
        runtime_admin: None,
    },
    HelpDetailTopicSpec {
        topic: "health",
        runtime_admin: None,
    },
    HelpDetailTopicSpec {
        topic: "transfer",
        runtime_admin: None,
    },
    HelpDetailTopicSpec {
        topic: "lookup",
        runtime_admin: None,
    },
    HelpDetailTopicSpec {
        topic: "cache",
        runtime_admin: None,
    },
    HelpDetailTopicSpec {
        topic: "auth",
        runtime_admin: None,
    },
    HelpDetailTopicSpec {
        topic: "config",
        runtime_admin: Some(RuntimeAdminHelpTopic::Config),
    },
    HelpDetailTopicSpec {
        topic: "targets",
        runtime_admin: Some(RuntimeAdminHelpTopic::Targets),
    },
    HelpDetailTopicSpec {
        topic: "downloads",
        runtime_admin: None,
    },
    HelpDetailTopicSpec {
        topic: "job",
        runtime_admin: None,
    },
    HelpDetailTopicSpec {
        topic: "menu",
        runtime_admin: None,
    },
];

const HELP_INDEX_PRIMARY_ROW_ONE: [HelpTopicButtonSpec; 3] = [
    HelpTopicButtonSpec {
        label: "转存",
        topic: "transfer",
        primary: true,
    },
    HelpTopicButtonSpec {
        label: "查询",
        topic: "lookup",
        primary: false,
    },
    HelpTopicButtonSpec {
        label: "下载列表",
        topic: "downloads",
        primary: false,
    },
];

const HELP_INDEX_PRIMARY_ROW_TWO: [HelpTopicButtonSpec; 2] = [
    HelpTopicButtonSpec {
        label: "任务控制",
        topic: "job",
        primary: false,
    },
    HelpTopicButtonSpec {
        label: "交互菜单",
        topic: "menu",
        primary: false,
    },
];

const HELP_INDEX_PRIMARY_ROWS: [&[HelpTopicButtonSpec]; 2] =
    [&HELP_INDEX_PRIMARY_ROW_ONE, &HELP_INDEX_PRIMARY_ROW_TWO];

const HELP_INDEX_ADMIN_ROW_ONE: [HelpTopicButtonSpec; 3] = [
    HelpTopicButtonSpec {
        label: "运行健康",
        topic: "health",
        primary: false,
    },
    HelpTopicButtonSpec {
        label: "文件缓存",
        topic: "cache",
        primary: false,
    },
    HelpTopicButtonSpec {
        label: "运行配置",
        topic: "config",
        primary: false,
    },
];

const HELP_INDEX_ADMIN_ROW_TWO: [HelpTopicButtonSpec; 2] = [
    HelpTopicButtonSpec {
        label: "目标配置",
        topic: "targets",
        primary: false,
    },
    HelpTopicButtonSpec {
        label: "授权管理",
        topic: "auth",
        primary: false,
    },
];

const HELP_INDEX_ADMIN_ROWS: [&[HelpTopicButtonSpec]; 2] =
    [&HELP_INDEX_ADMIN_ROW_ONE, &HELP_INDEX_ADMIN_ROW_TWO];

const HELP_INDEX_EXAMPLE_TOPICS: &[&str] = &[
    "transfer",
    "lookup",
    "downloads",
    "job",
    "menu",
    "help",
    "config",
    "targets",
    "auth",
    "health",
    "cache",
];

/// help 目录页的基础导航按钮行。
pub(super) fn help_index_primary_button_rows() -> &'static [&'static [HelpTopicButtonSpec]] {
    &HELP_INDEX_PRIMARY_ROWS
}

/// help 目录页的管理功能导航按钮行。
pub(super) fn help_index_admin_button_rows() -> &'static [&'static [HelpTopicButtonSpec]] {
    &HELP_INDEX_ADMIN_ROWS
}

/// help 目录页“示例命令”分区使用的 topic 顺序。
pub(super) fn help_index_example_topics() -> &'static [&'static str] {
    HELP_INDEX_EXAMPLE_TOPICS
}

/// 返回 help 详情页的最小 topic 规格。
pub(super) fn help_detail_topic_spec(topic: &str) -> Option<&'static HelpDetailTopicSpec> {
    HELP_DETAIL_TOPIC_SPECS
        .iter()
        .find(|spec| spec.topic == topic)
}

/// 判断 topic 是否属于统一运行态管理页。
pub(super) fn runtime_admin_help_topic(topic: &str) -> Option<RuntimeAdminHelpTopic> {
    help_detail_topic_spec(topic)?.runtime_admin
}

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
        "auth" => Ok("auth"),
        "config" => Ok("config"),
        "targets" => Ok("targets"),
        "downloads" | "download" => Ok("downloads"),
        "job" => Ok("job"),
        "menu" => Ok("menu"),
        other => anyhow::bail!("unknown help topic: {}", other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_admin_help_topic_marks_runtime_pages() {
        assert_eq!(
            runtime_admin_help_topic("config"),
            Some(RuntimeAdminHelpTopic::Config)
        );
        assert_eq!(
            runtime_admin_help_topic("targets"),
            Some(RuntimeAdminHelpTopic::Targets)
        );
        assert_eq!(runtime_admin_help_topic("downloads"), None);
    }

    #[test]
    fn test_auth_help_topic_is_available() -> anyhow::Result<()> {
        assert_eq!(normalize_help_topic("auth")?, "auth");
        assert_eq!(normalize_help_topic("/auth")?, "auth");
        assert!(help_detail_topic_spec("auth").is_some());
        Ok(())
    }
}
