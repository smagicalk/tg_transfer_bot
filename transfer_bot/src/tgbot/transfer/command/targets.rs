// `/targets` 命令：
// - 管理默认目标和目标别名
// - 写入数据库后立即刷新运行时 targets 配置

use crate::config::TargetsConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

use super::common::{
    CommandStyle, RuntimeAdminHelpDescriptor, RuntimeAdminUsageItem, build_page_empty_note,
    build_refresh_return_menu_row, build_runtime_admin_back_menu_row,
    build_runtime_admin_detail_text, build_runtime_admin_help_menu_row,
    build_runtime_admin_page_intro, build_runtime_admin_section_block, cleared_action_title,
    command_root, deleted_action_title, edit_runtime_admin_interaction_card_or_error,
    reset_action_title, send_runtime_admin_callback_error, targets_show_command,
    updated_action_title,
};
use super::menu::build_menu_targets_callback_data;
/// 目标页标题。
const TARGETS_PAGE_TITLE: &str = "目标配置";
/// 目标页简要说明。
const TARGETS_PAGE_DETAIL: &str =
    "默认目标未显式设置时会回落到当前请求私聊；别名先在分页列表点编号，再进入详情操作。";
/// 别名分页大小。
const TARGETS_LIST_PAGE_SIZE: usize = 5;
/// 别名搜索关键字最大长度。
///
/// 搜索关键字会被放进 callback payload 里用于分页；限制长度可以避免按钮数据过长。
const TARGETS_ALIAS_SEARCH_QUERY_MAX_CHARS: usize = 32;

/// 别名动作返回的列表上下文。
#[derive(Debug, Clone, PartialEq, Eq)]
enum AliasListContext {
    All { page: u64 },
    Search { query: String, page: u64 },
}

impl AliasListContext {
    fn page(&self) -> u64 {
        match self {
            Self::All { page } | Self::Search { page, .. } => *page,
        }
    }
}

/// `/targets` callback 前缀。
const TARGETS_CALLBACK_PREFIX: &str = "tcfg:";

/// `/targets` callback 动作。
#[derive(Debug, Clone, PartialEq, Eq)]
enum TargetsCallbackAction {
    Refresh,
    Reset,
    ConfirmReset,
    ClearDefault,
    ViewDefault,
    InputSetDefault,
    InputSetAlias,
    InputSearchAlias,
    ViewAliases(AliasListContext),
    ViewAlias {
        alias: String,
        context: AliasListContext,
    },
    EditAlias(String),
    DeleteAlias {
        alias: String,
        context: AliasListContext,
    },
    UseAliasAsDefault {
        alias: String,
        context: AliasListContext,
    },
}

impl TargetsCallbackAction {
    fn started_tip(&self) -> &'static str {
        match self {
            Self::Refresh => "正在刷新目标配置",
            Self::Reset => "正在重置目标配置",
            Self::ConfirmReset => "请确认重置",
            Self::ClearDefault => "正在恢复私聊默认目标",
            Self::ViewDefault => "正在打开默认目标",
            Self::InputSetDefault
            | Self::InputSetAlias
            | Self::InputSearchAlias
            | Self::EditAlias(_) => "请回复参数",
            Self::ViewAliases(AliasListContext::All { .. }) => "正在打开列表",
            Self::ViewAliases(AliasListContext::Search { .. }) => "正在搜索别名",
            Self::ViewAlias { .. } | Self::DeleteAlias { .. } | Self::UseAliasAsDefault { .. } => {
                "正在处理目标项"
            }
        }
    }
}

/// `/targets` 单步输入动作规格。
///
/// callback 按钮、help 示例、ForceReply 文案和输入解析都从这里读取，避免新增目标动作时漏改多处。
#[derive(Debug, Clone)]
pub(in crate::tgbot::transfer::command) struct TargetsInputSpec {
    pub action: super::menu::AdminInputAction,
    callback_action: TargetsCallbackAction,
    pub input_title: &'static str,
    pub input_detail: &'static str,
    pub input_placeholder: &'static str,
    pub subcommand: &'static str,
    pub expected_parts: usize,
    pub example_command: &'static str,
    pub interaction_detail: &'static str,
}

/// `/targets` 当前支持的全部输入式动作。
pub(in crate::tgbot::transfer::command) const TARGETS_INPUT_SPECS: &[TargetsInputSpec] = &[
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsSetDefault,
        callback_action: TargetsCallbackAction::InputSetDefault,
        input_title: "设置默认目标",
        input_detail: "请选择目标群组或频道，也可直接输入 chat_id；输入“取消”可退出。",
        input_placeholder: "选择聊天或输入 target_chat_id",
        subcommand: "set-default",
        expected_parts: 1,
        example_command: "/targets set-default 123456789",
        interaction_detail: "默认目标详情页：点“选择聊天”后使用原生选择器，也可直接输入 target_chat_id。",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsSetAlias,
        callback_action: TargetsCallbackAction::InputSetAlias,
        input_title: "设置目标别名",
        input_detail: "命令模式可回复 alias 和目标 chat_id，例如 archive 123456789；交互模式会先问 alias，再选择聊天或输入 target。",
        input_placeholder: "输入 alias target_chat_id",
        subcommand: "set-alias",
        expected_parts: 2,
        example_command: "/targets set-alias archive 123456789",
        interaction_detail: "别名列表页：点“新增别名”后，先回复 alias，再选择聊天或输入 target_chat_id。",
    },
];

/// 根据菜单输入动作反查 `/targets` 输入规格。
pub(in crate::tgbot::transfer::command) fn targets_input_spec_for_admin_action(
    action: super::menu::AdminInputAction,
) -> Option<&'static TargetsInputSpec> {
    TARGETS_INPUT_SPECS
        .iter()
        .find(|spec| spec.action == action)
}

/// 根据 callback 动作反查 `/targets` 输入规格。
fn targets_input_spec_for_callback_action(
    action: &TargetsCallbackAction,
) -> Option<&'static TargetsInputSpec> {
    TARGETS_INPUT_SPECS
        .iter()
        .find(|spec| spec.callback_action == *action)
}

