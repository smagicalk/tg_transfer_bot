// `/menu` 文案渲染。
// 菜单文案保持短句，主要操作放到按钮里，降低日常输入成本。

use crate::tgbot::transfer::card;

use super::super::common::{lookup_command, transfer_command};
use super::callback::MenuPage;
use super::input::MenuInputKind;

/// 菜单首页摘要。
///
/// 首页只展示影响操作决策的数字，详细列表仍交给 `/downloads`、`/health`、`/cache`。
#[derive(Debug, Clone, Default)]
pub(super) struct MenuHomeSummary {
    /// 当前查看者是否 admin；普通用户首页不展示全局管理入口。
    pub(super) is_admin: bool,
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
    /// 首页公告文本；空表示不展示公告区块。
    pub(super) announcement_text: Option<String>,
}

/// 构造菜单页文本。
pub(super) fn build_menu_text(page: MenuPage) -> String {
    match page {
        MenuPage::Home => build_menu_home_text(&MenuHomeSummary::default()),
        MenuPage::TasksHub => tasks_hub_text(),
        MenuPage::AccountHub => account_hub_text(),
        MenuPage::AdminHub => admin_hub_text(),
        MenuPage::Transfer => transfer_text(),
        MenuPage::Downloads => downloads_text(),
        MenuPage::Jobs => jobs_text(),
        MenuPage::Lookup => lookup_text(),
        MenuPage::Config => config_text(),
        MenuPage::Targets => targets_text(),
        MenuPage::Acl => acl_text(),
        MenuPage::Billing => billing_text(),
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
        "查看最近任务、运行任务、失败任务，或直接进入任务控制。".to_owned(),
        card::section("命令"),
        card::command_line("最近任务", "/downloads"),
        card::command_line("运行任务", "/downloads run"),
        card::command_line("任务控制", "/job status <job_id>"),
        card::command_line("查询结果", "/lookup <link> <target_chat_id>"),
    ]
    .join("\n")
}

/// 账户 hub。
fn account_hub_text() -> String {
    [
        "账户".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("操作"),
        "查看余额、积分流水和帮助。".to_owned(),
        card::section("命令"),
        card::command_line("余额", "/balance"),
        card::command_line("积分流水", "/balance history"),
        card::command_line("帮助", "/help"),
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
        "运行配置、目标配置、访问控制、计费、健康、缓存和用户流水统一放在这里。".to_owned(),
        card::section("命令"),
        card::command_line("运行配置", "/config show"),
        card::command_line("目标配置", "/targets show"),
        card::command_line("访问控制", "/acl show"),
        card::command_line("计费配置", "/billing show"),
        card::command_line("运行健康", "/health"),
        card::command_line("文件缓存", "/cache"),
        card::command_line("用户流水", "/points history <user_id>"),
    ]
    .join("\n")
}

/// 目标配置页。
fn targets_text() -> String {
    [
        "目标配置".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("操作"),
        "管理默认目标、请求 chat 路由和目标别名。".to_owned(),
        card::section("命令"),
        card::command_line("查看", "/targets show"),
        card::command_line("默认目标", "/targets set-default <target_chat_id>"),
        card::command_line(
            "请求路由",
            "/targets set-route <request_chat_id> <target_chat_id>",
        ),
        card::command_line("目标别名", "/targets set-alias <alias> <target_chat_id>"),
    ]
    .join("\n")
}

/// 访问控制页。
fn acl_text() -> String {
    [
        "访问控制".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("操作"),
        "管理数据库里的管理员、普通用户入口、黑名单和聊天白名单。".to_owned(),
        card::section("命令"),
        card::command_line("查看", "/acl show"),
        card::command_line("开放私聊", "/acl set allow_all_private_users true"),
        card::command_line("添加管理员", "/acl add-admin <user_id>"),
        card::command_line("添加用户", "/acl add-allow-user <user_id>"),
        card::command_line("添加目标", "/acl add-allow-target <chat_id>"),
    ]
    .join("\n")
}

