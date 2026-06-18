// `/acl` 命令：
// - 管理数据库访问控制白名单/黑名单和普通用户入口策略
// - bootstrap_admin_user_ids 只读，仍由 config.json 兜底

use crate::config::AccessControlConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;

use super::common::{
    CommandStyle, RuntimeAdminHelpCopyButton, RuntimeAdminHelpDescriptor, RuntimeAdminUsageItem,
    acl_show_command, added_action_title, build_command_examples, build_help_menu_row,
    build_runtime_admin_page_intro, command_root, deleted_action_title,
    edit_runtime_admin_interaction_card_or_error, released_action_title, reset_action_title,
    send_runtime_admin_callback_error, updated_action_title,
};
/// ACL 页标题。
const ACL_PAGE_TITLE: &str = "访问控制";
/// ACL 页简要说明。
const ACL_PAGE_DETAIL: &str =
    "管理管理员、普通用户、黑名单和聊天白名单；点输入按钮后按提示回复参数。";

/// `/acl` callback 前缀。
const ACL_CALLBACK_PREFIX: &str = "acfg:";

/// `/acl` callback 动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AclCallbackAction {
    Refresh,
    Reset,
    ToggleAllowAllPrivateUsers,
    InputAddAdmin,
    InputDelAdmin,
    InputAddAllowUser,
    InputDelAllowUser,
    InputAddBan,
    InputDelBan,
    InputAddAllowTarget,
    InputDelAllowTarget,
    InputAddAllowRequest,
    InputDelAllowRequest,
}

impl AclCallbackAction {
    fn started_tip(self) -> &'static str {
        match self {
            Self::Refresh => "正在刷新访问控制",
            Self::Reset => "正在重置访问控制",
            Self::ToggleAllowAllPrivateUsers => "正在更新私聊开放策略",
            Self::InputAddAdmin
            | Self::InputDelAdmin
            | Self::InputAddAllowUser
            | Self::InputDelAllowUser
            | Self::InputAddBan
            | Self::InputDelBan
            | Self::InputAddAllowTarget
            | Self::InputDelAllowTarget
            | Self::InputAddAllowRequest
            | Self::InputDelAllowRequest => "请回复参数",
        }
    }
}

/// `/acl` 单步输入动作规格。
///
/// 访问控制页动作数量较多，统一规格能避免 callback、help 和 ForceReply 文案漂移。
#[derive(Debug, Clone, Copy)]
pub(in crate::tgbot::transfer::command) struct AclInputSpec {
    pub action: super::menu::AdminInputAction,
    callback_action: AclCallbackAction,
    pub button_label: &'static str,
    pub input_title: &'static str,
    pub input_detail: &'static str,
    pub input_placeholder: &'static str,
    pub subcommand: &'static str,
    pub example_command: &'static str,
    pub copy_label: &'static str,
}

