use std::collections::{BTreeSet, HashMap};
use std::sync::{LazyLock, Mutex};

use crate::tgbot::transfer::card;

/// Telegram 原生用户选择按钮 ID；必须和 `MessageUsersShared.button_id` 一致。
pub(in crate::tgbot::transfer::command) const AUTH_USER_REQUEST_BUTTON_ID: i32 = 7003;
const AUTH_CALLBACK_PREFIX: &str = "au:";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthCallbackAction {
    Add,
    PickUser,
    ManualId,
    ShowCommands,
    HideCommands,
    Refresh,
    Cancel,
    Delete(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PendingAuthInput {
    UserPicker,
    ManualId,
}

type AuthInputKey = (i64, i64);

/// 授权向导只允许 owner 在同一个私聊中保留一个等待态。
static PENDING_AUTH_INPUTS: LazyLock<Mutex<HashMap<AuthInputKey, PendingAuthInput>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
/// 当前授权向导由 bot 发出的输入提示消息。
///
/// 用户选择、取消或切换输入方式后会删除旧提示，避免原生键盘和 ForceReply
/// 与刷新后的管理员列表同时留在会话中。
static PENDING_AUTH_PROMPTS: LazyLock<Mutex<HashMap<AuthInputKey, i64>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct UserProfileSnapshot {
    display_name: Option<String>,
    username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthListEntry {
    role: &'static str,
    user_id: i64,
    profile: UserProfileSnapshot,
    removable: bool,
}

/// 判断 callback 是否属于授权管理面板。
pub(in crate::tgbot::transfer::command) fn is_auth_callback_data(data: &str) -> bool {
    data.starts_with(AUTH_CALLBACK_PREFIX)
}

fn parse_auth_callback_data(data: &str) -> Option<AuthCallbackAction> {
    let payload = data.strip_prefix(AUTH_CALLBACK_PREFIX)?;
    match payload {
        "add" => Some(AuthCallbackAction::Add),
        "pick" => Some(AuthCallbackAction::PickUser),
        "id" => Some(AuthCallbackAction::ManualId),
        "commands" => Some(AuthCallbackAction::ShowCommands),
        "hide" => Some(AuthCallbackAction::HideCommands),
        "refresh" | "list" => Some(AuthCallbackAction::Refresh),
        "cancel" => Some(AuthCallbackAction::Cancel),
        value if value.starts_with("del:") => value
            .strip_prefix("del:")
            .and_then(|id| id.parse::<i64>().ok())
            .filter(|id| *id > 0)
            .map(AuthCallbackAction::Delete),
        _ => None,
    }
}

fn auth_callback_data(action: &str) -> String {
    format!("{AUTH_CALLBACK_PREFIX}{action}")
}

/// 生成授权管理首页 callback，供菜单的 owner 专属入口复用。
pub(in crate::tgbot::transfer::command) fn build_auth_panel_callback_data() -> String {
    auth_callback_data("refresh")
}

fn set_pending_auth_input(key: AuthInputKey, input: PendingAuthInput) {
    let mut guard = PENDING_AUTH_INPUTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    guard.insert(key, input);
}

fn take_pending_auth_input(key: AuthInputKey) -> Option<PendingAuthInput> {
    PENDING_AUTH_INPUTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&key)
}

fn pending_auth_input(key: AuthInputKey) -> Option<PendingAuthInput> {
    PENDING_AUTH_INPUTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&key)
        .copied()
}

fn clear_pending_auth_input(key: AuthInputKey) -> bool {
    PENDING_AUTH_INPUTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&key)
        .is_some()
}

/// 记录当前授权输入提示，并返回同一会话上一条未清理的提示 ID。
fn remember_auth_prompt(key: AuthInputKey, message_id: i64) -> Option<i64> {
    PENDING_AUTH_PROMPTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(key, message_id)
}

/// 取出并清除当前授权输入提示 ID。
fn take_auth_prompt(key: AuthInputKey) -> Option<i64> {
    PENDING_AUTH_PROMPTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&key)
}

/// 删除授权输入提示及其原生键盘；清理失败只记日志，不回滚业务状态。
async fn delete_auth_prompt(key: AuthInputKey, message_id: i64, client_id: i32) {
    if let Err(error) =
        crate::tgbot::send::delete_chat_reply_markup(key.0, message_id, client_id).await
    {
        tracing::debug!(
            chat_id = key.0,
            user_id = key.1,
            prompt_message_id = message_id,
            error = %error,
            "auth prompt reply markup could not be deleted"
        );
    }
    if let Err(error) = crate::tgbot::send::delete_message(key.0, message_id, client_id).await {
        tracing::debug!(
            chat_id = key.0,
            user_id = key.1,
            prompt_message_id = message_id,
            error = %error,
            "auth prompt message could not be deleted"
        );
    }
}

/// 静默消费授权等待态和旧提示，供 callback 刷新、命令切换和成功完成复用。
async fn clear_pending_auth_input_silently(
    key: AuthInputKey,
    client_id: i32,
) -> (Option<PendingAuthInput>, bool) {
    let previous = take_pending_auth_input(key);
    let mut prompt_cleared = false;
    if let Some(message_id) = take_auth_prompt(key) {
        prompt_cleared = true;
        delete_auth_prompt(key, message_id, client_id).await;
    }
    (previous, prompt_cleared)
}

/// 清理等待态时同时移除 Telegram 原生 reply keyboard，避免旧选择器继续提交。
async fn clear_pending_auth_input_and_remove_keyboard(
    key: AuthInputKey,
    client_id: i32,
    notice: &str,
) -> anyhow::Result<bool> {
    let (Some(_previous), _) = clear_pending_auth_input_silently(key, client_id).await else {
        return Ok(false);
    };
    crate::tgbot::send::send_card_message_with_remove_keyboard(notice.to_owned(), key.0, client_id)
        .await?;
    Ok(true)
}

/// 切换授权输入方式；只有从另一种方式切换时才额外移除旧键盘。
async fn switch_pending_auth_input_on(
    key: AuthInputKey,
    next: PendingAuthInput,
    client_id: i32,
) -> anyhow::Result<()> {
    clear_pending_auth_input_silently(key, client_id).await;
    set_pending_auth_input(key, next);
    Ok(())
}

pub(in crate::tgbot::transfer::command) fn auth_help_summary() -> &'static str {
    "交互式查看或管理管理员名单；仅 owner 可执行。"
}

