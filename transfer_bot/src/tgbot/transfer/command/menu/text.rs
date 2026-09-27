//! `/menu` 交互菜单卡片文案渲染模块。
//!
//! 负责生成各个菜单页面的标准化展示卡片文本（包括首页摘要、各二级中心 Hub 文案、配置概览、分步输入提示、上下文回显等）。
//! 遵循轻量短句卡片设计，主要交互由内联按钮承接，降低用户理解成本与交互疲劳。

use crate::tgbot::transfer::card;

use super::super::common::build_runtime_admin_landing_text;
use super::super::config_cmd::{config_help_descriptor, config_intro_lines, config_summary_lines};
use super::super::downloads::downloads_help_intro_lines;
use super::super::job::job_help_intro_lines;
use super::super::targets::{
    targets_help_descriptor, targets_input_entry_lines, targets_intro_lines,
};
use super::callback::MenuPage;

/// 菜单首页展示的实时运行状态摘要数据结构。
///
/// 首页仅汇总直接影响用户操作决策的指标数字，详细的任务与文件清单则由各细分面板承载。
#[derive(Debug, Clone, Default)]
pub(super) struct MenuHomeSummary {
    /// 当前正在执行或排队中的活跃任务总数（Pending + Running）。
    pub(super) active_jobs: i64,
    /// 执行失败或部分成功的任务数。
    pub(super) failed_jobs: i64,
    /// 处于暂停状态或等待恢复执行的任务数。
    pub(super) recoverable_jobs: i64,
    /// 到期等待后台清理的本地临时缓存文件数量。
    pub(super) due_cache_files: i64,
    /// 删除或清理失败的缓存文件记录数。
    pub(super) failed_cache_files: i64,
    /// 首页直接展示或关联的最近任务数量。
    pub(super) recent_jobs: usize,
    /// 当前操作者未完成的挂起输入流程标题（例如 "快速转存"、"修改并发"），若无挂起则为 None。
    pub(super) pending_input: Option<&'static str>,
}

/// 根据菜单页面枚举构建对应的卡片正文文本。
///
/// # 参数说明
/// - `page`: 目标菜单页面枚举
///
/// # 返回值
/// 格式化后的 Markdown/普通文本字符串。
pub(super) fn build_menu_text(page: MenuPage) -> String {
    match page {
        // 主菜单首页（默认空摘要）
        MenuPage::Home => build_menu_home_text(&MenuHomeSummary::default()),
        // 任务中心二级 Hub
        MenuPage::TasksHub => tasks_hub_text(),
        // 管理中心二级 Hub
        MenuPage::AdminHub => admin_hub_text(),
        // 下载队列过滤页面
        MenuPage::Downloads => downloads_text(),
        // 任务控制列表页面
        MenuPage::Jobs => jobs_text(),
        // 查重搜索页面
        MenuPage::Lookup => lookup_text(),
        // 运行参数配置页面
        MenuPage::Config => config_text(),
        // 转存目标配置页面
        MenuPage::Targets => targets_text(),
        // 帮助中心页面
        MenuPage::Help => help_text(),
    }
}

/// 构建“任务中心”（TasksHub）卡片文本。
///
/// 说明常用状态、指定目标查询与最近任务快捷入口的使用方式。
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

/// 构建“管理中心”（AdminHub）卡片文本。
///
/// 说明运行配置、目标配置、健康监控与缓存管理入口的聚合使用方式。
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

/// 构建“目标配置”（Targets）页面的落地卡片文案。
///
/// 汇总目标管理说明、默认目标、别名设置、路由规则等引导内容。
fn targets_text() -> String {
    let descriptor = targets_help_descriptor();
    let mut intro_lines = targets_intro_lines();
    intro_lines.extend(targets_input_entry_lines());
    build_runtime_admin_landing_text("目标配置", intro_lines, &descriptor)
}

/// 构建携带运行摘要的完整菜单首页卡片文本。
///
/// 包括：
/// 1. 标题与就绪状态；
/// 2. 分区“运行摘要”：活跃任务、失败任务、待恢复、待删缓存等；
/// 3. 分区“操作”：未完成输入提示、直达动作引导说明。
///
/// # 参数说明
/// - `summary`: 首页运行摘要指标快照引用
///
/// # 返回值
/// 返回多行拼装后的卡片文本。
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
        // 动态展示当前是否存在未完成的会话输入草稿
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

/// 构造携带步骤编号与取消指引的输入提示卡片文本。
///
/// 在 Telegram 中，带有 ForceReply 的输入提示无法挂载内联键盘，
/// 因此正文中必须明确提示当前所处的步骤编号（例如 "1/2"）以及如何通过文字退出当前流程。
///
/// # 参数说明
/// - `step`: 当前步骤指示文本（如 "1/1", "1/2"）
/// - `title`: 卡片主标题
/// - `detail`: 针对该步骤的详细格式要求与操作说明
pub(super) fn build_step_prompt_text(step: &str, title: &str, detail: &str) -> String {
    build_step_prompt_with_context("waiting-input", step, title, detail, None, None)
}

