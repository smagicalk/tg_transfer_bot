// `/menu` 文案渲染。
// 菜单文案保持短句，主要操作放到按钮里，降低日常输入成本。

use crate::tgbot::transfer::card;

use super::super::common::build_runtime_admin_landing_text;
use super::super::config_cmd::{config_help_descriptor, config_intro_lines, config_summary_lines};
use super::super::downloads::downloads_help_intro_lines;
use super::super::job::job_help_intro_lines;
use super::super::targets::{
    targets_help_descriptor, targets_input_entry_lines, targets_intro_lines,
};
use super::callback::MenuPage;

/// 菜单首页摘要。
///
/// 首页只展示影响操作决策的数字，详细列表仍交给 `/downloads`、`/health`、`/cache`。
#[derive(Debug, Clone, Default)]
pub(super) struct MenuHomeSummary {
    /// 当前活跃任务数。
    pub(super) active_jobs: i64,
    /// 失败或部分成功任务数。
    pub(super) failed_jobs: i64,
    /// 等待恢复的任务数。
    pub(super) recoverable_jobs: i64,
    /// 到期待删除的缓存文件数。
    pub(super) due_cache_files: i64,
    /// 删除失败的缓存文件数。
    pub(super) failed_cache_files: i64,
    /// 首页实际展示的最近任务数。
    pub(super) recent_jobs: usize,
    /// 当前未完成输入标题。
    pub(super) pending_input: Option<&'static str>,
}

/// 构造菜单页文本。
pub(super) fn build_menu_text(page: MenuPage) -> String {
    match page {
        MenuPage::Home => build_menu_home_text(&MenuHomeSummary::default()),
        MenuPage::TasksHub => tasks_hub_text(),
        MenuPage::AdminHub => admin_hub_text(),
        MenuPage::Downloads => downloads_text(),
        MenuPage::Jobs => jobs_text(),
        MenuPage::Lookup => lookup_text(),
        MenuPage::Config => config_text(),
        MenuPage::Targets => targets_text(),
        MenuPage::Help => help_text(),
    }
}

/// 任务 hub。
fn tasks_hub_text() -> String {
    [
        "任务".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("操作"),
        "常用状态、指定目标查询和最近任务都可以直接点击进入。需要命令时点击“查看命令”。".to_owned(),
    ]
    .join("\n")
}

/// 管理 hub。
fn admin_hub_text() -> String {
    [
        "管理".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("操作"),
        "运行配置、目标配置、健康和缓存统一放在这里。需要命令时点击“查看命令”。".to_owned(),
    ]
    .join("\n")
}

/// 目标配置页。
fn targets_text() -> String {
    let descriptor = targets_help_descriptor();
    let mut intro_lines = targets_intro_lines();
    intro_lines.extend(targets_input_entry_lines());
    build_runtime_admin_landing_text("目标配置", intro_lines, &descriptor)
}

/// 构造带运行摘要的菜单首页。
pub(super) fn build_menu_home_text(summary: &MenuHomeSummary) -> String {
    [
        "转存菜单".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("运行摘要"),
        card::field_pair(
            "活跃任务",
            summary.active_jobs,
            "失败任务",
            summary.failed_jobs,
        ),
        card::field_pair(
            "待恢复",
            summary.recoverable_jobs,
            "最近任务",
            summary.recent_jobs,
        ),
        card::field_pair(
            "待删缓存",
            summary.due_cache_files,
            "删失败",
            summary.failed_cache_files,
        ),
        card::section("操作"),
        if let Some(pending_input) = summary.pending_input {
            format!(
                "当前有未完成输入：{}，可点“继续输入”恢复提示。",
                card::code(pending_input)
            )
        } else {
            "当前没有未完成输入。".to_owned()
        },
        "首页已经放了常用直达动作：快速转存、指定目标和管理入口。".to_owned(),
        "任务状态、最近任务、任务控制和查询结果已下沉到“任务”页，首页只保留高频入口。".to_owned(),
        "日常操作都可以点击按钮完成，需要命令时点击“查看命令”。".to_owned(),
    ]
    .join("\n")
}