pub(in crate::tgbot::transfer::command) fn build_auth_help_detail_text() -> String {
    [
        "auth".to_owned(),
        "用途：查看管理员名称，并通过按钮选择用户或输入 ID 完成授权。".to_owned(),
        card::note("仅 owner 可以查看或修改授权；配置中的 owner/admin 不受此命令删除。"),
        card::DIVIDER.to_owned(),
        card::section("交互"),
        "发送 /auth 后点击“添加管理员”，再选择 Telegram 用户或输入用户 ID。".to_owned(),
        "在群聊中回复目标用户的消息后发送 /auth，可直接授权该用户（仅 owner）。".to_owned(),
        card::DIVIDER.to_owned(),
        card::section("命令"),
        card::code("/auth"),
        card::code("/auth list"),
        card::code("/auth add <user_id>"),
        card::code("/auth del <user_id>"),
    ]
    .join("\n")
}

/// `/auth` 命令入口；仅 owner 可管理动态授权名单。
pub(in crate::tgbot) async fn auth_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    config: &crate::config::BotConfig,
    request_message: &tdlib_rs::types::Message,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    ensure_owner(config, actor)?;
    let db_conn = crate::db::get_db().await?;
    match text.get(1).copied() {
        None | Some("list") => {
            if text.len() > 2 {
                anyhow::bail!("usage: /auth list");
            }
            if text.len() == 1 && request_message.reply_to.is_some() {
                authorize_replied_user_on(
                    db_conn,
                    app,
                    config,
                    request_message,
                    actor.request_chat_id,
                    client_id,
                )
                .await
            } else {
                send_auth_panel_on(db_conn, config, actor.request_chat_id, client_id, None).await
            }
        }
        Some("add") | Some("del") => {
            // 兼容旧命令；命令执行后直接回到带按钮的管理员列表，减少继续输入。
            let profile_user_id = if text.get(1) == Some(&"add") && text.len() == 3 {
                parse_user_id(&text, "/auth add <user_id>").ok()
            } else {
                None
            };
            let reply = execute_auth_command_on(db_conn, app, config, &text, actor).await?;
            if let Some(user_id) = profile_user_id {
                let snapshot = lookup_user_profile(user_id, client_id).await;
                // 旧命令没有名称参数；成功插入后补一份可选资料快照。
                if snapshot.display_name.is_some() || snapshot.username.is_some() {
                    let _ = crate::access::update_authorized_user_profile_on(
                        db_conn,
                        user_id,
                        snapshot.display_name.as_deref(),
                        snapshot.username.as_deref(),
                    )
                    .await;
                }
            }
            send_auth_panel_on(
                db_conn,
                config,
                actor.request_chat_id,
                client_id,
                Some(&reply),
            )
            .await
        }
        _ => {
            let reply = execute_auth_command_on(db_conn, app, config, &text, actor).await?;
            crate::tgbot::send::ReplyPanel::card(reply)
                .send(actor.request_chat_id, client_id)
                .await
        }
    }
}

/// 回复某条消息执行 `/auth` 时，从原消息读取普通用户并立即授权。
async fn authorize_replied_user_on(
    db_conn: &sea_orm::DatabaseConnection,
    app: &crate::app_context::AppContext,
    config: &crate::config::BotConfig,
    request_message: &tdlib_rs::types::Message,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (user_id, profile) = replied_regular_user_profile(request_message, client_id).await?;
    let (status, detail) =
        grant_authorized_user_with_profile_on(db_conn, app, config, user_id, &profile).await?;
    let text = [
        "授权管理".to_owned(),
        format!("状态：{}", card::code(status)),
        format!(
            "用户：{}  ID：{}",
            card::code(format_profile_label(&profile)),
            card::code(user_id)
        ),
        card::DIVIDER.to_owned(),
        card::note(detail),
    ]
    .join("\n");
    crate::tgbot::send::ReplyPanel::card(text)
        .send(request_chat_id, client_id)
        .await
}

/// 解析被回复消息的发送者；匿名管理员、频道身份、bot 和已删除用户都不能加入名单。
async fn replied_regular_user_profile(
    request_message: &tdlib_rs::types::Message,
    client_id: i32,
) -> anyhow::Result<(i64, UserProfileSnapshot)> {
    let tdlib_rs::enums::MessageReplyTo::Message(reply) = request_message
        .reply_to
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("请回复要授权用户的消息后再发送 /auth"))?
    else {
        anyhow::bail!("只能回复普通用户发送的消息进行授权");
    };
    if reply.message_id == 0 {
        anyhow::bail!("无法定位被回复消息，请改用“添加管理员”按钮选择用户");
    }
    let reply_chat_id = if reply.chat_id == 0 {
        request_message.chat_id
    } else {
        reply.chat_id
    };
    let replied = tdlib_rs::functions::get_message(reply_chat_id, reply.message_id, client_id)
        .await
        .map_err(|error| anyhow::anyhow!("无法读取被回复消息：{}", error.message))?;
    let tdlib_rs::enums::Message::Message(replied) = replied;
    let user_id = authorization_target_user_id(&replied.sender_id, replied.is_outgoing)?;
    let user = tdlib_rs::functions::get_user(user_id, client_id)
        .await
        .map_err(|error| anyhow::anyhow!("无法读取目标用户资料：{}", error.message))?;
    let tdlib_rs::enums::User::User(user) = user;
    if !matches!(user.r#type, tdlib_rs::enums::UserType::Regular) {
        anyhow::bail!("只能授权普通 Telegram 用户");
    }
    Ok((user_id, profile_from_user(&user)))
}

fn authorization_target_user_id(
    sender: &tdlib_rs::enums::MessageSender,
    is_outgoing: bool,
) -> anyhow::Result<i64> {
    if is_outgoing {
        anyhow::bail!("不能授权 bot 自己，请回复目标用户发送的消息");
    }
    let tdlib_rs::enums::MessageSender::User(sender) = sender else {
        anyhow::bail!(
            "被回复消息使用了聊天或匿名身份，无法确定用户 ID；请让目标用户以个人用户身份发送一条消息后再回复授权"
        );
    };
    if sender.user_id <= 0 {
        anyhow::bail!("被回复消息没有有效的用户 ID");
    }
    Ok(sender.user_id)
}

