// `/menu` callback payload 协议。
// payload 保持短格式，避免超过 Telegram callback_data 长度限制。

use super::input::{AdminInputAction, MenuJobAction};

/// 菜单按钮回调前缀。
const MENU_CALLBACK_PREFIX: &str = "m:";

/// 菜单页。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenuPage {
    Home,
    TasksHub,
    AdminHub,
    Downloads,
    Jobs,
    Lookup,
    Config,
    Targets,
    Help,
}

impl MenuPage {
    /// 页面标题，用于 callback 提示和文本标题。
    pub(super) fn title(self) -> &'static str {
        match self {
            Self::Home => "菜单",
            Self::TasksHub => "任务",
            Self::AdminHub => "管理",
            Self::Downloads => "下载",
            Self::Jobs => "任务",
            Self::Lookup => "查询",
            Self::Config => "配置",
            Self::Targets => "目标",
            Self::Help => "帮助",
        }
    }

    /// 页面短编码，写进 callback payload。
    fn code(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::TasksHub => "th",
            Self::AdminHub => "mh",
            Self::Downloads => "d",
            Self::Jobs => "j",
            Self::Lookup => "lk",
            Self::Config => "cfg",
            Self::Targets => "tg",
            Self::Help => "h",
        }
    }

    /// 从 callback 短编码解析页面。
    fn parse(code: &str) -> Option<Self> {
        match code {
            "home" => Some(Self::Home),
            "th" => Some(Self::TasksHub),
            "mh" => Some(Self::AdminHub),
            "d" => Some(Self::Downloads),
            "j" => Some(Self::Jobs),
            "lk" => Some(Self::Lookup),
            "cfg" => Some(Self::Config),
            "tg" => Some(Self::Targets),
            "h" => Some(Self::Help),
            _ => None,
        }
    }
}

/// 菜单 callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenuRequestAction {
    Page(MenuPage),
    NewTransfer,
    QuickTransferDefault,
    NewLookup,
    QuickLookupDefault,
    TargetDefault,
    TargetManual,
    TargetRequestChat,
    TargetAlias(i64),
    TargetConfirm,
    TargetBack,
    TargetSourceBack,
    JobIdInput(MenuJobAction),
    AdminInput(AdminInputAction),
    ContinueInput,
    CancelInput,
}

/// 判断 callback payload 是否属于 `/menu`。
pub(super) fn is_menu_callback_data(data: &str) -> bool {
    data.starts_with(MENU_CALLBACK_PREFIX)
}

/// 解析菜单 callback payload。
pub(super) fn parse_menu_callback_data(data: &str) -> Option<MenuRequestAction> {
    let payload = data.strip_prefix(MENU_CALLBACK_PREFIX)?;
    match payload {
        "new" | "t" => Some(MenuRequestAction::NewTransfer),
        "qtd" => Some(MenuRequestAction::QuickTransferDefault),
        "qlk" => Some(MenuRequestAction::NewLookup),
        "qld" => Some(MenuRequestAction::QuickLookupDefault),
        "td" => Some(MenuRequestAction::TargetDefault),
        "tm" => Some(MenuRequestAction::TargetManual),
        "tp" => Some(MenuRequestAction::TargetRequestChat),
        "tr" => Some(MenuRequestAction::TargetConfirm),
        "tb" => Some(MenuRequestAction::TargetBack),
        "ts" => Some(MenuRequestAction::TargetSourceBack),
        "jst" => Some(MenuRequestAction::JobIdInput(MenuJobAction::Status)),
        "jp" => Some(MenuRequestAction::JobIdInput(MenuJobAction::Pause)),
        "jr" => Some(MenuRequestAction::JobIdInput(MenuJobAction::Resume)),
        "js" => Some(MenuRequestAction::JobIdInput(MenuJobAction::Stop)),
        admin if admin.starts_with("ai:") => admin
            .strip_prefix("ai:")
            .and_then(AdminInputAction::parse)
            .map(MenuRequestAction::AdminInput),
        "ci" => Some(MenuRequestAction::ContinueInput),
        "cx" => Some(MenuRequestAction::CancelInput),
        alias if alias.starts_with("ta:") => alias
            .strip_prefix("ta:")
            .and_then(|chat_id| chat_id.parse::<i64>().ok())
            .map(MenuRequestAction::TargetAlias),
        page_code => MenuPage::parse(page_code).map(MenuRequestAction::Page),
    }
}

/// 页面导航 callback payload。
pub(super) fn menu_page_callback_data(page: MenuPage) -> String {
    menu_callback_data(page.code())
}

/// 新建转存 callback payload。
pub(super) fn new_transfer_callback_data() -> String {
    menu_callback_data("new")
}

/// 使用默认目标快速转存 callback payload。
pub(super) fn quick_transfer_default_callback_data() -> String {
    menu_callback_data("qtd")
}

/// 指定目标查询 callback payload。
pub(super) fn new_lookup_callback_data() -> String {
    menu_callback_data("qlk")
}

