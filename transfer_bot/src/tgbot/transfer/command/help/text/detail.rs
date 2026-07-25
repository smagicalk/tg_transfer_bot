// `/help <command>` 详情页文案。
// 每个命令的长说明集中在这里，后续调整参数说明时不影响命令入口。

use super::super::super::common::{CommandStyle, help_command as help_command_text};
use super::super::super::common::{
    RuntimeAdminHelpDescriptor, build_runtime_admin_help_detail_text,
};
use super::super::super::config_cmd::{
    config_help_descriptor, config_intro_lines, config_summary_lines,
};
use super::super::super::{
    auth::build_auth_help_detail_text,
    cache::build_cache_help_detail_text,
    downloads::build_downloads_help_detail_text,
    health::build_health_help_detail_text,
    job::build_job_help_detail_text,
    lookup::build_lookup_help_detail_text,
    menu::build_menu_help_detail_text,
    targets::{targets_help_descriptor, targets_input_entry_lines, targets_intro_lines},
    transfer_cmd::build_transfer_help_detail_text,
};
use super::super::topic::{RuntimeAdminHelpTopic, normalize_help_topic, runtime_admin_help_topic};
use crate::tgbot::transfer::card;

/// 构造命令详细帮助。
pub(in crate::tgbot::transfer::command::help) fn build_help_detail_text(
    command_name: &str,
) -> anyhow::Result<String> {
    let command_name = normalize_help_topic(command_name)?;
    if let Some(topic) = runtime_admin_help_topic(command_name) {
        return Ok(build_runtime_admin_topic_detail(topic));
    }
    let text = match command_name {
        "help" => build_help_detail(),
        "transfer" => build_transfer_help_detail_text(),
        "lookup" => build_lookup_help_detail_text(),
        "health" => build_health_help_detail_text(),
        "cache" => build_cache_help_detail_text(),
        "auth" => build_auth_help_detail_text(),
        "downloads" => build_downloads_help_detail_text(),
        "job" => build_job_help_detail_text(),
        "menu" => build_menu_help_detail_text(),
        _ => anyhow::bail!("unknown help topic: {command_name}"),
    };
    Ok(text)
}

/// 构造 `/help` 自身的说明。
fn build_help_detail() -> String {
    [
        "help".to_owned(),
        "用途：查看命令帮助。".to_owned(),
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        format!("{} [command]", help_command_text(None, CommandStyle::Long)),
        String::new(),
        "示例：".to_owned(),
        help_command_text(None, CommandStyle::Long),
        help_command_text(Some("transfer"), CommandStyle::Long),
    ]
    .join("\n")
}

/// 运行态管理 help 详情规格。
///
/// 这四页都复用同一套正文模板，只是“说明 / 额外摘要 / descriptor”来自各自模块。
struct RuntimeAdminDetailSpec {
    title: &'static str,
    detail_lines: fn() -> Vec<String>,
    extra_lines: fn() -> Vec<String>,
    descriptor: fn() -> RuntimeAdminHelpDescriptor,
}

/// 构造运行态管理类 topic 的 help 详情。
fn build_runtime_admin_topic_detail(topic: RuntimeAdminHelpTopic) -> String {
    let spec = runtime_admin_detail_spec(topic);
    build_runtime_admin_help_detail_text(
        spec.title,
        (spec.detail_lines)(),
        (spec.extra_lines)(),
        &(spec.descriptor)(),
    )
}

/// 返回四个运行态管理 topic 对应的正文规格。
fn runtime_admin_detail_spec(topic: RuntimeAdminHelpTopic) -> RuntimeAdminDetailSpec {
    match topic {
        RuntimeAdminHelpTopic::Config => RuntimeAdminDetailSpec {
            title: "config",
            detail_lines: config_intro_lines,
            extra_lines: config_summary_lines,
            descriptor: config_help_descriptor,
        },
        RuntimeAdminHelpTopic::Targets => RuntimeAdminDetailSpec {
            title: "targets",
            detail_lines: targets_intro_lines,
            extra_lines: targets_input_entry_lines,
            descriptor: targets_help_descriptor,
        },
    }
}

#[cfg(test)]
mod config_detail_tests {
    use super::{build_help_detail_text, build_runtime_admin_topic_detail};
    use crate::tgbot::transfer::command::help::topic::RuntimeAdminHelpTopic;

    #[test]
    fn test_build_config_detail_mentions_reset() {
        let text = build_runtime_admin_topic_detail(RuntimeAdminHelpTopic::Config);
        assert!(text.contains("/config reset"));
        assert!(text.contains("重置为启动配置中的默认值"));
        assert!(text.contains("■ 可调字段"));
    }

    #[test]
    fn test_runtime_admin_help_details_show_entry_sections() {
        let targets = build_runtime_admin_topic_detail(RuntimeAdminHelpTopic::Targets);

        assert!(targets.contains("■ 输入入口"));
        assert!(targets.contains("/targets set-default 123456789"));
    }

    #[test]
    fn test_auth_help_detail_is_owner_only_and_complete() -> anyhow::Result<()> {
        let text = build_help_detail_text("auth")?;

        assert!(text.contains("仅 owner"));
        assert!(text.contains("/auth list"));
        assert!(text.contains("/auth add <user_id>"));
        assert!(text.contains("/auth del <user_id>"));
        Ok(())
    }
}