async fn grant_authorized_user_with_profile_on(
    db_conn: &sea_orm::DatabaseConnection,
    app: &crate::app_context::AppContext,
    config: &crate::config::BotConfig,
    user_id: i64,
    profile: &UserProfileSnapshot,
) -> anyhow::Result<(&'static str, &'static str)> {
    if user_id == config.owner_user_id || config.admin_user_ids.contains(&user_id) {
        return Ok(("unchanged", "该用户已通过固定配置获得权限，无需重复添加。"));
    }
    let inserted = crate::access::grant_authorized_user_with_profile_on(
        db_conn,
        user_id,
        profile.display_name.as_deref(),
        profile.username.as_deref(),
    )
    .await?;
    app.access_control.authorize_user(user_id);
    Ok(if inserted {
        ("added", "授权已写入数据库并立即生效。")
    } else {
        ("unchanged", "该用户已经在动态管理员名单中；资料已刷新。")
    })
}

/// 执行授权命令的纯业务路径；发送层只负责把返回文本发给 owner。
async fn execute_auth_command_on(
    db_conn: &sea_orm::DatabaseConnection,
    app: &crate::app_context::AppContext,
    config: &crate::config::BotConfig,
    text: &[&str],
    actor: crate::config::RequestActor,
) -> anyhow::Result<String> {
    ensure_owner(config, actor)?;

    match text.get(1).copied() {
        None | Some("list") => {
            if text.len() > 2 {
                anyhow::bail!("usage: /auth list");
            }
            let dynamic_user_ids = crate::access::list_authorized_user_ids_on(db_conn).await?;
            Ok(format_auth_list(config, &dynamic_user_ids))
        }
        Some("add") => {
            let user_id = parse_user_id(text, "/auth add <user_id>")?;
            if user_id == config.owner_user_id || config.admin_user_ids.contains(&user_id) {
                return Ok(format_auth_result(
                    "unchanged",
                    user_id,
                    "该用户已通过 config.json 获得权限。",
                ));
            }

            let inserted = crate::access::grant_authorized_user_on(db_conn, user_id).await?;
            app.access_control.authorize_user(user_id);
            Ok(format_auth_result(
                if inserted { "added" } else { "unchanged" },
                user_id,
                if inserted {
                    "授权已写入数据库并立即生效。"
                } else {
                    "该用户已经在动态授权名单中。"
                },
            ))
        }
        Some("del") => {
            let user_id = parse_user_id(text, "/auth del <user_id>")?;
            if user_id == config.owner_user_id || config.admin_user_ids.contains(&user_id) {
                anyhow::bail!("owner 和 config.json 管理员不能通过 /auth del 删除");
            }

            let removed = crate::access::revoke_authorized_user_on(db_conn, user_id).await?;
            app.access_control.revoke_user(user_id);
            Ok(format_auth_result(
                if removed { "removed" } else { "unchanged" },
                user_id,
                if removed {
                    "动态授权已删除并立即生效。"
                } else {
                    "该用户不在动态授权名单中。"
                },
            ))
        }
        _ => anyhow::bail!("usage: /auth [list|add <user_id>|del <user_id>]"),
    }
}

/// 从 TDLib 用户对象提取可持久化的轻量资料快照。
fn profile_from_user(user: &tdlib_rs::types::User) -> UserProfileSnapshot {
    let display_name = format_display_name(&user.first_name, &user.last_name);
    let username = user
        .usernames
        .as_ref()
        .and_then(|usernames| usernames.active_usernames.first())
        .map(|username| username.trim().trim_start_matches('@').to_owned())
        .filter(|username| !username.is_empty());
    UserProfileSnapshot {
        display_name,
        username,
    }
}

/// 查询用户资料失败时返回空快照；管理员列表仍会显示数字 ID。
async fn lookup_user_profile(user_id: i64, client_id: i32) -> UserProfileSnapshot {
    match tdlib_rs::functions::get_user(user_id, client_id).await {
        Ok(tdlib_rs::enums::User::User(user)) => profile_from_user(&user),
        Err(err) => {
            tracing::debug!(user_id, error = %err.message, "unable to load Telegram user profile");
            UserProfileSnapshot::default()
        }
    }
}

fn format_display_name(first_name: &str, last_name: &str) -> Option<String> {
    let first_name = first_name.trim();
    let last_name = last_name.trim();
    let display_name = match (first_name.is_empty(), last_name.is_empty()) {
        (true, true) => return None,
        (false, true) => first_name.to_owned(),
        (true, false) => last_name.to_owned(),
        (false, false) => format!("{first_name} {last_name}"),
    };
    Some(display_name)
}

fn profile_from_shared_user(user: &tdlib_rs::types::SharedUser) -> UserProfileSnapshot {
    UserProfileSnapshot {
        display_name: format_display_name(&user.first_name, &user.last_name),
        username: (!user.username.trim().is_empty())
            .then(|| user.username.trim().trim_start_matches('@').to_owned()),
    }
}

fn format_profile_label(profile: &UserProfileSnapshot) -> String {
    let mut label = profile
        .display_name
        .clone()
        .unwrap_or_else(|| "未知用户".to_owned());
    if let Some(username) = profile.username.as_deref() {
        label.push_str(" (@");
        label.push_str(username);
        label.push(')');
    }
    label
}

async fn load_auth_list_entries(
    db_conn: &sea_orm::DatabaseConnection,
    config: &crate::config::BotConfig,
    client_id: i32,
) -> anyhow::Result<Vec<AuthListEntry>> {
    let mut entries = Vec::with_capacity(config.admin_user_ids.len() + 2);
    let owner_profile = lookup_user_profile(config.owner_user_id, client_id).await;
    entries.push(AuthListEntry {
        role: "所有者",
        user_id: config.owner_user_id,
        profile: owner_profile,
        removable: false,
    });

    let mut fixed_ids = BTreeSet::from([config.owner_user_id]);
    for user_id in &config.admin_user_ids {
        if !fixed_ids.insert(*user_id) {
            continue;
        }
        entries.push(AuthListEntry {
            role: "配置管理员",
            user_id: *user_id,
            profile: lookup_user_profile(*user_id, client_id).await,
            removable: false,
        });
    }

    for user in crate::access::list_authorized_users_on(db_conn).await? {
        if !fixed_ids.insert(user.user_id) {
            continue;
        }
        let mut profile = UserProfileSnapshot {
            display_name: user.display_name,
            username: user.username,
        };
        // 旧版本记录可能没有资料；只在缺失时尝试一次离线查询。
        if profile.display_name.is_none() && profile.username.is_none() {
            let refreshed = lookup_user_profile(user.user_id, client_id).await;
            if refreshed.display_name.is_some() || refreshed.username.is_some() {
                let _ = crate::access::update_authorized_user_profile_on(
                    db_conn,
                    user.user_id,
                    refreshed.display_name.as_deref(),
                    refreshed.username.as_deref(),
                )
                .await;
                profile = refreshed;
            }
        }
        entries.push(AuthListEntry {
            role: "动态管理员",
            user_id: user.user_id,
            profile,
            removable: true,
        });
    }
    Ok(entries)
}