/// 使用默认目标快速查询 callback payload。
pub(super) fn quick_lookup_default_callback_data() -> String {
    menu_callback_data("qld")
}

/// 使用默认目标的 callback payload。
pub(super) fn target_default_callback_data() -> String {
    menu_callback_data("td")
}

/// 进入手动输入目标的 callback payload。
pub(super) fn target_manual_callback_data() -> String {
    menu_callback_data("tm")
}

/// 打开 Telegram 原生目标聊天选择器的 callback payload。
pub(super) fn target_request_chat_callback_data() -> String {
    menu_callback_data("tp")
}

/// 使用某个目标 chat 的 callback payload。
pub(super) fn target_alias_callback_data(chat_id: i64) -> String {
    menu_callback_data(&format!("ta:{chat_id}"))
}

/// 确认执行当前草稿的 callback payload。
pub(super) fn target_confirm_callback_data() -> String {
    menu_callback_data("tr")
}

/// 返回目标选择页的 callback payload。
pub(super) fn target_back_callback_data() -> String {
    menu_callback_data("tb")
}

/// 返回来源输入的 callback payload。
pub(super) fn target_source_back_callback_data() -> String {
    menu_callback_data("ts")
}

/// 继续当前输入草稿的 callback payload。
pub(super) fn continue_input_callback_data() -> String {
    menu_callback_data("ci")
}

/// 取消当前输入草稿的 callback payload。
pub(super) fn cancel_input_callback_data() -> String {
    menu_callback_data("cx")
}

/// 生成菜单 callback payload。
fn menu_callback_data(action: &str) -> String {
    format!("{}{}", MENU_CALLBACK_PREFIX, action)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 菜单 callback 数据应能区分页面切换和输入流动作。
    #[test]
    fn test_parse_menu_callback_data() {
        assert_eq!(
            parse_menu_callback_data("m:home"),
            Some(MenuRequestAction::Page(MenuPage::Home))
        );
        assert_eq!(
            parse_menu_callback_data("m:th"),
            Some(MenuRequestAction::Page(MenuPage::TasksHub))
        );
        assert_eq!(
            parse_menu_callback_data("m:mh"),
            Some(MenuRequestAction::Page(MenuPage::AdminHub))
        );
        assert_eq!(
            parse_menu_callback_data("m:t"),
            Some(MenuRequestAction::NewTransfer)
        );
        assert_eq!(
            parse_menu_callback_data("m:tg"),
            Some(MenuRequestAction::Page(MenuPage::Targets))
        );
        assert_eq!(
            parse_menu_callback_data("m:new"),
            Some(MenuRequestAction::NewTransfer)
        );
        assert_eq!(
            parse_menu_callback_data("m:qtd"),
            Some(MenuRequestAction::QuickTransferDefault)
        );
        assert_eq!(
            parse_menu_callback_data("m:qlk"),
            Some(MenuRequestAction::NewLookup)
        );
        assert_eq!(
            parse_menu_callback_data("m:qld"),
            Some(MenuRequestAction::QuickLookupDefault)
        );
        assert_eq!(
            parse_menu_callback_data("m:td"),
            Some(MenuRequestAction::TargetDefault)
        );
        assert_eq!(
            parse_menu_callback_data("m:tm"),
            Some(MenuRequestAction::TargetManual)
        );
        assert_eq!(
            parse_menu_callback_data("m:tp"),
            Some(MenuRequestAction::TargetRequestChat)
        );
        assert_eq!(
            parse_menu_callback_data("m:ta:-100"),
            Some(MenuRequestAction::TargetAlias(-100))
        );
        assert_eq!(
            parse_menu_callback_data("m:tr"),
            Some(MenuRequestAction::TargetConfirm)
        );
        assert_eq!(
            parse_menu_callback_data("m:tb"),
            Some(MenuRequestAction::TargetBack)
        );
        assert_eq!(
            parse_menu_callback_data("m:ts"),
            Some(MenuRequestAction::TargetSourceBack)
        );
        assert_eq!(
            parse_menu_callback_data("m:jst"),
            Some(MenuRequestAction::JobIdInput(MenuJobAction::Status))
        );
        assert_eq!(
            parse_menu_callback_data("m:jp"),
            Some(MenuRequestAction::JobIdInput(MenuJobAction::Pause))
        );
        assert_eq!(
            parse_menu_callback_data("m:jr"),
            Some(MenuRequestAction::JobIdInput(MenuJobAction::Resume))
        );
        assert_eq!(
            parse_menu_callback_data("m:js"),
            Some(MenuRequestAction::JobIdInput(MenuJobAction::Stop))
        );
        assert_eq!(
            parse_menu_callback_data("m:ci"),
            Some(MenuRequestAction::ContinueInput)
        );
        assert_eq!(
            parse_menu_callback_data("m:cx"),
            Some(MenuRequestAction::CancelInput)
        );
        assert_eq!(parse_menu_callback_data("x:new"), None);
    }
}
