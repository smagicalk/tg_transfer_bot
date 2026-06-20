// `/targets` 命令：
// - 管理默认目标、按请求 chat 路由和目标别名
// - 写入数据库后立即刷新运行时 targets 配置

use crate::config::TargetsConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;

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
const TARGETS_PAGE_DETAIL: &str = "管理默认目标、请求路由和目标别名；点输入按钮后按提示回复参数。";

/// `/targets` callback 前缀。
const TARGETS_CALLBACK_PREFIX: &str = "tcfg:";

/// `/targets` callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl TargetsCallbackAction {
    fn started_tip(self) -> &'static str {
        match self {
            Self::Refresh => "正在刷新目标配置",
            Self::Reset => "正在重置目标配置",
            Self::ClearDefault => "正在清空默认目标",
            Self::InputSetDefault
            | Self::PickSetDefault
            | Self::InputSetRoute
            | Self::PickSetRoute
            | Self::InputDelRoute
            | Self::InputSetAlias
            | Self::InputDelAlias => "请回复参数",
        }
    }
}

/// `/targets` 单步输入动作规格。
///
/// callback 按钮、help 示例、ForceReply 文案和输入解析都从这里读取，避免新增目标动作时漏改多处。
#[derive(Debug, Clone, Copy)]
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
        input_detail: "请回复目标 chat_id，例如 -1001234567890；或发送 /cancel 取消。",
        input_placeholder: "输入 target_chat_id，或发送 /cancel",
        subcommand: "set-default",
        expected_parts: 1,
        example_command: "/targets set-default -1001234567890",
        copy_label: "复制默认",
        interaction_detail: "设默认：回复 target_chat_id。",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsPickDefault,
        callback_action: TargetsCallbackAction::PickSetDefault,
        button_label: "选默认",
        input_title: "选择默认目标",
        input_detail: "点击后会弹出 Telegram 原生选群器，选中的群会写入 default_chat_id。",
        input_placeholder: "选择目标群组",
        subcommand: "set-default",
        expected_parts: 1,
        example_command: "/targets set-default -1001234567890",
        copy_label: "复制默认",
        interaction_detail: "选默认：通过 Telegram 原生选群器选择目标群组。",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsSetRoute,
        callback_action: TargetsCallbackAction::InputSetRoute,
        button_label: "设路由",
        input_title: "设置请求路由",
        input_detail: "请回复 request_chat_id 和 target_chat_id，例如 123456789 -1001234567890。",
        input_placeholder: "输入 request_chat_id target_chat_id",
        subcommand: "set-route",
        expected_parts: 2,
        example_command: "/targets set-route 123456789 -1001234567890",
        copy_label: "复制路由",
        interaction_detail: "设路由：回复 request_chat_id target_chat_id。",
    },
    TargetsInputSpec {
        action: super::menu::AdminInputAction::TargetsPickRoute,
        callback_action: TargetsCallbackAction::PickSetRoute,
        button_label: "选路由",
        input_title: "选择请求路由目标",
        input_detail: "先回复 request_chat_id，再通过 Telegram 原生选群器选择目标群组。",
        input_placeholder: "输入 request_chat_id，随后选择群组",
        subcommand: "set-route",
        expected_parts: 2,
        example_command: "/targets set-route 123456789 -1001234567890",
        copy_label: "复制路由",
        interaction_detail: "选路由：先回复 request_chat_id，再用原生选群器选择目标群组。",
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
        input_detail: "请回复 alias 和 target_chat_id，例如 archive -1001234567890。",
        input_placeholder: "输入 alias target_chat_id",
        subcommand: "set-alias",
        expected_parts: 2,
        example_command: "/targets set-alias archive -1001234567890",
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
    action: TargetsCallbackAction,
) -> Option<&'static TargetsInputSpec> {
    TARGETS_INPUT_SPECS
        .iter()
        .find(|spec| spec.callback_action == action)
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
    let mut items = vec!["刷新 / 重置默认 / 清空默认：直接点按钮执行。".to_owned()];
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

    let action_result = match action {
        TargetsCallbackAction::Refresh => Ok(()),
        TargetsCallbackAction::Reset => reset_targets_config_to_default_on(app).await.map(|_| ()),
        TargetsCallbackAction::ClearDefault => {
            update_targets_with_on(app, &cleared_action_title("默认目标"), |config| {
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
            let Some(spec) = targets_input_spec_for_callback_action(action) else {
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

/// 格式化 targets 配置卡片。
fn format_targets_config_text(title: &str, config: &TargetsConfig) -> String {
    let mut lines = build_runtime_admin_page_intro(title, TARGETS_PAGE_DETAIL);
    lines.extend([
        card::section("默认目标"),
        card::field("default_chat_id", config.default_chat_id),
        String::new(),
        card::section("请求路由"),
    ]);

    if config.by_request_chat_id.is_empty() {
        lines.push(card::note("当前没有按请求 chat 的单独路由。"));
    } else {
        let mut routes = config
            .by_request_chat_id
            .iter()
            .map(|(request_chat_id, target_chat_id)| (*request_chat_id, *target_chat_id))
            .collect::<Vec<_>>();
        routes.sort_by_key(|(request_chat_id, _)| *request_chat_id);
        for (request_chat_id, target_chat_id) in routes {
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
        let mut aliases = config
            .aliases
            .iter()
            .map(|(alias, target_chat_id)| (alias.clone(), *target_chat_id))
            .collect::<Vec<_>>();
        aliases.sort_by(|left, right| left.0.cmp(&right.0));
        for (alias, target_chat_id) in aliases {
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
    let mut rows = vec![vec![
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
    ]];
    if config.default_chat_id != 0 {
        rows[0].push(send::build_callback_button(
            "清空默认",
            &build_targets_callback_data(TargetsCallbackAction::ClearDefault),
            tdlib_rs::enums::ButtonStyle::Danger,
        ));
    }
    rows.push(build_targets_input_row(&[
        TARGETS_INPUT_SPECS[0],
        TARGETS_INPUT_SPECS[1],
    ]));
    rows.push(build_targets_input_row(&[
        TARGETS_INPUT_SPECS[2],
        TARGETS_INPUT_SPECS[3],
        TARGETS_INPUT_SPECS[4],
    ]));
    rows.push(build_targets_input_row(&[
        TARGETS_INPUT_SPECS[5],
        TARGETS_INPUT_SPECS[6],
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
    rows.push(build_targets_copy_row([
        ("复制 show", targets_show_command(CommandStyle::Long)),
        targets_copy_item_for_action(super::menu::AdminInputAction::TargetsSetDefault),
    ]));
    rows.push(build_targets_copy_row([
        targets_copy_item_for_action(super::menu::AdminInputAction::TargetsSetRoute),
        targets_copy_item_for_action(super::menu::AdminInputAction::TargetsSetAlias),
    ]));
    rows
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
                &build_targets_callback_data(spec.callback_action),
                tdlib_rs::enums::ButtonStyle::Default,
            )
        })
        .collect()
}

/// 构造 targets 复制按钮行。
fn build_targets_copy_row<const N: usize>(
    items: [(&str, String); N],
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    items
        .into_iter()
        .map(|(label, command)| {
            send::build_copy_button(label, &command, tdlib_rs::enums::ButtonStyle::Default)
        })
        .collect()
}

/// 按输入动作构造 targets 复制按钮定义。
fn targets_copy_item_for_action(action: super::menu::AdminInputAction) -> (&'static str, String) {
    let spec = targets_input_spec_for_admin_action(action).expect("targets input spec exists");
    (spec.copy_label, spec.example_command.to_owned())
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
        _ => None,
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
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn test_targets_callback_roundtrip() {
        let refresh = build_targets_callback_data(TargetsCallbackAction::Refresh);
        let reset = build_targets_callback_data(TargetsCallbackAction::Reset);
        let clear = build_targets_callback_data(TargetsCallbackAction::ClearDefault);
        let pick_default = build_targets_callback_data(TargetsCallbackAction::PickSetDefault);
        let pick_route = build_targets_callback_data(TargetsCallbackAction::PickSetRoute);

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
        assert_eq!(rows[0][2].text, "清空默认");
        assert!(rows.iter().flatten().any(|button| button.text == "设默认"));
        assert!(rows.iter().flatten().any(|button| button.text == "选默认"));
        assert!(rows.iter().flatten().any(|button| button.text == "选路由"));
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
