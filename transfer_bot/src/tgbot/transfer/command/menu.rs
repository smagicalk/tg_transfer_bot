// `/menu` 交互式菜单入口。
// 菜单页只做导航和轻量输入引导，真正转存、查询、分页仍复用现有命令模块。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;
use crate::tgbot::transfer::card;

use super::super::store;
use super::common::CommandStyle;
use super::config_cmd;
use crate::tgbot::send::send_interaction_error_card;

mod callback;
mod input;
mod keyboard;
mod text;

use callback::{MenuPage, MenuRequestAction, parse_menu_callback_data};
use input::MenuInputKind;
use keyboard::build_menu_buttons_on;
use text::{
    MenuHomeSummary, build_menu_home_text, build_menu_no_pending_input_text,
    build_menu_status_text, build_menu_text, build_step_prompt_text,
};

pub(super) use input::AdminInputAction;

/// hub 入口的共享元数据。
///
/// `menu/text.rs` 用它渲染命令区，`menu/keyboard/hubs.rs` 用它生成按钮，
/// 这样按钮标题、顺序和命令模板就不会再维护两份。
#[derive(Debug, Clone)]
struct HubEntrySpec {
    text: &'static str,
    style: tdlib_rs::enums::ButtonStyle,
    action: HubEntryAction,
}

/// hub 入口对应的按钮动作。
///
/// 命令预览和按钮行为可能不同；按钮优先使用 callback，文本模式展示可直接发送的命令。
#[derive(Debug, Clone, Copy)]
enum HubEntryAction {
    DownloadsFilter { filter: &'static str, limit: u64 },
    MenuPage(MenuPage),
    QuickLookupDefault,
    NewLookup,
    HealthHome,
    CacheHome,
    AuthHome,
}

/// 任务 hub 的共享入口定义。
fn tasks_hub_specs() -> Vec<Vec<HubEntrySpec>> {
    vec![
        vec![
            HubEntrySpec {
                text: "最近任务",
                style: tdlib_rs::enums::ButtonStyle::Primary,
                action: HubEntryAction::DownloadsFilter {
                    filter: "all",
                    limit: 8,
                },
            },
            HubEntrySpec {
                text: "运行中",
                style: tdlib_rs::enums::ButtonStyle::Default,
                action: HubEntryAction::DownloadsFilter {
                    filter: "run",
                    limit: 8,
                },
            },
            HubEntrySpec {
                text: "已暂停",
                style: tdlib_rs::enums::ButtonStyle::Default,
                action: HubEntryAction::DownloadsFilter {
                    filter: "pause",
                    limit: 8,
                },
            },
        ],
        vec![
            HubEntrySpec {
                text: "失败任务",
                style: tdlib_rs::enums::ButtonStyle::Default,
                action: HubEntryAction::DownloadsFilter {
                    filter: "fail",
                    limit: 8,
                },
            },
            HubEntrySpec {
                text: "更多状态",
                style: tdlib_rs::enums::ButtonStyle::Default,
                action: HubEntryAction::MenuPage(MenuPage::Jobs),
            },
        ],
        vec![
            HubEntrySpec {
                text: "快速查询",
                style: tdlib_rs::enums::ButtonStyle::Primary,
                action: HubEntryAction::QuickLookupDefault,
            },
            HubEntrySpec {
                text: "指定目标",
                style: tdlib_rs::enums::ButtonStyle::Default,
                action: HubEntryAction::NewLookup,
            },
        ],
    ]
}

/// 管理 hub 的共享入口定义。
fn admin_hub_specs(is_owner: bool) -> Vec<Vec<HubEntrySpec>> {
    let mut rows = vec![
        vec![
            HubEntrySpec {
                text: "运行配置",
                style: tdlib_rs::enums::ButtonStyle::Primary,
                action: HubEntryAction::MenuPage(MenuPage::Config),
            },
            HubEntrySpec {
                text: "运行健康",
                style: tdlib_rs::enums::ButtonStyle::Default,
                action: HubEntryAction::HealthHome,
            },
        ],
        vec![
            HubEntrySpec {
                text: "目标配置",
                style: tdlib_rs::enums::ButtonStyle::Primary,
                action: HubEntryAction::MenuPage(MenuPage::Targets),
            },
            HubEntrySpec {
                text: "文件缓存",
                style: tdlib_rs::enums::ButtonStyle::Default,
                action: HubEntryAction::CacheHome,
            },
        ],
    ];
    if is_owner {
        rows.push(vec![HubEntrySpec {
            text: "授权管理",
            style: tdlib_rs::enums::ButtonStyle::Primary,
            action: HubEntryAction::AuthHome,
        }]);
    }
    rows
}

/// `menu` 帮助页和目录页共用的用途描述。
pub(in crate::tgbot::transfer::command) fn menu_help_purpose() -> &'static str {
    "打开转存菜单。"
}

