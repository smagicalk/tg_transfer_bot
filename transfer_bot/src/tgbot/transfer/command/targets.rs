// `/targets` 命令：
// - 管理默认目标、按请求 chat 路由和目标别名
// - 写入数据库后立即刷新运行时 targets 配置

use crate::config::TargetsConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

use super::common::{
    CommandStyle, RuntimeAdminHelpCopyButton, RuntimeAdminHelpDescriptor, RuntimeAdminUsageItem,
    build_command_examples, build_help_menu_row, build_runtime_admin_page_intro,
    cleared_action_title, command_root, deleted_action_title,
    edit_runtime_admin_interaction_card_or_error, reset_action_title,
    send_runtime_admin_callback_error, targets_show_command, updated_action_title,
};
/// 目标页标题。
const TARGETS_PAGE_TITLE: &str = "目标配置";
/// 目标页简要说明。
const TARGETS_PAGE_DETAIL: &str =
    "默认目标未显式设置时会回落到当前请求私聊；可按现有路由/别名逐项选择后再修改。";

/// `/targets` callback 前缀。
const TARGETS_CALLBACK_PREFIX: &str = "tcfg:";

/// `/targets` callback 动作。
#[derive(Debug, Clone, PartialEq, Eq)]
enum TargetsCallbackAction {
    Refresh,
    Reset,
    ClearDefault,
    InputSetDefault,
    PickSetDefault,
    InputSetRoute,
    PickSetRoute,
    InputDelRoute,
    InputSetAlias,
    InputDelAlias,
    ViewRoute(i64),
    ViewAlias(String),
    EditRoute(i64),
    EditAlias(String),
    DeleteRoute(i64),
    DeleteAlias(String),
    UseRouteAsDefault(i64),
    UseAliasAsDefault(String),
}