/// `/acl` 当前支持的全部输入式动作。
pub(in crate::tgbot::transfer::command) const ACL_INPUT_SPECS: &[AclInputSpec] = &[
    AclInputSpec {
        action: super::menu::AdminInputAction::AclAddAdmin,
        callback_action: AclCallbackAction::InputAddAdmin,
        button_label: "加管理员",
        input_title: "添加管理员",
        input_detail: "请回复 Telegram 用户 ID，例如 123456789；或发送 /cancel 取消。",
        input_placeholder: "输入 user_id，或发送 /cancel",
        subcommand: "add-admin",
        example_command: "/acl add-admin 123456789",
        copy_label: "复制管理员",
    },
    AclInputSpec {
        action: super::menu::AdminInputAction::AclDelAdmin,
        callback_action: AclCallbackAction::InputDelAdmin,
        button_label: "删管理员",
        input_title: "删除管理员",
        input_detail: "请回复 Telegram 用户 ID，例如 123456789；或发送 /cancel 取消。",
        input_placeholder: "输入 user_id，或发送 /cancel",
        subcommand: "del-admin",
        example_command: "/acl del-admin 123456789",
        copy_label: "复制删管理员",
    },
    AclInputSpec {
        action: super::menu::AdminInputAction::AclAddAllowUser,
        callback_action: AclCallbackAction::InputAddAllowUser,
        button_label: "加用户",
        input_title: "添加允许用户",
        input_detail: "请回复 Telegram 用户 ID，例如 123456789；或发送 /cancel 取消。",
        input_placeholder: "输入 user_id，或发送 /cancel",
        subcommand: "add-allow-user",
        example_command: "/acl add-allow-user 123456789",
        copy_label: "复制用户",
    },
    AclInputSpec {
        action: super::menu::AdminInputAction::AclDelAllowUser,
        callback_action: AclCallbackAction::InputDelAllowUser,
        button_label: "删用户",
        input_title: "删除允许用户",
        input_detail: "请回复 Telegram 用户 ID，例如 123456789；或发送 /cancel 取消。",
        input_placeholder: "输入 user_id，或发送 /cancel",
        subcommand: "del-allow-user",
        example_command: "/acl del-allow-user 123456789",
        copy_label: "复制删用户",
    },
    AclInputSpec {
        action: super::menu::AdminInputAction::AclAddBan,
        callback_action: AclCallbackAction::InputAddBan,
        button_label: "封禁",
        input_title: "添加封禁用户",
        input_detail: "请回复 Telegram 用户 ID，例如 123456789；或发送 /cancel 取消。",
        input_placeholder: "输入 user_id，或发送 /cancel",
        subcommand: "add-ban",
        example_command: "/acl add-ban 123456789",
        copy_label: "复制封禁",
    },
    AclInputSpec {
        action: super::menu::AdminInputAction::AclDelBan,
        callback_action: AclCallbackAction::InputDelBan,
        button_label: "解封",
        input_title: "解除封禁用户",
        input_detail: "请回复 Telegram 用户 ID，例如 123456789；或发送 /cancel 取消。",
        input_placeholder: "输入 user_id，或发送 /cancel",
        subcommand: "del-ban",
        example_command: "/acl del-ban 123456789",
        copy_label: "复制解封",
    },
    AclInputSpec {
        action: super::menu::AdminInputAction::AclAddAllowTarget,
        callback_action: AclCallbackAction::InputAddAllowTarget,
        button_label: "加目标",
        input_title: "添加目标白名单",
        input_detail: "请回复 chat_id，例如 -1001234567890；或发送 /cancel 取消。",
        input_placeholder: "输入 chat_id，或发送 /cancel",
        subcommand: "add-allow-target",
        example_command: "/acl add-allow-target -1001234567890",
        copy_label: "复制目标",
    },
    AclInputSpec {
        action: super::menu::AdminInputAction::AclDelAllowTarget,
        callback_action: AclCallbackAction::InputDelAllowTarget,
        button_label: "删目标",
        input_title: "删除目标白名单",
        input_detail: "请回复 chat_id，例如 -1001234567890；或发送 /cancel 取消。",
        input_placeholder: "输入 chat_id，或发送 /cancel",
        subcommand: "del-allow-target",
        example_command: "/acl del-allow-target -1001234567890",
        copy_label: "复制删目标",
    },
    AclInputSpec {
        action: super::menu::AdminInputAction::AclAddAllowRequest,
        callback_action: AclCallbackAction::InputAddAllowRequest,
        button_label: "加请求",
        input_title: "添加请求白名单",
        input_detail: "请回复 chat_id，例如 -1001234567890；或发送 /cancel 取消。",
        input_placeholder: "输入 chat_id，或发送 /cancel",
        subcommand: "add-allow-request",
        example_command: "/acl add-allow-request -1001234567890",
        copy_label: "复制请求",
    },
    AclInputSpec {
        action: super::menu::AdminInputAction::AclDelAllowRequest,
        callback_action: AclCallbackAction::InputDelAllowRequest,
        button_label: "删请求",
        input_title: "删除请求白名单",
        input_detail: "请回复 chat_id，例如 -1001234567890；或发送 /cancel 取消。",
        input_placeholder: "输入 chat_id，或发送 /cancel",
        subcommand: "del-allow-request",
        example_command: "/acl del-allow-request -1001234567890",
        copy_label: "复制删请求",
    },
];

/// 根据菜单输入动作反查 `/acl` 输入规格。
pub(in crate::tgbot::transfer::command) fn acl_input_spec_for_admin_action(
    action: super::menu::AdminInputAction,
) -> Option<&'static AclInputSpec> {
    ACL_INPUT_SPECS.iter().find(|spec| spec.action == action)
}