fn format_auth_panel_text(
    entries: &[AuthListEntry],
    notice: Option<&str>,
    show_commands: bool,
) -> String {
    let mut lines = vec![
        "授权管理".to_owned(),
        format!("状态：{}", card::code("ready")),
    ];
    if let Some(notice) = notice.filter(|notice| !notice.trim().is_empty()) {
        lines.push(card::note(notice));
    }
    lines.extend([card::DIVIDER.to_owned(), card::section("管理员列表")]);
    if entries.is_empty() {
        lines.push(card::note("暂无管理员记录。"));
    } else {
        for entry in entries {
            lines.push(format!(
                "{}：{}  ID：{}",
                entry.role,
                card::code(format_profile_label(&entry.profile)),
                card::code(entry.user_id),
            ));
        }
    }
    lines.extend([
        card::DIVIDER.to_owned(),
        card::section("操作"),
        card::note("点击“添加管理员”，再选择 Telegram 用户或输入用户 ID。"),
    ]);
    if show_commands {
        lines.extend([
            String::new(),
            card::section("命令"),
            card::code("/auth list"),
            card::code("/auth add <user_id>"),
            card::code("/auth del <user_id>"),
        ]);
    }
    lines.join("\n")
}

fn build_auth_callback_button(
    text: &str,
    action: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    crate::tgbot::send::build_callback_button(text, &auth_callback_data(action), style)
}

fn build_auth_panel_rows(
    entries: &[AuthListEntry],
    show_commands: bool,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = vec![vec![
        build_auth_callback_button("添加管理员", "add", tdlib_rs::enums::ButtonStyle::Primary),
        build_auth_callback_button("刷新", "refresh", tdlib_rs::enums::ButtonStyle::Default),
        build_auth_callback_button(
            if show_commands {
                "隐藏命令"
            } else {
                "查看命令"
            },
            if show_commands { "hide" } else { "commands" },
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]];
    for entry in entries.iter().filter(|entry| entry.removable) {
        rows.push(vec![build_auth_callback_button(
            &format!("删除 {}", entry.user_id),
            &format!("del:{}", entry.user_id),
            tdlib_rs::enums::ButtonStyle::Danger,
        )]);
    }
    // 授权管理由菜单进入，列表底部始终保留返回入口；动态删除按钮数量变化时也不影响导航。
    rows.push(vec![crate::tgbot::send::build_callback_button(
        "返回菜单",
        &super::build_menu_home_button_data(),
        tdlib_rs::enums::ButtonStyle::Default,
    )]);
    rows
}

async fn send_auth_panel_on(
    db_conn: &sea_orm::DatabaseConnection,
    config: &crate::config::BotConfig,
    chat_id: i64,
    client_id: i32,
    notice: Option<&str>,
) -> anyhow::Result<()> {
    let entries = load_auth_list_entries(db_conn, config, client_id).await?;
    crate::tgbot::send::ReplyPanel::card(format_auth_panel_text(&entries, notice, false))
        .rows(build_auth_panel_rows(&entries, false))
        .send(chat_id, client_id)
        .await
}

fn build_auth_add_options_text() -> String {
    [
        "授权管理".to_owned(),
        card::section("添加管理员"),
        card::note("选择 Telegram 用户会打开原生用户选择器；也可以输入数字 ID。"),
    ]
    .join("\n")
}

fn build_auth_add_options_rows() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![build_auth_callback_button(
            "选择 Telegram 用户",
            "pick",
            tdlib_rs::enums::ButtonStyle::Primary,
        )],
        vec![build_auth_callback_button(
            "输入用户 ID",
            "id",
            tdlib_rs::enums::ButtonStyle::Default,
        )],
        vec![
            build_auth_callback_button("取消", "cancel", tdlib_rs::enums::ButtonStyle::Danger),
            build_auth_callback_button(
                "返回列表",
                "refresh",
                tdlib_rs::enums::ButtonStyle::Default,
            ),
        ],
    ]
}

async fn edit_auth_panel_on(
    db_conn: &sea_orm::DatabaseConnection,
    config: &crate::config::BotConfig,
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    notice: Option<&str>,
    show_commands: bool,
) -> anyhow::Result<()> {
    let entries = load_auth_list_entries(db_conn, config, client_id).await?;
    let (text, keyboard) = crate::tgbot::send::ReplyPanel::card(format_auth_panel_text(
        &entries,
        notice,
        show_commands,
    ))
    .rows(build_auth_panel_rows(&entries, show_commands))
    .into_card_parts()?;
    crate::tgbot::send::edit_card_message_with_inline_keyboard(
        text, chat_id, message_id, keyboard, client_id,
    )
    .await
}

async fn edit_auth_add_options_on(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, keyboard) = crate::tgbot::send::ReplyPanel::card(build_auth_add_options_text())
        .rows(build_auth_add_options_rows())
        .into_card_parts()?;
    crate::tgbot::send::edit_card_message_with_inline_keyboard(
        text, chat_id, message_id, keyboard, client_id,
    )
    .await
}

/// 发送当前授权输入方式的提示和原生键盘。
async fn send_auth_input_prompt(
    input: PendingAuthInput,
    key: AuthInputKey,
    client_id: i32,
    note: &str,
) -> anyhow::Result<()> {
    let text = match input {
        PendingAuthInput::UserPicker => [
            "授权管理".to_owned(),
            card::section("选择用户"),
            card::note(note),
        ]
        .join("\n"),
        PendingAuthInput::ManualId => [
            "授权管理".to_owned(),
            card::section("输入用户 ID"),
            card::note(note),
        ]
        .join("\n"),
    };
    let sent = match input {
        PendingAuthInput::UserPicker => {
            crate::tgbot::send::send_card_message_with_user_request_keyboard_returning(
                text,
                key.0,
                AUTH_USER_REQUEST_BUTTON_ID,
                client_id,
            )
            .await?
        }
        PendingAuthInput::ManualId => {
            crate::tgbot::send::send_card_message_with_force_reply_returning(
                text,
                key.0,
                "输入用户 ID",
                client_id,
            )
            .await?
        }
    };
    if let Some(previous_id) = remember_auth_prompt(key, sent.id)
        && previous_id != sent.id
    {
        delete_auth_prompt(key, previous_id, client_id).await;
    }
    Ok(())
}

