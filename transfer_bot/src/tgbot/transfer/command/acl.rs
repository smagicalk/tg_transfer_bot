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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    ViewAdmin(i64),
    ViewAllowUser(i64),
    ViewBanUser(i64),
    ViewAllowTarget(i64),
    ViewAllowRequest(i64),
    DeleteAdmin(i64),
    DeleteAllowUser(i64),
    DeleteBanUser(i64),
    DeleteAllowTarget(i64),
    DeleteAllowRequest(i64),
}

impl AclCallbackAction {
    fn started_tip(&self) -> &'static str {
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
            Self::ViewAdmin(_)
            | Self::ViewAllowUser(_)
            | Self::ViewBanUser(_)
            | Self::ViewAllowTarget(_)
            | Self::ViewAllowRequest(_)
            | Self::DeleteAdmin(_)
            | Self::DeleteAllowUser(_)
            | Self::DeleteBanUser(_)
            | Self::DeleteAllowTarget(_)
            | Self::DeleteAllowRequest(_) => "正在打开详情",
        }
    }
}

/// `/acl` 单步输入动作规格。
///
/// 访问控制页动作数量较多，统一规格能避免 callback、help 和 ForceReply 文案漂移。
#[derive(Debug, Clone)]
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
fn acl_input_spec_for_callback_action(action: &AclCallbackAction) -> Option<&'static AclInputSpec> {
    ACL_INPUT_SPECS
        .iter()
        .find(|spec| spec.callback_action == *action)
}