impl TargetsCallbackAction {
    fn started_tip(&self) -> &'static str {
        match self {
            Self::Refresh => "正在刷新目标配置",
            Self::Reset => "正在重置目标配置",
            Self::ClearDefault => "正在恢复私聊默认目标",
            Self::InputSetDefault
            | Self::PickSetDefault
            | Self::InputSetRoute
            | Self::PickSetRoute
            | Self::InputDelRoute
            | Self::InputSetAlias
            | Self::InputDelAlias
            | Self::EditRoute(_)
            | Self::EditAlias(_) => "请回复参数",
            Self::ViewRoute(_)
            | Self::ViewAlias(_)
            | Self::DeleteRoute(_)
            | Self::DeleteAlias(_)
            | Self::UseRouteAsDefault(_)
            | Self::UseAliasAsDefault(_) => "正在打开目标详情",
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
    pub button_label: &'static str,
    pub input_title: &'static str,
    pub input_detail: &'static str,
    pub input_placeholder: &'static str,
    pub subcommand: &'static str,
    pub expected_parts: usize,
    pub example_command: &'static str,
    pub copy_label: &'static str,
    pub interaction_detail: &'static str,
}

/// `/targets` 当前支持的全部输入式动作。
pub(in crate::tgbot::transfer::command) const TARGETS_INPUT_SPECS: &[TargetsInputSpec] = &[
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsSetDefault,
        callback_action: TargetsCallbackAction::InputSetDefault,
        button_label: "设默认",
        input_title: "设置默认目标",
        input_detail: "请回复目标私聊 chat_id，例如 123456789；或发送 /cancel 取消。",
        input_placeholder: "输入 target_chat_id，或发送 /cancel",
        subcommand: "set-default",
        expected_parts: 1,
        example_command: "/targets set-default 123456789",
        copy_label: "复制默认",
        interaction_detail: "设默认：回复 target_chat_id。",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsPickDefault,
        callback_action: TargetsCallbackAction::PickSetDefault,
        button_label: "旧选默认",
        input_title: "选择默认目标",
        input_detail: "旧版入口：当前版本默认目标推荐直接恢复为“当前请求私聊”。",
        input_placeholder: "已不建议使用",
        subcommand: "set-default",
        expected_parts: 1,
        example_command: "/targets set-default 123456789",
        copy_label: "复制默认",
        interaction_detail: "旧选默认：兼容保留，推荐改用“恢复私聊默认”或手动输入。 ",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsSetRoute,
        callback_action: TargetsCallbackAction::InputSetRoute,
        button_label: "设路由",
        input_title: "设置请求路由",
        input_detail: "请回复 request_chat_id 和目标私聊 chat_id，例如 123456789 987654321。",
        input_placeholder: "输入 request_chat_id target_chat_id",
        subcommand: "set-route",
        expected_parts: 2,
        example_command: "/targets set-route 123456789 987654321",
        copy_label: "复制路由",
        interaction_detail: "设路由：回复 request_chat_id target_chat_id。",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsPickRoute,
        callback_action: TargetsCallbackAction::PickSetRoute,
        button_label: "旧选路由",
        input_title: "选择请求路由目标",
        input_detail: "旧版入口：当前版本建议直接输入 request_chat_id 和目标私聊 chat_id。",
        input_placeholder: "已不建议使用",
        subcommand: "set-route",
        expected_parts: 2,
        example_command: "/targets set-route 123456789 987654321",
        copy_label: "复制路由",
        interaction_detail: "旧选路由：兼容保留，推荐直接输入 request_chat_id 和目标私聊 chat_id。",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsDelRoute,
        callback_action: TargetsCallbackAction::InputDelRoute,
        button_label: "删路由",
        input_title: "删除请求路由",
        input_detail: "请回复要删除的 request_chat_id，例如 123456789；或发送 /cancel 取消。",
        input_placeholder: "输入 request_chat_id，或发送 /cancel",
        subcommand: "del-route",
        expected_parts: 1,
        example_command: "/targets del-route 123456789",
        copy_label: "复制删路由",
        interaction_detail: "删路由：回复 request_chat_id。",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsSetAlias,
        callback_action: TargetsCallbackAction::InputSetAlias,
        button_label: "设别名",
        input_title: "设置目标别名",
        input_detail: "请回复 alias 和目标私聊 chat_id，例如 archive 123456789。",
        input_placeholder: "输入 alias target_chat_id",
        subcommand: "set-alias",
        expected_parts: 2,
        example_command: "/targets set-alias archive 123456789",
        copy_label: "复制别名",
        interaction_detail: "设别名：回复 alias target_chat_id。",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsDelAlias,
        callback_action: TargetsCallbackAction::InputDelAlias,
        button_label: "删别名",
        input_title: "删除目标别名",
        input_detail: "请回复要删除的 alias，例如 archive；或发送 /cancel 取消。",
        input_placeholder: "输入 alias，或发送 /cancel",
        subcommand: "del-alias",
        expected_parts: 1,
        example_command: "/targets del-alias archive",
        copy_label: "复制删别名",
        interaction_detail: "删别名：回复 alias。",
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
        Some("set-route") => {
            let request_id = parse_i64_arg(
                &text,
                2,
                "usage: /targets set-route <request_chat_id> <target_chat_id>",
            )?;
            let target_chat_id = parse_i64_arg(
                &text,
                3,
                "usage: /targets set-route <request_chat_id> <target_chat_id>",
            )?;
            update_targets_with_on(app, &updated_action_title("请求路由"), |config| {
                config.by_request_chat_id.insert(request_id, target_chat_id);
            })
            .await?
        }
        Some("del-route") => {
            let request_id =
                parse_i64_arg(&text, 2, "usage: /targets del-route <request_chat_id>")?;
            update_targets_with_on(app, &deleted_action_title("请求路由"), |config| {
                config.by_request_chat_id.remove(&request_id);
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
        Some(other) => anyhow::bail!("unknown targets subcommand: {}", other),
    };

    send::ReplyPanel::card(reply)
        .rows(build_targets_buttons())
        .send(request_chat_id, client_id)
        .await
}

/// `targets` 管理页的最小帮助 descriptor。
pub(in crate::tgbot::transfer::command) fn targets_help_descriptor() -> RuntimeAdminHelpDescriptor {
    RuntimeAdminHelpDescriptor {
        synopsis: format!(
            "{} [show|reset|set-default|set-route|del-route|set-alias|del-alias]",
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
        help_copy_buttons: targets_help_copy_buttons(),
    }
}

/// `/targets` 帮助页和卡片共用的交互说明。
fn targets_interaction_items() -> Vec<String> {
    let mut items = vec![
        "刷新 / 重置默认 / 恢复私聊默认：直接点按钮执行。".to_owned(),
        "现有路由 / 现有别名：可先点进详情，再修改或删除。".to_owned(),
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

/// `/targets` help 详情页复制按钮。
fn targets_help_copy_buttons() -> Vec<RuntimeAdminHelpCopyButton> {
    let mut buttons = vec![RuntimeAdminHelpCopyButton::new(
        "复制 show",
        "/targets show",
        tdlib_rs::enums::ButtonStyle::Primary,
    )];
    buttons.extend(
        TARGETS_INPUT_SPECS
            .iter()
            .filter(|spec| {
                matches!(
                    spec.action,
                    super::menu::AdminInputAction::TargetsSetDefault
                        | super::menu::AdminInputAction::TargetsSetRoute
                        | super::menu::AdminInputAction::TargetsSetAlias
                )
            })
            .map(|spec| {
                RuntimeAdminHelpCopyButton::new(
                    spec.copy_label,
                    spec.example_command,
                    tdlib_rs::enums::ButtonStyle::Default,
                )
            }),
    );
    buttons
}

/// 判断 callback payload 是否属于 `/targets`。
pub(super) fn is_targets_callback_data(data: &str) -> bool {
    data.starts_with(TARGETS_CALLBACK_PREFIX)
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
        TargetsCallbackAction::Refresh => Ok(()),
        TargetsCallbackAction::Reset => reset_targets_config_to_default_on(app).await.map(|_| ()),
        TargetsCallbackAction::ClearDefault => {
            update_targets_with_on(app, &cleared_action_title("私聊默认目标"), |config| {
                config.default_chat_id = 0;
            })
            .await
            .map(|_| ())
        }
        TargetsCallbackAction::InputSetDefault
        | TargetsCallbackAction::PickSetDefault
        | TargetsCallbackAction::InputSetRoute
        | TargetsCallbackAction::PickSetRoute
        | TargetsCallbackAction::InputDelRoute
        | TargetsCallbackAction::InputSetAlias
        | TargetsCallbackAction::InputDelAlias => {
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
        TargetsCallbackAction::ViewRoute(request_chat_id) => {
            let config = crate::tgbot::transfer::targets_runtime_config_on(app);
            let Some(target_chat_id) = config.by_request_chat_id.get(&request_chat_id).copied()
            else {
                anyhow::bail!("route request_chat_id not found: {}", request_chat_id);
            };
            let (text, keyboard) =
                send::ReplyPanel::card(format_route_detail_text(request_chat_id, target_chat_id))
                    .rows(build_route_detail_buttons(request_chat_id))
                    .into_card_parts()?;
            edit_runtime_admin_interaction_card_or_error(
                text,
                update.chat_id,
                update.message_id,
                keyboard,
                client_id,
                "目标配置",
                "/targets show",
            )
            .await?;
            return Ok(());
        }
        TargetsCallbackAction::ViewAlias(alias) => {
            let config = crate::tgbot::transfer::targets_runtime_config_on(app);
            let Some(target_chat_id) = config.aliases.get(&alias).copied() else {
                anyhow::bail!("alias not found: {}", alias);
            };
            let (text, keyboard) =
                send::ReplyPanel::card(format_alias_detail_text(&alias, target_chat_id))
                    .rows(build_alias_detail_buttons(&alias))
                    .into_card_parts()?;
            edit_runtime_admin_interaction_card_or_error(
                text,
                update.chat_id,
                update.message_id,
                keyboard,
                client_id,
                "目标配置",
                "/targets show",
            )
            .await?;
            return Ok(());
        }
        TargetsCallbackAction::EditRoute(request_chat_id) => {
            return super::menu::start_admin_input_callback_with_context(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                super::menu::AdminInputAction::TargetsSetRoute,
                None,
                Some(request_chat_id),
                Some("修改请求路由".to_owned()),
                Some(format!(
                    "已选 request_chat_id：{}。请只回复新的目标私聊 chat_id；或发送 /cancel 取消。",
                    request_chat_id
                )),
                Some("输入新的 target_chat_id，或发送 /cancel".to_owned()),
                client_id,
            )
            .await;
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
                    "已选 alias：{}。请只回复新的目标私聊 chat_id；或发送 /cancel 取消。",
                    alias
                )),
                Some("输入新的 target_chat_id，或发送 /cancel".to_owned()),
                client_id,
            )
            .await;
        }
        TargetsCallbackAction::DeleteRoute(request_chat_id) => {
            update_targets_with_on(app, &deleted_action_title("请求路由"), |config| {
                config.by_request_chat_id.remove(&request_chat_id);
            })
            .await
            .map(|_| ())
        }
        TargetsCallbackAction::DeleteAlias(alias) => {
            update_targets_with_on(app, &deleted_action_title("目标别名"), |config| {
                config.aliases.remove(&alias);
            })
            .await
            .map(|_| ())
        }
        TargetsCallbackAction::UseRouteAsDefault(request_chat_id) => {
            let config = crate::tgbot::transfer::targets_runtime_config_on(app);
            let Some(target_chat_id) = config.by_request_chat_id.get(&request_chat_id).copied()
            else {
                anyhow::bail!("route request_chat_id not found: {}", request_chat_id);
            };
            update_targets_with_on(app, &updated_action_title("默认目标"), |config| {
                config.default_chat_id = target_chat_id;
            })
            .await
            .map(|_| ())
        }
        TargetsCallbackAction::UseAliasAsDefault(alias) => {
            let config = crate::tgbot::transfer::targets_runtime_config_on(app);
            let Some(target_chat_id) = config.aliases.get(&alias).copied() else {
                anyhow::bail!("alias not found: {}", alias);
            };
            update_targets_with_on(app, &updated_action_title("默认目标"), |config| {
                config.default_chat_id = target_chat_id;
            })
            .await
            .map(|_| ())
        }
    };
    if let Err(err) = action_result {
        send_targets_callback_error(update.chat_id, client_id, &err).await?;
        return Err(err);
    }

    let (text, keyboard) = send::ReplyPanel::card(format_targets_text_on(app, TARGETS_PAGE_TITLE))
        .rows(build_targets_buttons_on(app))
        .into_card_parts()?;
    edit_runtime_admin_interaction_card_or_error(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
        "目标配置",
        "/targets show",
    )
    .await?;
    Ok(())
}

/// 构造当前 targets 配置文本。
///
/// 菜单页在已经持有 `AppContext` 时优先用这个版本，避免重复抓全局。
pub(super) fn format_targets_text_on(app: &crate::app_context::AppContext, title: &str) -> String {
    format_targets_config_text(
        title,
        &crate::tgbot::transfer::targets_runtime_config_on(app),
    )
}

/// 现有请求路由详情卡片。
fn format_route_detail_text(request_chat_id: i64, target_chat_id: i64) -> String {
    [
        "请求路由".to_owned(),
        card::field("request_chat_id", request_chat_id),
        card::field("target_chat_id", target_chat_id),
        String::new(),
        card::section("下一步"),
        "可以直接修改目标、设为默认，或删除这条路由。".to_owned(),
    ]
    .join("\n")
}

/// 现有目标别名详情卡片。
fn format_alias_detail_text(alias: &str, target_chat_id: i64) -> String {
    [
        "目标别名".to_owned(),
        card::field("alias", alias),
        card::field("target_chat_id", target_chat_id),
        String::new(),
        card::section("下一步"),
        "可以直接修改目标、设为默认，或删除这个别名。".to_owned(),
    ]
    .join("\n")
}

/// 请求路由详情页按钮。
fn build_route_detail_buttons(
    request_chat_id: i64,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "改目标",
                &build_targets_callback_data(TargetsCallbackAction::EditRoute(request_chat_id)),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "设默认",
                &build_targets_callback_data(TargetsCallbackAction::UseRouteAsDefault(
                    request_chat_id,
                )),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "删路由",
                &build_targets_callback_data(TargetsCallbackAction::DeleteRoute(request_chat_id)),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ],
        build_help_menu_row(
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
        ),
    ]
}

/// 别名详情页按钮。
fn build_alias_detail_buttons(alias: &str) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![
            send::build_callback_button(
                "改目标",
                &build_targets_callback_data(TargetsCallbackAction::EditAlias(alias.to_owned())),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "设默认",
                &build_targets_callback_data(TargetsCallbackAction::UseAliasAsDefault(
                    alias.to_owned(),
                )),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "删别名",
                &build_targets_callback_data(TargetsCallbackAction::DeleteAlias(alias.to_owned())),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ],
        build_help_menu_row(
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
        ),
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
        route_count = config.by_request_chat_id.len(),
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

/// 规范化配置，避免空白 alias 或无意义 route 留在运行时状态里。
fn normalize_targets_config(config: &mut TargetsConfig) {
    config.aliases.retain(|alias, _| !alias.trim().is_empty());
}

/// 以稳定顺序返回请求路由，便于文本渲染和按钮布局。
fn sorted_routes(config: &TargetsConfig) -> Vec<(i64, i64)> {
    let mut routes = config
        .by_request_chat_id
        .iter()
        .map(|(request_chat_id, target_chat_id)| (*request_chat_id, *target_chat_id))
        .collect::<Vec<_>>();
    routes.sort_by_key(|(request_chat_id, _)| *request_chat_id);
    routes
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
        card::section("请求路由"),
    ]);