/// 数据库写入失败时保留当前步骤，并重新显示可操作的输入控件。
async fn report_auth_add_failure(
    key: AuthInputKey,
    input: PendingAuthInput,
    client_id: i32,
    error: anyhow::Error,
) -> anyhow::Result<()> {
    tracing::error!(error = %error, user_id = key.1, "dynamic authorization persistence failed");
    set_pending_auth_input(key, input);
    if let Err(prompt_error) = send_auth_input_prompt(
        input,
        key,
        client_id,
        &format!(
            "授权暂时失败：{error}。请重新选择 Telegram 用户或输入正整数 ID，也可以回复“取消”退出。"
        ),
    )
    .await
    {
        clear_pending_auth_input(key);
        return Err(prompt_error);
    }
    Ok(())
}

/// 处理授权管理 inline keyboard 回调。
pub(in crate::tgbot) async fn auth_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: std::sync::Arc<crate::config::BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let data = match &update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data,
        _ => {
            crate::tgbot::send::answer_callback_query(
                update.id,
                Some("暂不支持这种按钮类型"),
                client_id,
            )
            .await?;
            return Ok(());
        }
    };
    let Some(action) = parse_auth_callback_data(&data.data) else {
        crate::tgbot::send::answer_callback_query(update.id, Some("授权按钮参数无效"), client_id)
            .await?;
        return Ok(());
    };

    if actor.user_id != config.owner_user_id
        || actor.request_chat_id != update.chat_id
        || actor.user_id != update.sender_user_id
    {
        crate::tgbot::send::answer_callback_query(
            update.id,
            Some("仅 owner 可管理授权"),
            client_id,
        )
        .await?;
        return Ok(());
    }

    match action {
        AuthCallbackAction::Add => {
            crate::tgbot::send::answer_callback_query(update.id, Some("选择添加方式"), client_id)
                .await?;
            clear_pending_auth_input_silently((update.chat_id, update.sender_user_id), client_id)
                .await;
            edit_auth_add_options_on(update.chat_id, update.message_id, client_id).await
        }
        AuthCallbackAction::PickUser => {
            switch_pending_auth_input_on(
                (update.chat_id, update.sender_user_id),
                PendingAuthInput::UserPicker,
                client_id,
            )
            .await?;
            crate::tgbot::send::answer_callback_query(update.id, Some("请选择用户"), client_id)
                .await?;
            if let Err(err) = send_auth_input_prompt(
                PendingAuthInput::UserPicker,
                (update.chat_id, update.sender_user_id),
                client_id,
                "请选择要授权的 Telegram 用户；也可以直接发送数字 ID。",
            )
            .await
            {
                clear_pending_auth_input((update.chat_id, update.sender_user_id));
                return Err(err);
            }
            // 新的原生用户选择器已经承载当前步骤，旧的“选择添加方式”卡片不再可用。
            if let Err(error) =
                crate::tgbot::send::delete_message(update.chat_id, update.message_id, client_id)
                    .await
            {
                tracing::debug!(
                    chat_id = update.chat_id,
                    message_id = update.message_id,
                    error = %error,
                    "stale auth add-options card could not be deleted"
                );
            }
            Ok(())
        }
        AuthCallbackAction::ManualId => {
            switch_pending_auth_input_on(
                (update.chat_id, update.sender_user_id),
                PendingAuthInput::ManualId,
                client_id,
            )
            .await?;
            crate::tgbot::send::answer_callback_query(update.id, Some("请输入用户 ID"), client_id)
                .await?;
            if let Err(err) = send_auth_input_prompt(
                PendingAuthInput::ManualId,
                (update.chat_id, update.sender_user_id),
                client_id,
                "请输入正整数用户 ID，或回复“取消”退出。",
            )
            .await
            {
                clear_pending_auth_input((update.chat_id, update.sender_user_id));
                return Err(err);
            }
            // ForceReply 已经成为唯一输入入口，删除旧选项卡避免用户重复点击旧按钮。
            if let Err(error) =
                crate::tgbot::send::delete_message(update.chat_id, update.message_id, client_id)
                    .await
            {
                tracing::debug!(
                    chat_id = update.chat_id,
                    message_id = update.message_id,
                    error = %error,
                    "stale auth add-options card could not be deleted"
                );
            }
            Ok(())
        }
        toggle @ (AuthCallbackAction::ShowCommands | AuthCallbackAction::HideCommands) => {
            let show_commands = toggle == AuthCallbackAction::ShowCommands;
            crate::tgbot::send::answer_callback_query(
                update.id,
                Some(if show_commands {
                    "已显示命令"
                } else {
                    "已隐藏命令"
                }),
                client_id,
            )
            .await?;
            let db_conn = crate::db::get_db().await?;
            edit_auth_panel_on(
                db_conn,
                config.as_ref(),
                update.chat_id,
                update.message_id,
                client_id,
                None,
                show_commands,
            )
            .await
        }
        AuthCallbackAction::Refresh => {
            crate::tgbot::send::answer_callback_query(update.id, Some("已刷新"), client_id).await?;
            clear_pending_auth_input_silently((update.chat_id, update.sender_user_id), client_id)
                .await;
            let db_conn = crate::db::get_db().await?;
            edit_auth_panel_on(
                db_conn,
                config.as_ref(),
                update.chat_id,
                update.message_id,
                client_id,
                None,
                false,
            )
            .await
        }
        AuthCallbackAction::Cancel => {
            crate::tgbot::send::answer_callback_query(update.id, Some("已取消"), client_id).await?;
            let key = (update.chat_id, update.sender_user_id);
            let (previous, prompt_cleared) =
                clear_pending_auth_input_silently(key, client_id).await;
            if previous.is_some() && !prompt_cleared {
                crate::tgbot::send::send_card_message_with_remove_keyboard(
                    "授权添加已取消。".to_owned(),
                    update.chat_id,
                    client_id,
                )
                .await?;
            }
            let db_conn = crate::db::get_db().await?;
            edit_auth_panel_on(
                db_conn,
                config.as_ref(),
                update.chat_id,
                update.message_id,
                client_id,
                None,
                false,
            )
            .await
        }
        AuthCallbackAction::Delete(user_id) => {
            if user_id == config.owner_user_id || config.admin_user_ids.contains(&user_id) {
                crate::tgbot::send::answer_callback_query(
                    update.id,
                    Some("固定管理员不能删除"),
                    client_id,
                )
                .await?;
                return Ok(());
            }
            clear_pending_auth_input_silently((update.chat_id, update.sender_user_id), client_id)
                .await;
            let db_conn = crate::db::get_db().await?;
            let removed = crate::access::revoke_authorized_user_on(db_conn, user_id).await?;
            app.access_control.revoke_user(user_id);
            crate::tgbot::send::answer_callback_query(
                update.id,
                Some(if removed {
                    "已删除"
                } else {
                    "用户不在动态名单"
                }),
                client_id,
            )
            .await?;
            edit_auth_panel_on(
                db_conn,
                config.as_ref(),
                update.chat_id,
                update.message_id,
                client_id,
                Some(if removed {
                    "动态管理员已删除并立即失效。"
                } else {
                    "该用户不在动态管理员名单中。"
                }),
                false,
            )
            .await
        }
    }
}