/// `menu` 帮助页和目录页共用的一句话摘要。
pub(in crate::tgbot::transfer::command) fn menu_help_summary() -> &'static str {
    "打开转存菜单；bot token 模式显示按钮，运行配置与目标配置也支持输入流。"
}

/// `menu` 帮助详情页共用的开场说明。
pub(in crate::tgbot::transfer::command) fn menu_help_intro_lines() -> Vec<String> {
    vec![
        "bot token 模式使用 inline keyboard；手机号/OCR 用户号模式会自动降级为文本命令菜单。"
            .to_owned(),
        "授权用户菜单提供转存、任务、管理和帮助入口。".to_owned(),
    ]
}

/// `/help menu` 共用的详细说明正文。
///
/// 菜单页的能力说明和输入流说明放在菜单模块自身维护，避免 help 模块重复理解菜单向导细节。
pub(in crate::tgbot::transfer::command) fn build_menu_help_detail_text() -> String {
    let mut lines = vec!["menu".to_owned(), format!("用途：{}", menu_help_purpose())];
    lines.extend(
        menu_help_intro_lines()
            .into_iter()
            .map(|line| format!("说明：{line}")),
    );
    lines.extend([
        card::DIVIDER.to_owned(),
        "命令：".to_owned(),
        super::common::menu_command(CommandStyle::Long),
        String::new(),
        "可做操作：".to_owned(),
        "转存：按钮引导输入源链接、目标和确认；默认目标可选，不设则使用当前私聊。".to_owned(),
        "查询：按钮引导输入源链接和目标，并查询全部历史结果。".to_owned(),
        "下载：覆盖全部筛选参数，并可进入分页列表。".to_owned(),
        "任务：从列表进入详情后可暂停、恢复、停止、刷新。".to_owned(),
        "配置：config / targets 支持按钮 + 输入流混合操作。".to_owned(),
        "配置：targets 支持先点现有项详情；config 支持先点字段详情。".to_owned(),
        "帮助：覆盖所有 help topic，可原地切换详情页。".to_owned(),
        String::new(),
        "管理输入：".to_owned(),
        "进入输入流后，会发送 ForceReply；回复参数即可，发送其他命令时命令优先。".to_owned(),
        "取消输入：".to_owned(),
        "回复“取消”即可结束当前输入流程。".to_owned(),
    ]);
    lines.join("\n")
}

/// 判断 callback payload 是否属于 `/menu`。
pub(super) fn is_menu_callback_data(data: &str) -> bool {
    callback::is_menu_callback_data(data)
}

/// 生成菜单首页 callback 数据。
///
/// 供进度卡片、结果卡片等非菜单模块放置“返回菜单”按钮；
/// 外部不直接依赖 `MenuPage`，避免把菜单内部页面枚举扩散出去。
pub(super) fn build_menu_home_callback_data() -> String {
    callback::menu_page_callback_data(MenuPage::Home)
}

/// 生成菜单任务中心 callback 数据。
///
/// 供任务列表、任务详情等菜单外部页面直接回到任务操作入口。
pub(super) fn build_menu_tasks_hub_callback_data() -> String {
    callback::menu_page_callback_data(MenuPage::TasksHub)
}

/// 给菜单外部模块复用统一的恢复态卡片正文。
pub(super) fn build_menu_recovery_text_for_outer(
    title: &str,
    status: &str,
    detail: &str,
) -> String {
    text::build_menu_recovery_text(title, status, detail)
}

/// 生成菜单“开始转存”回调，供帮助页等外部模块直接跳入交互流程。
pub(super) fn build_menu_new_transfer_callback_data() -> String {
    callback::new_transfer_callback_data()
}

/// 生成菜单“快速转存”回调，供帮助页等外部模块复用。
pub(super) fn build_menu_quick_transfer_default_callback_data() -> String {
    callback::quick_transfer_default_callback_data()
}

/// 生成菜单“指定目标查询”回调，供帮助页等外部模块直接跳入查询流程。
pub(super) fn build_menu_new_lookup_callback_data() -> String {
    callback::new_lookup_callback_data()
}

/// 生成菜单“快速查询”回调，供帮助页等外部模块复用。
pub(super) fn build_menu_quick_lookup_default_callback_data() -> String {
    callback::quick_lookup_default_callback_data()
}

/// 生成菜单配置页 callback 数据。
pub(super) fn build_menu_config_callback_data() -> String {
    callback::menu_page_callback_data(MenuPage::Config)
}

/// 生成菜单目标配置页 callback 数据。
pub(super) fn build_menu_targets_callback_data() -> String {
    callback::menu_page_callback_data(MenuPage::Targets)
}

/// 启动管理配置单步输入。
pub(super) async fn start_admin_input_callback(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    action: AdminInputAction,
    client_id: i32,
) -> anyhow::Result<()> {
    input::admin_input_callback_query(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        action,
        client_id,
    )
    .await
}