/// 计费配置页。
fn billing_text() -> String {
    [
        "计费配置".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("操作"),
        "管理积分扣费、新用户初始积分和首页公告。".to_owned(),
        card::section("命令"),
        card::command_line("查看", "/billing show"),
        card::command_line("计费开关", "/billing set enabled true"),
        card::command_line("基础扣分", "/billing set base_cost_points 1"),
        card::command_line("单项扣分", "/billing set item_cost_points 1"),
        card::command_line("首页公告", "/billing set announcement_text <text>"),
    ]
    .join("\n")
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
        if summary.announcement_text.is_some() {
            card::section("公告")
        } else {
            String::new()
        },
        summary.announcement_text.clone().unwrap_or_default(),
        card::section("操作"),
        if let Some(pending_input) = summary.pending_input {
            format!(
                "当前有未完成输入：{}，可点“继续输入”恢复提示。",
                card::code(pending_input)
            )
        } else {
            "当前没有未完成输入。".to_owned()
        },
        if summary.is_admin {
            "首页已经放了常用直达动作：开始转存、快速转存和管理入口。".to_owned()
        } else {
            "首页已经放了常用直达动作：开始转存、快速转存和账户入口。".to_owned()
        },
        "任务状态、最近任务、任务控制和查询结果已下沉到“任务”页，首页只保留高频入口。".to_owned(),
        card::section("命令"),
        card::command_line("转存", "/transfer"),
        card::command_line("下载列表", "/downloads run"),
        if summary.is_admin {
            card::command_line("运行健康", "/health")
        } else {
            card::command_line("余额", "/balance")
        },
        if summary.is_admin {
            card::command_line("文件缓存", "/cache")
        } else {
            card::command_line("积分流水", "/balance history")
        },
        card::command_line("帮助", "/help"),
    ]
    .join("\n")
}

/// 构造 reply_markup 不可用时的纯文本兜底菜单。
///
/// 正常配置会强制 bot 作为交互端；这里保留防御性兜底，避免异常配置或测试场景下只提示“点按钮”
/// 但实际看不到 inline keyboard。
pub(super) fn build_user_account_menu_text() -> String {
    [
        "转存菜单".to_owned(),
        format!(
            "状态：{}  模式：{}",
            card::code("ready"),
            card::code("text")
        ),
        card::DIVIDER.to_owned(),
        card::section("当前无法显示按钮"),
        "当前发送端没有启用 bot reply_markup。正常配置应保持 interaction_client = bot。".to_owned(),
        "下面命令可直接复制发送；修正配置后 /menu 会显示按钮菜单。".to_owned(),
        card::section("常用命令"),
        card::command_line("快速转存", "/transfer <link> <target_chat_id>"),
        card::command_line("快速转存", "/transfer <link>"),
        card::command_line("下载列表", "/downloads"),
        card::command_line("运行列表", "/downloads run"),
        card::command_line("积分流水", "/balance history"),
        card::command_line("运行健康", "/health"),
        card::command_line("文件缓存", "/cache"),
        card::command_line("任务详情", "/job status <job_id>"),
        card::command_line("暂停任务", "/job pause <job_id>"),
        card::command_line("恢复任务", "/job resume <job_id>"),
        card::command_line("停止任务", "/job stop <job_id>"),
        card::command_line("查询结果", "/lookup <link> <target_chat_id>"),
        card::command_line("帮助目录", "/help"),
    ]
    .join("\n")
}

/// 构造带步骤编号的输入提示文本。
///
/// ForceReply 本身不能再挂 inline keyboard，所以正文必须明确告诉用户当前步骤和取消方式。
pub(super) fn build_step_prompt_text(step: &str, title: &str, detail: &str) -> String {
    build_step_prompt_with_context("waiting-input", step, title, detail, None, None)
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
    lines.push(format!("取消：{}", card::code("/cancel")));
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
        lines.push(card::field("来源", source_link));
    }
    if let Some(target_chat_id) = target_chat_id {
        lines.push(card::field("目标", target_chat_id));
    }
    lines
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
        card::command_line("菜单", "/menu"),
    ]
    .join("\n")
}

