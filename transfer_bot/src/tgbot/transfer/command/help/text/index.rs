// `/help` 目录页文案。
// 目录页只展示命令概览，详细参数放到 `/help <command>` 中展开。

use super::super::super::common::{
    CommandStyle, balance_command, build_page_command_section, build_ready_page_header,
    cache_command, command_root, downloads_command, health_command as health_command_text,
    help_command as help_command_text, lookup_command, menu_command, points_show_command,
    transfer_command,
};
use crate::tgbot::transfer::card;

/// 构造帮助目录文本。
/// 只列出命令名和一句话描述，方便后续继续细分。
pub(in crate::tgbot::transfer::command::help) fn build_help_index_text(is_admin: bool) -> String {
    let mut lines = build_ready_page_header("命令中心");
    lines.extend([
        "说明：统一使用长命令，便于命令列表、帮助文档和脚本保持一致。".to_owned(),
        card::section("常用命令"),
        "转存".to_owned(),
        String::new(),
        build_page_command_section(),
        transfer_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
        "说明：转存单条消息或相册链接。".to_owned(),
        String::new(),
        "查询".to_owned(),
        build_page_command_section(),
        lookup_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
        "说明：按源链接查询历史转存结果。".to_owned(),
        String::new(),
        "任务列表".to_owned(),
        build_page_command_section(),
        downloads_command(None, None, None, CommandStyle::Long),
        "说明：查看任务列表、状态和真实下载进度。".to_owned(),
        String::new(),
        "积分账户".to_owned(),
        build_page_command_section(),
        balance_command(CommandStyle::Long),
        if is_admin {
            "说明：查看余额；管理员可用 points 调整或查询普通用户积分。".to_owned()
        } else {
            "说明：查看当前用户余额和流水。".to_owned()
        },
        String::new(),
    ]);

    if is_admin {
        lines.extend([
            "运行健康".to_owned(),
            build_page_command_section(),
            health_command_text(CommandStyle::Long),
            "说明：查看运行配置、并发、恢复队列、任务和缓存总体状态。".to_owned(),
            String::new(),
            "文件缓存".to_owned(),
            build_page_command_section(),
            cache_command(None, None, None, CommandStyle::Long),
            "说明：查看 file_cache 概览和最近缓存记录；只读，不执行清理。".to_owned(),
            String::new(),
            "交互菜单".to_owned(),
            build_page_command_section(),
            menu_command(CommandStyle::Long),
            "说明：打开转存菜单；bot token 模式显示按钮，四个运行态管理页也支持输入流。".to_owned(),
            String::new(),
            "运行配置".to_owned(),
            build_page_command_section(),
            format!(
                "{} [show|set <key> <value>]",
                command_root("config", CommandStyle::Long)
            ),
            "说明：查看或修改可动态生效的运行配置；支持按钮小步调整和输入式设置。".to_owned(),
            String::new(),
            "目标配置".to_owned(),
            build_page_command_section(),
            format!(
                "{} [show|set-default|set-route|del-route|set-alias|del-alias]",
                command_root("targets", CommandStyle::Long)
            ),
            "说明：管理默认目标、按请求 chat 路由和目标别名；支持输入式设置。".to_owned(),
            String::new(),
            "访问控制".to_owned(),
            build_page_command_section(),
            format!(
                "{} [show|add-admin|add-allow-user|add-ban|set ...]",
                command_root("acl", CommandStyle::Long)
            ),
            "说明：管理管理员、允许用户、封禁用户和目标白名单；支持输入式设置。".to_owned(),
            String::new(),
            "计费配置".to_owned(),
            build_page_command_section(),
            format!(
                "{} [show|set|clear announcement_text]",
                command_root("billing", CommandStyle::Long)
            ),
            "说明：管理积分计费参数和首页公告；支持按钮调整和公告输入流。".to_owned(),
            String::new(),
        ]);
    }

    lines.extend([
        "任务控制".to_owned(),
        build_page_command_section(),
        format!(
            "{} <pause|resume|stop|status> <job_id>",
            command_root("job", CommandStyle::Long)
        ),
        "说明：查看详情，或手动暂停、恢复、停止转存任务。".to_owned(),
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
        "可复制示例：".to_owned(),
        help_command_text(Some("transfer"), CommandStyle::Long),
        help_command_text(Some("lookup"), CommandStyle::Long),
    ]);

    if is_admin {
        lines.extend([
            help_command_text(Some("config"), CommandStyle::Long),
            help_command_text(Some("targets"), CommandStyle::Long),
            help_command_text(Some("acl"), CommandStyle::Long),
            help_command_text(Some("billing"), CommandStyle::Long),
        ]);
    }

    lines.extend([
        help_command_text(Some("downloads"), CommandStyle::Long),
        help_command_text(Some("points"), CommandStyle::Long),
    ]);

    if is_admin {
        lines.extend([
            help_command_text(Some("health"), CommandStyle::Long),
            help_command_text(Some("cache"), CommandStyle::Long),
        ]);
    }

    lines.extend([
        help_command_text(Some("job"), CommandStyle::Long),
        help_command_text(Some("menu"), CommandStyle::Long),
        help_command_text(Some("help"), CommandStyle::Long),
    ]);

    if is_admin {
        lines.extend([
            String::new(),
            "管理员示例：".to_owned(),
            points_show_command(123456789, CommandStyle::Long),
        ]);
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_help_index_text_uses_ready_header_and_command_sections() {
        let text = build_help_index_text(true);

        assert!(text.contains("命令中心"));
        assert!(text.contains("状态：‹ready›"));
        assert!(text.contains("■ 常用命令"));
        assert!(text.contains("■ 命令"));
        assert!(text.contains("说明：统一使用长命令"));
    }
}