/// 构造手动目标会话输入的专用提示卡片，并自动回显当前来源链接上下文。
///
/// # 参数说明
/// - `source_link`: 来源消息或频道链接
/// - `title`: 卡片标题
/// - `detail`: 详细说明
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

/// 构造带有上下文回显（来源链接/目标会话 ID）的多步输入提示卡片文本。
///
/// 在多步交互流程中，用户在后续步骤容易遗忘前面步骤输入的内容，
/// 通过显式回显已输入参数，能显著减少确认时的疑惑。
///
/// # 参数说明
/// - `status`: 状态标识（如 "waiting-input"）
/// - `step`: 步骤指示（如 "2/3"）
/// - `title`: 卡片主标题
/// - `detail`: 步骤要求说明
/// - `source_link`: 当前已确认的来源链接（可选）
/// - `target_chat_id`: 当前已确认的目标聊天 ID（可选）
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
    // 追加回显上下文
    lines.extend(build_menu_context_lines(source_link, target_chat_id));
    lines.push(card::note(detail));
    lines.push("取消：点击“取消”按钮，或回复“取消”结束当前流程。".to_owned());
    lines.join("\n")
}

/// 构造统一格式的菜单状态行（如：`状态：‹ready›`）。
pub(super) fn build_menu_state_line(status: &str) -> String {
    format!("状态：{}", card::code(status))
}

/// 构造带有步骤说明的菜单状态行（如：`状态：‹waiting-input›  步骤：‹1/2›`）。
pub(super) fn build_menu_step_state_line(status: &str, step: &str) -> String {
    format!("状态：{}  步骤：{}", card::code(status), card::code(step))
}

/// 构造包含状态、目标会话及步骤的组合状态行。
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

/// 构造输入流程的“当前上下文”回显区块多行文本。
///
/// 若来源与目标均未指定，则返回空行列表；
/// 若存在，则展示 "■ 当前上下文" 分区，并打印来源与目标 chat_id。
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

/// 将内部使用的消息源标识符格式化为人类更易读的卡片文本。
///
/// 例如将内部协议前缀 `bot-message:-100123:456` 解析转换为友好文本 `bot 可见消息 -100123/456`。
fn format_source_context(source_link: &str) -> String {
    if let Some(payload) = source_link.strip_prefix("bot-message:")
        && let Some((chat_id, message_id)) = payload.split_once(':')
    {
        return format!("bot 可见消息 {chat_id}/{message_id}");
    }
    source_link.to_owned()
}

/// 构造统一的菜单操作结果/状态通知卡片正文。
///
/// 用于操作完成、已取消、输入过期等短消息通知。
///
/// # 参数说明
/// - `title`: 通知标题
/// - `status`: 状态码（如 "cancelled", "expired", "success"）
/// - `detail`: 详细说明或后续引导文案
pub(super) fn build_menu_status_text(title: &str, status: &str, detail: &str) -> String {
    [
        title.to_owned(),
        build_menu_state_line(status),
        card::DIVIDER.to_owned(),
        card::note(detail),
    ]
    .join("\n")
}

/// 构造“输入已过期”或重置时的恢复提示卡片文本。
///
/// 显式标记终态（如 ‹expired›），防止用户误以为仍在等待当前输入。
pub(super) fn build_menu_recovery_text(title: &str, status: &str, detail: &str) -> String {
    build_menu_status_text(title, status, detail)
}

/// 构造统一的“没有未完成输入”空态卡片文本。
pub(super) fn build_menu_no_pending_input_text() -> String {
    build_menu_status_text(
        "没有未完成输入",
        "empty",
        "当前没有可继续的菜单输入，可重新开始转存或查询。",
    )
}

/// 构建“下载队列”（Downloads）页面的介绍卡片文案。
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

/// 构建“任务控制”（Jobs）页面的介绍卡片文案。
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

/// 构建“查重检索”（Lookup）页面的使用说明卡片文案。
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

/// 构建“运行配置”（Config）页面的概览与参数卡片文案。
fn config_text() -> String {
    let descriptor = config_help_descriptor();
    let mut intro_lines = config_intro_lines();
    intro_lines.extend(config_summary_lines());
    build_runtime_admin_landing_text("运行配置", intro_lines, &descriptor)
}