/// 根据 callback 动作反查 `/acl` 输入规格。
fn acl_input_spec_for_callback_action(action: AclCallbackAction) -> Option<&'static AclInputSpec> {
    ACL_INPUT_SPECS
        .iter()
        .find(|spec| spec.callback_action == action)
}

/// `/acl` 命令入口。
pub async fn acl_command(
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let reply = match text.get(1).copied() {
        None | Some("show") => format_acl_text(ACL_PAGE_TITLE),
        Some("reset") => reset_acl_to_default().await?,
        Some("add-admin") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl add-admin <user_id>")?;
            update_acl_with(&added_action_title("管理员"), |config| {
                push_unique(&mut config.admin_user_ids, user_id);
                config.banned_user_ids.retain(|id| *id != user_id);
            })
            .await?
        }
        Some("del-admin") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl del-admin <user_id>")?;
            update_acl_with(&deleted_action_title("管理员"), |config| {
                config.admin_user_ids.retain(|id| *id != user_id);
            })
            .await?
        }
        Some("add-allow-user") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl add-allow-user <user_id>")?;
            update_acl_with(&added_action_title("允许用户"), |config| {
                push_unique(&mut config.allowed_user_ids, user_id);
            })
            .await?
        }
        Some("del-allow-user") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl del-allow-user <user_id>")?;
            update_acl_with(&deleted_action_title("允许用户"), |config| {
                config.allowed_user_ids.retain(|id| *id != user_id);
            })
            .await?
        }
        Some("add-ban") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl add-ban <user_id>")?;
            update_acl_with(&added_action_title("封禁用户"), |config| {
                if config.bootstrap_admin_user_ids.contains(&user_id) {
                    return;
                }
                config.admin_user_ids.retain(|id| *id != user_id);
                config.allowed_user_ids.retain(|id| *id != user_id);
                push_unique(&mut config.banned_user_ids, user_id);
            })
            .await?
        }
        Some("del-ban") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl del-ban <user_id>")?;
            update_acl_with(&released_action_title("封禁用户"), |config| {
                config.banned_user_ids.retain(|id| *id != user_id);
            })
            .await?
        }
        Some("set") => {
            let key = text.get(2).copied().ok_or_else(|| {
                anyhow::anyhow!("usage: /acl set allow_all_private_users <true|false>")
            })?;
            let value = text.get(3).copied().ok_or_else(|| {
                anyhow::anyhow!("usage: /acl set allow_all_private_users <true|false>")
            })?;
            match key {
                "allow_all_private_users" => {
                    let enabled = parse_bool_arg(value)?;
                    update_acl_with(&updated_action_title("私聊开放策略"), |config| {
                        config.allow_all_private_users = enabled;
                    })
                    .await?
                }
                other => anyhow::bail!("unsupported acl key: {}", other),
            }
        }
        Some("add-allow-target") => {
            let chat_id = parse_i64_arg(&text, 2, "usage: /acl add-allow-target <chat_id>")?;
            update_acl_with(&added_action_title("目标白名单"), |config| {
                push_unique(&mut config.allowed_target_chat_ids, chat_id);
            })
            .await?
        }
        Some("del-allow-target") => {
            let chat_id = parse_i64_arg(&text, 2, "usage: /acl del-allow-target <chat_id>")?;
            update_acl_with(&deleted_action_title("目标白名单"), |config| {
                config.allowed_target_chat_ids.retain(|id| *id != chat_id);
            })
            .await?
        }
        Some("add-allow-request") => {
            let chat_id = parse_i64_arg(&text, 2, "usage: /acl add-allow-request <chat_id>")?;
            update_acl_with(&added_action_title("请求白名单"), |config| {
                push_unique(&mut config.allowed_request_chat_ids, chat_id);
            })
            .await?
        }
        Some("del-allow-request") => {
            let chat_id = parse_i64_arg(&text, 2, "usage: /acl del-allow-request <chat_id>")?;
            update_acl_with(&deleted_action_title("请求白名单"), |config| {
                config.allowed_request_chat_ids.retain(|id| *id != chat_id);
            })
            .await?
        }
        Some(other) => anyhow::bail!("unknown acl subcommand: {}", other),
    };

    send::ReplyPanel::card(reply)
        .rows(build_acl_buttons())
        .send(request_chat_id, client_id)
        .await
}

