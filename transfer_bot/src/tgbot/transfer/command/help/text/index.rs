// `/help` 目录页文案。
// 目录页只展示命令概览，详细参数放到 `/help <command>` 中展开。

use super::super::super::common::{
    CommandStyle, build_page_command_section, build_ready_page_header, cache_command, command_root,
    downloads_command, health_command as health_command_text, help_command as help_command_text,
    lookup_command, menu_command, transfer_command,
};
use super::super::super::{
    auth::auth_help_summary, cache::cache_help_summary, config_cmd::config_help_descriptor,
    health::health_help_summary, menu::menu_help_summary, targets::targets_help_descriptor,
};
use super::super::super::{downloads::downloads_help_summary, job::job_help_summary};
use super::super::topic::help_index_example_topics;
use crate::tgbot::transfer::card;

/// 构造帮助目录文本。
/// 只列出命令名和一句话描述，方便后续继续细分。
pub(in crate::tgbot::transfer::command::help) fn build_help_index_text() -> String {
    let mut lines = build_ready_page_header("命令中心");
    lines.extend([
        "说明：统一使用长命令，便于命令列表、帮助文档和脚本保持一致。".to_owned(),
        card::section("常用命令"),
        "转存".to_owned(),
        String::new(),
        build_page_command_section(),
        transfer_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
        "说明：转存单条消息或相册链接；不填 target 时使用预先配置的目标。".to_owned(),
        String::new(),
        "查询".to_owned(),
        build_page_command_section(),
        lookup_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
        "说明：按源链接查询历史转存结果。".to_owned(),
        String::new(),
        "任务列表".to_owned(),
        build_page_command_section(),
        downloads_command(None, None, None, CommandStyle::Long),
        format!("说明：{}", downloads_help_summary()),
        String::new(),
    ]);

    lines.extend(build_management_help_index_blocks());

    lines.extend([
        "任务控制".to_owned(),
        build_page_command_section(),
        format!(
            "{} <pause|resume|stop|status> <job_id>",
            command_root("job", CommandStyle::Long)
        ),
        format!("说明：{}", job_help_summary()),
        String::new(),
        card::DIVIDER.to_owned(),
        card::section("帮助详情"),
        build_page_command_section(),
        help_command_text(None, CommandStyle::Long),
        format!(
            "说明：查看命令目录，或使用 {} 查看详情。",
            card::code("/help <command>")
        ),
        String::new(),
        "示例命令：".to_owned(),
        help_command_text(Some("transfer"), CommandStyle::Long),
        help_command_text(Some("lookup"), CommandStyle::Long),
    ]);

    lines.extend(build_help_example_commands());

    lines.join("\n")
}

/// help 目录页“示例命令”区统一来源。
///
/// 目录页示例 topic 顺序和按钮导航共享同一份 topic 元数据，避免后续新增 topic 时漏改目录正文。
fn build_help_example_commands() -> Vec<String> {
    help_index_example_topics()
        .iter()
        .map(|topic| help_command_text(Some(topic), CommandStyle::Long))
        .collect()
}

/// 运行态管理命令在 help 目录页中的简要描述。
struct RuntimeAdminIndexTopic {
    title: &'static str,
    synopsis: String,
    summary: &'static str,
}

/// 构造管理命令的 help 目录区块。
fn build_management_help_index_blocks() -> Vec<String> {
    let mut lines = vec![
        "运行健康".to_owned(),
        build_page_command_section(),
        health_command_text(CommandStyle::Long),
        format!("说明：{}", health_help_summary()),
        String::new(),
        "文件缓存".to_owned(),
        build_page_command_section(),
        cache_command(None, None, None, CommandStyle::Long),
        format!("说明：{}", cache_help_summary()),
        String::new(),
        "交互菜单".to_owned(),
        build_page_command_section(),
        menu_command(CommandStyle::Long),
        format!("说明：{}", menu_help_summary()),
        String::new(),
    ];

    for topic in runtime_admin_index_topics() {
        lines.extend([
            topic.title.to_owned(),
            build_page_command_section(),
            topic.synopsis,
            format!("说明：{}", topic.summary),
            String::new(),
        ]);
    }

    lines
}

/// 四个运行态管理命令在 help 目录页里的统一概览数据。
fn runtime_admin_index_topics() -> Vec<RuntimeAdminIndexTopic> {
    let config = config_help_descriptor();
    let targets = targets_help_descriptor();
    vec![
        RuntimeAdminIndexTopic {
            title: "运行配置",
            synopsis: config.synopsis,
            summary: config.summary,
        },
        RuntimeAdminIndexTopic {
            title: "目标配置",
            synopsis: targets.synopsis,
            summary: targets.summary,
        },
        RuntimeAdminIndexTopic {
            title: "授权管理",
            synopsis: "/auth（打开交互式管理员面板）".to_owned(),
            summary: auth_help_summary(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_help_index_text_uses_ready_header_and_command_sections() {
        let text = build_help_index_text();

        assert!(text.contains("命令中心"));
        assert!(text.contains("状态：‹ready›"));
        assert!(text.contains("■ 常用命令"));
        assert!(text.contains("■ 命令"));
        assert!(text.contains("说明：统一使用长命令"));
    }
}
