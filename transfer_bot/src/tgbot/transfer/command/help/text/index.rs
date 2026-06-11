// `/help` 目录页文案。
// 目录页只展示命令概览，详细参数放到 `/help <command>` 中展开。

use super::super::super::common::{
    CommandStyle, balance_command, cache_command, command_root, downloads_command,
    health_command as health_command_text, help_command as help_command_text, lookup_command,
    menu_command, points_show_command, short_and_long, transfer_command,
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
            transfer_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target]"),
            transfer_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
        ),
        "说明：转存单条消息或相册链接。".to_owned(),
        String::new(),
        "查询".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            lookup_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target]"),
            lookup_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
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
        "积分账户".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            balance_command(CommandStyle::Short),
            balance_command(CommandStyle::Long),
        ),
        "说明：查看当前用户余额和流水；管理员可用 points 调整或查询普通用户积分。".to_owned(),
        String::new(),
        "运行健康".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            health_command_text(CommandStyle::Short),
            health_command_text(CommandStyle::Long),
        ),
        "说明：查看运行配置、并发、恢复队列、任务和缓存总体状态。".to_owned(),
        String::new(),
        "文件缓存".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            cache_command(None, None, None, CommandStyle::Short),
            cache_command(None, None, None, CommandStyle::Long),
        ),
        "说明：查看 file_cache 概览和最近缓存记录；只读，不执行清理。".to_owned(),
        String::new(),
        "交互菜单".to_owned(),
        "命令：".to_owned(),
        short_and_long(
            menu_command(CommandStyle::Short),
            menu_command(CommandStyle::Long),
        ),
        "说明：打开转存菜单；bot token 模式显示按钮，手机号/OCR 模式显示文本命令。".to_owned(),
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
            help_command_text(Some("points"), CommandStyle::Short),
            help_command_text(Some("points"), CommandStyle::Long),
        ),
        short_and_long(
            help_command_text(Some("health"), CommandStyle::Short),
            help_command_text(Some("health"), CommandStyle::Long),
        ),
        short_and_long(
            help_command_text(Some("cache"), CommandStyle::Short),
            help_command_text(Some("cache"), CommandStyle::Long),
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
        String::new(),
        "管理员示例：".to_owned(),
        short_and_long(
            points_show_command(123456789, CommandStyle::Short),
            points_show_command(123456789, CommandStyle::Long),
        ),
    ]
    .join("\n")
}