/// 构造带步骤编号的输入提示文本。
///
/// ForceReply 本身不能再挂 inline keyboard，所以正文必须明确告诉用户当前步骤和取消方式。
pub(super) fn build_step_prompt_text(step: &str, title: &str, detail: &str) -> String {
    build_step_prompt_with_context("waiting-input", step, title, detail, None, None)
}

/// 构造手动目标输入提示，并始终回显当前来源。
pub(super) fn build_target_input_prompt_text(
    source_link: &str,
    title: &str,
    detail: &str,
) -> String {
    build_step_prompt_with_context(
        "waiting-input",
        "2/3",
        title,
        detail,
        Some(source_link),
        None,
    )
}

/// 构造带上下文摘要的步骤提示文本。
///
/// 在 `2/3` 和 `3/3` 这类多步流程中，用户容易忘记当前草稿对应的源链接或目标 chat。
/// 这里把关键上下文回显出来，减少“输到一半不知道自己在确认什么”的情况。
pub(super) fn build_step_prompt_with_context(
    status: &str,
    step: &str,
    title: &str,
    detail: &str,
    source_link: Option<&str>,
    target_chat_id: Option<i64>,
) -> String {
    let mut lines = vec![
        title.to_owned(),
        build_menu_step_state_line(status, step),
        card::DIVIDER.to_owned(),
    ];
    lines.extend(build_menu_context_lines(source_link, target_chat_id));
    lines.push(card::note(detail));
    lines.push("取消：点击“取消”按钮，或回复“取消”结束当前流程。".to_owned());
    lines.join("\n")
}

/// 构造菜单通用状态行。
pub(super) fn build_menu_state_line(status: &str) -> String {
    format!("状态：{}", card::code(status))
}

/// 构造菜单步骤状态行。
pub(super) fn build_menu_step_state_line(status: &str, step: &str) -> String {
    format!("状态：{}  步骤：{}", card::code(status), card::code(step))
}

/// 构造菜单“状态 + 目标 + 步骤”状态行。
pub(super) fn build_menu_target_step_state_line(
    status: &str,
    target_chat_id: i64,
    step: &str,
) -> String {
    format!(
        "{}  步骤：{}",
        card::status_target(status, target_chat_id),
        card::code(step)
    )
}

/// 构造菜单输入流程的上下文摘要。
///
/// 这里统一用“当前上下文”分区回显来源和目标，避免各阶段各写一套来源/目标展示。
pub(super) fn build_menu_context_lines(
    source_link: Option<&str>,
    target_chat_id: Option<i64>,
) -> Vec<String> {
    if source_link.is_none() && target_chat_id.is_none() {
        return Vec::new();
    }

    let mut lines = vec![card::section("当前上下文")];
    if let Some(source_link) = source_link {
        lines.push(card::field("来源", format_source_context(source_link)));
    }
    if let Some(target_chat_id) = target_chat_id {
        lines.push(card::field("目标", target_chat_id));
    }
    lines
}

/// 把内部 source 标识转换成更易读的卡片文案。
fn format_source_context(source_link: &str) -> String {
    if let Some(payload) = source_link.strip_prefix("bot-message:")
        && let Some((chat_id, message_id)) = payload.split_once(':')
    {
        return format!("bot 可见消息 {chat_id}/{message_id}");
    }
    source_link.to_owned()
}

/// 构造菜单输入流程的状态提示文本。
///
/// 取消、过期、键盘清理这类短提示不应继续显示 `waiting-input`，
/// 因此单独传入状态值，让用户一眼能分辨当前流程已经结束还是仍在等待。
pub(super) fn build_menu_status_text(title: &str, status: &str, detail: &str) -> String {
    [
        title.to_owned(),
        build_menu_state_line(status),
        card::DIVIDER.to_owned(),
        card::note(detail),
    ]
    .join("\n")
}

/// 构造“输入已过期”这类终态恢复卡片。
///
/// 这类提示不应继续沿用 `waiting-input` 的步骤式文本，否则用户会误以为流程仍在等待输入。
pub(super) fn build_menu_recovery_text(title: &str, status: &str, detail: &str) -> String {
    build_menu_status_text(title, status, detail)
}