/// `acl` 管理页的最小帮助 descriptor。
pub(in crate::tgbot::transfer::command) fn acl_help_descriptor() -> RuntimeAdminHelpDescriptor {
    RuntimeAdminHelpDescriptor {
        synopsis: format!(
            "{} [show|reset|add-admin|del-admin|add-allow-user|del-allow-user|add-ban|del-ban|set allow_all_private_users <bool>|add-allow-target|del-allow-target]",
            command_root("acl", CommandStyle::Long)
        ),
        usage_items: vec![
            RuntimeAdminUsageItem {
                command: acl_show_command(CommandStyle::Long),
                detail: "显示当前访问控制配置。".to_owned(),
            },
            RuntimeAdminUsageItem {
                command: "/acl reset".to_owned(),
                detail: "把访问控制配置重置为启动配置中的默认值，并立即生效。".to_owned(),
            },
        ],
        interaction_items: acl_interaction_items(),
        example_commands: acl_example_commands(),
        help_copy_buttons: acl_help_copy_buttons(),
    }
}

/// `/acl` 帮助页和卡片共用的交互说明。
fn acl_interaction_items() -> Vec<String> {
    vec![
        "开放/关闭任意私聊、刷新、重置默认：直接点按钮执行。".to_owned(),
        "加管理员 / 删管理员 / 加用户 / 删用户 / 封禁 / 解封：回复 user_id。".to_owned(),
        "加目标 / 删目标 / 加请求 / 删请求：回复 chat_id。".to_owned(),
    ]
}

/// `/acl` 帮助页和卡片共用的示例命令。
fn acl_example_commands() -> Vec<String> {
    let mut commands = vec![
        acl_show_command(CommandStyle::Long),
        "/acl reset".to_owned(),
        "/acl set allow_all_private_users true".to_owned(),
    ];
    commands.extend(
        ACL_INPUT_SPECS
            .iter()
            .map(|spec| spec.example_command.to_owned()),
    );
    commands
}

