// `/menu` 文案渲染。
// 菜单文案保持短句，主要操作放到按钮里，降低日常输入成本。

use crate::tgbot::transfer::card;

use super::keyboard::MenuPage;

/// 构造菜单页文本。
pub(super) fn build_menu_text(page: MenuPage) -> String {
    match page {
        MenuPage::Home => home_text(),
        MenuPage::Transfer => transfer_text(),
        MenuPage::Downloads => downloads_text(),
        MenuPage::Jobs => jobs_text(),
        MenuPage::Lookup => lookup_text(),
        MenuPage::Config => config_text(),
        MenuPage::Help => help_text(),
    }
}

/// 构造用户号登录时的纯文本菜单。
///
/// TDLib `sendMessage.reply_markup` 标注为 “for bots only”，手机号/OCR 登录不会稳定显示按钮。
/// 因此这里必须把可操作命令直接写进正文，不能依赖复制按钮或 callback 按钮。
pub(super) fn build_user_account_menu_text() -> String {
    [
        "转存菜单".to_owned(),
        format!(
            "状态：{}  模式：{}",
            card::code("ready"),
            card::code("text")
        ),
        card::DIVIDER.to_owned(),
        card::section("当前登录不显示按钮"),
        "当前是手机号/OCR 用户号登录，Telegram 不会展示 inline keyboard。".to_owned(),
        "下面命令可直接复制发送；如果改用 bot token 登录，/m 会显示按钮菜单。".to_owned(),
        card::section("常用命令"),
        card::command_line("快速转存", "/t <link> <target_chat_id>"),
        card::command_line("默认目标", "/t <link>"),
        card::command_line("下载列表", "/d"),
        card::command_line("运行列表", "/d run"),
        card::command_line("任务详情", "/j st <job_id>"),
        card::command_line("暂停任务", "/j p <job_id>"),
        card::command_line("恢复任务", "/j r <job_id>"),
        card::command_line("停止任务", "/j s <job_id>"),
        card::command_line("查询结果", "/lk <link> <target_chat_id>"),
        card::command_line("帮助目录", "/h"),
    ]
    .join("\n")
}

/// 构造 ForceReply 输入提示文本。
pub(super) fn build_transfer_prompt_text(title: &str, detail: &str) -> String {
    [
        title.to_owned(),
        format!("状态：{}", card::code("waiting-input")),
        card::DIVIDER.to_owned(),
        card::note(detail),
        format!("取消：{}", card::code("/cancel")),
    ]
    .join("\n")
}

/// 菜单首页。
fn home_text() -> String {
    [
        "转存菜单".to_owned(),
        format!("状态：{}", card::code("ready")),
        card::DIVIDER.to_owned(),
        card::section("操作"),
        "点按钮即可进入对应功能。".to_owned(),
        "常用场景点“快速转存”，只需要回复一次源链接。".to_owned(),
        card::section("命令"),
        card::command_line("快速转存", "/t <link> <target_chat_id>"),
        card::command_line("下载列表", "/d run"),
        card::command_line("帮助", "/h"),
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
        "点击“快速转存”，只回复源链接，目标 chat 使用配置默认值。".to_owned(),
        "点击“指定目标”，按提示依次回复源链接和目标 chat。".to_owned(),
        "也可以复制命令模板后手动补齐。".to_owned(),
        card::section("命令"),
        card::command_line("默认目标", "/t <link>"),
        card::command_line("指定目标", "/t <link> <target_chat_id>"),
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
        card::command_line("全部", "/d"),
        card::command_line("运行中", "/d run"),
        card::command_line("失败", "/d fail"),
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
        "知道 job_id 时也可以复制模板手动输入。".to_owned(),
        card::section("命令"),
        card::command_line("详情", "/j st <job_id>"),
        card::command_line("暂停", "/j p <job_id>"),
        card::command_line("恢复", "/j r <job_id>"),
        card::command_line("停止", "/j s <job_id>"),
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
        card::command_line("默认目标", "/lk <link>"),
        card::command_line("指定目标", "/lk <link> <target_chat_id>"),
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
        "第一版菜单只复制命令，避免误触直接修改配置。".to_owned(),
        card::section("命令"),
        card::command_line("查看配置", "/cfg show"),
        card::command_line("改并发", "/cfg set job_concurrency 4"),
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
        assert!(text.contains("点按钮"));
    }

    // 用户号模式下不能继续提示按钮；正文必须包含可复制命令作为降级入口。
    #[test]
    fn test_build_user_account_menu_text() {
        let text = build_user_account_menu_text();

        assert!(text.contains("当前登录不显示按钮"));
        assert!(!text.contains("点按钮"));
        assert!(text.contains("‹/t <link> <target_chat_id>›"));
        assert!(text.contains("‹/d›"));
        assert!(text.contains("‹/j st <job_id>›"));
        assert!(text.contains("‹/h›"));
    }

    // ForceReply 提示必须包含取消方式，避免输入流程卡住。
    #[test]
    fn test_build_transfer_prompt_text() {
        let text = build_transfer_prompt_text("源链接", "请回复链接。");

        assert!(text.contains("源链接"));
        assert!(text.contains("‹/cancel›"));
    }
}
