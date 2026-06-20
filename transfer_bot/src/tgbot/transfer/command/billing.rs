// `/billing` 命令：
// - 管理运行时积分计费配置和首页公告
// - 修改后写库并同步刷新运行时计费状态与首页公告

use crate::config::BillingConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;

use super::common::{
    CommandStyle, RuntimeAdminHelpCopyButton, RuntimeAdminHelpDescriptor, RuntimeAdminUsageItem,
    billing_show_command, build_command_examples, build_copy_only_row, build_help_menu_row,
    build_runtime_admin_page_intro, cleared_action_title, command_root,
    edit_runtime_admin_interaction_card_or_error, reset_action_title,
    send_runtime_admin_callback_error, updated_action_title,
};
/// 计费页标题。
const BILLING_PAGE_TITLE: &str = "计费配置";
/// 计费页简要说明。
const BILLING_PAGE_DETAIL: &str =
    "按钮可直接开关或微调；点“设基础 / 设单项 / 设初始 / 设公告”后回复一个值。";

/// `/billing` callback 前缀。
const BILLING_CALLBACK_PREFIX: &str = "bcfg:";

/// `/billing` callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BillingCallbackAction {
    Refresh,
    Reset,
    ToggleEnabled,
    Adjust {
        field: BillingNumericField,
        delta: i64,
    },
    ClearAnnouncement,
    InputSetNumeric {
        field: BillingNumericField,
    },
    InputSetAnnouncement,
}

impl BillingCallbackAction {
    fn started_tip(self) -> &'static str {
        match self {
            Self::Refresh => "正在刷新计费配置",
            Self::Reset => "正在重置计费配置",
            Self::ToggleEnabled => "正在更新计费开关",
            Self::Adjust { .. } => "正在更新计费参数",
            Self::ClearAnnouncement => "正在清空公告",
            Self::InputSetNumeric { .. } => "请回复计费参数",
            Self::InputSetAnnouncement => "请回复公告内容",
        }
    }
}

/// `/billing` 可微调的数值字段。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::tgbot::transfer::command) enum BillingNumericField {
    BaseCost,
    ItemCost,
    InitialPoints,
}

impl BillingNumericField {
    fn spec(self) -> &'static BillingNumericSpec {
        BILLING_NUMERIC_SPECS
            .iter()
            .find(|spec| spec.field == self)
            .expect("billing numeric spec must exist")
    }

    fn parse_code(code: &str) -> Option<Self> {
        BILLING_NUMERIC_SPECS
            .iter()
            .find(|spec| spec.code == code)
            .map(|spec| spec.field)
    }

    fn parse_key(key: &str) -> Option<Self> {
        BILLING_NUMERIC_SPECS
            .iter()
            .find(|spec| spec.key == key)
            .map(|spec| spec.field)
    }
}

/// `/billing` 数值字段规格。
///
/// 按钮、help 示例、输入流命令和 callback payload 都从这里读取，避免计费字段文案漂移。
#[derive(Debug, Clone, Copy)]
pub(in crate::tgbot::transfer::command) struct BillingNumericSpec {
    field: BillingNumericField,
    code: &'static str,
    pub key: &'static str,
    title: &'static str,
    short_label: &'static str,
    pub input_label: &'static str,
    pub input_title: &'static str,
    pub input_detail: &'static str,
    pub input_placeholder: &'static str,
    pub example_value: i64,
    adjust_step: i64,
    pub copy_label: &'static str,
    pub admin_input_action: super::menu::AdminInputAction,
}

