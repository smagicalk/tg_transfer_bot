// `/help` 目录页文案。
// 目录页只展示命令概览，详细参数放到 `/help <command>` 中展开。

use super::super::super::common::{
    CommandStyle, command_root, downloads_command, help_command as help_command_text,
    lookup_command, short_and_long, transfer_command,
};

/// 构造帮助目录文本。
/// 只列出命令名和一句话描述，方便后续继续细分。
pub(in crate::tgbot::transfer::command::help) fn build_help_index_text() -> String {
    [
        "*命令中心*",
        "状态：`ready`",
        "说明：短命令适合日常输入，长命令适合脚本或排查。",
        "━━━━━━━━━━━━",
        "*常用命令*",
        "转存",
        "",
        "命令：",
        &short_and_long(
            transfer_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target_chat_id]"),
            transfer_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target_chat_id]"),
        ),
        "说明：转存单条消息或相册链接。",
        "",
        "查询",
        "命令：",
        &short_and_long(
            lookup_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target_chat_id]"),
            lookup_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target_chat_id]"),
        ),
        "说明：按源链接查询历史转存结果。",
        "",
        "任务列表",
        "命令：",
        &short_and_long(
            downloads_command(None, None, None, CommandStyle::Short),
            downloads_command(None, None, None, CommandStyle::Long),
        ),
        "说明：查看任务列表、状态和真实下载进度。",
        "",
        "运行配置",
        "命令：",
        &short_and_long(
            format!(
                "{} [show|set <key> <value>]",
                command_root("config", CommandStyle::Short)
            ),
            format!(
                "{} [show|set <key> <value>]",
                command_root("config", CommandStyle::Long)
            ),
        ),
        "说明：查看或修改可动态生效的运行配置。",
        "",
        "任务控制",
        "命令：",
        &short_and_long(
            format!(
                "{} <pause|resume|stop> <job_id>",
                command_root("job", CommandStyle::Short)
            ),
            format!(
                "{} <pause|resume|stop> <job_id>",
                command_root("job", CommandStyle::Long)
            ),
        ),
        "说明：手动暂停、恢复、停止转存任务。",
        "",
        "━━━━━━━━━━━━",
        "*帮助详情*",
        "命令：",
        &short_and_long(
            help_command_text(None, CommandStyle::Short),
            help_command_text(None, CommandStyle::Long),
        ),
        "说明：查看命令目录，或使用 `/help <command>` 查看详情。",
        "",
        "可复制示例：",
        &short_and_long(
            help_command_text(Some("transfer"), CommandStyle::Short),
            help_command_text(Some("transfer"), CommandStyle::Long),
        ),
        &short_and_long(
            help_command_text(Some("lookup"), CommandStyle::Short),
            help_command_text(Some("lookup"), CommandStyle::Long),
        ),
        &short_and_long(
            help_command_text(Some("config"), CommandStyle::Short),
            help_command_text(Some("config"), CommandStyle::Long),
        ),
        &short_and_long(
            help_command_text(Some("downloads"), CommandStyle::Short),
            help_command_text(Some("downloads"), CommandStyle::Long),
        ),
        &short_and_long(
            help_command_text(Some("job"), CommandStyle::Short),
            help_command_text(Some("job"), CommandStyle::Long),
        ),
        &short_and_long(
            help_command_text(Some("help"), CommandStyle::Short),
            help_command_text(Some("help"), CommandStyle::Long),
        ),
    ]
    .join("\n")
}
