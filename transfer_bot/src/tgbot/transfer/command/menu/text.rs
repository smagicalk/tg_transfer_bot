// `/menu` 文案渲染。
// 菜单文案保持短句，主要操作放到按钮里，降低日常输入成本。

use crate::tgbot::transfer::card;

use super::callback::MenuPage;

/// 菜单首页摘要。
///
/// 首页只展示影响操作决策的数字，详细列表仍交给 `/downloads`、`/health`、`/cache`。
#[derive(Debug, Clone, Copy, Default)]
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
}

/// 构造菜单页文本。
pub(super) fn build_menu_text(page: MenuPage) -> String {
    match page {
        MenuPage::Home => build_menu_home_text(&MenuHomeSummary::default()),
        MenuPage::Transfer => transfer_text(),
        MenuPage::Downloads => downloads_text(),
        MenuPage::Jobs => jobs_text(),
        MenuPage::Lookup => lookup_text(),
        MenuPage::Config => config_text(),
        MenuPage::Help => help_text(),
    }
}

/// 构造带运行摘要的菜单首页。
pub(super) fn build_menu_home_text(summary: &MenuHomeSummary) -> String {
    [
        "转存菜单".to_owned(),
        format!("状态：{}", card::code("ready")),
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
        if summary.is_admin {
            "首页已经放了常用直达动作：开始转存、快速转存、快速查询和管理入口。".to_owned()
        } else {
            "首页已经放了常用直达动作：开始转存、快速转存、快速查询和我的任务。".to_owned()
        },
        "运行任务、失败任务、已暂停也能直接点，不需要先进入下载页。".to_owned(),
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
        card::command_line("任务详情", "/job st <job_id>"),
        card::command_line("暂停任务", "/job p <job_id>"),
        card::command_line("恢复任务", "/job r <job_id>"),
        card::command_line("停止任务", "/job s <job_id>"),
        card::command_line("查询结果", "/lookup <link> <target_chat_id>"),
        card::command_line("帮助目录", "/help"),
    ]
    .join("\n")
}

/// 构造 ForceReply 输入提示文本。
pub(super) fn build_transfer_prompt_text(title: &str, detail: &str) -> String {
    build_step_prompt_text("输入", title, detail)
}

/// 构造带步骤编号的输入提示文本。
///
/// ForceReply 本身不能再挂 inline keyboard，所以正文必须明确告诉用户当前步骤和取消方式。
pub(super) fn build_step_prompt_text(step: &str, title: &str, detail: &str) -> String {
    [
        title.to_owned(),
        format!(
            "状态：{}  步骤：{}",
            card::code("waiting-input"),
            card::code(step)
        ),
        card::DIVIDER.to_owned(),
        card::note(detail),
        format!("取消：{}", card::code("/cancel")),
    ]
    .join("\n")
}

/// 构造菜单输入流程的状态提示文本。
///
/// 取消、过期、键盘清理这类短提示不应继续显示 `waiting-input`，
/// 因此单独传入状态值，让用户一眼能分辨当前流程已经结束还是仍在等待。
pub(super) fn build_menu_status_text(title: &str, status: &str, detail: &str) -> String {
    [
        title.to_owned(),
        format!("状态：{}", card::code(status)),
        card::DIVIDER.to_owned(),
        card::note(detail),
        card::command_line("菜单", "/menu"),
    ]
    .join("\n")
}

/// 构造菜单页的权限拒绝文案。
///
/// 按钮隐藏只能改善正常路径；callback payload 仍可能被手工构造，因此页面渲染层也必须返回
/// 明确的权限卡片，而不是继续渲染 admin-only 内容。
pub(super) fn build_permission_denied_menu_text(title: &str, detail: &str) -> String {
    [
        title.to_owned(),
        format!("状态：{}", card::code("denied")),
        card::DIVIDER.to_owned(),
        card::section("权限"),
        card::note(detail),
        card::command_line("返回菜单", "/menu"),
    ]
    .join("\n")
}

/// 转存页。
fn transfer_text() -> String {
    [
        "转存".to_owned(),
        format!("状态：{}", card::code("ready")),
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
        format!("状态：{}", card::code("ready")),
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
        format!("状态：{}", card::code("ready")),
        card::DIVIDER.to_owned(),
        card::section("操作"),
        "先进入最近任务或运行任务，再点任务详情进行暂停、恢复、停止。".to_owned(),
        "知道 job_id 时可点“输入详情/暂停/恢复/停止”，按提示回复编号。".to_owned(),
        "命令模式仍可复制短/长命令模板手动输入。".to_owned(),
        card::section("命令"),
        card::command_line("详情", "/job st <job_id>"),
        card::command_line("暂停", "/job p <job_id>"),
        card::command_line("恢复", "/job r <job_id>"),
        card::command_line("停止", "/job s <job_id>"),
    ]
    .join("\n")
}

/// 查询页。
fn lookup_text() -> String {
    [
        "查询".to_owned(),
        format!("状态：{}", card::code("ready")),
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
        format!("状态：{}", card::code("ready")),
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
        format!("状态：{}", card::code("ready")),
        card::DIVIDER.to_owned(),
        card::section("说明"),
        "点按钮可直接切换帮助页，也可以复制帮助命令。".to_owned(),
        card::section("命令"),
        card::command_line("帮助目录", "/h"),
        card::command_line("转存帮助", "/h transfer"),
        card::command_line("任务帮助", "/h job"),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    // 首页应突出“按钮操作”，避免继续让用户记复杂命令。
    #[test]
    fn test_build_menu_text_home() {
        let text = build_menu_text(MenuPage::Home);

        assert!(text.contains("转存菜单"));
        assert!(text.contains("开始转存"));
        assert!(text.contains("快速查询"));
        assert!(text.contains("活跃任务"));
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
        });

        assert!(text.contains("活跃任务：‹2›"));
        assert!(text.contains("失败任务：‹1›"));
        assert!(text.contains("待删缓存：‹3›"));
        assert!(text.contains("当前有未完成输入：‹快速转存›"));
    }

    // reply_markup 不可用时不能继续提示按钮；正文必须包含可复制命令作为降级入口。
    #[test]
    fn test_build_user_account_menu_text() {
        let text = build_user_account_menu_text();

        assert!(text.contains("当前无法显示按钮"));
        assert!(!text.contains("点按钮"));
        assert!(text.contains("‹/transfer <link> <target_chat_id>›"));
        assert!(text.contains("‹/downloads›"));
        assert!(text.contains("‹/job st <job_id>›"));
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

    // ForceReply 提示必须包含取消方式，避免输入流程卡住。
    #[test]
    fn test_build_transfer_prompt_text() {
        let text = build_transfer_prompt_text("源链接", "请回复链接。");

        assert!(text.contains("源链接"));
        assert!(text.contains("步骤：‹输入›"));
        assert!(text.contains("‹/cancel›"));
    }

    // 分步提示应明确告诉用户当前是第几步，减少多消息流程里的迷路感。
    #[test]
    fn test_build_step_prompt_text() {
        let text = build_step_prompt_text("1/3", "源链接", "请回复链接。");

        assert!(text.contains("步骤：‹1/3›"));
        assert!(text.contains("‹/cancel›"));
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
}