/// 构造“输入已过期”这类终态恢复卡片。
///
/// 这类提示不应继续沿用 `waiting-input` 的步骤式文本，否则用户会误以为流程仍在等待输入。
pub(super) fn build_menu_recovery_text(title: &str, status: &str, detail: &str) -> String {
    [
        title.to_owned(),
        build_menu_state_line(status),
        card::DIVIDER.to_owned(),
        card::note(detail),
        card::command_line("返回菜单", "/menu"),
    ]
    .join("\n")
}

/// 构造统一的“目标不可用”终态卡片。
pub(super) fn build_menu_target_unavailable_text(detail: &str) -> String {
    build_menu_recovery_text("目标不可用", "target-unavailable", detail)
}

/// 构造统一的“没有未完成输入”空态卡片。
pub(super) fn build_menu_no_pending_input_text() -> String {
    build_menu_status_text(
        "没有未完成输入",
        "empty",
        "当前没有可继续的菜单输入，可重新开始转存或查询。",
    )
}

/// 构造菜单页的权限拒绝文案。
///
/// 按钮隐藏只能改善正常路径；callback payload 仍可能被手工构造，因此页面渲染层也必须返回
/// 明确的权限卡片，而不是继续渲染 admin-only 内容。
pub(super) fn build_permission_denied_menu_text(title: &str, detail: &str) -> String {
    [
        title.to_owned(),
        build_menu_state_line("denied"),
        card::DIVIDER.to_owned(),
        card::section("权限"),
        card::note(detail),
        card::command_line("返回菜单", "/menu"),
    ]
    .join("\n")
}

/// 构造确认页命令预览。
///
/// 按钮是主入口，但在执行前把最终命令展示出来，可以帮助用户快速复核“源链接 + 目标 chat”是否正确。
pub(super) fn build_confirm_command_preview(
    kind: MenuInputKind,
    source_link: &str,
    target_chat_id: i64,
) -> String {
    if kind.command_kind() == MenuInputKind::Transfer {
        transfer_command(
            source_link,
            target_chat_id,
            super::super::common::CommandStyle::Long,
        )
    } else {
        lookup_command(
            source_link,
            target_chat_id,
            super::super::common::CommandStyle::Long,
        )
    }
}

/// 转存页。
fn transfer_text() -> String {
    [
        "转存".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("最简单用法"),
        "点击“开始转存”，按提示发送源链接，然后选择目标群。".to_owned(),
        "点击“快速转存”，只回复源链接，目标 chat 使用配置默认值。".to_owned(),
        "也可以复制短命令或长命令模板后手动补齐。".to_owned(),
        card::section("命令"),
        card::command_line("打开向导", "/transfer"),
        card::command_line("快速转存", "/transfer <link>"),
        card::command_line("指定目标", "/transfer <link> <target_chat_id>"),
    ]
    .join("\n")
}

/// 下载页。
fn downloads_text() -> String {
    [
        "下载列表".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("筛选"),
        "直接点筛选按钮查看列表；列表页内可继续翻页、刷新和进入任务详情。".to_owned(),
        card::section("命令"),
        card::command_line("全部", "/downloads"),
        card::command_line("运行中", "/downloads run"),
        card::command_line("失败", "/downloads fail"),
    ]
    .join("\n")
}

/// 任务页。
fn jobs_text() -> String {
    [
        "任务控制".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("操作"),
        "先进入最近任务或运行任务，再点任务详情进行暂停、恢复、停止。".to_owned(),
        "知道 job_id 时可点“输入详情/暂停/恢复/停止”，按提示回复编号。".to_owned(),
        "命令模式仍可复制长命令模板手动输入。".to_owned(),
        card::section("命令"),
        card::command_line("详情", "/job status <job_id>"),
        card::command_line("暂停", "/job pause <job_id>"),
        card::command_line("恢复", "/job resume <job_id>"),
        card::command_line("停止", "/job stop <job_id>"),
    ]
    .join("\n")
}