/// `/acl` help 详情页复制按钮。
fn acl_help_copy_buttons() -> Vec<RuntimeAdminHelpCopyButton> {
    let mut buttons = vec![RuntimeAdminHelpCopyButton::new(
        "复制 show",
        "/acl show",
        tdlib_rs::enums::ButtonStyle::Primary,
    )];
    buttons.extend(
        ACL_INPUT_SPECS
            .iter()
            .filter(|spec| {
                matches!(
                    spec.action,
                    super::menu::AdminInputAction::AclAddAdmin
                        | super::menu::AdminInputAction::AclAddAllowTarget
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

/// 判断 callback payload 是否属于 `/acl`。
pub(super) fn is_acl_callback_data(data: &str) -> bool {
    data.starts_with(ACL_CALLBACK_PREFIX)
}

/// `/acl` inline keyboard 回调入口。
pub(super) async fn acl_callback_query(
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

    let Some(action) = parse_acl_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("访问控制按钮参数无效"), client_id).await?;
        return Ok(());
    };
    send::answer_callback_query(update.id, Some(action.started_tip()), client_id).await?;

    let action_result = match action {
        AclCallbackAction::Refresh => Ok(()),
        AclCallbackAction::Reset => reset_acl_to_default().await.map(|_| ()),
        AclCallbackAction::ToggleAllowAllPrivateUsers => {
            let enabled =
                !crate::tgbot::transfer::access_control_runtime_config().allow_all_private_users;
            update_acl_with(&updated_action_title("私聊开放策略"), |config| {
                config.allow_all_private_users = enabled;
            })
            .await
            .map(|_| ())
        }
        AclCallbackAction::InputAddAdmin
        | AclCallbackAction::InputDelAdmin
        | AclCallbackAction::InputAddAllowUser
        | AclCallbackAction::InputDelAllowUser
        | AclCallbackAction::InputAddBan
        | AclCallbackAction::InputDelBan
        | AclCallbackAction::InputAddAllowTarget
        | AclCallbackAction::InputDelAllowTarget
        | AclCallbackAction::InputAddAllowRequest
        | AclCallbackAction::InputDelAllowRequest => {
            let Some(spec) = acl_input_spec_for_callback_action(action) else {
                anyhow::bail!("missing acl input spec for callback action: {:?}", action);
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
        send_acl_callback_error(update.chat_id, client_id, &err).await?;
        return Err(err);
    }

    let (text, keyboard) = send::ReplyPanel::card(format_acl_text(ACL_PAGE_TITLE))
        .rows(build_acl_buttons())
        .into_card_parts()?;
    edit_runtime_admin_interaction_card_or_error(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
        "访问控制",
        "/acl show",
    )
    .await?;
    Ok(())
}

pub(super) fn format_acl_text(title: &str) -> String {
    format_acl_config_text(
        title,
        &crate::tgbot::transfer::access_control_runtime_config(),
    )
}

async fn reset_acl_to_default() -> anyhow::Result<String> {
    let config = crate::tgbot::transfer::access_control_runtime_default_config();
    persist_acl_config(&config).await?;
    tracing::info!("access control runtime config reset to startup defaults");
    Ok(format_acl_config_text(
        &reset_action_title("访问控制"),
        &config,
    ))
}

async fn update_acl_with(
    title: &str,
    updater: impl FnOnce(&mut AccessControlConfig),
) -> anyhow::Result<String> {
    let mut config = crate::tgbot::transfer::access_control_runtime_config();
    updater(&mut config);
    normalize_acl_config(&mut config);
    persist_acl_config(&config).await?;
    tracing::info!(
        admin_count = config.admin_user_ids.len(),
        allowed_user_count = config.allowed_user_ids.len(),
        banned_user_count = config.banned_user_ids.len(),
        allow_all_private_users = config.allow_all_private_users,
        allowed_target_chat_count = config.allowed_target_chat_ids.len(),
        "access control runtime config updated"
    );
    Ok(format_acl_config_text(title, &config))
}

async fn persist_acl_config(config: &AccessControlConfig) -> anyhow::Result<()> {
    crate::tgbot::transfer::save_access_control_runtime_config(config).await?;
    crate::tgbot::transfer::update_access_control_runtime_config(config.clone());
    Ok(())
}

fn normalize_acl_config(config: &mut AccessControlConfig) {
    sort_dedup(&mut config.admin_user_ids);
    sort_dedup(&mut config.allowed_user_ids);
    sort_dedup(&mut config.banned_user_ids);
    sort_dedup(&mut config.allowed_request_chat_ids);
    sort_dedup(&mut config.allowed_target_chat_ids);

    config
        .admin_user_ids
        .retain(|user_id| !config.bootstrap_admin_user_ids.contains(user_id));
    config.banned_user_ids.retain(|user_id| {
        !config.bootstrap_admin_user_ids.contains(user_id)
            && !config.admin_user_ids.contains(user_id)
    });
}

fn format_acl_config_text(title: &str, config: &AccessControlConfig) -> String {
    let mut lines = build_runtime_admin_page_intro(title, ACL_PAGE_DETAIL);
    lines.extend([
        card::section("管理员"),
        format_id_line("bootstrap_admin_user_ids", &config.bootstrap_admin_user_ids),
        format_id_line("admin_user_ids", &config.admin_user_ids),
        String::new(),
        card::section("普通用户入口"),
        card::field(
            "allow_all_private_users",
            if config.allow_all_private_users {
                "true"
            } else {
                "false"
            },
        ),
        format_id_line("allowed_user_ids", &config.allowed_user_ids),
        format_id_line("banned_user_ids", &config.banned_user_ids),
        String::new(),
        card::section("聊天白名单"),
        format_id_line("allowed_request_chat_ids", &config.allowed_request_chat_ids),
        format_id_line("allowed_target_chat_ids", &config.allowed_target_chat_ids),
    ]);
    lines.extend(build_command_examples(
        acl_example_commands()
            .into_iter()
            .filter(|command| command != "/acl reset"),
    ));
    lines.join("\n")
}

pub(super) fn build_acl_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let config = crate::tgbot::transfer::access_control_runtime_config();
    let allow_all_label = if config.allow_all_private_users {
        "关闭任意私聊"
    } else {
        "开放任意私聊"
    };
    vec![
        vec![
            send::build_callback_button(
                allow_all_label,
                &build_acl_callback_data(AclCallbackAction::ToggleAllowAllPrivateUsers),
                tdlib_rs::enums::ButtonStyle::Primary,
            ),
            send::build_callback_button(
                "刷新",
                &build_acl_callback_data(AclCallbackAction::Refresh),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "重置默认",
                &build_acl_callback_data(AclCallbackAction::Reset),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
        build_acl_input_row(&[
            super::menu::AdminInputAction::AclAddAdmin,
            super::menu::AdminInputAction::AclDelAdmin,
        ]),
        build_acl_input_row(&[
            super::menu::AdminInputAction::AclAddAllowUser,
            super::menu::AdminInputAction::AclDelAllowUser,
            super::menu::AdminInputAction::AclAddBan,
        ]),
        build_acl_input_row(&[
            super::menu::AdminInputAction::AclAddAllowTarget,
            super::menu::AdminInputAction::AclDelAllowTarget,
            super::menu::AdminInputAction::AclAddAllowRequest,
        ]),
        build_acl_input_row(&[
            super::menu::AdminInputAction::AclDelAllowRequest,
            super::menu::AdminInputAction::AclDelBan,
        ]),
        build_help_menu_row(
            send::build_callback_button(
                "帮助",
                &super::help::build_help_callback_data(Some("acl")),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "菜单",
                &super::build_menu_home_button_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ),
        build_acl_copy_row([
            ("复制 show", acl_show_command(CommandStyle::Long)),
            acl_copy_item_for_action(super::menu::AdminInputAction::AclAddAdmin),
        ]),
        build_acl_copy_row([
            acl_copy_item_for_action(super::menu::AdminInputAction::AclAddAllowUser),
            acl_copy_item_for_action(super::menu::AdminInputAction::AclAddAllowTarget),
        ]),
        build_acl_copy_row([acl_copy_item_for_action(
            super::menu::AdminInputAction::AclAddBan,
        )]),
    ]
}

/// 构造 ACL 输入按钮行。
fn build_acl_input_row(
    actions: &[super::menu::AdminInputAction],
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    actions
        .iter()
        .map(|action| {
            let spec = acl_input_spec_for_admin_action(*action).expect("acl input spec exists");
            send::build_callback_button(
                spec.button_label,
                &build_acl_callback_data(spec.callback_action),
                tdlib_rs::enums::ButtonStyle::Default,
            )
        })
        .collect()
}

/// 构造 ACL 复制按钮行。
fn build_acl_copy_row<const N: usize>(
    items: [(&str, String); N],
) -> Vec<tdlib_rs::types::InlineKeyboardButton> {
    items
        .into_iter()
        .map(|(label, command)| {
            send::build_copy_button(label, &command, tdlib_rs::enums::ButtonStyle::Default)
        })
        .collect()
}

/// 按输入动作构造 ACL 复制按钮定义。
fn acl_copy_item_for_action(action: super::menu::AdminInputAction) -> (&'static str, String) {
    let spec = acl_input_spec_for_admin_action(action).expect("acl input spec exists");
    (spec.copy_label, spec.example_command.to_owned())
}

fn parse_acl_callback_data(data: &str) -> Option<AclCallbackAction> {
    let payload = data.strip_prefix(ACL_CALLBACK_PREFIX)?;
    match payload {
        "r" => Some(AclCallbackAction::Refresh),
        "x" => Some(AclCallbackAction::Reset),
        "p" => Some(AclCallbackAction::ToggleAllowAllPrivateUsers),
        "aa" => Some(AclCallbackAction::InputAddAdmin),
        "ad" => Some(AclCallbackAction::InputDelAdmin),
        "ua" => Some(AclCallbackAction::InputAddAllowUser),
        "ud" => Some(AclCallbackAction::InputDelAllowUser),
        "ba" => Some(AclCallbackAction::InputAddBan),
        "bd" => Some(AclCallbackAction::InputDelBan),
        "ta" => Some(AclCallbackAction::InputAddAllowTarget),
        "td" => Some(AclCallbackAction::InputDelAllowTarget),
        "ra" => Some(AclCallbackAction::InputAddAllowRequest),
        "rd" => Some(AclCallbackAction::InputDelAllowRequest),
        _ => None,
    }
}

fn build_acl_callback_data(action: AclCallbackAction) -> String {
    let suffix = match action {
        AclCallbackAction::Refresh => "r",
        AclCallbackAction::Reset => "x",
        AclCallbackAction::ToggleAllowAllPrivateUsers => "p",
        AclCallbackAction::InputAddAdmin => "aa",
        AclCallbackAction::InputDelAdmin => "ad",
        AclCallbackAction::InputAddAllowUser => "ua",
        AclCallbackAction::InputDelAllowUser => "ud",
        AclCallbackAction::InputAddBan => "ba",
        AclCallbackAction::InputDelBan => "bd",
        AclCallbackAction::InputAddAllowTarget => "ta",
        AclCallbackAction::InputDelAllowTarget => "td",
        AclCallbackAction::InputAddAllowRequest => "ra",
        AclCallbackAction::InputDelAllowRequest => "rd",
    };
    format!("{ACL_CALLBACK_PREFIX}{suffix}")
}

async fn send_acl_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_runtime_admin_callback_error(request_chat_id, client_id, "访问控制", err).await
}

fn format_id_line(label: &str, ids: &[i64]) -> String {
    if ids.is_empty() {
        format!("{}：{}", label, card::code("empty"))
    } else {
        format!(
            "{}：{}",
            label,
            ids.iter()
                .map(|id| card::code(*id))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn push_unique(list: &mut Vec<i64>, value: i64) {
    if !list.contains(&value) {
        list.push(value);
    }
}

fn sort_dedup(list: &mut Vec<i64>) {
    list.sort_unstable();
    list.dedup();
}

fn parse_i64_arg(text: &[&str], index: usize, usage: &str) -> anyhow::Result<i64> {
    text.get(index)
        .ok_or_else(|| anyhow::anyhow!("{}", usage))?
        .parse::<i64>()
        .map_err(Into::into)
}

fn parse_bool_arg(value: &str) -> anyhow::Result<bool> {
    match value {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => anyhow::bail!("invalid bool value: {}", value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn test_normalize_acl_config_preserves_bootstrap_admin() {
        let mut config = AccessControlConfig {
            bootstrap_admin_user_ids: vec![1],
            admin_user_ids: vec![1, 2, 2],
            banned_user_ids: vec![1, 2, 3, 3],
            ..Default::default()
        };

        normalize_acl_config(&mut config);

        assert_eq!(config.admin_user_ids, vec![2]);
        assert_eq!(config.banned_user_ids, vec![3]);
    }

    #[test]
    fn test_format_acl_text_contains_sections() {
        let text = format_acl_config_text(
            "当前访问控制",
            &AccessControlConfig {
                bootstrap_admin_user_ids: vec![1],
                admin_user_ids: vec![2],
                allowed_user_ids: vec![3],
                allow_all_private_users: true,
                banned_user_ids: vec![4],
                allowed_request_chat_ids: vec![5],
                allowed_target_chat_ids: vec![6],
            },
        );

        assert!(text.contains("bootstrap_admin_user_ids"));
        assert!(text.contains("allow_all_private_users"));
        assert!(text.contains("allowed_target_chat_ids"));
        assert!(text.contains("/acl add-admin"));
    }

    #[test]
    fn test_acl_callback_roundtrip() {
        let refresh = build_acl_callback_data(AclCallbackAction::Refresh);
        let reset = build_acl_callback_data(AclCallbackAction::Reset);
        let toggle = build_acl_callback_data(AclCallbackAction::ToggleAllowAllPrivateUsers);

        assert!(is_acl_callback_data(&refresh));
        assert_eq!(
            parse_acl_callback_data(&refresh),
            Some(AclCallbackAction::Refresh)
        );
        assert_eq!(
            parse_acl_callback_data(&reset),
            Some(AclCallbackAction::Reset)
        );
        assert_eq!(
            parse_acl_callback_data(&toggle),
            Some(AclCallbackAction::ToggleAllowAllPrivateUsers)
        );
        assert_eq!(parse_acl_callback_data("acfg:bad"), None);
    }

    #[test]
    fn test_build_acl_buttons_use_callback_actions() {
        let app = crate::app_context::app_context();
        app.access_control_runtime
            .update_runtime_config(AccessControlConfig::default());

        let rows = build_acl_buttons();
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("toggle button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "acfg:p");
        assert_eq!(rows[5][0].text, "帮助");
        assert_eq!(rows[5][1].text, "菜单");
        assert!(
            rows.iter()
                .flatten()
                .any(|button| button.text == "加管理员")
        );
        assert!(rows.iter().flatten().any(|button| button.text == "加目标"));
    }
}
