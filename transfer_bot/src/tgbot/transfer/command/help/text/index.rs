// `/help` 目录页文案。
// 目录页只展示命令概览，详细参数放到 `/help <command>` 中展开。

use super::super::super::common::{
    CommandStyle, command_root, downloads_command, help_command as help_command_text,
    lookup_command, menu_command, short_and_long, transfer_command,
};
use crate::tgbot::transfer::card;

/// 构造帮助目录文本。
/// 只列出命令名和一句话描述，方便后续继续细分。
pub(in crate::tgbot::transfer::command::help) fn build_help_index_text() -> String {
    vec![
        "命令中心".to_owned(),
        format!("状态：{}", card::code("ready")),
        "说明：短命令适合日常输入，长命令适合脚本或排查。".to_owned(),
        card::DIVIDER.to_owned(),
        card::section("常用命令"),
        "转存".to_owned(),
        String::new(),
        "命令：".to_owned(),
        short_and_long(
            transfer_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target_chat_id]"),
            transfer_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target_chat_id]"),
        ),
        "说明：转存单条消息或相册链接。".to_owned(),
        String::new(),
        "查询".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            lookup_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target_chat_id]"),
            lookup_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target_chat_id]"),
        ),
        "说明：按源链接查询历史转存结果。".to_owned(),
        String::new(),
        "任务列表".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            downloads_command(None, None, None, CommandStyle::Short),
            downloads_command(None, None, None, CommandStyle::Long),
        ),
        "说明：查看任务列表、状态和真实下载进度。".to_owned(),
        String::new(),
        "交互菜单".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            menu_command(CommandStyle::Short),
            menu_command(CommandStyle::Long),
        ),
        "说明：打开按钮式菜单，支持复制命令和引导输入转存参数。".to_owned(),
        String::new(),
        "运行配置".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            format!(
                "{} [show|set <key> <value>]",
                command_root("config", CommandStyle::Short)
            ),
            format!(
                "{} [show|set <key> <value>]",
                command_root("config", CommandStyle::Long)
            ),
        ),
        "说明：查看或修改可动态生效的运行配置。".to_owned(),
        String::new(),
        "任务控制".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            format!(
                "{} <p|r|s|st> <job_id>",
                command_root("job", CommandStyle::Short)
            ),
            format!(
                "{} <pause|resume|stop|status> <job_id>",
                command_root("job", CommandStyle::Long)
            ),
        ),
        "说明：查看详情，或手动暂停、恢复、停止转存任务。".to_owned(),
        String::new(),
        card::DIVIDER.to_owned(),
        card::section("帮助详情"),
        "命令：".to_owned(),
        short_and_long(
            help_command_text(None, CommandStyle::Short),
            help_command_text(None, CommandStyle::Long),
        ),
        format!(
            "说明：查看命令目录，或使用 {} 查看详情。",
            card::code("/help <command>")
        ),
        String::new(),
        "可复制示例：".to_owned(),
        short_and_long(
            help_command_text(Some("transfer"), CommandStyle::Short),
            help_command_text(Some("transfer"), CommandStyle::Long),
        ),
        short_and_long(
            help_command_text(Some("lookup"), CommandStyle::Short),
            help_command_text(Some("lookup"), CommandStyle::Long),
        ),
        short_and_long(
            help_command_text(Some("config"), CommandStyle::Short),
            help_command_text(Some("config"), CommandStyle::Long),
        ),
        short_and_long(
            help_command_text(Some("downloads"), CommandStyle::Short),
            help_command_text(Some("downloads"), CommandStyle::Long),
        ),
        short_and_long(
            help_command_text(Some("job"), CommandStyle::Short),
            help_command_text(Some("job"), CommandStyle::Long),
        ),
        short_and_long(
            help_command_text(Some("menu"), CommandStyle::Short),
            help_command_text(Some("menu"), CommandStyle::Long),
        ),
        short_and_long(
            help_command_text(Some("help"), CommandStyle::Short),
            help_command_text(Some("help"), CommandStyle::Long),
        ),
    ]
    .join("\n")
}