/// 启动带上下文的管理输入。
///
/// 供运行时管理页把“已选中的现有项”挂到草稿上，例如先选 alias，再只输入新的 target_chat_id。
#[allow(clippy::too_many_arguments)]
pub(super) async fn start_admin_input_callback_with_context(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    action: AdminInputAction,
    context_text: Option<String>,
    context_i64: Option<i64>,
    prompt_title: Option<String>,
    prompt_detail: Option<String>,
    prompt_placeholder: Option<String>,
    client_id: i32,
) -> anyhow::Result<()> {
    input::admin_input_callback_query_with_context(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        action,
        context_text,
        context_i64,
        prompt_title,
        prompt_detail,
        prompt_placeholder,
        client_id,
    )
    .await
}

/// 在指定上下文上执行 `/menu` 命令。
pub async fn menu_command_on(
    app: &crate::app_context::AppContext,
    _text: Vec<&str>,
    config: &BotConfig,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    send_menu_page_on(
        app,
        MenuPage::Home,
        actor,
        actor.user_id == config.owner_user_id,
        client_id,
    )
    .await
}

/// 在指定上下文上处理 `/menu` callback。
pub async fn menu_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: Arc<BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let action = match resolve_menu_callback_decision(
        &update.payload,
        update.chat_id,
        update.sender_user_id,
        actor,
    ) {
        MenuCallbackDecision::Dispatch(action) => action,
        MenuCallbackDecision::UnsupportedPayload => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
        MenuCallbackDecision::InvalidPayload => {
            send::answer_callback_query(update.id, Some("菜单按钮参数无效"), client_id).await?;
            return Ok(());
        }
        MenuCallbackDecision::ActorMismatch(action) => {
            tracing::warn!(
                chat_id = update.chat_id,
                actor_chat_id = actor.request_chat_id,
                sender_user_id = update.sender_user_id,
                actor_user_id = actor.user_id,
                action = ?action,
                "menu input callback rejected because actor does not own this interaction"
            );
            send::answer_callback_query(update.id, Some("只能由当前会话发起人操作"), client_id)
                .await?;
            return Ok(());
        }
    };

    let route = route_menu_callback_action(action);

    match route {
        MenuCallbackRoute::Page(page) => {
            send::answer_callback_query(update.id, Some(page.title()), client_id).await?;
            let (text, rows) =
                match build_menu_page_on(app, page, actor, actor.user_id == config.owner_user_id)
                    .await
                {
                    Ok(page) => page,
                    Err(err) => {
                        send_menu_callback_error(update.chat_id, client_id, &err).await?;
                        return Err(err);
                    }
                };
            let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
            send::edit_interaction_card_or_error(
                text,
                update.chat_id,
                update.message_id,
                keyboard,
                client_id,
                "菜单刷新失败",
                "菜单页已生成，但原消息编辑失败；请使用错误卡片上的“菜单”按钮重新进入。",
            )
            .await
        }
        MenuCallbackRoute::StartInput(kind) => {
            start_input_prompt(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                client_id,
                kind,
            )
            .await
        }
        MenuCallbackRoute::StartAdminInput(action) => {
            start_admin_input_callback(
                update.id,
                update.chat_id,
                update.message_id,
                update.sender_user_id,
                action,
                client_id,
            )
            .await
        }
        MenuCallbackRoute::ContinueInput => {
            send::answer_callback_query(update.id, Some("继续输入"), client_id).await?;
            let continued = input::continue_current_input_on(
                app,
                update.chat_id,
                update.sender_user_id,
                config,
                client_id,
            )
            .await?;
            if !continued {
                send::ReplyPanel::card(build_continue_input_empty_text())
                    .row(vec![send::build_callback_button(
                        "首页",
                        &build_menu_home_callback_data(),
                        tdlib_rs::enums::ButtonStyle::Primary,
                    )])
                    .send(update.chat_id, client_id)
                    .await?;
            }
            Ok(())
        }
        MenuCallbackRoute::Forward(action) => match action {
            MenuRequestAction::TargetDefault => {
                input::target_default_callback_query(
                    app,
                    update.id,
                    update.chat_id,
                    update.message_id,
                    update.sender_user_id,
                    config,
                    client_id,
                )
                .await
            }
            MenuRequestAction::TargetManual => {
                input::target_manual_callback_query(
                    update.id,
                    update.chat_id,
                    update.message_id,
                    update.sender_user_id,
                    client_id,
                )
                .await
            }
            MenuRequestAction::TargetRequestChat => {
                input::target_request_chat_callback_query(
                    update.id,
                    update.chat_id,
                    update.message_id,
                    update.sender_user_id,
                    client_id,
                )
                .await
            }
            MenuRequestAction::TargetAlias(target_chat_id) => {
                input::target_alias_callback_query(
                    update.id,
                    update.chat_id,
                    update.message_id,
                    update.sender_user_id,
                    target_chat_id,
                    client_id,
                )
                .await
            }
            MenuRequestAction::TargetConfirm => {
                input::target_confirm_callback_query(
                    app,
                    update.id,
                    update.chat_id,
                    update.message_id,
                    update.sender_user_id,
                    config,
                    actor,
                    client_id,
                )
                .await
            }
            MenuRequestAction::TargetBack => {
                input::target_back_callback_query(
                    app,
                    update.id,
                    update.chat_id,
                    update.message_id,
                    update.sender_user_id,
                    config,
                    client_id,
                )
                .await
            }
            MenuRequestAction::TargetSourceBack => {
                input::target_source_back_callback_query(
                    update.id,
                    update.chat_id,
                    update.message_id,
                    update.sender_user_id,
                    client_id,
                )
                .await
            }
            MenuRequestAction::JobIdInput(action) => {
                input::job_id_input_callback_query(
                    update.id,
                    update.chat_id,
                    update.message_id,
                    update.sender_user_id,
                    action,
                    client_id,
                )
                .await
            }
            MenuRequestAction::CancelInput => {
                input::cancel_input_callback_query(
                    update.id,
                    update.chat_id,
                    update.message_id,
                    update.sender_user_id,
                    client_id,
                )
                .await
            }
            MenuRequestAction::Page(_)
            | MenuRequestAction::NewTransfer
            | MenuRequestAction::QuickTransferDefault
            | MenuRequestAction::NewLookup
            | MenuRequestAction::QuickLookupDefault
            | MenuRequestAction::AdminInput(_)
            | MenuRequestAction::ContinueInput => unreachable!("routed earlier"),
        },
    }
}