/// 查询页。
fn lookup_text() -> String {
    [
        "查询".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("用途"),
        "点击“快速查询”，只回复源链接，目标 chat 使用配置默认值。".to_owned(),
        "点击“指定目标”，按提示输入源链接和目标 chat。".to_owned(),
        "命中后会返回结果链接或定位。".to_owned(),
        card::section("命令"),
        card::command_line("快速查询", "/lookup <link>"),
        card::command_line("指定目标", "/lookup <link> <target_chat_id>"),
    ]
    .join("\n")
}

/// 配置页。
fn config_text() -> String {
    [
        "运行配置".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("可调项"),
        card::code("job_concurrency"),
        card::code("file_delete_delay_minutes"),
        card::code("file_gc_interval_seconds"),
        card::code("progress_edit_interval_seconds"),
        card::code("downloads_default_page_size"),
        card::code("menu_input_timeout_seconds"),
        "常用项可点按钮小步调整；其他字段复制命令后手动修改。".to_owned(),
        card::section("命令"),
        card::command_line("查看配置", "/config show"),
        card::command_line("重置默认", "/config reset"),
        card::command_line("改并发", "/config set job_concurrency 4"),
        card::command_line("改分页", "/config set downloads_default_page_size 10"),
        card::command_line("改菜单超时", "/config set menu_input_timeout_seconds 900"),
    ]
    .join("\n")
}