/// 在指定上下文上执行 `/targets` 文本命令。
pub async fn targets_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let reply = match text.get(1).copied() {
        None | Some("show") => format_targets_text_on(app, TARGETS_PAGE_TITLE),
        Some("reset") => reset_targets_config_to_default_on(app).await?,
        Some("set-default") => {
            let target_chat_id =
                parse_i64_arg(&text, 2, "usage: /targets set-default <target_chat_id>")?;
            update_targets_with_on(app, &updated_action_title("默认目标"), |config| {
                config.default_chat_id = target_chat_id;
            })
            .await?
        }
        Some("set-alias") => {
            let alias = parse_alias_arg(
                &text,
                2,
                "usage: /targets set-alias <alias> <target_chat_id>",
            )?;
            let target_chat_id = parse_i64_arg(
                &text,
                3,
                "usage: /targets set-alias <alias> <target_chat_id>",
            )?;
            update_targets_with_on(app, &updated_action_title("目标别名"), |config| {
                config.aliases.insert(alias.clone(), target_chat_id);
            })
            .await?
        }
        Some("del-alias") => {
            let alias = parse_alias_arg(&text, 2, "usage: /targets del-alias <alias>")?;
            update_targets_with_on(app, &deleted_action_title("目标别名"), |config| {
                config.aliases.remove(&alias);
            })
            .await?
        }
        Some(other) => anyhow::bail!("unknown targets subcommand: {other}"),
    };

    send::ReplyPanel::card(reply)
        .rows(build_targets_buttons())
        .send(request_chat_id, client_id)
        .await
}

/// `targets` 管理页的最小帮助 descriptor。
pub(in crate::tgbot::transfer::command) fn targets_help_descriptor() -> RuntimeAdminHelpDescriptor {
    RuntimeAdminHelpDescriptor {
        purpose: "管理转存默认目标和目标别名。",
        summary: "管理默认目标和目标别名；支持列表入口和输入式设置。",
        synopsis: format!(
            "{} [show|reset|set-default|set-alias|del-alias]",
            command_root("targets", CommandStyle::Long)
        ),
        usage_items: vec![
            RuntimeAdminUsageItem {
                command: targets_show_command(CommandStyle::Long),
                detail: "显示当前目标配置。".to_owned(),
            },
            RuntimeAdminUsageItem {
                command: "/targets reset".to_owned(),
                detail: "把目标配置重置为启动配置中的默认值，并立即生效。".to_owned(),
            },
        ],
        interaction_items: targets_interaction_items(),
        example_commands: targets_example_commands(),
    }
}

/// 构造目标页共用的“输入入口”摘要区块。
///
/// target 页面和 help 详情页都直接消费这份摘要，减少别名入口描述漂移。
pub(in crate::tgbot::transfer::command) fn targets_input_entry_lines() -> Vec<String> {
    build_runtime_admin_section_block(
        "输入入口",
        TARGETS_INPUT_SPECS
            .iter()
            .map(|spec| card::field(spec.input_title, spec.subcommand)),
    )
}

/// `targets` 页在菜单和帮助详情里共用的开场说明。
pub(in crate::tgbot::transfer::command) fn targets_intro_lines() -> Vec<String> {
    vec![
        "默认目标未显式设置时，会直接回落到当前请求私聊。".to_owned(),
        "目标入口拆成默认目标和别名列表两类。".to_owned(),
        "现有项先在分页列表里点编号进入详情，再修改、设默认或删除。".to_owned(),
    ]
}

/// `/targets` 帮助页和卡片共用的交互说明。
fn targets_interaction_items() -> Vec<String> {
    let mut items = vec![
        "刷新可直接执行；重置全部需要二次确认；恢复私聊默认只清除显式默认目标。".to_owned(),
        "默认目标：进入详情页后可手动设置，或恢复为当前私聊默认。".to_owned(),
        "现有别名：进入分页列表后，先点编号进入详情，再修改、设默认或删除。".to_owned(),
    ];
    items.extend(
        TARGETS_INPUT_SPECS
            .iter()
            .map(|spec| spec.interaction_detail.to_owned()),
    );
    items
}

/// `/targets` 帮助页和卡片共用的示例命令。
fn targets_example_commands() -> Vec<String> {
    let mut commands = vec![
        targets_show_command(CommandStyle::Long),
        "/targets reset".to_owned(),
    ];
    commands.extend(
        TARGETS_INPUT_SPECS
            .iter()
            .map(|spec| spec.example_command.to_owned()),
    );
    commands
}

/// 判断 callback payload 是否属于 `/targets`。
pub(super) fn is_targets_callback_data(data: &str) -> bool {
    data.starts_with(TARGETS_CALLBACK_PREFIX)
}

/// 构造“默认目标详情”按钮数据，供帮助页等外层导航入口复用。
pub(in crate::tgbot::transfer::command) fn build_targets_default_detail_button_data() -> String {
    build_targets_callback_data(TargetsCallbackAction::ViewDefault)
}

/// 构造“别名列表分页”按钮数据，供帮助页等外层导航入口复用。
pub(in crate::tgbot::transfer::command) fn build_targets_aliases_page_button_data(
    page: u64,
) -> String {
    build_targets_callback_data(TargetsCallbackAction::ViewAliases(AliasListContext::All {
        page,
    }))
}

/// 构造 `/help targets` 的入口按钮行。
///
/// 目标配置页入口比较稳定，直接由 targets 模块自己输出，避免 help 层重复维护。
pub(in crate::tgbot::transfer::command) fn build_targets_help_entry_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![vec![
        send::build_callback_button(
            "打开目标页",
            &build_menu_targets_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "默认目标",
            &build_targets_default_detail_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            "别名列表",
            &build_targets_aliases_page_button_data(1),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]]
}