/// 在指定上下文上执行 `/acl` 文本命令。
pub async fn acl_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let reply = match text.get(1).copied() {
        None | Some("show") => format_acl_text_on(app, ACL_PAGE_TITLE),
        Some("reset") => reset_acl_to_default_on(app).await?,
        Some("add-admin") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl add-admin <user_id>")?;
            update_acl_with_on(app, &added_action_title("管理员"), |config| {
                push_unique(&mut config.admin_user_ids, user_id);
                config.banned_user_ids.retain(|id| *id != user_id);
            })
            .await?
        }
        Some("del-admin") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl del-admin <user_id>")?;
            update_acl_with_on(app, &deleted_action_title("管理员"), |config| {
                config.admin_user_ids.retain(|id| *id != user_id);
            })
            .await?
        }
        Some("add-allow-user") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl add-allow-user <user_id>")?;
            update_acl_with_on(app, &added_action_title("允许用户"), |config| {
                push_unique(&mut config.allowed_user_ids, user_id);
            })
            .await?
        }
        Some("del-allow-user") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl del-allow-user <user_id>")?;
            update_acl_with_on(app, &deleted_action_title("允许用户"), |config| {
                config.allowed_user_ids.retain(|id| *id != user_id);
            })
            .await?
        }
        Some("add-ban") => {
            let user_id = parse_i64_arg(&text, 2, "usage: /acl add-ban <user_id>")?;
            update_acl_with_on(app, &added_action_title("封禁用户"), |config| {
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
            update_acl_with_on(app, &released_action_title("封禁用户"), |config| {
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
                    update_acl_with_on(app, &updated_action_title("私聊开放策略"), |config| {
                        config.allow_all_private_users = enabled;
                    })
                    .await?
                }
                other => anyhow::bail!("unsupported acl key: {}", other),
            }
        }
        Some("add-allow-target") => {
            let chat_id = parse_i64_arg(&text, 2, "usage: /acl add-allow-target <chat_id>")?;
            update_acl_with_on(app, &added_action_title("目标白名单"), |config| {
                push_unique(&mut config.allowed_target_chat_ids, chat_id);
            })
            .await?
        }
        Some("del-allow-target") => {
            let chat_id = parse_i64_arg(&text, 2, "usage: /acl del-allow-target <chat_id>")?;
            update_acl_with_on(app, &deleted_action_title("目标白名单"), |config| {
                config.allowed_target_chat_ids.retain(|id| *id != chat_id);
            })
            .await?
        }
        Some("add-allow-request") => {
            let chat_id = parse_i64_arg(&text, 2, "usage: /acl add-allow-request <chat_id>")?;
            update_acl_with_on(app, &added_action_title("请求白名单"), |config| {
                push_unique(&mut config.allowed_request_chat_ids, chat_id);
            })
            .await?
        }
        Some("del-allow-request") => {
            let chat_id = parse_i64_arg(&text, 2, "usage: /acl del-allow-request <chat_id>")?;
            update_acl_with_on(app, &deleted_action_title("请求白名单"), |config| {
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
        "现有管理员 / 用户 / 黑名单 / 请求白名单 / 目标白名单：可先点进详情，再删除。".to_owned(),
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

/// 在指定上下文上处理 `/acl` callback。
pub async fn acl_callback_query_on(
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

    let Some(action) = parse_acl_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("访问控制按钮参数无效"), client_id).await?;
        return Ok(());
    };
    send::answer_callback_query(update.id, Some(action.started_tip()), client_id).await?;

    let action_result = match action.clone() {
        AclCallbackAction::Refresh => Ok(()),
        AclCallbackAction::Reset => reset_acl_to_default_on(app).await.map(|_| ()),
        AclCallbackAction::ToggleAllowAllPrivateUsers => {
            let enabled = !crate::tgbot::transfer::access_control_runtime_config_on(app)
                .allow_all_private_users;
            update_acl_with_on(app, &updated_action_title("私聊开放策略"), |config| {
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
            let Some(spec) = acl_input_spec_for_callback_action(&action) else {
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
        AclCallbackAction::ViewAdmin(user_id) => {
            let (text, keyboard) = send::ReplyPanel::card(format_acl_entry_detail_text(
                "管理员详情",
                "user_id",
                user_id,
                "可以删除这名管理员。bootstrap_admin_user_ids 不会在这里显示为可删项。",
            ))
            .rows(build_acl_entry_detail_buttons(
                AclCallbackAction::DeleteAdmin(user_id),
            ))
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
            return Ok(());
        }
        AclCallbackAction::ViewAllowUser(user_id) => {
            let (text, keyboard) = send::ReplyPanel::card(format_acl_entry_detail_text(
                "允许用户详情",
                "user_id",
                user_id,
                "可以删除这名允许用户。",
            ))
            .rows(build_acl_entry_detail_buttons(
                AclCallbackAction::DeleteAllowUser(user_id),
            ))
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
            return Ok(());
        }
        AclCallbackAction::ViewBanUser(user_id) => {
            let (text, keyboard) = send::ReplyPanel::card(format_acl_entry_detail_text(
                "封禁用户详情",
                "user_id",
                user_id,
                "可以解除这名封禁用户。",
            ))
            .rows(build_acl_entry_detail_buttons(
                AclCallbackAction::DeleteBanUser(user_id),
            ))
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
            return Ok(());
        }
        AclCallbackAction::ViewAllowTarget(chat_id) => {
            let (text, keyboard) = send::ReplyPanel::card(format_acl_entry_detail_text(
                "目标白名单详情",
                "chat_id",
                chat_id,
                "可以删除这条目标白名单记录。",
            ))
            .rows(build_acl_entry_detail_buttons(
                AclCallbackAction::DeleteAllowTarget(chat_id),
            ))
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
            return Ok(());
        }
        AclCallbackAction::ViewAllowRequest(chat_id) => {
            let (text, keyboard) = send::ReplyPanel::card(format_acl_entry_detail_text(
                "请求白名单详情",
                "chat_id",
                chat_id,
                "可以删除这条请求白名单记录。",
            ))
            .rows(build_acl_entry_detail_buttons(
                AclCallbackAction::DeleteAllowRequest(chat_id),
            ))
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
            return Ok(());
        }
        AclCallbackAction::DeleteAdmin(user_id) => {
            update_acl_with_on(app, &deleted_action_title("管理员"), |config| {
                config.admin_user_ids.retain(|id| *id != user_id);
            })
            .await
            .map(|_| ())
        }
        AclCallbackAction::DeleteAllowUser(user_id) => {
            update_acl_with_on(app, &deleted_action_title("允许用户"), |config| {
                config.allowed_user_ids.retain(|id| *id != user_id);
            })
            .await
            .map(|_| ())
        }
        AclCallbackAction::DeleteBanUser(user_id) => {
            update_acl_with_on(app, &released_action_title("封禁用户"), |config| {
                config.banned_user_ids.retain(|id| *id != user_id);
            })
            .await
            .map(|_| ())
        }
        AclCallbackAction::DeleteAllowTarget(chat_id) => {
            update_acl_with_on(app, &deleted_action_title("目标白名单"), |config| {
                config.allowed_target_chat_ids.retain(|id| *id != chat_id);
            })
            .await
            .map(|_| ())
        }
        AclCallbackAction::DeleteAllowRequest(chat_id) => {
            update_acl_with_on(app, &deleted_action_title("请求白名单"), |config| {
                config.allowed_request_chat_ids.retain(|id| *id != chat_id);
            })
            .await
            .map(|_| ())
        }
    };
    if let Err(err) = action_result {
        send_acl_callback_error(update.chat_id, client_id, &err).await?;
        return Err(err);
    }

    let (text, keyboard) = send::ReplyPanel::card(format_acl_text_on(app, ACL_PAGE_TITLE))
        .rows(build_acl_buttons_on(app))
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

/// 构造当前访问控制配置文本。
///
/// 菜单页和 help 页在已经持有 `AppContext` 时优先用这个版本，避免重复抓全局。
pub(super) fn format_acl_text_on(app: &crate::app_context::AppContext, title: &str) -> String {
    format_acl_config_text(
        title,
        &crate::tgbot::transfer::access_control_runtime_config_on(app),
    )
}

async fn reset_acl_to_default_on(app: &crate::app_context::AppContext) -> anyhow::Result<String> {
    let config = crate::tgbot::transfer::access_control_runtime_default_config_on(app);
    persist_acl_config_on(app, &config).await?;
    tracing::info!("access control runtime config reset to startup defaults");
    Ok(format_acl_config_text(
        &reset_action_title("访问控制"),
        &config,
    ))
}

async fn update_acl_with_on(
    app: &crate::app_context::AppContext,
    title: &str,
    updater: impl FnOnce(&mut AccessControlConfig),
) -> anyhow::Result<String> {
    let mut config = crate::tgbot::transfer::access_control_runtime_config_on(app);
    updater(&mut config);
    normalize_acl_config(&mut config);
    persist_acl_config_on(app, &config).await?;
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

async fn persist_acl_config_on(
    app: &crate::app_context::AppContext,
    config: &AccessControlConfig,
) -> anyhow::Result<()> {
    crate::tgbot::transfer::save_access_control_runtime_config(config).await?;
    crate::tgbot::transfer::update_access_control_runtime_config_on(app, config.clone());
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

/// 排序后的管理员列表。
fn sorted_admin_ids(config: &AccessControlConfig) -> Vec<i64> {
    let mut ids = config.admin_user_ids.clone();
    ids.sort_unstable();
    ids
}

/// 排序后的允许用户列表。
fn sorted_allowed_user_ids(config: &AccessControlConfig) -> Vec<i64> {
    let mut ids = config.allowed_user_ids.clone();
    ids.sort_unstable();
    ids
}

/// 排序后的封禁用户列表。
fn sorted_banned_user_ids(config: &AccessControlConfig) -> Vec<i64> {
    let mut ids = config.banned_user_ids.clone();
    ids.sort_unstable();
    ids
}

/// 排序后的目标白名单。
fn sorted_allowed_target_ids(config: &AccessControlConfig) -> Vec<i64> {
    let mut ids = config.allowed_target_chat_ids.clone();
    ids.sort_unstable();
    ids
}

/// 排序后的请求白名单。
fn sorted_allowed_request_ids(config: &AccessControlConfig) -> Vec<i64> {
    let mut ids = config.allowed_request_chat_ids.clone();
    ids.sort_unstable();
    ids
}

/// ACL 现有项详情。
fn format_acl_entry_detail_text(title: &str, label: &str, value: i64, detail: &str) -> String {
    [
        title.to_owned(),
        card::field(label, value),
        String::new(),
        card::section("说明"),
        detail.to_owned(),
    ]
    .join("\n")
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

/// ACL 详情页按钮。
fn build_acl_entry_detail_buttons(
    delete_action: AclCallbackAction,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![send::build_callback_button(
            "删除",
            &build_acl_callback_data(delete_action),
            tdlib_rs::enums::ButtonStyle::Danger,
        )],
        build_help_menu_row(
            send::build_callback_button(
                "返回权限",
                &build_acl_callback_data(AclCallbackAction::Refresh),
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

pub(super) fn build_acl_buttons() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let app_context = crate::app_context::app_context();
    build_acl_buttons_on(app_context.as_ref())
}

/// `/acl` 页按钮的上下文版本。
pub(super) fn build_acl_buttons_on(
    app: &crate::app_context::AppContext,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let config = crate::tgbot::transfer::access_control_runtime_config_on(app);
    let allow_all_label = if config.allow_all_private_users {
        "关闭任意私聊"
    } else {
        "开放任意私聊"
    };
    let mut rows = vec![
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
            super::menu::AdminInputAction::AclAddAllowUser,
        ]),
        build_acl_input_row(&[
            super::menu::AdminInputAction::AclAddBan,
            super::menu::AdminInputAction::AclAddAllowTarget,
            super::menu::AdminInputAction::AclAddAllowRequest,
        ]),
    ];
    rows.extend(chunk_acl_id_buttons(
        sorted_admin_ids(&config),
        "管理员",
        AclCallbackAction::ViewAdmin,
    ));
    rows.extend(chunk_acl_id_buttons(
        sorted_allowed_user_ids(&config),
        "允许用户",
        AclCallbackAction::ViewAllowUser,
    ));
    rows.extend(chunk_acl_id_buttons(
        sorted_banned_user_ids(&config),
        "封禁",
        AclCallbackAction::ViewBanUser,
    ));
    rows.extend(chunk_acl_id_buttons(
        sorted_allowed_target_ids(&config),
        "目标",
        AclCallbackAction::ViewAllowTarget,
    ));
    rows.extend(chunk_acl_id_buttons(
        sorted_allowed_request_ids(&config),
        "请求",
        AclCallbackAction::ViewAllowRequest,
    ));
    rows.push(build_acl_input_row(&[
        super::menu::AdminInputAction::AclDelAdmin,
        super::menu::AdminInputAction::AclDelAllowUser,
        super::menu::AdminInputAction::AclAddBan,
        super::menu::AdminInputAction::AclDelAllowTarget,
        super::menu::AdminInputAction::AclDelAllowRequest,
        super::menu::AdminInputAction::AclDelBan,
    ]));
    rows.push(build_help_menu_row(
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
    ));
    rows
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
                &build_acl_callback_data(spec.callback_action.clone()),
                tdlib_rs::enums::ButtonStyle::Default,
            )
        })
        .collect()
}

/// 把 ACL 现有项分块成按钮行。
fn chunk_acl_id_buttons(
    ids: Vec<i64>,
    prefix: &str,
    action_builder: impl Fn(i64) -> AclCallbackAction,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let buttons = ids
        .into_iter()
        .map(|id| {
            send::build_callback_button(
                &format!("{prefix} {id}"),
                &build_acl_callback_data(action_builder(id)),
                tdlib_rs::enums::ButtonStyle::Default,
            )
        })
        .collect::<Vec<_>>();
    buttons.chunks(2).map(<[_]>::to_vec).collect()
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
        _ => {
            let (kind, raw) = payload.split_once(':')?;
            let id = raw.parse::<i64>().ok()?;
            match kind {
                "va" => Some(AclCallbackAction::ViewAdmin(id)),
                "vu" => Some(AclCallbackAction::ViewAllowUser(id)),
                "vb" => Some(AclCallbackAction::ViewBanUser(id)),
                "vt" => Some(AclCallbackAction::ViewAllowTarget(id)),
                "vr" => Some(AclCallbackAction::ViewAllowRequest(id)),
                "da" => Some(AclCallbackAction::DeleteAdmin(id)),
                "du" => Some(AclCallbackAction::DeleteAllowUser(id)),
                "db" => Some(AclCallbackAction::DeleteBanUser(id)),
                "dt" => Some(AclCallbackAction::DeleteAllowTarget(id)),
                "dr" => Some(AclCallbackAction::DeleteAllowRequest(id)),
                _ => None,
            }
        }
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
        AclCallbackAction::ViewAdmin(user_id) => {
            return format!("{ACL_CALLBACK_PREFIX}va:{user_id}");
        }
        AclCallbackAction::ViewAllowUser(user_id) => {
            return format!("{ACL_CALLBACK_PREFIX}vu:{user_id}");
        }
        AclCallbackAction::ViewBanUser(user_id) => {
            return format!("{ACL_CALLBACK_PREFIX}vb:{user_id}");
        }
        AclCallbackAction::ViewAllowTarget(chat_id) => {
            return format!("{ACL_CALLBACK_PREFIX}vt:{chat_id}");
        }
        AclCallbackAction::ViewAllowRequest(chat_id) => {
            return format!("{ACL_CALLBACK_PREFIX}vr:{chat_id}");
        }
        AclCallbackAction::DeleteAdmin(user_id) => {
            return format!("{ACL_CALLBACK_PREFIX}da:{user_id}");
        }
        AclCallbackAction::DeleteAllowUser(user_id) => {
            return format!("{ACL_CALLBACK_PREFIX}du:{user_id}");
        }
        AclCallbackAction::DeleteBanUser(user_id) => {
            return format!("{ACL_CALLBACK_PREFIX}db:{user_id}");
        }
        AclCallbackAction::DeleteAllowTarget(chat_id) => {
            return format!("{ACL_CALLBACK_PREFIX}dt:{chat_id}");
        }
        AclCallbackAction::DeleteAllowRequest(chat_id) => {
            return format!("{ACL_CALLBACK_PREFIX}dr:{chat_id}");
        }
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
            .update_runtime_config(AccessControlConfig {
                admin_user_ids: vec![11],
                allowed_target_chat_ids: vec![22],
                ..AccessControlConfig::default()
            });

        let rows = build_acl_buttons();
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[0][0].r#type
        else {
            panic!("toggle button must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "acfg:p");
        let footer = rows.last().expect("acl page should have footer");
        assert_eq!(footer[0].text, "帮助");
        assert_eq!(footer[1].text, "菜单");
        assert!(
            rows.iter()
                .flatten()
                .any(|button| button.text == "加管理员")
        );
        assert!(rows.iter().flatten().any(|button| button.text == "加目标"));
        assert!(
            rows.iter()
                .flatten()
                .any(|button| button.text == "管理员 11")
        );
        assert!(rows.iter().flatten().any(|button| button.text == "目标 22"));
    }
}