/// 帮助页。
fn help_text() -> String {
    [
        "帮助".to_owned(),
        build_menu_state_line("ready"),
        card::DIVIDER.to_owned(),
        card::section("说明"),
        "点按钮可直接切换帮助页，命令仍可在下方查看。".to_owned(),
        card::section("命令"),
        card::command_line("帮助目录", "/help"),
        card::command_line("转存帮助", "/help transfer"),
        card::command_line("任务帮助", "/help job"),
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
        assert!(text.contains("开始转存"));
        assert!(text.contains("查询结果已下沉到“任务”页"));
        assert!(text.contains("活跃任务"));
    }

    #[test]
    fn test_build_hub_texts() {
        let tasks = build_menu_text(MenuPage::TasksHub);
        let account = build_menu_text(MenuPage::AccountHub);
        let admin = build_menu_text(MenuPage::AdminHub);

        assert!(tasks.contains("最近任务"));
        assert!(tasks.contains("/downloads run"));
        assert!(account.contains("积分流水"));
        assert!(account.contains("/balance"));
        assert!(admin.contains("运行配置"));
        assert!(admin.contains("/config show"));
        assert!(admin.contains("/targets show"));
        assert!(admin.contains("/acl show"));
        assert!(admin.contains("/billing show"));
    }

    #[test]
    fn test_build_runtime_admin_page_texts() {
        let targets = build_menu_text(MenuPage::Targets);
        let acl = build_menu_text(MenuPage::Acl);
        let billing = build_menu_text(MenuPage::Billing);

        assert!(targets.contains("目标配置"));
        assert!(targets.contains("/targets set-default <target_chat_id>"));
        assert!(targets.contains("/targets set-route <request_chat_id> <target_chat_id>"));
        assert!(acl.contains("访问控制"));
        assert!(acl.contains("/acl set allow_all_private_users true"));
        assert!(acl.contains("/acl add-admin <user_id>"));
        assert!(billing.contains("计费配置"));
        assert!(billing.contains("/billing set enabled true"));
        assert!(billing.contains("/billing set announcement_text <text>"));
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
            is_admin: true,
            active_jobs: 2,
            failed_jobs: 1,
            recoverable_jobs: 1,
            due_cache_files: 3,
            failed_cache_files: 4,
            recent_jobs: 5,
            pending_input: Some("快速转存"),
            announcement_text: Some("今晚 22:00 维护，期间转存可能延迟。".to_owned()),
        });

        assert!(text.contains("活跃任务：‹2›"));
        assert!(text.contains("失败任务：‹1›"));
        assert!(text.contains("待删缓存：‹3›"));
        assert!(text.contains("■ 公告"));
        assert!(text.contains("今晚 22:00 维护"));
        assert!(text.contains("当前有未完成输入：‹快速转存›"));
        assert!(text.contains("查询结果已下沉到“任务”页"));
    }

    #[test]
    fn test_build_tasks_hub_text() {
        let text = build_menu_text(MenuPage::TasksHub);

        assert!(text.contains("最近任务"));
        assert!(text.contains("/downloads run"));
        assert!(text.contains("/lookup <link> <target_chat_id>"));
    }

    // reply_markup 不可用时不能继续提示按钮；正文必须包含可复制命令作为降级入口。
    #[test]
    fn test_build_user_account_menu_text() {
        let text = build_user_account_menu_text();

        assert!(text.contains("当前无法显示按钮"));
        assert!(!text.contains("点按钮"));
        assert!(text.contains("‹/transfer <link> <target_chat_id>›"));
        assert!(text.contains("‹/downloads›"));
        assert!(text.contains("‹/job status <job_id>›"));
        assert!(text.contains("‹/help›"));
    }

    // 配置页应列出 `/config set` 实际支持的动态字段，避免菜单文案落后于命令实现。
    #[test]
    fn test_build_menu_text_config_contains_runtime_fields() {
        let text = build_menu_text(MenuPage::Config);

        assert!(text.contains("job_concurrency"));
        assert!(text.contains("file_delete_delay_minutes"));
        assert!(text.contains("file_gc_interval_seconds"));
        assert!(text.contains("progress_edit_interval_seconds"));
        assert!(text.contains("downloads_default_page_size"));
        assert!(text.contains("menu_input_timeout_seconds"));
    }

    // 分步提示应明确告诉用户当前是第几步，减少多消息流程里的迷路感。
    #[test]
    fn test_build_step_prompt_text() {
        let text = build_step_prompt_text("1/3", "源链接", "请回复链接。");

        assert!(text.contains("步骤：‹1/3›"));
        assert!(text.contains("‹/cancel›"));
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

    // 取消/过期这类终态提示应显示真实状态，而不是复用等待输入状态。
    #[test]
    fn test_build_menu_status_text() {
        let text = build_menu_status_text("已取消", "cancelled", "流程已结束。");

        assert!(text.contains("已取消"));
        assert!(text.contains("‹cancelled›"));
        assert!(text.contains("‹/menu›"));
        assert!(!text.contains("waiting-input"));
    }

    // 过期/恢复提示应是终态卡片，而不是继续显示等待输入状态。
    #[test]
    fn test_build_menu_recovery_text() {
        let text = build_menu_recovery_text("输入已过期", "expired", "请重新打开 /menu。");

        assert!(text.contains("输入已过期"));
        assert!(text.contains("‹expired›"));
        assert!(text.contains("‹/menu›"));
        assert!(!text.contains("waiting-input"));
    }

    // 目标不可用也应走统一终态卡片，不复用等待态提示。
    #[test]
    fn test_build_menu_target_unavailable_text() {
        let text = build_menu_target_unavailable_text("请选择其他目标。");

        assert!(text.contains("目标不可用"));
        assert!(text.contains("‹target-unavailable›"));
        assert!(text.contains("请选择其他目标。"));
    }

    // “没有未完成输入”应是空态卡片，不应临时在入口里手写。
    #[test]
    fn test_build_menu_no_pending_input_text() {
        let text = build_menu_no_pending_input_text();

        assert!(text.contains("没有未完成输入"));
        assert!(text.contains("‹empty›"));
        assert!(text.contains("当前没有可继续的菜单输入"));
    }

    // 确认页应展示最终命令预览，方便执行前快速复核。
    #[test]
    fn test_build_confirm_command_preview() {
        assert_eq!(
            build_confirm_command_preview(MenuInputKind::Transfer, "https://t.me/c/1/2", -100),
            "/transfer https://t.me/c/1/2 -100"
        );
        assert_eq!(
            build_confirm_command_preview(MenuInputKind::Lookup, "https://t.me/c/1/2", -100),
            "/lookup https://t.me/c/1/2 -100"
        );
    }
}
