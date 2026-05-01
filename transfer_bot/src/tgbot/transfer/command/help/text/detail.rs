// `/help <command>` 详情页文案。
// 每个命令的长说明集中在这里，后续调整参数说明时不影响命令入口。

use super::super::super::common::{
    CommandStyle, command_root, config_set_command, config_show_command, downloads_command,
    help_command as help_command_text, job_command, lookup_command, short_and_long,
    transfer_command,
};
use super::super::topic::normalize_help_topic;

/// 构造命令详细帮助。
pub(in crate::tgbot::transfer::command::help) fn build_help_detail_text(
    command_name: &str,
) -> anyhow::Result<String> {
    let command_name = normalize_help_topic(command_name)?;
    let text = match command_name {
        "help" => build_help_detail(),
        "transfer" => build_transfer_detail(),
        "lookup" => build_lookup_detail(),
        "config" => build_config_detail(),
        "downloads" => build_downloads_detail(),
        "job" => build_job_detail(),
        _ => anyhow::bail!("unknown help topic: {}", command_name),
    };
    Ok(text)
}

/// 构造 `/help` 自身的说明。
fn build_help_detail() -> String {
    [
        "*help*",
        "用途：查看命令帮助。",
        "━━━━━━━━━━━━",
        "命令：",
        &short_and_long(
            format!("{} [command]", help_command_text(None, CommandStyle::Short)),
            format!("{} [command]", help_command_text(None, CommandStyle::Long)),
        ),
        "",
        "示例：",
        &short_and_long(
            help_command_text(None, CommandStyle::Short),
            help_command_text(None, CommandStyle::Long),
        ),
        &short_and_long(
            help_command_text(Some("transfer"), CommandStyle::Short),
            help_command_text(Some("transfer"), CommandStyle::Long),
        ),
    ]
    .join("\n")
}

/// 构造 `/transfer` 的说明。
fn build_transfer_detail() -> String {
    [
        "*transfer*",
        "用途：转存单条消息或相册链接。",
        "说明：不传 target_chat_id 时按配置里的 target_map 解析。",
        "━━━━━━━━━━━━",
        "命令：",
        &short_and_long(
            transfer_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target_chat_id]"),
            transfer_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target_chat_id]"),
        ),
        "",
        "示例：",
        &short_and_long(
            "/t https://t.me/c/123/456".to_owned(),
            "/transfer https://t.me/c/123/456".to_owned(),
        ),
        &short_and_long(
            "/t https://t.me/c/123/456 -1001234567890".to_owned(),
            "/transfer https://t.me/c/123/456 -1001234567890".to_owned(),
        ),
    ]
    .join("\n")
}

/// 构造 `/lookup` 的说明。
fn build_lookup_detail() -> String {
    [
        "*lookup*",
        "用途：按源链接查询历史转存结果。",
        "说明：命中成功任务时会返回目标消息入口或定位信息。",
        "━━━━━━━━━━━━",
        "命令：",
        &short_and_long(
            lookup_command("<link>", 0, CommandStyle::Short).replace(" 0", " [target_chat_id]"),
            lookup_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target_chat_id]"),
        ),
        "",
        "示例：",
        &short_and_long(
            "/lk https://t.me/c/123/456".to_owned(),
            "/lookup https://t.me/c/123/456".to_owned(),
        ),
        &short_and_long(
            "/lk https://t.me/c/123/456 -1001234567890".to_owned(),
            "/lookup https://t.me/c/123/456 -1001234567890".to_owned(),
        ),
    ]
    .join("\n")
}

/// 构造 `/config` 的说明。
fn build_config_detail() -> String {
    [
        "*config*",
        "用途：查看或修改可动态生效的运行配置。",
        "━━━━━━━━━━━━",
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
        "",
        &short_and_long(
            config_show_command(CommandStyle::Short),
            config_show_command(CommandStyle::Long),
        ),
        "显示当前可调配置。",
        "",
        &short_and_long(
            format!(
                "{} set <key> <value>",
                command_root("config", CommandStyle::Short)
            ),
            format!(
                "{} set <key> <value>",
                command_root("config", CommandStyle::Long)
            ),
        ),
        "修改并持久化某个可调配置，修改后立即生效。",
        "",
        "可调字段：",
        "`job_concurrency`",
        "`file_delete_delay_minutes`",
        "`file_gc_interval_seconds`",
        "",
        "示例：",
        &short_and_long(
            config_show_command(CommandStyle::Short),
            config_show_command(CommandStyle::Long),
        ),
        &short_and_long(
            config_set_command("job_concurrency", 4, CommandStyle::Short),
            config_set_command("job_concurrency", 4, CommandStyle::Long),
        ),
        &short_and_long(
            config_set_command("file_delete_delay_minutes", 3, CommandStyle::Short),
            config_set_command("file_delete_delay_minutes", 3, CommandStyle::Long),
        ),
        &short_and_long(
            config_set_command("file_gc_interval_seconds", 30, CommandStyle::Short),
            config_set_command("file_gc_interval_seconds", 30, CommandStyle::Long),
        ),
    ]
    .join("\n")
}

/// 构造 `/downloads` 的说明。
fn build_downloads_detail() -> String {
    [
        "*downloads*",
        "用途：查看任务列表、状态和真实下载进度。",
        "━━━━━━━━━━━━",
        "命令：",
        &short_and_long(
            format!(
                "{} [filter] [limit] [page]",
                downloads_command(None, None, None, CommandStyle::Short)
            ),
            format!(
                "{} [filter] [limit] [page]",
                downloads_command(None, None, None, CommandStyle::Long)
            ),
        ),
        "",
        "筛选参数：",
        "`all | wait | dl | up | done | ok | fail | run | ready | pause | cancelling | cancel`",
        "",
        "示例：",
        &short_and_long(
            downloads_command(None, None, None, CommandStyle::Short),
            downloads_command(None, None, None, CommandStyle::Long),
        ),
        &short_and_long(
            downloads_command(None, Some(10), None, CommandStyle::Short),
            downloads_command(None, Some(10), None, CommandStyle::Long),
        ),
        &short_and_long(
            downloads_command(Some("dl"), None, None, CommandStyle::Short),
            downloads_command(Some("dl"), None, None, CommandStyle::Long),
        ),
        &short_and_long(
            downloads_command(Some("done"), Some(5), None, CommandStyle::Short),
            downloads_command(Some("done"), Some(5), None, CommandStyle::Long),
        ),
        &short_and_long(
            downloads_command(Some("done"), Some(5), Some(2), CommandStyle::Short),
            downloads_command(Some("done"), Some(5), Some(2), CommandStyle::Long),
        ),
    ]
    .join("\n")
}

/// 构造 `/job` 的说明。
fn build_job_detail() -> String {
    [
        "*job*",
        "用途：手动控制转存任务。",
        "━━━━━━━━━━━━",
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
        "",
        "动作：",
        "`pause | p`：暂停任务，当前单次 TDLib 调用会在安全点停止。",
        "`resume | r`：唤醒 paused/pending/running 任务；若当前进程已有执行器则不会重复派发。",
        "`stop | s`：停止任务并释放文件引用，文件按删除队列延迟清理。",
        "",
        "示例：",
        &short_and_long(
            job_command("p", 123, CommandStyle::Short),
            job_command("pause", 123, CommandStyle::Long),
        ),
        &short_and_long(
            job_command("r", 123, CommandStyle::Short),
            job_command("resume", 123, CommandStyle::Long),
        ),
        &short_and_long(
            job_command("s", 123, CommandStyle::Short),
            job_command("stop", 123, CommandStyle::Long),
        ),
    ]
    .join("\n")
}