    if config.by_request_chat_id.is_empty() {
        lines.push(card::note("当前没有按请求 chat 的单独路由。"));
    } else {
        for (request_chat_id, target_chat_id) in sorted_routes(config) {
            lines.push(format!(
                "{} -> {}",
                card::code(request_chat_id),
                card::code(target_chat_id)
            ));
        }
    }

    lines.extend([String::new(), card::section("目标别名")]);
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

    lines.extend(build_command_examples(
        targets_example_commands()
            .into_iter()
            .filter(|command| command != "/targets reset"),
    ));

    lines.join("\n")
}

/// `/targets` 页按钮。
pub(super) fn build_targets_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let app_context = crate::app_context::app_context();
    build_targets_buttons_on(app_context.as_ref())
}

/// `/targets` 页按钮的上下文版本。
pub(super) fn build_targets_buttons_on(
    app: &crate::app_context::AppContext,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let config = crate::tgbot::transfer::targets_runtime_config_on(app);
    let mut rows = vec![
        vec![
            send::build_callback_button(
                "刷新",
                &build_targets_callback_data(TargetsCallbackAction::Refresh),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "重置默认",
                &build_targets_callback_data(TargetsCallbackAction::Reset),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "恢复私聊默认",
                &build_targets_callback_data(TargetsCallbackAction::ClearDefault),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_targets_input_row(&[
            TARGETS_INPUT_SPECS[0].clone(),
            TARGETS_INPUT_SPECS[2].clone(),
            TARGETS_INPUT_SPECS[5].clone(),
        ]),
    ];
    let route_buttons = sorted_routes(&config)
        .into_iter()
        .map(|(request_chat_id, _target_chat_id)| {
            send::build_callback_button(
                &format!("路由 {}", request_chat_id),
                &build_targets_callback_data(TargetsCallbackAction::ViewRoute(request_chat_id)),
                tdlib_rs::enums::ButtonStyle::Default,
            )
        })
        .collect::<Vec<_>>();
    rows.extend(chunk_buttons(route_buttons, 2));
    let alias_buttons = sorted_aliases(&config)
        .into_iter()
        .map(|(alias, _target_chat_id)| {
            let callback_data =
                build_targets_callback_data(TargetsCallbackAction::ViewAlias(alias.clone()));
            send::build_callback_button(
                &alias,
                &callback_data,
                tdlib_rs::enums::ButtonStyle::Default,
            )
        })
        .collect::<Vec<_>>();
    rows.extend(chunk_buttons(alias_buttons, 2));
    rows.push(build_targets_input_row(&[
        TARGETS_INPUT_SPECS[1].clone(),
        TARGETS_INPUT_SPECS[3].clone(),
        TARGETS_INPUT_SPECS[4].clone(),
        TARGETS_INPUT_SPECS[6].clone(),
    ]));
    rows.push(build_help_menu_row(
        send::build_callback_button(
            "帮助",
            &super::help::build_help_callback_data(Some("targets")),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            "菜单",
            &super::build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ));
    rows
}

/// 把按钮按列数分行。
fn chunk_buttons(
    buttons: Vec<tdlib_rs::types::InlineKeyboardButton>,
    chunk_size: usize,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    if chunk_size == 0 {
        return vec![buttons];
    }
    buttons.chunks(chunk_size).map(<[_]>::to_vec).collect()
}

/// 构造 targets 输入按钮行。
fn build_targets_input_row(
    specs: &[TargetsInputSpec],
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    specs
        .iter()
        .map(|spec| {
            send::build_callback_button(
                spec.button_label,
                &build_targets_callback_data(spec.callback_action.clone()),
                tdlib_rs::enums::ButtonStyle::Default,
            )
        })
        .collect()
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
        "d" => Some(TargetsCallbackAction::ClearDefault),
        "is" => Some(TargetsCallbackAction::InputSetDefault),
        "ps" => Some(TargetsCallbackAction::PickSetDefault),
        "ir" => Some(TargetsCallbackAction::InputSetRoute),
        "pr" => Some(TargetsCallbackAction::PickSetRoute),
        "id" => Some(TargetsCallbackAction::InputDelRoute),
        "ia" => Some(TargetsCallbackAction::InputSetAlias),
        "ix" => Some(TargetsCallbackAction::InputDelAlias),
        _ => {
            let (kind, raw) = payload.split_once(':')?;
            match kind {
                "vr" => raw
                    .parse::<i64>()
                    .ok()
                    .map(TargetsCallbackAction::ViewRoute),
                "er" => raw
                    .parse::<i64>()
                    .ok()
                    .map(TargetsCallbackAction::EditRoute),
                "dr" => raw
                    .parse::<i64>()
                    .ok()
                    .map(TargetsCallbackAction::DeleteRoute),
                "ur" => raw
                    .parse::<i64>()
                    .ok()
                    .map(TargetsCallbackAction::UseRouteAsDefault),
                "va" => decode_alias_payload(raw).map(TargetsCallbackAction::ViewAlias),
                "ea" => decode_alias_payload(raw).map(TargetsCallbackAction::EditAlias),
                "da" => decode_alias_payload(raw).map(TargetsCallbackAction::DeleteAlias),
                "ua" => decode_alias_payload(raw).map(TargetsCallbackAction::UseAliasAsDefault),
                _ => None,
            }
        }
    }
}

fn build_targets_callback_data(action: TargetsCallbackAction) -> String {
    let suffix = match action {
        TargetsCallbackAction::Refresh => "r",
        TargetsCallbackAction::Reset => "x",
        TargetsCallbackAction::ClearDefault => "d",
        TargetsCallbackAction::InputSetDefault => "is",
        TargetsCallbackAction::PickSetDefault => "ps",
        TargetsCallbackAction::InputSetRoute => "ir",
        TargetsCallbackAction::PickSetRoute => "pr",
        TargetsCallbackAction::InputDelRoute => "id",
        TargetsCallbackAction::InputSetAlias => "ia",
        TargetsCallbackAction::InputDelAlias => "ix",
        TargetsCallbackAction::ViewRoute(request_chat_id) => {
            return format!("{TARGETS_CALLBACK_PREFIX}vr:{request_chat_id}");
        }
        TargetsCallbackAction::ViewAlias(alias) => {
            return format!(
                "{TARGETS_CALLBACK_PREFIX}va:{}",
                encode_alias_payload(&alias)
            );
        }
        TargetsCallbackAction::EditRoute(request_chat_id) => {
            return format!("{TARGETS_CALLBACK_PREFIX}er:{request_chat_id}");
        }
        TargetsCallbackAction::EditAlias(alias) => {
            return format!(
                "{TARGETS_CALLBACK_PREFIX}ea:{}",
                encode_alias_payload(&alias)
            );
        }
        TargetsCallbackAction::DeleteRoute(request_chat_id) => {
            return format!("{TARGETS_CALLBACK_PREFIX}dr:{request_chat_id}");
        }
        TargetsCallbackAction::DeleteAlias(alias) => {
            return format!(
                "{TARGETS_CALLBACK_PREFIX}da:{}",
                encode_alias_payload(&alias)
            );
        }
        TargetsCallbackAction::UseRouteAsDefault(request_chat_id) => {
            return format!("{TARGETS_CALLBACK_PREFIX}ur:{request_chat_id}");
        }
        TargetsCallbackAction::UseAliasAsDefault(alias) => {
            return format!(
                "{TARGETS_CALLBACK_PREFIX}ua:{}",
                encode_alias_payload(&alias)
            );
        }
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
        .ok_or_else(|| anyhow::anyhow!("{}", usage))?
        .parse::<i64>()
        .map_err(Into::into)
}

fn parse_alias_arg(text: &[&str], index: usize, usage: &str) -> anyhow::Result<String> {
    let alias = text
        .get(index)
        .ok_or_else(|| anyhow::anyhow!("{}", usage))?
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
        let clear = build_targets_callback_data(TargetsCallbackAction::ClearDefault);
        let pick_default = build_targets_callback_data(TargetsCallbackAction::PickSetDefault);
        let pick_route = build_targets_callback_data(TargetsCallbackAction::PickSetRoute);
        let view_route = build_targets_callback_data(TargetsCallbackAction::ViewRoute(42));
        let view_alias =
            build_targets_callback_data(TargetsCallbackAction::ViewAlias("archive".to_owned()));
        let edit_alias =
            build_targets_callback_data(TargetsCallbackAction::EditAlias("archive".to_owned()));

        assert!(is_targets_callback_data(&refresh));
        assert_eq!(
            parse_targets_callback_data(&refresh),
            Some(TargetsCallbackAction::Refresh)
        );
        assert_eq!(
            parse_targets_callback_data(&reset),
            Some(TargetsCallbackAction::Reset)
        );
        assert_eq!(
            parse_targets_callback_data(&clear),
            Some(TargetsCallbackAction::ClearDefault)
        );
        assert_eq!(
            parse_targets_callback_data(&pick_default),
            Some(TargetsCallbackAction::PickSetDefault)
        );
        assert_eq!(
            parse_targets_callback_data(&pick_route),
            Some(TargetsCallbackAction::PickSetRoute)
        );
        assert_eq!(
            parse_targets_callback_data(&view_route),
            Some(TargetsCallbackAction::ViewRoute(42))
        );
        assert_eq!(
            parse_targets_callback_data(&view_alias),
            Some(TargetsCallbackAction::ViewAlias("archive".to_owned()))
        );
        assert_eq!(
            parse_targets_callback_data(&edit_alias),
            Some(TargetsCallbackAction::EditAlias("archive".to_owned()))
        );
        assert_eq!(parse_targets_callback_data("tcfg:bad"), None);
    }

    #[test]
    fn test_format_targets_config_text_contains_sections() {
        let text = format_targets_config_text(
            "当前目标配置",
            &TargetsConfig {
                default_chat_id: -100,
                by_request_chat_id: std::collections::HashMap::from([(1, -200)]),
                aliases: std::collections::HashMap::from([("archive".to_owned(), -300)]),
            },
        );

        assert!(text.contains("default_chat_id"));
        assert!(text.contains("请求路由"));
        assert!(text.contains("目标别名"));
        assert!(text.contains("/targets set-default"));
    }

    #[test]
    fn test_build_targets_buttons_use_callback_actions() {
        let app = crate::app_context::app_context();
        app.targets_runtime.update_runtime_config(TargetsConfig {
            default_chat_id: -100,
            by_request_chat_id: std::collections::HashMap::from([(1, 10001)]),
            aliases: std::collections::HashMap::from([("archive".to_owned(), 10002)]),
            ..Default::default()
        });

        let rows = build_targets_buttons();
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("refresh button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "tcfg:r");
        assert_eq!(rows[0][2].text, "恢复私聊默认");
        assert!(rows.iter().flatten().any(|button| button.text == "设默认"));
        assert!(rows.iter().flatten().any(|button| button.text == "路由 1"));
        assert!(rows.iter().flatten().any(|button| button.text == "archive"));
        assert!(rows.iter().flatten().any(|button| button.text == "设别名"));
    }

    #[test]
    fn test_normalize_targets_config_removes_empty_alias() {
        let mut config = TargetsConfig {
            default_chat_id: 0,
            by_request_chat_id: Default::default(),
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