/// 构造统一的“没有未完成输入”空态卡片。
pub(super) fn build_menu_no_pending_input_text() -> String {
    build_menu_status_text(
        "没有未完成输入",
        "empty",
        "当前没有可继续的菜单输入，可重新开始转存或查询。",
    )
}

/// 下载页。
fn downloads_text() -> String {
    let mut lines = vec![
        "下载列表".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("筛选"),
    ];
    lines.extend(downloads_help_intro_lines());
    lines.push("筛选、分页和任务详情都可以直接点击按钮，需要命令时点击“查看命令”。".to_owned());
    lines.join("\n")
}

/// 任务页。
fn jobs_text() -> String {
    let mut lines = vec![
        "任务控制".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("操作"),
    ];
    lines.extend(job_help_intro_lines());
    lines.push("任务详情、控制和刷新都可以直接点击按钮，需要命令时点击“查看命令”。".to_owned());
    lines.join("\n")
}

/// 查询页。
fn lookup_text() -> String {
    [
        "查询".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("用途"),
        "点击“快速查询”，只回复源链接，目标 chat 使用预先配置的目标。".to_owned(),
        "点击“指定目标”，按提示输入源链接和目标 chat。".to_owned(),
        "命中后会返回结果链接或定位。".to_owned(),
        "需要命令时点击“查看命令”。".to_owned(),
    ]
    .join("\n")
}

/// 配置页。
fn config_text() -> String {
    let descriptor = config_help_descriptor();
    let mut intro_lines = config_intro_lines();
    intro_lines.extend(config_summary_lines());
    build_runtime_admin_landing_text("运行配置", intro_lines, &descriptor)
}