async fn complete_auth_add_on(
    app: &crate::app_context::AppContext,
    config: &crate::config::BotConfig,
    user_id: i64,
    profile: UserProfileSnapshot,
    input: PendingAuthInput,
    key: AuthInputKey,
    client_id: i32,
) -> anyhow::Result<()> {
    let request_chat_id = key.0;
    if user_id <= 0 {
        clear_pending_auth_input(key);
        crate::tgbot::send::send_card_message_with_remove_keyboard(
            "用户 ID 必须是正整数。".to_owned(),
            request_chat_id,
            client_id,
        )
        .await?;
        return Ok(());
    }
    let db_conn = match crate::db::get_db().await {
        Ok(db_conn) => db_conn,
        Err(error) => return report_auth_add_failure(key, input, client_id, error).await,
    };
    let detail = match grant_authorized_user_with_profile_on(
        db_conn, app, config, user_id, &profile,
    )
    .await
    {
        Ok((_, detail)) => detail,
        Err(error) => return report_auth_add_failure(key, input, client_id, error).await,
    };
    clear_pending_auth_input(key);
    if let Some(message_id) = take_auth_prompt(key) {
        delete_auth_prompt(key, message_id, client_id).await;
    }
    // 管理员列表已经包含本次授权结果提示；只发送这一张刷新后的列表，
    // 避免“成功卡片 + 列表卡片”连续出现两条重复回复。
    send_auth_panel_on(db_conn, config, request_chat_id, client_id, Some(detail)).await
}

/// 处理授权向导中的普通文本输入；返回 true 表示消息已被授权流程消费。
pub(in crate::tgbot) async fn handle_auth_text_input_on(
    app: &crate::app_context::AppContext,
    text: &str,
    config: std::sync::Arc<crate::config::BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<bool> {
    let key = (actor.request_chat_id, actor.user_id);
    let Some(input) = pending_auth_input(key) else {
        return Ok(false);
    };
    if text.trim().eq_ignore_ascii_case("/cancel") || text.trim() == "取消" {
        clear_pending_auth_input_and_remove_keyboard(key, client_id, "授权添加已取消。").await?;
        return Ok(true);
    }
    if text.trim().starts_with('/') {
        return Ok(false);
    }
    let Ok(user_id) = text.trim().parse::<i64>() else {
        send_auth_input_prompt(
            input,
            key,
            client_id,
            "用户 ID 必须是正整数，请重新输入或回复“取消”退出。",
        )
        .await?;
        return Ok(true);
    };
    if user_id <= 0 {
        send_auth_input_prompt(
            input,
            key,
            client_id,
            "用户 ID 必须是正整数，请重新输入或回复“取消”退出。",
        )
        .await?;
        return Ok(true);
    }
    let profile = if matches!(
        input,
        PendingAuthInput::ManualId | PendingAuthInput::UserPicker
    ) {
        lookup_user_profile(user_id, client_id).await
    } else {
        UserProfileSnapshot::default()
    };
    complete_auth_add_on(
        app,
        config.as_ref(),
        user_id,
        profile,
        input,
        (actor.request_chat_id, actor.user_id),
        client_id,
    )
    .await?;
    Ok(true)
}

/// 处理 Telegram 原生 `messageUsersShared` 事件。
pub(in crate::tgbot) async fn handle_auth_shared_user_input(
    app: &crate::app_context::AppContext,
    shared: &tdlib_rs::types::MessageUsersShared,
    config: std::sync::Arc<crate::config::BotConfig>,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    if shared.button_id != AUTH_USER_REQUEST_BUTTON_ID {
        return Ok(false);
    }
    let key = (request_chat_id, sender_user_id);
    let Some(input) = pending_auth_input(key) else {
        crate::tgbot::send::send_card_message_with_remove_keyboard(
            "用户选择已过期，请重新点击“添加管理员”。".to_owned(),
            request_chat_id,
            client_id,
        )
        .await?;
        return Ok(true);
    };
    if input != PendingAuthInput::UserPicker {
        // 迟到的选择结果不能清掉当前手动 ID 输入流程。
        return Ok(true);
    }
    let Some(user) = shared.users.first() else {
        set_pending_auth_input(key, PendingAuthInput::UserPicker);
        if let Err(error) = send_auth_input_prompt(
            PendingAuthInput::UserPicker,
            key,
            client_id,
            "没有收到用户，请重新选择或直接发送数字 ID。",
        )
        .await
        {
            clear_pending_auth_input(key);
            return Err(error);
        }
        return Ok(true);
    };
    // 成功收到选择后先消费等待态，避免重复 update 重复授权；数据库失败时由完成函数恢复。
    take_pending_auth_input(key);
    let mut profile = profile_from_shared_user(user);
    if profile.display_name.is_none() && profile.username.is_none() {
        profile = lookup_user_profile(user.user_id, client_id).await;
    }
    complete_auth_add_on(
        app,
        config.as_ref(),
        user.user_id,
        profile,
        input,
        (request_chat_id, sender_user_id),
        client_id,
    )
    .await?;
    Ok(true)
}

/// `/cancel` 和新命令都应清理授权向导，避免旧键盘继续提交用户。
pub(in crate::tgbot) async fn cancel_auth_input(
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    clear_pending_auth_input_and_remove_keyboard(
        (request_chat_id, sender_user_id),
        client_id,
        "授权添加已取消。",
    )
    .await
}

pub(in crate::tgbot) async fn discard_auth_input_for_command(
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    let key = (request_chat_id, sender_user_id);
    let (previous, prompt_cleared) = clear_pending_auth_input_silently(key, client_id).await;
    if previous.is_some() && !prompt_cleared {
        crate::tgbot::send::send_card_message_with_remove_keyboard(
            "已切换操作，授权添加已关闭。".to_owned(),
            request_chat_id,
            client_id,
        )
        .await?;
    }
    Ok(previous.is_some())
}

fn ensure_owner(
    config: &crate::config::BotConfig,
    actor: crate::config::RequestActor,
) -> anyhow::Result<()> {
    if actor.user_id != config.owner_user_id {
        anyhow::bail!("仅 owner 可管理授权");
    }
    Ok(())
}

fn parse_user_id(text: &[&str], usage: &str) -> anyhow::Result<i64> {
    if text.len() != 3 {
        anyhow::bail!("usage: {usage}");
    }
    let user_id = text[2]
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("user_id 必须是正整数；usage: {usage}"))?;
    if user_id <= 0 {
        anyhow::bail!("user_id 必须是正整数；usage: {usage}");
    }
    Ok(user_id)
}