/// 在指定上下文上处理 `/targets` callback。
pub async fn targets_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    let Some(action) = parse_targets_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("目标配置按钮参数无效"), client_id).await?;
        return Ok(());
    };
    send::answer_callback_query(update.id, Some(action.started_tip()), client_id).await?;

    let action_result = match action.clone() {
        TargetsCallbackAction::Refresh => {
            return render_targets_home_on(app, update.chat_id, update.message_id, client_id).await;
        }
        TargetsCallbackAction::Reset => reset_targets_config_to_default_on(app).await.map(|_| ()),
        TargetsCallbackAction::ConfirmReset => {
            return render_targets_reset_confirm_on(update.chat_id, update.message_id, client_id)
                .await;
        }
        TargetsCallbackAction::ClearDefault => {
            update_targets_with_on(app, &cleared_action_title("私聊默认目标"), |config| {
                config.default_chat_id = 0;
            })
            .await
            .map(|_| ())
        }
        TargetsCallbackAction::ViewDefault => {
            let config = crate::tgbot::transfer::targets_runtime_config_on(app);
            let (text, keyboard) =
                send::ReplyPanel::card(format_default_target_detail_text(&config))
                    .rows(build_default_detail_buttons())
                    .into_card_parts()?;
            edit_runtime_admin_interaction_card_or_error(
                text,
                update.chat_id,
                update.message_id,
                keyboard,
                client_id,
                "目标配置",
            )
            .await?;
            return Ok(());
        }
        TargetsCallbackAction::InputSetDefault => {
            let Some(spec) = targets_input_spec_for_callback_action(&action) else {
                anyhow::bail!(
                    "missing targets input spec for callback action: {:?}",
                    action
                );
            };
            return super::menu::start_admin_input_callback(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                spec.action,
                client_id,
            )
            .await;
        }
        TargetsCallbackAction::InputSetAlias => {
            return super::menu::start_admin_input_callback(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                super::menu::AdminInputAction::TargetsAliasName,
                client_id,
            )
            .await;
        }
        TargetsCallbackAction::InputSearchAlias => {
            return super::menu::start_admin_input_callback(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                super::menu::AdminInputAction::TargetsAliasSearch,
                client_id,
            )
            .await;
        }
        TargetsCallbackAction::ViewAliases(context) => {
            return render_aliases_context_on(
                app,
                &context,
                update.chat_id,
                update.message_id,
                client_id,
            )
            .await;
        }
        TargetsCallbackAction::ViewAlias { alias, context } => {
            let config = crate::tgbot::transfer::targets_runtime_config_on(app);
            let Some(target_chat_id) = config.aliases.get(&alias).copied() else {
                anyhow::bail!("alias not found: {alias}");
            };
            let (text, keyboard) =
                send::ReplyPanel::card(format_alias_detail_text(&alias, target_chat_id))
                    .rows(build_alias_detail_buttons(&alias, &context))
                    .into_card_parts()?;
            edit_runtime_admin_interaction_card_or_error(
                text,
                update.chat_id,
                update.message_id,
                keyboard,
                client_id,
                "目标配置",
            )
            .await?;
            return Ok(());
        }
        TargetsCallbackAction::EditAlias(alias) => {
            return super::menu::start_admin_input_callback_with_context(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                super::menu::AdminInputAction::TargetsSetAlias,
                Some(alias.clone()),
                None,
                Some("修改目标别名".to_owned()),
                Some(format!(
                    "已选 alias：{}。请选择新的目标群组或频道，也可直接输入 chat_id；输入“取消”可退出。",
                    alias
                )),
                Some("选择聊天或输入新的 target_chat_id".to_owned()),
                client_id,
            )
            .await;
        }
        TargetsCallbackAction::DeleteAlias { alias, context } => {
            update_targets_with_on(app, &deleted_action_title("目标别名"), |config| {
                config.aliases.remove(&alias);
            })
            .await
            .map(|_| ())?;
            return render_aliases_context_on(
                app,
                &context,
                update.chat_id,
                update.message_id,
                client_id,
            )
            .await;
        }
        TargetsCallbackAction::UseAliasAsDefault { alias, context } => {
            let config = crate::tgbot::transfer::targets_runtime_config_on(app);
            let Some(target_chat_id) = config.aliases.get(&alias).copied() else {
                anyhow::bail!("alias not found: {alias}");
            };
            update_targets_with_on(app, &updated_action_title("默认目标"), |config| {
                config.default_chat_id = target_chat_id;
            })
            .await
            .map(|_| ())?;
            return render_aliases_context_on(
                app,
                &context,
                update.chat_id,
                update.message_id,
                client_id,
            )
            .await;
        }
    };
    if let Err(err) = action_result {
        send_targets_callback_error(update.chat_id, client_id, &err).await?;
        return Err(err);
    }
    render_targets_home_on(app, update.chat_id, update.message_id, client_id).await
}

/// 构造当前 targets 配置文本。
///
/// 菜单页在已经持有 `AppContext` 时优先用这个版本，避免重复抓全局。
pub(super) fn format_targets_text_on(app: &crate::app_context::AppContext, title: &str) -> String {
    let _ = title;
    format_targets_home_text(&crate::tgbot::transfer::targets_runtime_config_on(app))
}

/// 现有目标别名详情卡片。
fn format_alias_detail_text(alias: &str, target_chat_id: i64) -> String {
    build_runtime_admin_detail_text(
        "目标别名",
        vec![
            card::field("alias", alias),
            card::field("target_chat_id", target_chat_id),
        ],
        "下一步",
        vec!["可以直接修改目标、设为默认，或删除这个别名。".to_owned()],
    )
}

/// 渲染目标配置主页，保持概览简短，把具体管理下沉到分页子页。
fn format_targets_home_text(config: &TargetsConfig) -> String {
    let mut lines = build_runtime_admin_page_intro(TARGETS_PAGE_TITLE, TARGETS_PAGE_DETAIL);
    lines.extend(build_runtime_admin_section_block(
        "默认目标",
        vec![if config.default_chat_id == 0 {
            card::note("未显式配置，当前默认回落到请求私聊。")
        } else {
            card::field("default_chat_id", config.default_chat_id)
        }],
    ));
    lines.extend(build_runtime_admin_section_block(
        "概览",
        vec![format!("目标别名：{}", card::code(config.aliases.len()))],
    ));
    lines.extend(build_runtime_admin_section_block(
        "操作建议",
        vec!["点“默认目标”查看当前默认值；别名列表先点编号进入详情，再执行修改或删除。".to_owned()],
    ));
    lines.join("\n")
}

/// 渲染目标别名分页页。
fn format_aliases_page_text(config: &TargetsConfig, page: u64) -> String {
    format_aliases_list_page_text(config, page, None)
}

/// 渲染目标别名搜索结果页。
fn format_aliases_search_page_text(config: &TargetsConfig, query: &str, page: u64) -> String {
    format_aliases_list_page_text(config, page, Some(query))
}

/// 渲染目标别名分页文本，可选按别名关键字过滤。
fn format_aliases_list_page_text(config: &TargetsConfig, page: u64, query: Option<&str>) -> String {
    let aliases = filtered_aliases(config, query);
    let detail = match query {
        Some(query) => format!(
            "搜索关键字：{}。先点编号进入详情，再修改目标、设默认或删除。",
            card::code(query)
        ),
        None => "新增别名按 alias -> target 两步输入；现有别名先点编号进入详情，再修改目标、设默认或删除。"
            .to_owned(),
    };
    let empty_note = match query {
        Some(_) => "没有匹配的目标别名。",
        None => "当前没有目标别名。",
    };

    format_targets_list_page_text(
        if query.is_some() {
            "目标别名搜索"
        } else {
            "目标别名列表"
        },
        &detail,
        aliases
            .into_iter()
            .map(|(alias, target_chat_id)| {
                format!("{} → {}", card::code(alias), card::code(target_chat_id))
            })
            .collect(),
        page,
        empty_note,
    )
}

