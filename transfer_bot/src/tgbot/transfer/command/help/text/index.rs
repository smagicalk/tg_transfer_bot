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
        "*命令目录*",
        &short_and_long(
            help_command_text(None, CommandStyle::Short),
            help_command_text(None, CommandStyle::Long),
        ),
        "查看命令目录，或使用 `/help <command>` 查看详情。",
        "",
        &short_and_long(
            transfer_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target_chat_id]"),
            transfer_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target_chat_id]"),
        ),
        "转存单条消息或相册链接。",
        "",
        &short_and_long(
            lookup_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target_chat_id]"),
            lookup_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target_chat_id]"),
        ),
        "按源链接查询历史转存结果。",
        "",
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
        "查看或修改可动态生效的运行配置。",
        "",
        &short_and_long(
            downloads_command(None, None, None, CommandStyle::Short),
            downloads_command(None, None, None, CommandStyle::Long),
        ),
        "查看任务列表、状态、真实下载进度。",
        "",
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
        "手动暂停、恢复、停止转存任务。",
        "",
        "查看详情示例：",
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