fn format_auth_result(status: &str, user_id: i64, detail: &str) -> String {
    [
        "授权管理".to_owned(),
        card::field_pair("状态", status, "用户", user_id),
        card::DIVIDER.to_owned(),
        card::note(detail),
    ]
    .join("\n")
}

fn format_auth_list(
    config: &crate::config::BotConfig,
    dynamic_user_ids: &std::collections::BTreeSet<i64>,
) -> String {
    let configured_admins = if config.admin_user_ids.is_empty() {
        "无".to_owned()
    } else {
        config
            .admin_user_ids
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    };
    let mut lines = vec![
        "授权管理".to_owned(),
        format!("状态：{}", card::code("ready")),
        card::DIVIDER.to_owned(),
        card::section("固定权限"),
        card::field("所有者", config.owner_user_id),
        card::field("配置管理员", configured_admins),
        String::new(),
        card::section("动态授权"),
    ];
    if dynamic_user_ids.is_empty() {
        lines.push(card::note("暂无动态授权用户。"));
    } else {
        lines.extend(
            dynamic_user_ids
                .iter()
                .map(|user_id| card::field("动态用户", user_id)),
        );
    }
    lines.extend([
        String::new(),
        card::section("命令"),
        card::code("/auth list"),
        card::code("/auth add <user_id>"),
        card::code("/auth del <user_id>"),
    ]);
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose};
    use rand::RngExt;
    use std::collections::BTreeSet;

    use super::{
        AuthCallbackAction, AuthListEntry, PendingAuthInput, UserProfileSnapshot,
        authorization_target_user_id, build_auth_panel_rows, clear_pending_auth_input,
        execute_auth_command_on, format_auth_panel_text, format_display_name, format_profile_label,
        parse_auth_callback_data, profile_from_shared_user, set_pending_auth_input,
        take_pending_auth_input,
    };

    #[test]
    fn test_auth_callback_data_routes_interactive_actions() {
        assert_eq!(
            parse_auth_callback_data("au:add"),
            Some(AuthCallbackAction::Add)
        );
        assert_eq!(
            parse_auth_callback_data("au:pick"),
            Some(AuthCallbackAction::PickUser)
        );
        assert_eq!(
            parse_auth_callback_data("au:id"),
            Some(AuthCallbackAction::ManualId)
        );
        assert_eq!(
            parse_auth_callback_data("au:commands"),
            Some(AuthCallbackAction::ShowCommands)
        );
        assert_eq!(
            parse_auth_callback_data("au:hide"),
            Some(AuthCallbackAction::HideCommands)
        );
        assert_eq!(
            parse_auth_callback_data("au:del:123456"),
            Some(AuthCallbackAction::Delete(123456))
        );
        assert_eq!(parse_auth_callback_data("au:del:-1"), None);
        assert_eq!(parse_auth_callback_data("m:add"), None);
    }

    #[test]
    fn test_auth_help_mentions_reply_shortcut() {
        let text = super::build_auth_help_detail_text();

        assert!(text.contains("回复目标用户的消息"));
        assert!(text.contains("/auth"));
    }

    #[test]
    fn test_user_profile_prefers_name_and_keeps_username() {
        assert_eq!(format_display_name("张", "三"), Some("张 三".to_owned()));
        assert_eq!(format_display_name("张三", ""), Some("张三".to_owned()));
        assert_eq!(format_display_name("", ""), None);

        let shared = tdlib_rs::types::SharedUser {
            user_id: 123456,
            first_name: " 张三 ".to_owned(),
            last_name: String::new(),
            username: "@zhangsan".to_owned(),
            photo: None,
        };
        let profile = profile_from_shared_user(&shared);
        assert_eq!(profile.display_name.as_deref(), Some("张三"));
        assert_eq!(profile.username.as_deref(), Some("zhangsan"));
        assert_eq!(format_profile_label(&profile), "张三 (@zhangsan)");
    }

    #[test]
    fn test_auth_panel_lists_names_ids_and_only_dynamic_delete_buttons() {
        let entries = vec![
            AuthListEntry {
                role: "所有者",
                user_id: 1,
                profile: UserProfileSnapshot {
                    display_name: Some("Owner".to_owned()),
                    username: Some("owner".to_owned()),
                },
                removable: false,
            },
            AuthListEntry {
                role: "动态管理员",
                user_id: 2,
                profile: UserProfileSnapshot {
                    display_name: Some("张三".to_owned()),
                    username: Some("zhangsan".to_owned()),
                },
                removable: true,
            },
        ];

        let text = format_auth_panel_text(&entries, None, false);
        let rows = build_auth_panel_rows(&entries, false);

        assert!(text.contains("管理员列表"));
        assert!(text.contains("所有者：‹Owner (@owner)›  ID：‹1›"));
        assert!(text.contains("动态管理员：‹张三 (@zhangsan)›  ID：‹2›"));
        assert!(!text.contains("/auth list"));
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0][0].text, "添加管理员");
        assert_eq!(rows[0][2].text, "查看命令");
        assert_eq!(rows[1][0].text, "删除 2");
        assert_eq!(rows[2][0].text, "返回菜单");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[2][0].r#type
        else {
            panic!("return button must be callback");
        };
        assert_eq!(
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap(),
            super::super::build_menu_home_button_data(),
        );

        let commands_text = format_auth_panel_text(&entries, None, true);
        let commands_rows = build_auth_panel_rows(&entries, true);
        assert!(commands_text.contains("■ 命令"));
        assert!(commands_text.contains("/auth list"));
        assert!(commands_text.contains("/auth add <user_id>"));
        assert!(commands_text.contains("/auth del <user_id>"));
        assert_eq!(commands_rows[0][2].text, "隐藏命令");
        assert_eq!(commands_rows.last().unwrap()[0].text, "返回菜单");
    }

    #[test]
    fn test_pending_auth_input_is_scoped_to_private_actor() {
        let owner_key = (91_001, 91_001);
        let other_key = (91_002, 91_002);
        clear_pending_auth_input(owner_key);
        clear_pending_auth_input(other_key);

        set_pending_auth_input(owner_key, PendingAuthInput::UserPicker);
        set_pending_auth_input(other_key, PendingAuthInput::ManualId);

        assert_eq!(
            take_pending_auth_input(owner_key),
            Some(PendingAuthInput::UserPicker)
        );
        assert_eq!(
            take_pending_auth_input(other_key),
            Some(PendingAuthInput::ManualId)
        );
    }

    #[test]
    fn test_reply_authorization_accepts_only_incoming_user_messages() {
        let user = tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
            user_id: 123456,
        });
        let anonymous = tdlib_rs::enums::MessageSender::Chat(tdlib_rs::types::MessageSenderChat {
            chat_id: -100123,
        });

        assert_eq!(authorization_target_user_id(&user, false).unwrap(), 123456);
        assert!(
            authorization_target_user_id(&user, true)
                .unwrap_err()
                .to_string()
                .contains("bot 自己")
        );
        assert!(
            authorization_target_user_id(&anonymous, false)
                .unwrap_err()
                .to_string()
                .contains("个人用户身份")
        );
    }

    // owner 在任意群回复授权时只校验发送者 user_id；负数群 ID 仅作为回复位置。
    #[test]
    fn test_group_reply_owner_validation_ignores_chat_id() {
        let config = crate::config::BotConfig {
            owner_user_id: 123456,
            ..crate::config::BotConfig::default()
        };

        assert!(
            super::ensure_owner(
                &config,
                crate::config::RequestActor {
                    request_chat_id: -100987654,
                    user_id: 123456,
                },
            )
            .is_ok()
        );
        assert!(
            super::ensure_owner(
                &config,
                crate::config::RequestActor {
                    request_chat_id: -100987654,
                    user_id: -100987654,
                },
            )
            .is_err()
        );
    }

    /// owner 添加用户后应同时更新数据库和当前进程权限。
    #[tokio::test]
    async fn test_owner_can_grant_authorized_user() -> anyhow::Result<()> {
        let _guard = crate::db::TEST_DB_LOCK.lock().await;
        let db = crate::db::get_db().await?;
        crate::db::ensure_test_schema_current(db).await?;
        let user_id = rand::rng().random_range(10_000_000..=99_999_999);
        crate::access::revoke_authorized_user_on(db, user_id).await?;

        let app = crate::app_context::AppContext::default();
        let config = crate::config::BotConfig {
            owner_user_id: 1,
            ..crate::config::BotConfig::default()
        };
        let actor = crate::config::RequestActor {
            request_chat_id: 1,
            user_id: 1,
        };
        let user_id_text = user_id.to_string();
        let reply = execute_auth_command_on(
            db,
            &app,
            &config,
            &["/auth", "add", user_id_text.as_str()],
            actor,
        )
        .await?;

        assert!(reply.contains(&user_id_text));
        assert!(app.access_control.is_authorized(user_id));
        assert!(
            crate::access::list_authorized_user_ids_on(db)
                .await?
                .contains(&user_id)
        );
        crate::access::revoke_authorized_user_on(db, user_id).await?;
        Ok(())
    }

    /// owner 删除用户后应同时撤销数据库和当前进程权限。
    #[tokio::test]
    async fn test_owner_can_revoke_authorized_user() -> anyhow::Result<()> {
        let _guard = crate::db::TEST_DB_LOCK.lock().await;
        let db = crate::db::get_db().await?;
        crate::db::ensure_test_schema_current(db).await?;
        let user_id = rand::rng().random_range(100_000_000..=199_999_999);
        crate::access::grant_authorized_user_on(db, user_id).await?;

        let app = crate::app_context::AppContext::default();
        app.access_control.authorize_user(user_id);
        let config = crate::config::BotConfig {
            owner_user_id: 1,
            ..crate::config::BotConfig::default()
        };
        let actor = crate::config::RequestActor {
            request_chat_id: 1,
            user_id: 1,
        };
        let user_id_text = user_id.to_string();
        let reply = execute_auth_command_on(
            db,
            &app,
            &config,
            &["/auth", "del", user_id_text.as_str()],
            actor,
        )
        .await?;

        assert!(reply.contains(&user_id_text));
        assert!(!app.access_control.is_authorized(user_id));
        assert!(
            !crate::access::list_authorized_user_ids_on(db)
                .await?
                .contains(&user_id)
        );
        Ok(())
    }

    /// 授权列表应区分固定角色和动态名单，便于确认权限来源。
    #[tokio::test]
    async fn test_owner_can_list_all_authorization_sources() -> anyhow::Result<()> {
        let _guard = crate::db::TEST_DB_LOCK.lock().await;
        let db = crate::db::get_db().await?;
        crate::db::ensure_test_schema_current(db).await?;
        let user_id = rand::rng().random_range(200_000_000..=299_999_999);
        crate::access::grant_authorized_user_on(db, user_id).await?;

        let app = crate::app_context::AppContext::default();
        app.access_control.authorize_user(user_id);
        let config = crate::config::BotConfig {
            owner_user_id: 1,
            admin_user_ids: BTreeSet::from([2]),
            ..crate::config::BotConfig::default()
        };
        let actor = crate::config::RequestActor {
            request_chat_id: 1,
            user_id: 1,
        };
        let reply = execute_auth_command_on(db, &app, &config, &["/auth", "list"], actor).await?;

        assert!(reply.contains("所有者：‹1›"));
        assert!(reply.contains("配置管理员：‹2›"));
        assert!(reply.contains(&format!("动态用户：‹{user_id}›")));
        crate::access::revoke_authorized_user_on(db, user_id).await?;
        Ok(())
    }

    /// 静态管理员可以使用 bot，但不能继续扩散授权。
    #[tokio::test]
    async fn test_non_owner_cannot_manage_authorization() -> anyhow::Result<()> {
        let _guard = crate::db::TEST_DB_LOCK.lock().await;
        let db = crate::db::get_db().await?;
        let app = crate::app_context::AppContext::default();
        let config = crate::config::BotConfig {
            owner_user_id: 1,
            admin_user_ids: BTreeSet::from([2]),
            ..crate::config::BotConfig::default()
        };
        let actor = crate::config::RequestActor {
            request_chat_id: 2,
            user_id: 2,
        };

        let err = execute_auth_command_on(db, &app, &config, &["/auth", "list"], actor)
            .await
            .expect_err("non-owner must be rejected");
        assert!(err.to_string().contains("仅 owner"));
        Ok(())
    }
}