/// `/billing` 当前支持微调和输入的数值字段。
pub(in crate::tgbot::transfer::command) const BILLING_NUMERIC_SPECS: &[BillingNumericSpec] = &[
    BillingNumericSpec {
        field: BillingNumericField::BaseCost,
        code: "b",
        key: "base_cost_points",
        title: "基础扣分",
        short_label: "基础",
        input_label: "设基础",
        input_title: "设置基础扣分",
        input_detail: "请回复非负整数，例如 1；或发送 /cancel 取消。",
        input_placeholder: "输入非负整数，或发送 /cancel",
        example_value: 1,
        adjust_step: 1,
        copy_label: "复制基础扣分",
        admin_input_action: super::menu::AdminInputAction::BillingSetBaseCost,
    },
    BillingNumericSpec {
        field: BillingNumericField::ItemCost,
        code: "i",
        key: "item_cost_points",
        title: "单项扣分",
        short_label: "单项",
        input_label: "设单项",
        input_title: "设置单项扣分",
        input_detail: "请回复非负整数，例如 1；或发送 /cancel 取消。",
        input_placeholder: "输入非负整数，或发送 /cancel",
        example_value: 1,
        adjust_step: 1,
        copy_label: "复制单项扣分",
        admin_input_action: super::menu::AdminInputAction::BillingSetItemCost,
    },
    BillingNumericSpec {
        field: BillingNumericField::InitialPoints,
        code: "n",
        key: "initial_user_points",
        title: "新用户初始积分",
        short_label: "初始",
        input_label: "设初始",
        input_title: "设置新用户初始积分",
        input_detail: "请回复非负整数，例如 100；或发送 /cancel 取消。",
        input_placeholder: "输入非负整数，或发送 /cancel",
        example_value: 10,
        adjust_step: 10,
        copy_label: "复制初始积分",
        admin_input_action: super::menu::AdminInputAction::BillingSetInitialUserPoints,
    },
];

/// `/billing` 公告输入规格。
#[derive(Debug, Clone, Copy)]
pub(in crate::tgbot::transfer::command) struct BillingAnnouncementSpec {
    pub key: &'static str,
    pub input_label: &'static str,
    pub input_title: &'static str,
    pub input_detail: &'static str,
    pub input_placeholder: &'static str,
    pub example_command: &'static str,
    pub copy_label: &'static str,
    pub admin_input_action: super::menu::AdminInputAction,
}

/// 首页公告是自由文本，单独建规格，避免和数值字段混在一起。
pub(in crate::tgbot::transfer::command) const BILLING_ANNOUNCEMENT_SPEC: BillingAnnouncementSpec =
    BillingAnnouncementSpec {
        key: "announcement_text",
        input_label: "设公告",
        input_title: "设置首页公告",
        input_detail: "请回复公告全文；或发送 /cancel 取消。",
        input_placeholder: "输入公告内容，或发送 /cancel",
        example_command: "/billing set announcement_text welcome",
        copy_label: "复制公告",
        admin_input_action: super::menu::AdminInputAction::BillingSetAnnouncement,
    };

/// 根据菜单输入动作反查 `/billing` 数值字段规格。
pub(in crate::tgbot::transfer::command) fn billing_numeric_spec_for_admin_action(
    action: super::menu::AdminInputAction,
) -> Option<&'static BillingNumericSpec> {
    BILLING_NUMERIC_SPECS
        .iter()
        .find(|spec| spec.admin_input_action == action)
}

/// 判断菜单输入动作是否是公告输入。
pub(in crate::tgbot::transfer::command) fn billing_announcement_spec_for_admin_action(
    action: super::menu::AdminInputAction,
) -> Option<&'static BillingAnnouncementSpec> {
    (BILLING_ANNOUNCEMENT_SPEC.admin_input_action == action).then_some(&BILLING_ANNOUNCEMENT_SPEC)
}

/// 在指定上下文上执行 `/billing` 文本命令。
pub async fn billing_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let reply = match text.get(1).copied() {
        None | Some("show") => format_billing_text_on(app, BILLING_PAGE_TITLE),
        Some("reset") => reset_billing_to_default_on(app).await?,
        Some("set") => {
            let key = text
                .get(2)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("usage: /billing set <key> <value>"))?;
            let value = text
                .get(3..)
                .map(|parts| parts.join(" "))
                .filter(|value| !value.is_empty())
                .ok_or_else(|| anyhow::anyhow!("usage: /billing set <key> <value>"))?;
            update_billing_key_on(app, key, &value).await?
        }
        Some("clear") => {
            let key = text
                .get(2)
                .copied()
                .ok_or_else(|| anyhow::anyhow!("usage: /billing clear announcement_text"))?;
            match key {
                "announcement_text" => {
                    update_billing_with_on(app, &cleared_action_title("公告"), |config| {
                        config.announcement_text = None;
                    })
                    .await?
                }
                other => anyhow::bail!("unsupported billing clear key: {}", other),
            }
        }
        Some(other) => anyhow::bail!("unknown billing subcommand: {}", other),
    };

    send::ReplyPanel::card(reply)
        .rows(build_billing_buttons())
        .send(request_chat_id, client_id)
        .await
}