impl MenuRequestAction {
    /// 是否会创建、推进或消费菜单输入草稿。
    ///
    /// 这些动作必须绑定到当前 callback 的真实点击者；只读页面导航可以不拦截。
    fn requires_actor_owned_input(self) -> bool {
        matches!(
            self,
            Self::NewTransfer
                | Self::QuickTransferDefault
                | Self::NewLookup
                | Self::QuickLookupDefault
                | Self::TargetDefault
                | Self::TargetManual
                | Self::TargetRequestChat
                | Self::TargetAlias(_)
                | Self::TargetConfirm
                | Self::TargetBack
                | Self::TargetSourceBack
                | Self::JobIdInput(_)
                | Self::AdminInput(_)
                | Self::ContinueInput
                | Self::CancelInput
        )
    }
}

/// 菜单 callback 入口的纯决策结果。
///
/// 入口函数用它决定要 ACK 什么提示；真正的页面刷新和任务执行仍在后续 match 中完成。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuCallbackDecision {
    UnsupportedPayload,
    InvalidPayload,
    ActorMismatch(MenuRequestAction),
    Dispatch(MenuRequestAction),
}

/// `/menu` callback 在入口层的分发结果。
///
/// 入口负责 ACK 和真正的副作用调用；这层先把路由意图稳定下来，减少一个大 match 混着三种关注点。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuCallbackRoute {
    Page(MenuPage),
    StartInput(MenuInputKind),
    StartAdminInput(AdminInputAction),
    ContinueInput,
    Forward(MenuRequestAction),
}

/// 解析 callback payload，并在进入副作用分支前完成输入类按钮的归属校验。
fn resolve_menu_callback_decision(
    payload: &tdlib_rs::enums::CallbackQueryPayload,
    chat_id: i64,
    sender_user_id: i64,
    actor: crate::config::RequestActor,
) -> MenuCallbackDecision {
    let tdlib_rs::enums::CallbackQueryPayload::Data(data) = payload else {
        return MenuCallbackDecision::UnsupportedPayload;
    };

    let Some(action) = parse_menu_callback_data(&data.data) else {
        return MenuCallbackDecision::InvalidPayload;
    };

    if action.requires_actor_owned_input()
        && !menu_input_callback_allowed(chat_id, sender_user_id, actor)
    {
        return MenuCallbackDecision::ActorMismatch(action);
    }

    MenuCallbackDecision::Dispatch(action)
}

/// 把菜单动作映射为入口路由类别。
fn route_menu_callback_action(action: MenuRequestAction) -> MenuCallbackRoute {
    match action {
        MenuRequestAction::Page(page) => MenuCallbackRoute::Page(page),
        MenuRequestAction::NewTransfer => MenuCallbackRoute::StartInput(MenuInputKind::Transfer),
        MenuRequestAction::QuickTransferDefault => {
            MenuCallbackRoute::StartInput(MenuInputKind::TransferDefault)
        }
        MenuRequestAction::NewLookup => MenuCallbackRoute::StartInput(MenuInputKind::Lookup),
        MenuRequestAction::QuickLookupDefault => {
            MenuCallbackRoute::StartInput(MenuInputKind::LookupDefault)
        }
        MenuRequestAction::AdminInput(action) => MenuCallbackRoute::StartAdminInput(action),
        MenuRequestAction::ContinueInput => MenuCallbackRoute::ContinueInput,
        other => MenuCallbackRoute::Forward(other),
    }
}