/// 渲染通用分页列表文本。
fn format_targets_list_page_text(
    title: &str,
    detail: &str,
    items: Vec<String>,
    page: u64,
    empty_note: &str,
) -> String {
    let total_pages = total_targets_list_pages(items.len());
    let current_page = normalize_targets_list_page(page, total_pages);
    let (start, page_items) = slice_targets_page_items(&items, current_page);

    let mut lines = build_runtime_admin_page_intro(title, detail);
    lines.push(format!(
        "页码：{} / {}  每页：{}  总数：{}",
        card::code(current_page),
        card::code(total_pages),
        card::code(TARGETS_LIST_PAGE_SIZE),
        card::code(items.len())
    ));
    lines.push(String::new());
    lines.push(card::section("当前页"));
    if page_items.is_empty() {
        lines.push(build_page_empty_note(empty_note));
    } else {
        for (offset, item) in page_items.iter().enumerate() {
            lines.push(format!("{}. {}", start + offset + 1, item));
        }
    }
    lines.join("\n")
}

/// 默认目标详情页正文。
fn format_default_target_detail_text(config: &TargetsConfig) -> String {
    build_runtime_admin_detail_text(
        "默认目标",
        vec![if config.default_chat_id == 0 {
            card::note("当前默认回落到请求私聊。")
        } else {
            card::field("default_chat_id", config.default_chat_id)
        }],
        "下一步",
        vec![
            "点“选择聊天”后使用 Telegram 原生选择器，也可直接输入 chat_id。".to_owned(),
            "也可以进入别名详情页，再把该项设为默认目标。".to_owned(),
            "点“恢复私聊默认”会清掉显式默认值。".to_owned(),
        ],
    )
}

/// 默认目标详情页按钮。
fn build_default_detail_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "选择聊天",
                &build_targets_callback_data(TargetsCallbackAction::InputSetDefault),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "恢复私聊默认",
                &build_targets_callback_data(TargetsCallbackAction::ClearDefault),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![send::build_callback_button(
            "别名列表",
            &build_targets_callback_data(TargetsCallbackAction::ViewAliases(
                AliasListContext::All { page: 1 },
            )),
            tdlib_rs::enums::ButtonStyle::Default,
        )],
        build_runtime_admin_back_menu_row(send::build_callback_button(
            "返回目标",
            &build_targets_callback_data(TargetsCallbackAction::Refresh),
            tdlib_rs::enums::ButtonStyle::Default,
        )),
    ]
}

/// 别名详情页按钮。
fn build_alias_detail_buttons(
    alias: &str,
    context: &AliasListContext,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let default_action = TargetsCallbackAction::UseAliasAsDefault {
        alias: alias.to_owned(),
        context: context.clone(),
    };
    let delete_action = TargetsCallbackAction::DeleteAlias {
        alias: alias.to_owned(),
        context: context.clone(),
    };
    let back_action = TargetsCallbackAction::ViewAliases(context.clone());
    vec![
        vec![
            send::build_callback_button(
                "改目标",
                &build_targets_callback_data(TargetsCallbackAction::EditAlias(alias.to_owned())),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "设默认",
                &build_targets_callback_data(default_action),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "删别名",
                &build_targets_callback_data(delete_action),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ],
        build_runtime_admin_back_menu_row(send::build_callback_button(
            match context {
                AliasListContext::Search { .. } => "返回搜索结果",
                AliasListContext::All { .. } => "返回别名列表",
            },
            &build_targets_callback_data(back_action),
            tdlib_rs::enums::ButtonStyle::Default,
        )),
    ]
}

/// 在指定上下文上按闭包更新 targets 配置并立即刷新运行时状态。
async fn update_targets_with_on(
    app: &crate::app_context::AppContext,
    title: &str,
    updater: impl FnOnce(&mut TargetsConfig),
) -> anyhow::Result<String> {
    let mut config = crate::tgbot::transfer::targets_runtime_config_on(app);
    updater(&mut config);
    normalize_targets_config(&mut config);
    persist_targets_config_on(app, &config).await?;
    tracing::info!(
        default_chat_id = config.default_chat_id,
        alias_count = config.aliases.len(),
        "targets runtime config updated"
    );
    Ok(format_targets_config_text(title, &config))
}

/// 在指定上下文上把 targets 重置为启动默认值。
async fn reset_targets_config_to_default_on(
    app: &crate::app_context::AppContext,
) -> anyhow::Result<String> {
    let mut config = crate::tgbot::transfer::targets_runtime_default_config_on(app);
    normalize_targets_config(&mut config);
    persist_targets_config_on(app, &config).await?;
    tracing::info!("targets runtime config reset to startup defaults");
    Ok(format_targets_config_text(
        &reset_action_title("目标配置"),
        &config,
    ))
}

/// 在指定上下文上写库并刷新内存运行时。
async fn persist_targets_config_on(
    app: &crate::app_context::AppContext,
    config: &TargetsConfig,
) -> anyhow::Result<()> {
    crate::tgbot::transfer::save_targets_runtime_config(config).await?;
    crate::tgbot::transfer::update_targets_runtime_config_on(app, config.clone());
    Ok(())
}

/// 规范化配置，避免空白 alias 留在运行时状态里。
fn normalize_targets_config(config: &mut TargetsConfig) {
    config.aliases.retain(|alias, _| !alias.trim().is_empty());
}

/// 以稳定顺序返回目标别名，便于文本渲染和按钮布局。
fn sorted_aliases(config: &TargetsConfig) -> Vec<(String, i64)> {
    let mut aliases = config
        .aliases
        .iter()
        .map(|(alias, target_chat_id)| (alias.clone(), *target_chat_id))
        .collect::<Vec<_>>();
    aliases.sort_by(|left, right| left.0.cmp(&right.0));
    aliases
}

/// 按别名关键字过滤并保持稳定顺序。
fn filtered_aliases(config: &TargetsConfig, query: Option<&str>) -> Vec<(String, i64)> {
    let aliases = sorted_aliases(config);
    let Some(query) = query.map(str::trim).filter(|query| !query.is_empty()) else {
        return aliases;
    };
    let lowered = query.to_ascii_lowercase();
    aliases
        .into_iter()
        .filter(|(alias, _)| alias.to_ascii_lowercase().contains(&lowered))
        .collect()
}

/// 规范化别名搜索关键字。
fn normalize_alias_search_query(input: &str) -> Option<String> {
    let query = input.trim();
    if query.is_empty() {
        return None;
    }
    Some(
        query
            .chars()
            .take(TARGETS_ALIAS_SEARCH_QUERY_MAX_CHARS)
            .collect(),
    )
}

/// 格式化 targets 配置卡片。
fn format_targets_config_text(title: &str, config: &TargetsConfig) -> String {
    let mut lines = build_runtime_admin_page_intro(title, TARGETS_PAGE_DETAIL);
    lines.extend([
        card::section("默认目标"),
        if config.default_chat_id == 0 {
            card::note("未显式配置时，默认回落到当前请求私聊。")
        } else {
            card::field("default_chat_id", config.default_chat_id)
        },
        String::new(),
        card::section("目标别名"),
    ]);
    if config.aliases.is_empty() {
        lines.push(card::note("当前没有目标别名。"));
    } else {
        for (alias, target_chat_id) in sorted_aliases(config) {
            lines.push(format!(
                "{} -> {}",
                card::code(alias),
                card::code(target_chat_id)
            ));
        }
    }

    lines.join("\n")
}

/// `/targets` 页按钮。
pub(super) fn build_targets_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let app_context = crate::app_context::app_context();
    build_targets_buttons_on(app_context.as_ref())
}