/// `billing` 管理页的最小帮助 descriptor。
pub(in crate::tgbot::transfer::command) fn billing_help_descriptor() -> RuntimeAdminHelpDescriptor {
    RuntimeAdminHelpDescriptor {
        synopsis: format!(
            "{} [show|reset|set <key> <value>|clear announcement_text]",
            command_root("billing", CommandStyle::Long)
        ),
        usage_items: vec![
            RuntimeAdminUsageItem {
                command: billing_show_command(CommandStyle::Long),
                detail: "显示当前计费配置。".to_owned(),
            },
            RuntimeAdminUsageItem {
                command: "/billing reset".to_owned(),
                detail: "把计费配置重置为启动配置中的默认值，并立即生效。".to_owned(),
            },
        ],
        interaction_items: billing_interaction_items(),
        example_commands: billing_example_commands(),
        help_copy_buttons: billing_help_copy_buttons(),
    }
}

/// `/billing` 帮助页和卡片共用的交互说明。
fn billing_interaction_items() -> Vec<String> {
    vec![
        "开启/关闭计费、基础扣分增减、单项扣分增减、新用户积分增减、清空公告：直接点按钮执行。"
            .to_owned(),
        "设基础 / 设单项 / 设初始：进入输入流，回复一个非负整数。".to_owned(),
        "设公告：进入输入流，回复公告全文。".to_owned(),
    ]
}

/// `/billing` 帮助页和卡片共用的示例命令。
fn billing_example_commands() -> Vec<String> {
    let mut commands = vec![
        billing_show_command(CommandStyle::Long),
        "/billing reset".to_owned(),
        "/billing set enabled true".to_owned(),
    ];
    commands.extend(
        BILLING_NUMERIC_SPECS
            .iter()
            .map(|spec| format!("/billing set {} {}", spec.key, spec.example_value)),
    );
    commands.extend([
        BILLING_ANNOUNCEMENT_SPEC.example_command.to_owned(),
        "/billing clear announcement_text".to_owned(),
    ]);
    commands
}