/// 构造“继续输入但当前没有草稿”时的短状态文案。
fn build_continue_input_empty_text() -> String {
    build_menu_no_pending_input_text()
}

/// 判断菜单输入类 callback 是否属于当前 actor。
///
/// 普通页面 callback 只读；输入类 callback 会改数据库草稿、消费确认态或执行任务，
/// 因此必须确认 TDLib 上报的点击者就是当前请求 actor，避免群聊/转发场景误操作他人的向导。
fn menu_input_callback_allowed(
    chat_id: i64,
    sender_user_id: i64,
    actor: crate::config::RequestActor,
) -> bool {
    chat_id == actor.request_chat_id && sender_user_id == actor.user_id
}

/// 菜单按钮失败提示。
///
/// callback 已经先 ACK，失败时不能再 answer 同一个 callback，因此发送独立卡片帮助排查。
async fn send_menu_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_interaction_error_card(
        request_chat_id,
        client_id,
        "菜单刷新失败",
        "菜单未更新，请检查日志或复制错误信息。",
        err,
    )
    .await
}

/// 发送 ForceReply 输入提示，并记录对应输入流程。
async fn start_input_prompt(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
    kind: MenuInputKind,
) -> anyhow::Result<()> {
    input::start_menu_input(chat_id, sender_user_id, kind).await?;
    send::answer_callback_query(callback_query_id, Some("请输入源链接"), client_id).await?;
    let prompt = send::send_card_message_with_force_reply_returning(
        build_step_prompt_text(
            kind.source_step_label(),
            kind.source_title(),
            kind.source_detail(),
        ),
        chat_id,
        "输入源链接（回复“取消”可退出）",
        client_id,
    )
    .await?;
    // 输入卡已经承载当前流程；删除触发 callback 的旧菜单，避免同一状态出现两张卡片。
    if message_id > 0
        && message_id != prompt.id
        && let Err(error) = send::delete_message(chat_id, message_id, client_id).await
    {
        tracing::debug!(chat_id, message_id, error = %error, "stale menu card could not be deleted");
    }
    Ok(())
}

/// 从显式命令启动转存输入向导。
///
/// `/transfer` 不带参数时进入这里；如果用户是回复媒体消息，上层会优先走回复消息转存。
pub(super) async fn start_transfer_input_from_command(
    chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    input::start_menu_input(chat_id, sender_user_id, MenuInputKind::Transfer).await?;
    send::send_card_message_with_force_reply_returning(
        build_step_prompt_text(
            MenuInputKind::Transfer.source_step_label(),
            MenuInputKind::Transfer.source_title(),
            MenuInputKind::Transfer.source_detail(),
        ),
        chat_id,
        "输入源链接（回复“取消”可退出）",
        client_id,
    )
    .await?;
    Ok(())
}

/// 从 bot 可见媒体消息直接启动“已带源”的转存目标选择流程。
///
/// 这样用户发送媒体后不必再补打一条 `/transfer`，而是直接选择目标并确认执行。
pub(in crate::tgbot) async fn start_transfer_target_choice_from_bot_message(
    app: &crate::app_context::AppContext,
    config: Arc<BotConfig>,
    chat_id: i64,
    sender_user_id: i64,
    source_chat_id: i64,
    source_message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    input::start_transfer_target_choice_with_source_on(
        app,
        config,
        chat_id,
        sender_user_id,
        MenuInputKind::Transfer,
        format!("bot-message:{source_chat_id}:{source_message_id}"),
        client_id,
    )
    .await
}

/// 从私聊里发送的单独一条链接文本直接启动目标选择流程。
pub(in crate::tgbot) async fn start_transfer_target_choice_from_link_message(
    app: &crate::app_context::AppContext,
    config: Arc<BotConfig>,
    chat_id: i64,
    sender_user_id: i64,
    source_link: String,
    client_id: i32,
) -> anyhow::Result<()> {
    input::start_transfer_target_choice_from_link_on(
        app,
        config,
        chat_id,
        sender_user_id,
        source_link,
        client_id,
    )
    .await
}