/// 帮助页。
fn help_text() -> String {
    [
        "帮助".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("说明"),
        "点按钮可直接切换帮助主题；完整命令目录通过“查看命令”入口打开。".to_owned(),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tgbot::transfer::command::common::{
        build_page_command_section, build_page_empty_note, build_ready_page_header,
    };

    // 首页应突出“按钮操作”，避免继续让用户记复杂命令。
    #[test]
    fn test_build_menu_text_home() {
        let text = build_menu_text(MenuPage::Home);

        assert!(text.contains("转存菜单"));
        assert!(text.contains("快速转存"));
        assert!(text.contains("指定目标"));
        assert!(text.contains("查询结果已下沉到“任务”页"));
        assert!(text.contains("活跃任务"));
    }

    #[test]
    fn test_build_hub_texts() {
        let tasks = build_menu_text(MenuPage::TasksHub);
        let admin = build_menu_text(MenuPage::AdminHub);

        assert!(tasks.contains("最近任务"));
        assert!(tasks.contains("查看命令"));
        assert!(!tasks.contains("/downloads run"));
        assert!(admin.contains("运行配置"));
        assert!(admin.contains("查看命令"));
        assert!(!admin.contains("/config show"));
        assert!(!admin.contains("/targets show"));
    }

    #[test]
    fn test_build_runtime_admin_page_texts() {
        let targets = build_menu_text(MenuPage::Targets);

        assert!(targets.contains("目标配置"));
        assert!(targets.contains("■ 输入入口"));
        assert!(targets.contains("设置默认目标：‹set-default›"));
        assert!(!targets.contains("/targets set-default 123456789"));
    }

    #[test]
    fn test_build_ready_page_header() {
        let lines = build_ready_page_header("示例页");

        assert_eq!(lines[0], "示例页");
        assert!(lines[1].contains("状态：‹ready›"));
        assert_eq!(lines[2], card::DIVIDER);
    }

    #[test]
    fn test_build_page_helpers() {
        assert_eq!(build_page_command_section(), "■ 命令");
        assert!(build_page_empty_note("暂无数据").contains("说明：暂无数据"));
    }

    // 首页应能展示实时摘要，避免用户进入列表前不知道当前是否有异常。
    #[test]
    fn test_build_menu_home_text_with_summary() {
        let text = build_menu_home_text(&MenuHomeSummary {
            active_jobs: 2,
            failed_jobs: 1,
            recoverable_jobs: 1,
            due_cache_files: 3,
            failed_cache_files: 4,
            recent_jobs: 5,
            pending_input: Some("快速转存"),
        });

        assert!(text.contains("活跃任务：‹2›"));
        assert!(text.contains("失败任务：‹1›"));
        assert!(text.contains("待删缓存：‹3›"));
        assert!(text.contains("当前有未完成输入：‹快速转存›"));
        assert!(text.contains("查询结果已下沉到“任务”页"));
    }

    #[test]
    fn test_build_tasks_hub_text() {
        let text = build_menu_text(MenuPage::TasksHub);

        assert!(text.contains("最近任务"));
        assert!(text.contains("查看命令"));
        assert!(!text.contains("/downloads run"));
        assert!(!text.contains("/lookup <link> <target_chat_id>"));
    }

    // 配置页应列出可调字段，但默认不展开命令区。
    #[test]
    fn test_build_menu_text_config_contains_runtime_fields() {
        let text = build_menu_text(MenuPage::Config);

        assert!(text.contains("job_concurrency"));
        assert!(text.contains("file_delete_delay_minutes"));
        assert!(text.contains("file_gc_interval_seconds"));
        assert!(text.contains("progress_edit_interval_seconds"));
        assert!(text.contains("downloads_default_page_size"));
        assert!(text.contains("menu_input_timeout_seconds"));
        assert!(!text.contains("■ 命令"));
        assert!(!text.contains("/config set job_concurrency 4"));
    }

    // 分步提示应明确告诉用户当前是第几步，减少多消息流程里的迷路感。
    #[test]
    fn test_build_step_prompt_text() {
        let text = build_step_prompt_text("1/3", "源链接", "请回复链接。");

        assert!(text.contains("步骤：‹1/3›"));
        assert!(text.contains("回复“取消”结束当前流程"));
    }

    // 带上下文的步骤提示应把来源/目标回显出来，减少多步流程里的迷路感。
    #[test]
    fn test_build_step_prompt_with_context() {
        let text = build_step_prompt_with_context(
            "waiting-target",
            "2/3",
            "输入目标",
            "请回复目标 chat。",
            Some("https://t.me/c/1/2"),
            Some(-100),
        );

        assert!(text.contains("状态：‹waiting-target›  步骤：‹2/3›"));
        assert!(text.contains("■ 当前上下文"));
        assert!(text.contains("来源：‹https://t.me/c/1/2›"));
        assert!(text.contains("目标：‹-100›"));
    }

    // 手动目标输入的进入、恢复和重试提示都必须保留来源上下文。
    #[test]
    fn test_build_target_input_prompt_text_keeps_source_context() {
        let text = build_target_input_prompt_text(
            "https://t.me/c/1/2",
            "输入目标",
            "请回复目标 chat_id。",
        );

        assert!(text.contains("状态：‹waiting-input›"));
        assert!(text.contains("步骤：‹2/3›"));
        assert!(text.contains("来源：‹https://t.me/c/1/2›"));
        assert!(text.contains("请回复目标 chat_id。"));
    }

    // 取消/过期这类终态提示应显示真实状态，而不是复用等待输入状态。
    #[test]
    fn test_build_menu_status_text() {
        let text = build_menu_status_text("已取消", "cancelled", "流程已结束。");

        assert!(text.contains("已取消"));
        assert!(text.contains("‹cancelled›"));
        assert!(!text.contains("/menu"));
        assert!(!text.contains("waiting-input"));
    }

    // 过期/恢复提示应是终态卡片，而不是继续显示等待输入状态。
    #[test]
    fn test_build_menu_recovery_text() {
        let text = build_menu_recovery_text("输入已过期", "expired", "请返回菜单重新开始。");

        assert!(text.contains("输入已过期"));
        assert!(text.contains("‹expired›"));
        assert!(!text.contains("/menu"));
        assert!(!text.contains("waiting-input"));
    }

    // “没有未完成输入”应是空态卡片，不应临时在入口里手写。
    #[test]
    fn test_build_menu_no_pending_input_text() {
        let text = build_menu_no_pending_input_text();

        assert!(text.contains("没有未完成输入"));
        assert!(text.contains("‹empty›"));
        assert!(text.contains("当前没有可继续的菜单输入"));
    }
}
