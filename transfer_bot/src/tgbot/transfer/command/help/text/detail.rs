// `/help <command>` 详情页文案。
// 每个命令的长说明集中在这里，后续调整参数说明时不影响命令入口。

use super::super::super::common::{
    CommandStyle, balance_command, balance_history_command, cache_command, command_root,
    downloads_command, health_command as health_command_text, help_command as help_command_text,
    job_command, lookup_command, menu_command, points_change_command, points_history_command,
    points_show_command, transfer_command,
};
use super::super::super::common::{
    build_runtime_admin_examples_block, build_runtime_admin_interaction_block,
    build_runtime_admin_usage_block,
};
use super::super::super::config_cmd::config_help_descriptor;
use super::super::super::{
    acl::acl_help_descriptor, billing::billing_help_descriptor, targets::targets_help_descriptor,
};
use super::super::topic::normalize_help_topic;
use crate::tgbot::transfer::card;

/// 构造命令详细帮助。
pub(in crate::tgbot::transfer::command::help) fn build_help_detail_text(
    command_name: &str,
) -> anyhow::Result<String> {
    let command_name = normalize_help_topic(command_name)?;
    let text = match command_name {
        "help" => build_help_detail(),
        "transfer" => build_transfer_detail(),
        "lookup" => build_lookup_detail(),
        "points" => build_points_detail(),
        "health" => build_health_detail(),
        "cache" => build_cache_detail(),
        "config" => build_config_detail(),
        "targets" => build_targets_detail(),
        "acl" => build_acl_detail(),
        "billing" => build_billing_detail(),
        "downloads" => build_downloads_detail(),
        "job" => build_job_detail(),
        "menu" => build_menu_detail(),
        _ => anyhow::bail!("unknown help topic: {}", command_name),
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

/// 构造 `/balance` 和 `/points` 的说明。
fn build_points_detail() -> String {
    [
        "points".to_owned(),
        "用途：查看余额、查询积分流水，或由管理员调整普通用户积分。".to_owned(),
        "说明：普通用户创建转存任务前会按消息数量扣积分；admin 不扣积分；失败/取消会按规则退款并写入流水。".to_owned(),
        card::DIVIDER.to_owned(),
        "普通用户命令：".to_owned(),
        balance_command(CommandStyle::Long),
        balance_history_command(10, 1, CommandStyle::Long),
        String::new(),
        "管理员命令：".to_owned(),
        format!(
            "{} <show|history|add|sub> <user_id> [amount|limit] [reason|page]",
            command_root("points", CommandStyle::Long)
        ),
        String::new(),
        "动作：".to_owned(),
        format!("{}：查看指定用户积分。", card::code("show | s")),
        format!("{}：分页查看指定用户积分流水。", card::code("history | h")),
        format!("{}：给指定用户增加积分。", card::code("add | a")),
        format!("{}：扣除指定用户积分。", card::code("sub")),
        String::new(),
        "示例：".to_owned(),
        balance_command(CommandStyle::Long),
        balance_history_command(10, 1, CommandStyle::Long),
        points_show_command(123456789, CommandStyle::Long),
        points_history_command(123456789, 10, 1, CommandStyle::Long),
        points_change_command("add", 123456789, 10, "admin_adjust", CommandStyle::Long),
        points_change_command("sub", 123456789, 10, "admin_adjust", CommandStyle::Long),
    ]
    .join("\n")
}

/// 构造 `/health` 的说明。
fn build_health_detail() -> String {
    [
        "health".to_owned(),
        "用途：只读查看运行健康状态。".to_owned(),
        "说明：展示任务规模、恢复队列、缓存队列、并发和运行时配置，不修改任何状态。".to_owned(),
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        health_command_text(CommandStyle::Long),
        String::new(),
        "示例：".to_owned(),
        health_command_text(CommandStyle::Long),
    ]
    .join("\n")
}

/// 构造 `/cache` 的说明。
fn build_cache_detail() -> String {
    [
        "cache".to_owned(),
        "用途：只读查看 file_cache 状态。".to_owned(),
        "说明：默认展示状态概览；page 模式展示最近更新的缓存记录，不执行删除。".to_owned(),
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        cache_command(None, None, None, CommandStyle::Long),
        cache_command(Some("page"), None, None, CommandStyle::Long),
        String::new(),
        "示例：".to_owned(),
        cache_command(None, None, None, CommandStyle::Long),
        cache_command(Some("page"), Some(10), Some(1), CommandStyle::Long),
    ]
    .join("\n")
}

/// 构造 `/transfer` 的说明。
fn build_transfer_detail() -> String {
    [
        "transfer".to_owned(),
        "用途：转存单条消息或相册链接。".to_owned(),
        "说明：target 可填数字 chat_id 或配置里的别名；不传时使用 targets 默认目标。".to_owned(),
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        transfer_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
        String::new(),
        "示例：".to_owned(),
        "/transfer https://t.me/c/123/456".to_owned(),
        "/transfer https://t.me/c/123/456 -1001234567890".to_owned(),
        "/transfer https://t.me/c/123/456 archive".to_owned(),
    ]
    .join("\n")
}

/// 构造 `/lookup` 的说明。
fn build_lookup_detail() -> String {
    [
        "lookup".to_owned(),
        "用途：按源链接查询历史转存结果。".to_owned(),
        "说明：target 可填数字 chat_id 或配置里的别名；命中成功任务时会返回目标消息入口或定位信息。".to_owned(),
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        lookup_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
        String::new(),
        "示例：".to_owned(),
        "/lookup https://t.me/c/123/456".to_owned(),
        "/lookup https://t.me/c/123/456 -1001234567890".to_owned(),
        "/lookup https://t.me/c/123/456 archive".to_owned(),
    ]
    .join("\n")
}

/// 构造 `/config` 的说明。
fn build_config_detail() -> String {
    let descriptor = config_help_descriptor();
    let mut lines = vec![
        "config".to_owned(),
        "用途：查看或修改可动态生效的运行配置。".to_owned(),
        "说明：配置页同时支持两种方式：".to_owned(),
        "1. 直接点按钮做小步增减。".to_owned(),
        "2. 点“设并发 / 设删除 / 设GC / 设进度 / 设分页 / 设超时”进入输入流，再回复一个值。"
            .to_owned(),
        card::DIVIDER.to_owned(),
    ];
    lines.extend(build_runtime_admin_usage_block(&descriptor));
    lines.extend([
        String::new(),
        "可调字段：".to_owned(),
        card::code("job_concurrency"),
        card::code("file_delete_delay_minutes"),
        card::code("file_gc_interval_seconds"),
        card::code("progress_edit_interval_seconds"),
        card::code("downloads_default_page_size"),
        card::code("menu_input_timeout_seconds"),
    ]);
    lines.extend(build_runtime_admin_interaction_block(&descriptor));
    lines.extend(build_runtime_admin_examples_block(&descriptor));
    lines.join("\n")
}

/// 构造 `/targets` 的说明。
fn build_targets_detail() -> String {
    let descriptor = targets_help_descriptor();
    vec![
        "targets".to_owned(),
        "用途：管理转存默认目标、按请求 chat 路由和目标别名。".to_owned(),
        "说明：targets 页支持直接 callback 操作，也支持按钮进入输入流。".to_owned(),
        card::DIVIDER.to_owned(),
    ]
    .into_iter()
    .chain(build_runtime_admin_usage_block(&descriptor))
    .chain(build_runtime_admin_interaction_block(&descriptor))
    .chain(build_runtime_admin_examples_block(&descriptor))
    .collect::<Vec<_>>()
    .join("\n")
}

/// 构造 `/acl` 的说明。
fn build_acl_detail() -> String {
    let descriptor = acl_help_descriptor();
    vec![
        "acl".to_owned(),
        "用途：管理访问控制规则。".to_owned(),
        "说明：bootstrap_admin_user_ids 仍由 config.json 提供，这里只管理数据库运行态规则。"
            .to_owned(),
        "说明：ACL 页支持直接 callback 开关，也支持按钮进入输入流。".to_owned(),
        card::DIVIDER.to_owned(),
    ]
    .into_iter()
    .chain(build_runtime_admin_usage_block(&descriptor))
    .chain(build_runtime_admin_interaction_block(&descriptor))
    .chain(build_runtime_admin_examples_block(&descriptor))
    .collect::<Vec<_>>()
    .join("\n")
}

/// 构造 `/billing` 的说明。
fn build_billing_detail() -> String {
    let descriptor = billing_help_descriptor();
    vec![
        "billing".to_owned(),
        "用途：管理积分计费和首页公告。".to_owned(),
        "说明：billing 页支持按钮直接调整数值，也支持按钮进入数值/公告输入流。".to_owned(),
        card::DIVIDER.to_owned(),
    ]
    .into_iter()
    .chain(build_runtime_admin_usage_block(&descriptor))
    .chain(build_runtime_admin_interaction_block(&descriptor))
    .chain(build_runtime_admin_examples_block(&descriptor))
    .collect::<Vec<_>>()
    .join("\n")
}

/// 构造 `/downloads` 的说明。
fn build_downloads_detail() -> String {
    vec![
        "downloads".to_owned(),
        "用途：查看任务列表、状态和真实下载进度。".to_owned(),
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        format!(
            "{} [filter] [limit] [page]",
            downloads_command(None, None, None, CommandStyle::Long)
        ),
        String::new(),
        "筛选参数：".to_owned(),
        card::code(
            "all | wait | dl | up | done | ok | fail | run | ready | pause | cancelling | cancel",
        ),
        String::new(),
        "示例：".to_owned(),
        downloads_command(None, None, None, CommandStyle::Long),
        downloads_command(None, Some(10), None, CommandStyle::Long),
        downloads_command(Some("dl"), None, None, CommandStyle::Long),
        downloads_command(Some("done"), Some(5), None, CommandStyle::Long),
        downloads_command(Some("done"), Some(5), Some(2), CommandStyle::Long),
    ]
    .join("\n")
}

/// 构造 `/job` 的说明。
fn build_job_detail() -> String {
    vec![
        "job".to_owned(),
        "用途：手动控制转存任务。".to_owned(),
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        format!(
            "{} <pause|resume|stop|status> <job_id>",
            command_root("job", CommandStyle::Long)
        ),
        String::new(),
        "动作：".to_owned(),
        format!(
            "{}：暂停任务，当前单次 TDLib 调用会在安全点停止。",
            card::code("pause")
        ),
        format!(
            "{}：唤醒 paused/pending/running 任务；若当前进程已有执行器则不会重复派发。",
            card::code("resume")
        ),
        format!(
            "{}：停止任务并释放文件引用，文件按删除队列延迟清理。",
            card::code("stop")
        ),
        format!(
            "{}：查看单任务详情、阶段计数和真实下载进度。",
            card::code("status")
        ),
        String::new(),
        "示例：".to_owned(),
        job_command("pause", 123, CommandStyle::Long),
        job_command("resume", 123, CommandStyle::Long),
        job_command("stop", 123, CommandStyle::Long),
        job_command("status", 123, CommandStyle::Long),
    ]
    .join("\n")
}

/// 构造 `/menu` 的说明。
fn build_menu_detail() -> String {
    [
        "menu".to_owned(),
        "用途：打开转存菜单。".to_owned(),
        "说明：bot token 模式使用 inline keyboard；手机号/OCR 用户号模式会自动降级为文本命令菜单。"
            .to_owned(),
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        menu_command(CommandStyle::Long),
        String::new(),
        "可做操作：".to_owned(),
        "转存：按钮引导输入源链接、目标和确认；需要手输时只展示长命令模板。".to_owned(),
        "查询：按钮引导输入源链接和目标；需要手输时只展示长命令模板。".to_owned(),
        "下载：覆盖全部筛选参数，并可进入分页列表。".to_owned(),
        "任务：从列表进入详情后可暂停、恢复、停止、刷新。".to_owned(),
        "配置：config / targets / acl / billing 都支持按钮 + 输入流混合操作。".to_owned(),
        "帮助：覆盖所有 help topic，可原地切换详情页。".to_owned(),
        String::new(),
        "管理输入：".to_owned(),
        "进入输入流后，会发送 ForceReply；回复参数即可，发送其他命令时命令优先。".to_owned(),
        "取消输入：".to_owned(),
        card::code("/cancel"),
    ]
    .join("\n")
}

#[cfg(test)]
mod config_detail_tests {
    use super::build_config_detail;

    #[test]
    fn test_build_config_detail_mentions_reset() {
        let text = build_config_detail();
        assert!(text.contains("/config reset"));
        assert!(text.contains("重置为启动配置中的默认值"));
    }
}