/// 在指定上下文上处理菜单 ForceReply 输入。
pub(in crate::tgbot) async fn handle_menu_text_input_on(
    app: &crate::app_context::AppContext,
    text: &str,
    config: Arc<BotConfig>,
    key: (i64, i64),
    request_message_id: i64,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<bool> {
    input::handle_menu_input_on(app, text, config, key, request_message_id, actor, client_id).await
}

/// 处理 Telegram 原生目标聊天选择器返回的共享聊天消息。
pub(in crate::tgbot) async fn handle_menu_shared_chat_input(
    shared: &tdlib_rs::types::MessageChatShared,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    input::handle_shared_chat_input_on(shared, request_chat_id, sender_user_id, client_id).await
}

/// 丢弃当前聊天里的菜单输入草稿。
pub async fn discard_menu_input(request_chat_id: i64, sender_user_id: i64) -> anyhow::Result<bool> {
    input::cancel_menu_input(request_chat_id, sender_user_id).await
}

/// 当前用户发送新命令时丢弃菜单输入草稿。
pub async fn discard_menu_input_for_command(
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    let cancelled = input::cancel_menu_input_with_result(request_chat_id, sender_user_id).await?;
    if cancelled.remove_reply_keyboard {
        let cleared =
            input::clear_native_picker_messages(request_chat_id, sender_user_id, client_id).await;
        if !cleared {
            send::send_card_message_with_remove_keyboard(
                build_menu_status_text(
                    "已切换操作",
                    "superseded",
                    "上一轮目标聊天选择已关闭，继续执行新命令。",
                ),
                request_chat_id,
                client_id,
            )
            .await?;
        }
    }
    Ok(cancelled.removed)
}

/// 取消当前聊天里的菜单输入草稿，并给用户明确反馈。
pub async fn cancel_menu_input(
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    let cancelled = input::cancel_menu_input_with_result(request_chat_id, sender_user_id).await?;
    if !cancelled.removed {
        return Ok(false);
    }

    let text = build_menu_status_text(
        "已取消",
        "cancelled",
        "当前菜单输入已取消，可从菜单重新开始。",
    );

    if cancelled.remove_reply_keyboard
        && !input::clear_native_picker_messages(request_chat_id, sender_user_id, client_id).await
    {
        send::send_card_message_with_remove_keyboard(text, request_chat_id, client_id).await?;
        return Ok(true);
    }

    send::ReplyPanel::card(text)
        .row(vec![send::build_callback_button(
            "返回菜单",
            &build_menu_home_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        )])
        .send(request_chat_id, client_id)
        .await?;
    Ok(true)
}

/// 在指定上下文上发送一个菜单页。
async fn send_menu_page_on(
    app: &crate::app_context::AppContext,
    page: MenuPage,
    actor: crate::config::RequestActor,
    is_owner: bool,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, rows) = build_menu_page_on(app, page, actor, is_owner).await?;
    send::ReplyPanel::card(text)
        .rows(rows)
        .send(actor.request_chat_id, client_id)
        .await
}