/// `/targets` 页按钮的上下文版本。
pub(super) fn build_targets_buttons_on(
    _app: &crate::app_context::AppContext,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "默认目标",
                &build_targets_callback_data(TargetsCallbackAction::ViewDefault),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "别名列表",
                &build_targets_callback_data(TargetsCallbackAction::ViewAliases(
                    AliasListContext::All { page: 1 },
                )),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        vec![
            send::build_callback_button(
                "刷新",
                &build_targets_callback_data(TargetsCallbackAction::Refresh),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "重置全部",
                &build_targets_callback_data(TargetsCallbackAction::ConfirmReset),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
            send::build_callback_button(
                "恢复私聊默认",
                &build_targets_callback_data(TargetsCallbackAction::ClearDefault),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_runtime_admin_help_menu_row("targets"),
    ]
}

/// 计算列表总页数，空列表也按 1 页渲染，保证分页按钮协议稳定。
fn total_targets_list_pages(total_items: usize) -> u64 {
    ((total_items.max(1) - 1) / TARGETS_LIST_PAGE_SIZE + 1) as u64
}

/// 把外部 page 规范到有效区间。
fn normalize_targets_list_page(page: u64, total_pages: u64) -> u64 {
    page.max(1).min(total_pages.max(1))
}

/// 取当前页元素切片及其全局起始下标。
fn slice_targets_page_items<T>(items: &[T], page: u64) -> (usize, &[T]) {
    let total_pages = total_targets_list_pages(items.len());
    let current_page = normalize_targets_list_page(page, total_pages);
    let start = ((current_page - 1) as usize) * TARGETS_LIST_PAGE_SIZE;
    let end = (start + TARGETS_LIST_PAGE_SIZE).min(items.len());
    (start, &items[start.min(items.len())..end])
}

/// 构造目标别名分页页按钮。
fn build_aliases_page_buttons(
    config: &TargetsConfig,
    page: u64,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    build_aliases_page_buttons_with_query(config, page, None)
}

/// 构造目标别名搜索结果页按钮。
fn build_aliases_search_page_buttons(
    config: &TargetsConfig,
    query: &str,
    page: u64,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    build_aliases_page_buttons_with_query(config, page, normalize_alias_search_query(query))
}

/// 构造目标别名分页页按钮，可选按关键字过滤。
fn build_aliases_page_buttons_with_query(
    config: &TargetsConfig,
    page: u64,
    query: Option<String>,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let aliases = filtered_aliases(config, query.as_deref());
    let total_pages = total_targets_list_pages(aliases.len());
    let current_page = normalize_targets_list_page(page, total_pages);
    let (start, page_items) = slice_targets_page_items(&aliases, current_page);

    // 搜索结果页的第一行只放“继续搜索”和“回列表”，避免和普通列表页的新增入口混在一起。
    let mut rows = vec![match query.as_ref() {
        Some(_) => vec![
            send::build_callback_button(
                "重新搜索",
                &build_targets_callback_data(TargetsCallbackAction::InputSearchAlias),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "返回别名列表",
                &build_targets_callback_data(TargetsCallbackAction::ViewAliases(
                    AliasListContext::All { page: 1 },
                )),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        None => vec![
            send::build_callback_button(
                "新增别名",
                &build_targets_callback_data(TargetsCallbackAction::InputSetAlias),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "搜索别名",
                &build_targets_callback_data(TargetsCallbackAction::InputSearchAlias),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    }];

    if !page_items.is_empty() {
        rows.push(
            page_items
                .iter()
                .enumerate()
                .map(|(offset, (alias, _))| {
                    let context = match query.as_ref() {
                        Some(query) => AliasListContext::Search {
                            query: query.clone(),
                            page: current_page,
                        },
                        None => AliasListContext::All { page: current_page },
                    };
                    let action = TargetsCallbackAction::ViewAlias {
                        alias: alias.clone(),
                        context,
                    };
                    send::build_callback_button(
                        &(start + offset + 1).to_string(),
                        &build_targets_callback_data(action),
                        tdlib_rs::enums::ButtonStyle::Default,
                    )
                })
                .collect(),
        );
    }

    let current_list_page = match query.as_ref() {
        Some(query) => AliasListContext::Search {
            query: query.clone(),
            page: current_page,
        },
        None => AliasListContext::All { page: current_page },
    };

    rows.push(build_refresh_return_menu_row(
        send::build_callback_button(
            "刷新",
            &build_targets_list_page_callback_data(&current_list_page, current_page),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "返回目标",
            &build_targets_callback_data(TargetsCallbackAction::Refresh),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            "菜单",
            &super::build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ));
    rows.push(build_targets_pagination_row(current_list_page, total_pages));
    rows
}

/// 构造别名列表分页按钮。
fn build_targets_pagination_row(
    context: AliasListContext,
    total_pages: u64,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    let current_page = context.page();
    let first_page = 1u64;
    let prev_page = current_page.saturating_sub(1).max(1);
    let next_page = (current_page + 1).min(total_pages.max(1));
    let last_page = total_pages.max(1);
    vec![
        build_targets_list_nav_button("首页", &context, first_page),
        build_targets_list_nav_button("上页", &context, prev_page),
        send::build_callback_button(
            &format!("{}/{}", current_page, total_pages.max(1)),
            &build_targets_list_page_callback_data(&context, current_page),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        build_targets_list_nav_button("下页", &context, next_page),
        build_targets_list_nav_button("末页", &context, last_page),
    ]
}

/// 构造分页导航按钮。
fn build_targets_list_nav_button(
    text: &str,
    context: &AliasListContext,
    target_page: u64,
) -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        text,
        &build_targets_list_page_callback_data(context, target_page),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// 构造列表页回调数据。
fn build_targets_list_page_callback_data(context: &AliasListContext, target_page: u64) -> String {
    let target = match context {
        AliasListContext::All { .. } => AliasListContext::All { page: target_page },
        AliasListContext::Search { query, .. } => AliasListContext::Search {
            query: query.clone(),
            page: target_page,
        },
    };
    build_targets_callback_data(TargetsCallbackAction::ViewAliases(target))
}

/// 原地打开目标配置全量重置确认页。
async fn render_targets_reset_confirm_on(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let text = build_runtime_admin_detail_text(
        "确认重置目标配置",
        vec![card::field("范围", "默认目标和全部别名")],
        "影响",
        vec![
            "当前运行态目标配置会被启动配置完整覆盖。".to_owned(),
            "如果只想让默认目标回到当前私聊，请使用“恢复私聊默认”。".to_owned(),
        ],
    );
    let rows = build_targets_reset_confirm_buttons();
    let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
    edit_runtime_admin_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        "目标配置",
    )
    .await
}

/// 目标配置重置确认页按钮。
fn build_targets_reset_confirm_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![send::build_callback_button(
            "确认重置全部",
            &build_targets_callback_data(TargetsCallbackAction::Reset),
            tdlib_rs::enums::ButtonStyle::Danger,
        )],
        build_runtime_admin_back_menu_row(send::build_callback_button(
            "取消",
            &build_targets_callback_data(TargetsCallbackAction::Refresh),
            tdlib_rs::enums::ButtonStyle::Default,
        )),
    ]
}

/// 渲染主页。
async fn render_targets_home_on(
    app: &crate::app_context::AppContext,
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, keyboard) = send::ReplyPanel::card(format_targets_text_on(app, TARGETS_PAGE_TITLE))
        .rows(build_targets_buttons_on(app))
        .into_card_parts()?;
    edit_runtime_admin_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        "目标配置",
    )
    .await
}

/// 按列表上下文渲染目标别名分页页。
async fn render_aliases_context_on(
    app: &crate::app_context::AppContext,
    context: &AliasListContext,
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let config = crate::tgbot::transfer::targets_runtime_config_on(app);
    let (text, rows, title) = match context {
        AliasListContext::All { page } => (
            format_aliases_page_text(&config, *page),
            build_aliases_page_buttons(&config, *page),
            "目标别名列表",
        ),
        AliasListContext::Search { query, page } => {
            let query = normalize_alias_search_query(query)
                .ok_or_else(|| anyhow::anyhow!("alias search query cannot be empty"))?;
            (
                format_aliases_search_page_text(&config, &query, *page),
                build_aliases_search_page_buttons(&config, &query, *page),
                "目标别名搜索",
            )
        }
    };
    let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
    edit_runtime_admin_interaction_card_or_error(
        text, chat_id, message_id, keyboard, client_id, title,
    )
    .await
}

/// 在指定 chat 中发送别名搜索结果页。
///
/// ForceReply 文本输入无法稳定编辑原列表消息，所以搜索结果使用新卡片展示。
pub(in crate::tgbot::transfer::command) async fn send_alias_search_result_page_on(
    app: &crate::app_context::AppContext,
    query: &str,
    page: u64,
    chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let query = normalize_alias_search_query(query)
        .ok_or_else(|| anyhow::anyhow!("alias search query cannot be empty"))?;
    let config = crate::tgbot::transfer::targets_runtime_config_on(app);
    send::ReplyPanel::card(format_aliases_search_page_text(&config, &query, page))
        .rows(build_aliases_search_page_buttons(&config, &query, page))
        .send(chat_id, client_id)
        .await
}

/// 编码 alias 到 callback payload。
fn encode_alias_payload(alias: &str) -> String {
    URL_SAFE_NO_PAD.encode(alias)
}

/// 从 callback payload 解码 alias。
fn decode_alias_payload(payload: &str) -> Option<String> {
    let decoded = URL_SAFE_NO_PAD.decode(payload).ok()?;
    String::from_utf8(decoded).ok()
}

fn parse_targets_callback_data(data: &str) -> Option<TargetsCallbackAction> {
    let payload = data.strip_prefix(TARGETS_CALLBACK_PREFIX)?;
    match payload {
        "r" => Some(TargetsCallbackAction::Refresh),
        "x" => Some(TargetsCallbackAction::Reset),
        "xc" => Some(TargetsCallbackAction::ConfirmReset),
        "d" => Some(TargetsCallbackAction::ClearDefault),
        "v" => Some(TargetsCallbackAction::ViewDefault),
        "is" => Some(TargetsCallbackAction::InputSetDefault),
        "ia" => Some(TargetsCallbackAction::InputSetAlias),
        "ias" => Some(TargetsCallbackAction::InputSearchAlias),
        _ => {
            let (kind, raw) = payload.split_once(':')?;
            match kind {
                "la" => raw
                    .parse::<u64>()
                    .ok()
                    .map(|page| TargetsCallbackAction::ViewAliases(AliasListContext::All { page })),
                "las" => {
                    let (query, page) = raw.rsplit_once('@')?;
                    Some(TargetsCallbackAction::ViewAliases(
                        AliasListContext::Search {
                            query: decode_alias_payload(query)?,
                            page: page.parse::<u64>().ok()?,
                        },
                    ))
                }
                "va" => {
                    if let Some((alias, page)) = raw.rsplit_once('@') {
                        Some(TargetsCallbackAction::ViewAlias {
                            alias: decode_alias_payload(alias)?,
                            context: AliasListContext::All {
                                page: page.parse::<u64>().ok()?,
                            },
                        })
                    } else {
                        Some(TargetsCallbackAction::ViewAlias {
                            alias: decode_alias_payload(raw)?,
                            context: AliasListContext::All { page: 1 },
                        })
                    }
                }
                "vas" => {
                    let (left, page) = raw.rsplit_once('@')?;
                    let (alias, query) = left.split_once('@')?;
                    Some(TargetsCallbackAction::ViewAlias {
                        alias: decode_alias_payload(alias)?,
                        context: AliasListContext::Search {
                            query: decode_alias_payload(query)?,
                            page: page.parse::<u64>().ok()?,
                        },
                    })
                }
                "ea" => decode_alias_payload(raw).map(TargetsCallbackAction::EditAlias),
                "da" => {
                    let (alias, page) = raw.rsplit_once('@')?;
                    Some(TargetsCallbackAction::DeleteAlias {
                        alias: decode_alias_payload(alias)?,
                        context: AliasListContext::All {
                            page: page.parse::<u64>().ok()?,
                        },
                    })
                }
                "das" => {
                    let (left, page) = raw.rsplit_once('@')?;
                    let (alias, query) = left.split_once('@')?;
                    Some(TargetsCallbackAction::DeleteAlias {
                        alias: decode_alias_payload(alias)?,
                        context: AliasListContext::Search {
                            query: decode_alias_payload(query)?,
                            page: page.parse::<u64>().ok()?,
                        },
                    })
                }
                "ua" => {
                    let (alias, page) = raw.rsplit_once('@')?;
                    Some(TargetsCallbackAction::UseAliasAsDefault {
                        alias: decode_alias_payload(alias)?,
                        context: AliasListContext::All {
                            page: page.parse::<u64>().ok()?,
                        },
                    })
                }
                "uas" => {
                    let (left, page) = raw.rsplit_once('@')?;
                    let (alias, query) = left.split_once('@')?;
                    Some(TargetsCallbackAction::UseAliasAsDefault {
                        alias: decode_alias_payload(alias)?,
                        context: AliasListContext::Search {
                            query: decode_alias_payload(query)?,
                            page: page.parse::<u64>().ok()?,
                        },
                    })
                }
                _ => None,
            }
        }
    }
}

fn build_targets_callback_data(action: TargetsCallbackAction) -> String {
    let suffix = match action {
        TargetsCallbackAction::Refresh => "r",
        TargetsCallbackAction::Reset => "x",
        TargetsCallbackAction::ConfirmReset => "xc",
        TargetsCallbackAction::ClearDefault => "d",
        TargetsCallbackAction::ViewDefault => "v",
        TargetsCallbackAction::InputSetDefault => "is",
        TargetsCallbackAction::InputSetAlias => "ia",
        TargetsCallbackAction::InputSearchAlias => "ias",
        TargetsCallbackAction::ViewAliases(context) => match context {
            AliasListContext::All { page } => {
                return format!("{TARGETS_CALLBACK_PREFIX}la:{page}");
            }
            AliasListContext::Search { query, page } => {
                return format!(
                    "{TARGETS_CALLBACK_PREFIX}las:{}@{}",
                    encode_alias_payload(&query),
                    page
                );
            }
        },
        TargetsCallbackAction::ViewAlias { alias, context } => match context {
            AliasListContext::All { page } => {
                return format!(
                    "{TARGETS_CALLBACK_PREFIX}va:{}@{}",
                    encode_alias_payload(&alias),
                    page
                );
            }
            AliasListContext::Search { query, page } => {
                return format!(
                    "{TARGETS_CALLBACK_PREFIX}vas:{}@{}@{}",
                    encode_alias_payload(&alias),
                    encode_alias_payload(&query),
                    page
                );
            }
        },
        TargetsCallbackAction::EditAlias(alias) => {
            return format!(
                "{TARGETS_CALLBACK_PREFIX}ea:{}",
                encode_alias_payload(&alias)
            );
        }
        TargetsCallbackAction::DeleteAlias { alias, context } => match context {
            AliasListContext::All { page } => {
                return format!(
                    "{TARGETS_CALLBACK_PREFIX}da:{}@{}",
                    encode_alias_payload(&alias),
                    page
                );
            }
            AliasListContext::Search { query, page } => {
                return format!(
                    "{TARGETS_CALLBACK_PREFIX}das:{}@{}@{}",
                    encode_alias_payload(&alias),
                    encode_alias_payload(&query),
                    page
                );
            }
        },
        TargetsCallbackAction::UseAliasAsDefault { alias, context } => match context {
            AliasListContext::All { page } => {
                return format!(
                    "{TARGETS_CALLBACK_PREFIX}ua:{}@{}",
                    encode_alias_payload(&alias),
                    page
                );
            }
            AliasListContext::Search { query, page } => {
                return format!(
                    "{TARGETS_CALLBACK_PREFIX}uas:{}@{}@{}",
                    encode_alias_payload(&alias),
                    encode_alias_payload(&query),
                    page
                );
            }
        },
    };
    format!("{TARGETS_CALLBACK_PREFIX}{suffix}")
}

async fn send_targets_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_runtime_admin_callback_error(request_chat_id, client_id, "目标配置", err).await
}

fn parse_i64_arg(text: &[&str], index: usize, usage: &str) -> anyhow::Result<i64> {
    text.get(index)
        .ok_or_else(|| anyhow::anyhow!("{usage}"))?
        .parse::<i64>()
        .map_err(Into::into)
}

fn parse_alias_arg(text: &[&str], index: usize, usage: &str) -> anyhow::Result<String> {
    let alias = text
        .get(index)
        .ok_or_else(|| anyhow::anyhow!("{usage}"))?
        .trim();
    if alias.is_empty() {
        anyhow::bail!("alias cannot be empty");
    }
    Ok(alias.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose;

    #[test]
    fn test_targets_callback_roundtrip() {
        let refresh = build_targets_callback_data(TargetsCallbackAction::Refresh);
        let reset = build_targets_callback_data(TargetsCallbackAction::Reset);
        let confirm_reset = build_targets_callback_data(TargetsCallbackAction::ConfirmReset);
        let clear = build_targets_callback_data(TargetsCallbackAction::ClearDefault);
        let view_default = build_targets_callback_data(TargetsCallbackAction::ViewDefault);
        let view_alias = build_targets_callback_data(TargetsCallbackAction::ViewAlias {
            alias: "archive".to_owned(),
            context: AliasListContext::All { page: 2 },
        });
        let view_alias_search = build_targets_callback_data(TargetsCallbackAction::ViewAlias {
            alias: "archive".to_owned(),
            context: AliasListContext::Search {
                query: "arc".to_owned(),
                page: 2,
            },
        });
        let edit_alias =
            build_targets_callback_data(TargetsCallbackAction::EditAlias("archive".to_owned()));
        let search_alias = build_targets_callback_data(TargetsCallbackAction::ViewAliases(
            AliasListContext::Search {
                query: "arc".to_owned(),
                page: 2,
            },
        ));
        let delete_alias_search = build_targets_callback_data(TargetsCallbackAction::DeleteAlias {
            alias: "archive".to_owned(),
            context: AliasListContext::Search {
                query: "arc".to_owned(),
                page: 2,
            },
        });

        assert!(is_targets_callback_data(&refresh));
        assert_eq!(
            parse_targets_callback_data(&refresh),
            Some(TargetsCallbackAction::Refresh)
        );
        assert_eq!(
            parse_targets_callback_data(&reset),
            Some(TargetsCallbackAction::Reset)
        );
        assert_eq!(confirm_reset, "tcfg:xc");
        assert_eq!(
            parse_targets_callback_data(&confirm_reset),
            Some(TargetsCallbackAction::ConfirmReset)
        );
        assert_eq!(
            parse_targets_callback_data(&clear),
            Some(TargetsCallbackAction::ClearDefault)
        );
        assert_eq!(
            parse_targets_callback_data(&view_default),
            Some(TargetsCallbackAction::ViewDefault)
        );
        assert_eq!(
            parse_targets_callback_data(&view_alias),
            Some(TargetsCallbackAction::ViewAlias {
                alias: "archive".to_owned(),
                context: AliasListContext::All { page: 2 }
            })
        );
        assert_eq!(
            parse_targets_callback_data(&view_alias_search),
            Some(TargetsCallbackAction::ViewAlias {
                alias: "archive".to_owned(),
                context: AliasListContext::Search {
                    query: "arc".to_owned(),
                    page: 2
                }
            })
        );
        assert_eq!(
            parse_targets_callback_data(&edit_alias),
            Some(TargetsCallbackAction::EditAlias("archive".to_owned()))
        );
        assert_eq!(
            parse_targets_callback_data(&search_alias),
            Some(TargetsCallbackAction::ViewAliases(
                AliasListContext::Search {
                    query: "arc".to_owned(),
                    page: 2
                }
            ))
        );
        assert_eq!(
            parse_targets_callback_data(&delete_alias_search),
            Some(TargetsCallbackAction::DeleteAlias {
                alias: "archive".to_owned(),
                context: AliasListContext::Search {
                    query: "arc".to_owned(),
                    page: 2
                }
            })
        );
        assert_eq!(parse_targets_callback_data("tcfg:bad"), None);
    }

    #[test]
    fn test_targets_view_alias_callback_keeps_legacy_payload_compatible() {
        let legacy_view_alias = format!("tcfg:va:{}", encode_alias_payload("archive"));

        assert_eq!(
            parse_targets_callback_data(&legacy_view_alias),
            Some(TargetsCallbackAction::ViewAlias {
                alias: "archive".to_owned(),
                context: AliasListContext::All { page: 1 }
            })
        );
    }

    #[test]
    fn test_format_targets_config_text_contains_sections() {
        let text = format_targets_config_text(
            "当前目标配置",
            &TargetsConfig {
                default_chat_id: -100,
                aliases: std::collections::HashMap::from([("archive".to_owned(), -300)]),
            },
        );

        assert!(text.contains("default_chat_id"));
        assert!(!text.contains("请求路由"));
        assert!(text.contains("目标别名"));
        assert!(!text.contains("/targets"));
    }

    #[test]
    fn test_build_targets_buttons_use_callback_actions() {
        let app = crate::app_context::app_context();
        app.targets_runtime.update_runtime_config(TargetsConfig {
            default_chat_id: -100,
            aliases: std::collections::HashMap::from([("archive".to_owned(), 10002)]),
        });

        let rows = build_targets_buttons();
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("default target button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "tcfg:v");
        assert_eq!(rows[1][0].text, "刷新");
        assert_eq!(rows[1][2].text, "恢复私聊默认");
        assert!(labels.contains(&"默认目标"));
        assert!(labels.contains(&"别名列表"));
        assert!(labels.contains(&"重置全部"));
        assert!(labels.contains(&"查看命令"));
        assert!(!labels.contains(&"重置默认"));
        assert!(!labels.contains(&"设默认"));
        assert!(!labels.contains(&"设路由"));
        assert!(!labels.contains(&"设别名"));
    }

    #[test]
    fn test_targets_reset_confirm_buttons() {
        let rows = build_targets_reset_confirm_buttons();
        assert_eq!(rows[0][0].text, "确认重置全部");
        assert_eq!(rows[1][0].text, "取消");

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("targets reset confirm must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "tcfg:x");
    }

    #[test]
    fn test_build_aliases_page_buttons_select_detail_by_number() {
        let rows = build_aliases_page_buttons(
            &TargetsConfig {
                default_chat_id: 0,
                aliases: std::collections::HashMap::from([
                    ("archive".to_owned(), 10001),
                    ("backup".to_owned(), 10002),
                ]),
            },
            1,
        );

        assert!(rows.iter().flatten().any(|button| button.text == "1"));
        assert!(rows.iter().flatten().any(|button| button.text == "2"));
        assert!(!rows.iter().flatten().any(|button| button.text == "改1"));
        assert!(!rows.iter().flatten().any(|button| button.text == "默认1"));
        assert!(!rows.iter().flatten().any(|button| button.text == "删1"));
        assert!(rows.iter().flatten().any(|button| button.text == "首页"));
        assert!(rows.iter().flatten().any(|button| button.text == "末页"));
        assert_eq!(
            rows.iter()
                .flatten()
                .filter(|button| button.text == "返回目标")
                .count(),
            1
        );
        assert_eq!(rows[0].len(), 2);
        assert_eq!(rows[0][0].text, "新增别名");
        assert_eq!(rows[0][1].text, "搜索别名");
    }

    #[test]
    fn test_build_aliases_search_page_buttons_keep_numbered_actions() {
        let rows = build_aliases_search_page_buttons(
            &TargetsConfig {
                default_chat_id: 0,
                aliases: std::collections::HashMap::from([
                    ("archive".to_owned(), 10001),
                    ("backup".to_owned(), 10002),
                ]),
            },
            "arc",
            1,
        );
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"重新搜索"));
        assert!(labels.contains(&"返回别名列表"));
        assert!(labels.contains(&"1"));
        assert!(!labels.contains(&"改1"));
        assert!(!labels.contains(&"默认1"));
        assert!(!labels.contains(&"删1"));
        assert!(!labels.contains(&"2"));
        assert!(!labels.contains(&"新增别名"));
    }

    #[test]
    fn test_targets_detail_buttons_return_to_source_list() {
        let alias_rows = build_alias_detail_buttons("archive", &AliasListContext::All { page: 2 });
        let search_rows = build_alias_detail_buttons(
            "archive",
            &AliasListContext::Search {
                query: "arc".to_owned(),
                page: 4,
            },
        );

        assert!(
            alias_rows
                .iter()
                .flatten()
                .any(|button| button.text == "返回别名列表")
        );
        assert!(
            search_rows
                .iter()
                .flatten()
                .any(|button| button.text == "返回搜索结果")
        );
    }

    #[test]
    fn test_filtered_aliases_is_case_insensitive() {
        let config = TargetsConfig {
            default_chat_id: 0,
            aliases: std::collections::HashMap::from([
                ("Archive".to_owned(), 10001),
                ("backup".to_owned(), 10002),
            ]),
        };

        let filtered = filtered_aliases(&config, Some("arc"));

        assert_eq!(filtered, vec![("Archive".to_owned(), 10001)]);
    }

    #[test]
    fn test_format_aliases_search_page_text_shows_query() {
        let text = format_aliases_search_page_text(
            &TargetsConfig {
                default_chat_id: 0,
                aliases: std::collections::HashMap::from([("archive".to_owned(), 10001)]),
            },
            "arc",
            1,
        );

        assert!(text.contains("目标别名搜索"));
        assert!(text.contains("搜索关键字：‹arc›"));
        assert!(text.contains("archive"));
    }

    #[test]
    fn test_normalize_targets_config_removes_empty_alias() {
        let mut config = TargetsConfig {
            default_chat_id: 0,
            aliases: std::collections::HashMap::from([
                ("archive".to_owned(), -100),
                ("".to_owned(), -200),
            ]),
        };

        normalize_targets_config(&mut config);

        assert_eq!(config.aliases.len(), 1);
        assert_eq!(config.aliases.get("archive"), Some(&-100));
    }
}