/// `/billing` help 详情页复制按钮。
fn billing_help_copy_buttons() -> Vec<RuntimeAdminHelpCopyButton> {
    let mut buttons = vec![
        RuntimeAdminHelpCopyButton::new(
            "复制 show",
            "/billing show",
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        RuntimeAdminHelpCopyButton::new(
            "复制开关",
            "/billing set enabled true",
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ];
    if let Some(spec) = BILLING_NUMERIC_SPECS.first() {
        buttons.push(RuntimeAdminHelpCopyButton::new(
            spec.copy_label,
            format!("/billing set {} {}", spec.key, spec.example_value),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    buttons.push(RuntimeAdminHelpCopyButton::new(
        BILLING_ANNOUNCEMENT_SPEC.copy_label,
        BILLING_ANNOUNCEMENT_SPEC.example_command,
        tdlib_rs::enums::ButtonStyle::Default,
    ));
    buttons
}

/// 判断 callback payload 是否属于 `/billing`。
pub(super) fn is_billing_callback_data(data: &str) -> bool {
    data.starts_with(BILLING_CALLBACK_PREFIX)
}

/// 在指定上下文上处理 `/billing` callback。
pub async fn billing_callback_query_on(
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

    let Some(action) = parse_billing_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("计费配置按钮参数无效"), client_id).await?;
        return Ok(());
    };
    send::answer_callback_query(update.id, Some(action.started_tip()), client_id).await?;

    let action_result = match action {
        BillingCallbackAction::Refresh => Ok(()),
        BillingCallbackAction::Reset => reset_billing_to_default_on(app).await.map(|_| ()),
        BillingCallbackAction::ToggleEnabled => {
            let enabled = !crate::tgbot::transfer::billing_runtime_config_on(app).enabled;
            update_billing_with_on(app, &updated_action_title("计费开关"), |config| {
                config.enabled = enabled;
            })
            .await
            .map(|_| ())
        }
        BillingCallbackAction::Adjust { field, delta } => {
            adjust_billing_numeric_on(app, field, delta).await
        }
        BillingCallbackAction::ClearAnnouncement => {
            update_billing_with_on(app, &cleared_action_title("公告"), |config| {
                config.announcement_text = None;
            })
            .await
            .map(|_| ())
        }
        BillingCallbackAction::InputSetNumeric { field } => {
            return super::menu::start_admin_input_callback(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                field.spec().admin_input_action,
                client_id,
            )
            .await;
        }
        BillingCallbackAction::InputSetAnnouncement => {
            return super::menu::start_admin_input_callback(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                BILLING_ANNOUNCEMENT_SPEC.admin_input_action,
                client_id,
            )
            .await;
        }
    };
    if let Err(err) = action_result {
        send_billing_callback_error(update.chat_id, client_id, &err).await?;
        return Err(err);
    }

    let (text, keyboard) = send::ReplyPanel::card(format_billing_text_on(app, BILLING_PAGE_TITLE))
        .rows(build_billing_buttons_on(app))
        .into_card_parts()?;
    edit_runtime_admin_interaction_card_or_error(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
        "计费配置",
        "/billing show",
    )
    .await?;
    Ok(())
}

/// 构造当前计费配置文本。
///
/// 菜单页在已经持有 `AppContext` 时优先用这个版本，避免重复抓全局。
pub(super) fn format_billing_text_on(app: &crate::app_context::AppContext, title: &str) -> String {
    format_billing_config_text(
        title,
        &crate::tgbot::transfer::billing_runtime_config_on(app),
    )
}

async fn reset_billing_to_default_on(
    app: &crate::app_context::AppContext,
) -> anyhow::Result<String> {
    let config = crate::tgbot::transfer::billing_runtime_default_config_on(app);
    persist_billing_config_on(app, &config).await?;
    tracing::info!("billing runtime config reset to startup defaults");
    Ok(format_billing_config_text(
        &reset_action_title("计费配置"),
        &config,
    ))
}

async fn update_billing_key_on(
    app: &crate::app_context::AppContext,
    key: &str,
    value: &str,
) -> anyhow::Result<String> {
    match key {
        "enabled" => {
            let enabled = parse_bool_arg(value)?;
            update_billing_with_on(app, &updated_action_title("计费开关"), |config| {
                config.enabled = enabled;
            })
            .await
        }
        key if BillingNumericField::parse_key(key).is_some() => {
            let field = BillingNumericField::parse_key(key).expect("billing numeric key checked");
            let spec = field.spec();
            let points = parse_non_negative_i64(value, spec.key)?;
            update_billing_with_on(app, &updated_action_title(spec.title), |config| {
                set_billing_numeric_value(config, field, points);
            })
            .await
        }
        key if key == BILLING_ANNOUNCEMENT_SPEC.key => {
            let announcement = value.trim();
            if announcement.is_empty() {
                anyhow::bail!("announcement_text cannot be empty");
            }
            update_billing_with_on(app, &updated_action_title("公告"), |config| {
                config.announcement_text = Some(announcement.to_owned());
            })
            .await
        }
        other => anyhow::bail!("unsupported billing key: {}", other),
    }
}

async fn update_billing_with_on(
    app: &crate::app_context::AppContext,
    title: &str,
    updater: impl FnOnce(&mut BillingConfig),
) -> anyhow::Result<String> {
    let mut config = crate::tgbot::transfer::billing_runtime_config_on(app);
    updater(&mut config);
    persist_billing_config_on(app, &config).await?;
    tracing::info!(
        enabled = config.enabled,
        base_cost_points = config.base_cost_points,
        item_cost_points = config.item_cost_points,
        initial_user_points = config.initial_user_points,
        has_announcement = config.announcement_text.is_some(),
        "billing runtime config updated"
    );
    Ok(format_billing_config_text(title, &config))
}

async fn adjust_billing_numeric_on(
    app: &crate::app_context::AppContext,
    field: BillingNumericField,
    delta: i64,
) -> anyhow::Result<()> {
    let mut config = crate::tgbot::transfer::billing_runtime_config_on(app);
    let current = billing_numeric_value(&config, field);
    let next = current.saturating_add(delta).max(0);
    set_billing_numeric_value(&mut config, field, next);
    persist_billing_config_on(app, &config).await?;
    tracing::info!(
        field = field.spec().key,
        delta,
        enabled = config.enabled,
        base_cost_points = config.base_cost_points,
        item_cost_points = config.item_cost_points,
        initial_user_points = config.initial_user_points,
        "billing runtime config adjusted by callback"
    );
    Ok(())
}

/// 读取计费数值字段。
fn billing_numeric_value(config: &BillingConfig, field: BillingNumericField) -> i64 {
    match field {
        BillingNumericField::BaseCost => config.base_cost_points,
        BillingNumericField::ItemCost => config.item_cost_points,
        BillingNumericField::InitialPoints => config.initial_user_points,
    }
}

/// 写入计费数值字段。
fn set_billing_numeric_value(config: &mut BillingConfig, field: BillingNumericField, value: i64) {
    match field {
        BillingNumericField::BaseCost => config.base_cost_points = value,
        BillingNumericField::ItemCost => config.item_cost_points = value,
        BillingNumericField::InitialPoints => config.initial_user_points = value,
    }
}

async fn persist_billing_config_on(
    app: &crate::app_context::AppContext,
    config: &BillingConfig,
) -> anyhow::Result<()> {
    crate::tgbot::transfer::save_billing_runtime_config(config).await?;
    crate::tgbot::transfer::update_billing_runtime_config_on(app, config.clone());
    app.home_announcement
        .set_announcement_text(config.announcement_text.clone());
    Ok(())
}

fn format_billing_config_text(title: &str, config: &BillingConfig) -> String {
    let mut lines = build_runtime_admin_page_intro(title, BILLING_PAGE_DETAIL);
    lines.extend([
        card::section("计费"),
        card::field("enabled", if config.enabled { "true" } else { "false" }),
        card::field("base_cost_points", config.base_cost_points),
        card::field("item_cost_points", config.item_cost_points),
        card::field("initial_user_points", config.initial_user_points),
        String::new(),
        card::section("公告"),
        match &config.announcement_text {
            Some(text) => card::field("announcement_text", text),
            None => card::note("当前没有公告。"),
        },
    ]);
    lines.extend(build_command_examples(
        billing_example_commands()
            .into_iter()
            .filter(|command| command != "/billing reset"),
    ));
    lines.join("\n")
}

pub(super) fn build_billing_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let app_context = crate::app_context::app_context();
    build_billing_buttons_on(app_context.as_ref())
}

/// `/billing` 页按钮的上下文版本。
pub(super) fn build_billing_buttons_on(
    app: &crate::app_context::AppContext,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let config = crate::tgbot::transfer::billing_runtime_config_on(app);
    let enabled_label = if config.enabled {
        "关闭计费"
    } else {
        "开启计费"
    };
    let mut rows = vec![
        vec![
            send::build_callback_button(
                enabled_label,
                &build_billing_callback_data(BillingCallbackAction::ToggleEnabled),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "刷新",
                &build_billing_callback_data(BillingCallbackAction::Refresh),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "重置默认",
                &build_billing_callback_data(BillingCallbackAction::Reset),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_billing_adjust_row(BillingNumericField::BaseCost),
        build_billing_adjust_row(BillingNumericField::ItemCost),
        build_billing_adjust_row(BillingNumericField::InitialPoints),
        vec![
            send::build_callback_button(
                "清空公告",
                &build_billing_callback_data(BillingCallbackAction::ClearAnnouncement),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            build_billing_input_button(BillingNumericField::BaseCost),
            build_billing_input_button(BillingNumericField::ItemCost),
        ],
        vec![
            build_billing_input_button(BillingNumericField::InitialPoints),
            send::build_callback_button(
                BILLING_ANNOUNCEMENT_SPEC.input_label,
                &build_billing_callback_data(BillingCallbackAction::InputSetAnnouncement),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_help_menu_row(
            send::build_callback_button(
                "帮助",
                &super::help::build_help_callback_data(Some("billing")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &super::build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ),
        vec![
            send::build_copy_button(
                "复制 show",
                "/billing show",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            build_billing_announcement_copy_button(),
        ],
    ];
    if config.announcement_text.is_some() {
        rows.push(build_copy_only_row(send::build_copy_button(
            "复制清空",
            "/billing clear announcement_text",
            tdlib_rs::enums::ButtonStyle::Default,
        )));
    }
    rows
}

/// 构造计费数值字段微调按钮行。
fn build_billing_adjust_row(
    field: BillingNumericField,
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    let spec = field.spec();
    vec![
        send::build_callback_button(
            &format!("{} -{}", spec.short_label, spec.adjust_step),
            &build_billing_callback_data(BillingCallbackAction::Adjust {
                field,
                delta: -spec.adjust_step,
            }),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            &format!("{} +{}", spec.short_label, spec.adjust_step),
            &build_billing_callback_data(BillingCallbackAction::Adjust {
                field,
                delta: spec.adjust_step,
            }),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]
}

/// 构造计费数值字段输入按钮。
fn build_billing_input_button(field: BillingNumericField) -> tdlib_rs::types::InlineKeyboardButton {
    let spec = field.spec();
    send::build_callback_button(
        spec.input_label,
        // 数值输入也走 `/billing` 自己的 callback 前缀，保持本页按钮协议统一。
        &build_billing_callback_data(BillingCallbackAction::InputSetNumeric { field }),
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

/// 构造公告复制按钮。
fn build_billing_announcement_copy_button() -> tdlib_rs::types::InlineKeyboardButton {
    send::build_copy_button(
        BILLING_ANNOUNCEMENT_SPEC.copy_label,
        BILLING_ANNOUNCEMENT_SPEC.example_command,
        tdlib_rs::enums::ButtonStyle::Default,
    )
}

fn parse_billing_callback_data(data: &str) -> Option<BillingCallbackAction> {
    let payload = data.strip_prefix(BILLING_CALLBACK_PREFIX)?;
    let mut parts = payload.split(':');
    match parts.next()? {
        "r" => {
            if parts.next().is_none() {
                Some(BillingCallbackAction::Refresh)
            } else {
                None
            }
        }
        "x" => {
            if parts.next().is_none() {
                Some(BillingCallbackAction::Reset)
            } else {
                None
            }
        }
        "e" => {
            if parts.next().is_none() {
                Some(BillingCallbackAction::ToggleEnabled)
            } else {
                None
            }
        }
        "s" => {
            if parts.next().is_none() {
                Some(BillingCallbackAction::InputSetAnnouncement)
            } else {
                None
            }
        }
        "i" => {
            let field = BillingNumericField::parse_code(parts.next()?)?;
            if parts.next().is_none() {
                Some(BillingCallbackAction::InputSetNumeric { field })
            } else {
                None
            }
        }
        "c" => {
            if parts.next().is_none() {
                Some(BillingCallbackAction::ClearAnnouncement)
            } else {
                None
            }
        }
        code => {
            let field = BillingNumericField::parse_code(code)?;
            parse_delta(parts).map(|delta| BillingCallbackAction::Adjust { field, delta })
        }
    }
}

fn parse_delta<'a>(mut parts: impl Iterator<Item = &'a str>) -> Option<i64> {
    let delta = parts.next()?.parse::<i64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(delta)
}

fn build_billing_callback_data(action: BillingCallbackAction) -> String {
    match action {
        BillingCallbackAction::Refresh => format!("{BILLING_CALLBACK_PREFIX}r"),
        BillingCallbackAction::Reset => format!("{BILLING_CALLBACK_PREFIX}x"),
        BillingCallbackAction::ToggleEnabled => format!("{BILLING_CALLBACK_PREFIX}e"),
        BillingCallbackAction::ClearAnnouncement => format!("{BILLING_CALLBACK_PREFIX}c"),
        BillingCallbackAction::InputSetNumeric { field } => {
            format!("{}i:{}", BILLING_CALLBACK_PREFIX, field.spec().code)
        }
        BillingCallbackAction::InputSetAnnouncement => format!("{BILLING_CALLBACK_PREFIX}s"),
        BillingCallbackAction::Adjust { field, delta } => {
            format!("{}{}:{}", BILLING_CALLBACK_PREFIX, field.spec().code, delta)
        }
    }
}

async fn send_billing_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_runtime_admin_callback_error(request_chat_id, client_id, "计费配置", err).await
}

fn parse_bool_arg(value: &str) -> anyhow::Result<bool> {
    match value {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => anyhow::bail!("invalid bool value: {}", value),
    }
}

fn parse_non_negative_i64(value: &str, key: &str) -> anyhow::Result<i64> {
    let parsed = value.parse::<i64>()?;
    if parsed < 0 {
        anyhow::bail!("{key} cannot be negative");
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn test_format_billing_text_contains_sections() {
        let text = format_billing_config_text(
            "当前计费配置",
            &BillingConfig {
                enabled: true,
                base_cost_points: 1,
                item_cost_points: 2,
                initial_user_points: 3,
                announcement_text: Some("hello".to_owned()),
            },
        );

        assert!(text.contains("base_cost_points"));
        assert!(text.contains("announcement_text"));
        assert!(text.contains("/billing set enabled true"));
    }

    #[test]
    fn test_parse_non_negative_i64_rejects_negative() {
        assert!(parse_non_negative_i64("-1", "base_cost_points").is_err());
        assert_eq!(parse_non_negative_i64("0", "base_cost_points").unwrap(), 0);
    }

    #[test]
    fn test_billing_callback_roundtrip() {
        let refresh = build_billing_callback_data(BillingCallbackAction::Refresh);
        let toggle = build_billing_callback_data(BillingCallbackAction::ToggleEnabled);
        let base = build_billing_callback_data(BillingCallbackAction::Adjust {
            field: BillingNumericField::BaseCost,
            delta: 1,
        });
        let clear = build_billing_callback_data(BillingCallbackAction::ClearAnnouncement);
        let input_base = build_billing_callback_data(BillingCallbackAction::InputSetNumeric {
            field: BillingNumericField::BaseCost,
        });

        assert!(is_billing_callback_data(&refresh));
        assert_eq!(
            parse_billing_callback_data(&refresh),
            Some(BillingCallbackAction::Refresh)
        );
        assert_eq!(
            parse_billing_callback_data(&toggle),
            Some(BillingCallbackAction::ToggleEnabled)
        );
        assert_eq!(
            parse_billing_callback_data(&base),
            Some(BillingCallbackAction::Adjust {
                field: BillingNumericField::BaseCost,
                delta: 1,
            })
        );
        assert_eq!(
            parse_billing_callback_data(&clear),
            Some(BillingCallbackAction::ClearAnnouncement)
        );
        assert_eq!(
            parse_billing_callback_data(&input_base),
            Some(BillingCallbackAction::InputSetNumeric {
                field: BillingNumericField::BaseCost
            })
        );
        assert_eq!(parse_billing_callback_data("bcfg:bad"), None);
    }

    #[test]
    fn test_build_billing_buttons_use_callback_actions() {
        let app = crate::app_context::app_context();
        app.billing_runtime
            .update_runtime_config(BillingConfig::default());

        let rows = build_billing_buttons();
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("enabled button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "bcfg:e");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[4][1].r#type
        else {
            panic!("base cost input button must be billing callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "bcfg:i:b");
        assert_eq!(rows[6][0].text, "帮助");
        assert_eq!(rows[6][1].text, "菜单");
        assert!(rows.iter().flatten().any(|button| button.text == "设公告"));
    }
}