/// 在指定上下文上构造一个菜单页的正文和按钮。
async fn build_menu_page_on(
    app: &crate::app_context::AppContext,
    page: MenuPage,
    actor: crate::config::RequestActor,
    is_owner: bool,
) -> anyhow::Result<(String, Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>)> {
    let (recent_jobs, health, draft_summary) = if page == MenuPage::Home {
        let recent_jobs = store::list_recent_job_snapshots(app, 5).await?;
        let health = Some(store::list_transfer_health_snapshot(app).await?);
        let draft_summary =
            input::current_draft_summary(actor.request_chat_id, actor.user_id).await?;
        (recent_jobs, health, draft_summary)
    } else {
        (Vec::new(), None, None)
    };
    let text = if page == MenuPage::Config {
        config_cmd::format_current_transfer_config_text_on(app, "当前可调配置")
    } else if page == MenuPage::Targets {
        super::targets::format_targets_text_on(app, "当前目标配置")
    } else if page == MenuPage::Home {
        build_menu_home_text(&MenuHomeSummary {
            active_jobs: health.as_ref().map_or(0, |health| health.active_jobs),
            failed_jobs: health.as_ref().map_or(0, |health| health.failed_jobs),
            recoverable_jobs: health.as_ref().map_or(0, |health| health.recoverable_jobs),
            due_cache_files: health
                .as_ref()
                .map_or(0, |health| health.file_cache_due_rows),
            failed_cache_files: health
                .as_ref()
                .map_or(0, |health| health.file_cache_failed_rows),
            recent_jobs: recent_jobs.len(),
            pending_input: draft_summary.as_ref().map(|draft| draft.title),
        })
    } else {
        build_menu_text(page)
    };
    Ok((
        text,
        build_menu_buttons_on(app, page, &recent_jobs, draft_summary.as_ref(), is_owner),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `/menu` callback 入口的高层执行计划。
    ///
    /// 这是给测试和维护看的稳定接口：它描述入口最终想做什么，而不是直接暴露 TDLib 发送细节。
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum MenuCallbackPlan {
        AckAndRenderPage(MenuPage),
        AckAndStartInput(MenuInputKind),
        AckAndStartAdminInput(AdminInputAction),
        AckAndContinueInput,
        Delegate(MenuRequestAction),
    }

    /// 把入口路由进一步映射成更贴近真实行为的执行计划。
    fn plan_menu_callback_route(route: MenuCallbackRoute) -> MenuCallbackPlan {
        match route {
            MenuCallbackRoute::Page(page) => MenuCallbackPlan::AckAndRenderPage(page),
            MenuCallbackRoute::StartInput(kind) => MenuCallbackPlan::AckAndStartInput(kind),
            MenuCallbackRoute::StartAdminInput(action) => {
                MenuCallbackPlan::AckAndStartAdminInput(action)
            }
            MenuCallbackRoute::ContinueInput => MenuCallbackPlan::AckAndContinueInput,
            MenuCallbackRoute::Forward(action) => MenuCallbackPlan::Delegate(action),
        }
    }

    async fn prepare_schema() -> anyhow::Result<tokio::sync::MutexGuard<'static, ()>> {
        let guard = crate::db::TEST_DB_LOCK.lock().await;
        let db = crate::db::get_db().await?;
        crate::db::ensure_test_schema_current(db).await?;
        Ok(guard)
    }

    // 首页存在草稿时，应把 pending input 摘要带回菜单正文，避免用户不知道自己有未完成输入。
    #[tokio::test]
    async fn test_build_menu_page_home_shows_pending_input_summary() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let actor = crate::config::RequestActor {
            request_chat_id: 991,
            user_id: 992,
        };
        let app_context = crate::app_context::app_context();
        crate::tgbot::transfer::update_targets_runtime_config_on(
            app_context.as_ref(),
            crate::config::TargetsConfig {
                default_chat_id: -1001234567890,
                ..Default::default()
            },
        );
        input::start_menu_input(
            actor.request_chat_id,
            actor.user_id,
            MenuInputKind::Transfer,
        )
        .await?;

        let (text, _rows) =
            build_menu_page_on(app_context.as_ref(), MenuPage::Home, actor, true).await?;

        assert!(text.contains("当前有未完成输入"));
        assert!(text.contains("‹转存源链接›"));
        input::cancel_menu_input(actor.request_chat_id, actor.user_id).await?;
        Ok(())
    }

    // targets 是可选配置；即使没有默认目标/路由/别名，admin 首页也应正常进入菜单。
    #[tokio::test]
    async fn test_build_menu_page_home_renders_when_targets_are_empty() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let actor = crate::config::RequestActor {
            request_chat_id: 1991,
            user_id: 1992,
        };
        let app_context = crate::app_context::app_context();
        crate::tgbot::transfer::update_targets_runtime_config_on(
            app_context.as_ref(),
            crate::config::TargetsConfig::default(),
        );
        let (text, rows) =
            build_menu_page_on(app_context.as_ref(), MenuPage::Home, actor, true).await?;

        assert!(text.contains("转存菜单"));
        assert!(text.contains("运行摘要"));
        assert!(!text.contains("初始化引导"));
        assert!(
            rows.iter()
                .flatten()
                .any(|button| button.text == "快速转存")
        );
        assert!(
            rows.iter()
                .flatten()
                .any(|button| button.text == "指定目标")
        );
        Ok(())
    }

    // 输入类 callback 会改草稿或执行任务，必须绑定当前点击者；页面导航是只读动作。
    #[test]
    fn test_menu_request_action_marks_input_mutations() {
        assert!(!MenuRequestAction::Page(MenuPage::Home).requires_actor_owned_input());
        assert!(MenuRequestAction::NewTransfer.requires_actor_owned_input());
        assert!(MenuRequestAction::TargetConfirm.requires_actor_owned_input());
        assert!(MenuRequestAction::TargetBack.requires_actor_owned_input());
        assert!(MenuRequestAction::ContinueInput.requires_actor_owned_input());
        assert!(MenuRequestAction::CancelInput.requires_actor_owned_input());
    }

    // callback 的 chat/user 必须同时匹配当前 actor，避免多人点击同一张输入卡片时串改草稿。
    #[test]
    fn test_menu_input_callback_allowed_requires_actor_match() {
        let actor = crate::config::RequestActor {
            request_chat_id: 100,
            user_id: 200,
        };

        assert!(menu_input_callback_allowed(100, 200, actor));
        assert!(!menu_input_callback_allowed(101, 200, actor));
        assert!(!menu_input_callback_allowed(100, 201, actor));
    }

    // callback 入口的纯决策层应先区分 TDLib payload 类型和菜单 payload 格式。
    #[test]
    fn test_resolve_menu_callback_decision_rejects_invalid_payloads() {
        let actor = crate::config::RequestActor {
            request_chat_id: 100,
            user_id: 200,
        };

        assert_eq!(
            resolve_menu_callback_decision(
                &tdlib_rs::enums::CallbackQueryPayload::Game(
                    tdlib_rs::types::CallbackQueryPayloadGame {
                        game_short_name: "demo".to_owned(),
                    },
                ),
                100,
                200,
                actor,
            ),
            MenuCallbackDecision::UnsupportedPayload
        );
        assert_eq!(
            resolve_menu_callback_decision(
                &tdlib_rs::enums::CallbackQueryPayload::Data(
                    tdlib_rs::types::CallbackQueryPayloadData {
                        data: "bad".to_owned(),
                    },
                ),
                100,
                200,
                actor,
            ),
            MenuCallbackDecision::InvalidPayload
        );
    }

    // 页面按钮是只读导航，允许正常分发；输入类按钮必须先校验点击者归属。
    #[test]
    fn test_resolve_menu_callback_decision_checks_actor_for_input_actions() {
        let actor = crate::config::RequestActor {
            request_chat_id: 100,
            user_id: 200,
        };

        assert_eq!(
            resolve_menu_callback_decision(
                &tdlib_rs::enums::CallbackQueryPayload::Data(
                    tdlib_rs::types::CallbackQueryPayloadData {
                        data: "m:home".to_owned(),
                    },
                ),
                101,
                201,
                actor,
            ),
            MenuCallbackDecision::Dispatch(MenuRequestAction::Page(MenuPage::Home))
        );
        assert_eq!(
            resolve_menu_callback_decision(
                &tdlib_rs::enums::CallbackQueryPayload::Data(
                    tdlib_rs::types::CallbackQueryPayloadData {
                        data: "m:new".to_owned(),
                    },
                ),
                101,
                200,
                actor,
            ),
            MenuCallbackDecision::ActorMismatch(MenuRequestAction::NewTransfer)
        );
        assert_eq!(
            resolve_menu_callback_decision(
                &tdlib_rs::enums::CallbackQueryPayload::Data(
                    tdlib_rs::types::CallbackQueryPayloadData {
                        data: "m:new".to_owned(),
                    },
                ),
                100,
                200,
                actor,
            ),
            MenuCallbackDecision::Dispatch(MenuRequestAction::NewTransfer)
        );
    }

    // 入口路由应把“页面 / 启动输入 / 继续输入 / 其余转发”四类动作稳定区分开。
    #[test]
    fn test_route_menu_callback_action_groups_entry_behaviors() {
        assert_eq!(
            route_menu_callback_action(MenuRequestAction::Page(MenuPage::Home)),
            MenuCallbackRoute::Page(MenuPage::Home)
        );
        assert_eq!(
            route_menu_callback_action(MenuRequestAction::NewTransfer),
            MenuCallbackRoute::StartInput(MenuInputKind::Transfer)
        );
        assert_eq!(
            route_menu_callback_action(MenuRequestAction::QuickLookupDefault),
            MenuCallbackRoute::StartInput(MenuInputKind::LookupDefault)
        );
        assert_eq!(
            route_menu_callback_action(MenuRequestAction::ContinueInput),
            MenuCallbackRoute::ContinueInput
        );
        assert_eq!(
            route_menu_callback_action(MenuRequestAction::TargetConfirm),
            MenuCallbackRoute::Forward(MenuRequestAction::TargetConfirm)
        );
    }

    // “继续输入”但没有草稿时的提示应明确收敛为终态卡片，而不是等待态文案。
    #[test]
    fn test_build_continue_input_empty_text_uses_empty_status() {
        let text = build_continue_input_empty_text();

        assert!(text.contains("没有未完成输入"));
        assert!(text.contains("状态：‹empty›"));
        assert!(text.contains("当前没有可继续的菜单输入"));
        assert!(!text.contains("/menu"));
    }

    // 首页和继续输入共用的“无草稿”提示应落到同一套空态文案。
    #[test]
    fn test_build_no_pending_input_text_matches_continue_empty() {
        let text = build_continue_input_empty_text();

        assert!(text.contains("没有未完成输入"));
        assert!(text.contains("当前没有可继续的菜单输入"));
    }

    // 高层入口计划应稳定表达 callback 最终意图，避免以后测试只能盯着入口大 match。
    #[test]
    fn test_plan_menu_callback_route_matches_entry_intent() {
        assert_eq!(
            plan_menu_callback_route(MenuCallbackRoute::Page(MenuPage::Downloads)),
            MenuCallbackPlan::AckAndRenderPage(MenuPage::Downloads)
        );
        assert_eq!(
            plan_menu_callback_route(MenuCallbackRoute::StartInput(
                MenuInputKind::TransferDefault
            )),
            MenuCallbackPlan::AckAndStartInput(MenuInputKind::TransferDefault)
        );
        assert_eq!(
            plan_menu_callback_route(MenuCallbackRoute::ContinueInput),
            MenuCallbackPlan::AckAndContinueInput
        );
        assert_eq!(
            plan_menu_callback_route(MenuCallbackRoute::Forward(MenuRequestAction::TargetBack)),
            MenuCallbackPlan::Delegate(MenuRequestAction::TargetBack)
        );
    }
}