/// 构建“帮助中心”（Help）页面的导航说明卡片文案。
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

    /// 测试主菜单首页文案渲染：
    /// 首页应突出按钮化轻量操作，引导快速转存与指定目标，同时展现基础指标统计，避免用户记忆繁琐命令行。
    #[test]
    fn test_build_menu_text_home() {
        let text = build_menu_text(MenuPage::Home);

        assert!(text.contains("转存菜单"));
        assert!(text.contains("快速转存"));
        assert!(text.contains("指定目标"));
        assert!(text.contains("查询结果已下沉到“任务”页"));
        assert!(text.contains("活跃任务"));
    }

    /// 测试任务中心与管理中心二级 Hub 文案渲染：
    /// 明确指示操作由内联按钮承接，仅在必要时通过“查看命令”引导查看详细命令，默认不直接打印命令行。
    #[test]
    fn test_build_hub_texts() {
        let tasks = build_menu_text(MenuPage::TasksHub);
        let admin = build_menu_text(MenuPage::AdminHub);

        // 任务中心包含引导但默认不展开底层命令行
        assert!(tasks.contains("最近任务"));
        assert!(tasks.contains("查看命令"));
        assert!(!tasks.contains("/downloads run"));
        // 管理中心包含运行配置且不直接内嵌命令行
        assert!(admin.contains("运行配置"));
        assert!(admin.contains("查看命令"));
        assert!(!admin.contains("/config show"));
        assert!(!admin.contains("/targets show"));
    }

    /// 测试运行时管理落地页卡片文案（以目标配置为例）：
    /// 包含输入入口指示与操作别名引导，但不展示测试用的具体硬编码命令行。
    #[test]
    fn test_build_runtime_admin_page_texts() {
        let targets = build_menu_text(MenuPage::Targets);

        assert!(targets.contains("目标配置"));
        assert!(targets.contains("■ 输入入口"));
        assert!(targets.contains("设置默认目标：‹set-default›"));
        assert!(!targets.contains("/targets set-default 123456789"));
    }

    /// 测试就绪页面头部标准三行结构：标题行、就绪状态行（ready）、水平分割线。
    #[test]
    fn test_build_ready_page_header() {
        let lines = build_ready_page_header("示例页");

        assert_eq!(lines[0], "示例页");
        assert!(lines[1].contains("状态：‹ready›"));
        assert_eq!(lines[2], card::DIVIDER);
    }

    /// 测试通用页面辅助函数：命令分区标题与空数据说明。
    #[test]
    fn test_build_page_helpers() {
        assert_eq!(build_page_command_section(), "■ 命令");
        assert!(build_page_empty_note("暂无数据").contains("说明：暂无数据"));
    }

    /// 测试带有实际运行指标快照的首页文案渲染：
    /// 验证活跃任务数、失败任务数、待删缓存数、未完成草稿提示均能正确回显。
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

    /// 测试任务中心二级 Hub 文案不泄漏具体的旧命令语法。
    #[test]
    fn test_build_tasks_hub_text() {
        let text = build_menu_text(MenuPage::TasksHub);

        assert!(text.contains("最近任务"));
        assert!(text.contains("查看命令"));
        assert!(!text.contains("/downloads run"));
        assert!(!text.contains("/lookup <link> <target_chat_id>"));
    }

    /// 测试系统运行配置页文案：
    /// 必须列出所有支持热调的运行参数字段名称，但默认折叠详细的 CLI 命令行。
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

    /// 测试步骤指示提示卡片文案：
    /// 必须包含当前所处的步骤编号（如 1/3）以及文字取消指引。
    #[test]
    fn test_build_step_prompt_text() {
        let text = build_step_prompt_text("1/3", "源链接", "请回复链接。");

        assert!(text.contains("步骤：‹1/3›"));
        assert!(text.contains("回复“取消”结束当前流程"));
    }

    /// 测试带有上下文回显的步骤卡片文案：
    /// 来源链接与目标 ID 回显在“■ 当前上下文”分区中，减少多步操作迷路感。
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

    /// 测试手动目标输入提示必须完整保留并回显来源链接上下文。
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

    /// 测试已取消等终态卡片状态文案：
    /// 必须如实显示 cancelled 状态，而不应复用 waiting-input 状态。
    #[test]
    fn test_build_menu_status_text() {
        let text = build_menu_status_text("已取消", "cancelled", "流程已结束。");

        assert!(text.contains("已取消"));
        assert!(text.contains("‹cancelled›"));
        assert!(!text.contains("/menu"));
        assert!(!text.contains("waiting-input"));
    }

    /// 测试已过期恢复卡片状态文案：
    /// 必须如实展示 expired 终态，并引导返回主菜单。
    #[test]
    fn test_build_menu_recovery_text() {
        let text = build_menu_recovery_text("输入已过期", "expired", "请返回菜单重新开始。");

        assert!(text.contains("输入已过期"));
        assert!(text.contains("‹expired›"));
        assert!(!text.contains("/menu"));
        assert!(!text.contains("waiting-input"));
    }

    /// 测试“没有未完成输入”的空态提示卡片文案：
    /// 状态标记为 empty，并说明当前无待继续输入。
    #[test]
    fn test_build_menu_no_pending_input_text() {
        let text = build_menu_no_pending_input_text();

        assert!(text.contains("没有未完成输入"));
        assert!(text.contains("‹empty›"));
        assert!(text.contains("当前没有可继续的菜单输入"));
    }
}
