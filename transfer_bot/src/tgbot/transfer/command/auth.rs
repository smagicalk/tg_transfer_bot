use std::collections::{BTreeSet, HashMap};
use std::sync::{LazyLock, Mutex};

use crate::tgbot::transfer::card;

/// Telegram 原生用户选择按钮 ID；必须和 `MessageUsersShared.button_id` 一致。
pub(in crate::tgbot::transfer::command) const AUTH_USER_REQUEST_BUTTON_ID: i32 = 7003;

/// 授权相关回调按钮数据前缀。
const AUTH_CALLBACK_PREFIX: &str = "au:";

/// 授权面板回调按钮动作枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthCallbackAction {
    /// 打开添加管理员方式选项卡
    Add,
    /// 触发原生用户选择器键盘
    PickUser,
    /// 触发 ForceReply 手动输入用户 ID
    ManualId,
    /// 展开显示命令行用法
    ShowCommands,
    /// 折叠隐藏命令行用法
    HideCommands,
    /// 刷新管理员列表面板
    Refresh,
    /// 取消当前输入流程并返回列表
    Cancel,
    /// 删除指定动态管理员（附带目标用户的 Telegram ID）
    Delete(i64),
}

/// 挂起的授权输入状态枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PendingAuthInput {
    /// 正在等待用户通过原生选择器选择联系人
    UserPicker,
    /// 正在等待用户通过文本或 ForceReply 输入纯数字 ID
    ManualId,
}

/// 授权输入状态键名：`(chat_id, user_id)`，将会话与操作用户严格隔离。
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

/// 用户资料简要快照（用于展示与记录）。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct UserProfileSnapshot {
    /// 显示名称（名 + 姓）
    display_name: Option<String>,
    /// Telegram 用户名（不带 @）
    username: Option<String>,
}

/// 授权列表中的单条成员信息条目。
#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthListEntry {
    /// 角色名称（如“所有者”、“配置管理员”、“动态管理员”）
    role: &'static str,
    /// Telegram 用户 ID
    user_id: i64,
    /// 用户快照资料
    profile: UserProfileSnapshot,
    /// 是否可被动态删除（静态配置的 owner/admin 为 false）
    removable: bool,
}

/// 判断 callback 是否属于授权管理面板。
///
/// # 参数
/// - `data`: 回调数据字符串
///
/// # 返回值
/// - `bool`: 若匹配 `au:` 前缀返回 true
pub(in crate::tgbot::transfer::command) fn is_auth_callback_data(data: &str) -> bool {
    data.starts_with(AUTH_CALLBACK_PREFIX)
}

/// 解析授权回调数据为 `AuthCallbackAction` 枚举。
///
/// # 参数
/// - `data`: 原始回调数据
///
/// # 返回值
/// - `Option<AuthCallbackAction>`: 解析出的动作枚举
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

/// 拼接生成完整的授权回调数据。
///
/// # 参数
/// - `action`: 动作后缀字符串
///
/// # 返回值
/// - `String`: 附带前缀的回调字符串
fn auth_callback_data(action: &str) -> String {
    format!("{AUTH_CALLBACK_PREFIX}{action}")
}

/// 生成授权管理首页 callback，供菜单的 owner 专属入口复用。
///
/// # 返回值
/// - `String`: 刷新授权面板的回调数据
pub(in crate::tgbot::transfer::command) fn build_auth_panel_callback_data() -> String {
    auth_callback_data("refresh")
}

/// 登记挂起的授权输入状态。
///
/// # 参数
/// - `key`: 会话与操作者键
/// - `input`: 输入类型
fn set_pending_auth_input(key: AuthInputKey, input: PendingAuthInput) {
    let mut guard = PENDING_AUTH_INPUTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    guard.insert(key, input);
}

/// 消费并取走挂起的授权输入状态。
///
/// # 参数
/// - `key`: 会话与操作者键
///
/// # 返回值
/// - `Option<PendingAuthInput>`: 原有挂起状态
fn take_pending_auth_input(key: AuthInputKey) -> Option<PendingAuthInput> {
    PENDING_AUTH_INPUTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&key)
}

/// 查看挂起的授权输入状态（不移除）。
///
/// # 参数
/// - `key`: 会话与操作者键
///
/// # 返回值
/// - `Option<PendingAuthInput>`: 当前挂起状态
fn pending_auth_input(key: AuthInputKey) -> Option<PendingAuthInput> {
    PENDING_AUTH_INPUTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(&key)
        .copied()
}

/// 清除挂起的授权输入状态。
///
/// # 参数
/// - `key`: 会话与操作者键
///
/// # 返回值
/// - `bool`: 若之前存在状态返回 true
fn clear_pending_auth_input(key: AuthInputKey) -> bool {
    PENDING_AUTH_INPUTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&key)
        .is_some()
}

/// 记录当前授权输入提示，并返回同一会话上一条未清理的提示 ID。
///
/// # 参数
/// - `key`: 会话与操作者键
/// - `message_id`: 新提示消息 ID
///
/// # 返回值
/// - `Option<i64>`: 旧提示消息 ID
fn remember_auth_prompt(key: AuthInputKey, message_id: i64) -> Option<i64> {
    PENDING_AUTH_PROMPTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(key, message_id)
}

/// 取出并清除当前授权输入提示 ID。
///
/// # 参数
/// - `key`: 会话与操作者键
///
/// # 返回值
/// - `Option<i64>`: 原有提示消息 ID
fn take_auth_prompt(key: AuthInputKey) -> Option<i64> {
    PENDING_AUTH_PROMPTS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&key)
}

/// 删除授权输入提示及其原生键盘；清理失败只记日志，不回滚业务状态。
///
/// # 参数
/// - `key`: 会话与操作者键
/// - `message_id`: 提示消息 ID
/// - `client_id`: TDLib 客户端实例 ID
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
///
/// # 参数
/// - `key`: 会话与操作者键
/// - `client_id`: TDLib 客户端实例 ID
///
/// # 返回值
/// - `(Option<PendingAuthInput>, bool)`: 原挂起状态及旧提示是否被成功删除
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
///
/// # 参数
/// - `key`: 会话与操作者键
/// - `client_id`: TDLib 客户端实例 ID
/// - `notice`: 发送的通知文案
///
/// # 返回值
/// - `anyhow::Result<bool>`: 若存在挂起状态并已清理返回 Ok(true)
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
///
/// # 参数
/// - `key`: 会话与操作者键
/// - `next`: 下一个目标输入方式
/// - `client_id`: TDLib 客户端实例 ID
async fn switch_pending_auth_input_on(
    key: AuthInputKey,
    next: PendingAuthInput,
    client_id: i32,
) -> anyhow::Result<()> {
    clear_pending_auth_input_silently(key, client_id).await;
    set_pending_auth_input(key, next);
    Ok(())
}

/// 授权命令帮助概要。
pub(in crate::tgbot::transfer::command) fn auth_help_summary() -> &'static str {
    "交互式查看或管理管理员名单；仅 owner 可执行。"
}

/// 构造授权命令详情帮助正文。
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
///
/// 支持的调用模式：
/// - `/auth` 或 `/auth list`: 查看当前管理员列表卡片（若在群组中回复某用户消息，则直接授权该被回复用户）
/// - `/auth add <user_id>`: 直接通过命令行授权指定 Telegram 用户
/// - `/auth del <user_id>`: 撤销指定用户的动态授权
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `text`: 分词后的命令行参数
/// - `config`: Bot 配置引用
/// - `request_message`: 原始消息对象
/// - `actor`: 请求发起者信息
/// - `client_id`: TDLib 客户端实例 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 执行成功返回 `Ok(())`
pub(in crate::tgbot) async fn auth_command_on(
    app: &crate::app_context::AppContext,
    text: Vec<&str>,
    config: &crate::config::BotConfig,
    request_message: &tdlib_rs::types::Message,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    // 权限校验：仅允许所有者执行
    ensure_owner(config, actor)?;
    let db_conn = crate::db::get_db().await?;
    match text.get(1).copied() {
        None | Some("list") => {
            if text.len() > 2 {
                anyhow::bail!("usage: /auth list");
            }
            // 若为回复消息快捷授权
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
///
/// # 参数
/// - `db_conn`: 数据库连接
/// - `app`: 全局应用上下文
/// - `config`: Bot 配置
/// - `request_message`: 当前指令消息
/// - `request_chat_id`: 发送回复的目标会话
/// - `client_id`: TDLib 客户端 ID
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
///
/// # 参数
/// - `request_message`: 请求消息
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<(i64, UserProfileSnapshot)>`: 提取的目标用户 ID 与资料快照
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

/// 从消息发送者结构体提取合法的授权用户 ID。
///
/// 排除发送者为 bot 自身，或使用频道/匿名管理员身份的消息。
///
/// # 参数
/// - `sender`: 消息发送者枚举
/// - `is_outgoing`: 消息是否为当前客户端发出
///
/// # 返回值
/// - `anyhow::Result<i64>`: 目标 Telegram 用户 ID
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

/// 写入授权用户记录及其资料快照，并同步更新内存中的权限控制缓存。
///
/// # 参数
/// - `db_conn`: 数据库连接
/// - `app`: 全局应用上下文
/// - `config`: Bot 配置
/// - `user_id`: 目标用户 ID
/// - `profile`: 用户资料快照
///
/// # 返回值
/// - `anyhow::Result<(&'static str, &'static str)>`: `(状态代码, 详情说明)`
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
///
/// # 参数
/// - `db_conn`: 数据库连接
/// - `app`: 全局应用上下文
/// - `config`: Bot 配置
/// - `text`: 命令行切片
/// - `actor`: 请求发起者
///
/// # 返回值
/// - `anyhow::Result<String>`: 执行结果卡片文本
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
///
/// # 参数
/// - `user`: TDLib 用户对象引用
///
/// # 返回值
/// - `UserProfileSnapshot`: 提取的资料快照
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
///
/// # 参数
/// - `user_id`: 目标用户 ID
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `UserProfileSnapshot`: 用户资料快照
async fn lookup_user_profile(user_id: i64, client_id: i32) -> UserProfileSnapshot {
    match tdlib_rs::functions::get_user(user_id, client_id).await {
        Ok(tdlib_rs::enums::User::User(user)) => profile_from_user(&user),
        Err(err) => {
            tracing::debug!(user_id, error = %err.message, "unable to load Telegram user profile");
            UserProfileSnapshot::default()
        }
    }
}

/// 格式化用户全名（名与姓拼接）。
///
/// # 参数
/// - `first_name`: 名
/// - `last_name`: 姓
///
/// # 返回值
/// - `Option<String>`: 组合后的姓名字符串，两者皆空时返回 None
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

/// 从 Telegram 用户共享对象（`SharedUser`）中提取资料快照。
///
/// # 参数
/// - `user`: 共享用户对象引用
///
/// # 返回值
/// - `UserProfileSnapshot`: 提取的资料快照
fn profile_from_shared_user(user: &tdlib_rs::types::SharedUser) -> UserProfileSnapshot {
    UserProfileSnapshot {
        display_name: format_display_name(&user.first_name, &user.last_name),
        username: (!user.username.trim().is_empty())
            .then(|| user.username.trim().trim_start_matches('@').to_owned()),
    }
}

/// 格式化用户展示标签，如 `张三 (@zhangsan)` 或纯名字。
///
/// # 参数
/// - `profile`: 用户资料快照引用
///
/// # 返回值
/// - `String`: 组合后的展示标签
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

/// 加载并组装所有管理员条目（所有者 + 配置文件管理员 + 数据库动态管理员）。
///
/// # 参数
/// - `db_conn`: 数据库连接
/// - `config`: Bot 配置
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<Vec<AuthListEntry>>`: 排序排重后的管理员条目列表
async fn load_auth_list_entries(
    db_conn: &sea_orm::DatabaseConnection,
    config: &crate::config::BotConfig,
    client_id: i32,
) -> anyhow::Result<Vec<AuthListEntry>> {
    let mut entries = Vec::with_capacity(config.admin_user_ids.len() + 2);
    // 第一条固定为所有者
    let owner_profile = lookup_user_profile(config.owner_user_id, client_id).await;
    entries.push(AuthListEntry {
        role: "所有者",
        user_id: config.owner_user_id,
        profile: owner_profile,
        removable: false,
    });

    let mut fixed_ids = BTreeSet::from([config.owner_user_id]);
    // 加载配置文件中指定的静态管理员
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

    // 从数据库查询动态管理员
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

/// 格式化授权面板卡片正文。
///
/// # 参数
/// - `entries`: 管理员条目列表
/// - `notice`: 可选通知提示
/// - `show_commands`: 是否展开命令行用法说明
///
/// # 返回值
/// - `String`: 完整卡片文本
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

/// 构造授权回调按钮工具函数。
///
/// # 参数
/// - `text`: 按钮文本
/// - `action`: 动作后缀
/// - `style`: 按钮样式
///
/// # 返回值
/// - `InlineKeyboardButton`: 内联按钮对象
fn build_auth_callback_button(
    text: &str,
    action: &str,
    style: tdlib_rs::enums::ButtonStyle,
) -> tdlib_rs::types::InlineKeyboardButton {
    crate::tgbot::send::build_callback_button(text, &auth_callback_data(action), style)
}

/// 构造授权主面板按钮矩阵。
///
/// # 参数
/// - `entries`: 管理员条目
/// - `show_commands`: 当前是否处于显示命令状态
///
/// # 返回值
/// - `Vec<Vec<InlineKeyboardButton>>`: 内联键盘按钮矩阵
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

/// 发送新的授权管理面板卡片。
///
/// # 参数
/// - `db_conn`: 数据库连接
/// - `config`: Bot 配置
/// - `chat_id`: 目标会话 ID
/// - `client_id`: TDLib 客户端 ID
/// - `notice`: 可选顶部提示
///
/// # 返回值
/// - `anyhow::Result<()>`: 发送成功返回 Ok(())
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

/// 添加管理员方式选项卡正文。
///
/// # 返回值
/// - `String`: 选项卡提示文本
fn build_auth_add_options_text() -> String {
    [
        "授权管理".to_owned(),
        card::section("添加管理员"),
        card::note("选择 Telegram 用户会打开原生用户选择器；也可以输入数字 ID。"),
    ]
    .join("\n")
}

/// 添加管理员方式选项卡按钮。
///
/// # 返回值
/// - `Vec<Vec<InlineKeyboardButton>>`: 内联键盘按钮矩阵
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

/// 编辑已有的授权管理面板卡片消息。
///
/// 当用户点击“刷新”、“查看命令”、“隐藏命令”或“删除”等按钮时，通过编辑现有消息刷新列表，
/// 避免在聊天窗口中产生过多历史消息刷屏。
///
/// # 参数
/// - `db_conn`: 数据库连接引用
/// - `config`: Bot 配置引用
/// - `chat_id`: 会话 ID
/// - `message_id`: 目标消息 ID
/// - `client_id`: TDLib 客户端 ID
/// - `notice`: 可选的顶部提示信息（如操作成功/失败反馈）
/// - `show_commands`: 是否展示文本命令说明
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功编辑返回 Ok(())
async fn edit_auth_panel_on(
    db_conn: &sea_orm::DatabaseConnection,
    config: &crate::config::BotConfig,
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    notice: Option<&str>,
    show_commands: bool,
) -> anyhow::Result<()> {
    // 重新从数据库与配置中加载最新的管理员与用户信息列表
    let entries = load_auth_list_entries(db_conn, config, client_id).await?;
    // 渲染卡片正文与按钮矩阵
    let (text, keyboard) = crate::tgbot::send::ReplyPanel::card(format_auth_panel_text(
        &entries,
        notice,
        show_commands,
    ))
    .rows(build_auth_panel_rows(&entries, show_commands))
    .into_card_parts()?;
    // 调用接口就地更新卡片消息文本与键盘
    crate::tgbot::send::edit_card_message_with_inline_keyboard(
        text, chat_id, message_id, keyboard, client_id,
    )
    .await
}

/// 编辑现有卡片为“添加管理员方式选择”选项卡。
///
/// 向用户呈现“选择 Telegram 用户”和“输入用户 ID”两个分支选项。
///
/// # 参数
/// - `chat_id`: 会话 ID
/// - `message_id`: 目标卡片消息 ID
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功编辑返回 Ok(())
async fn edit_auth_add_options_on(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    // 构建添加方式选项卡内容与内联键盘
    let (text, keyboard) = crate::tgbot::send::ReplyPanel::card(build_auth_add_options_text())
        .rows(build_auth_add_options_rows())
        .into_card_parts()?;
    crate::tgbot::send::edit_card_message_with_inline_keyboard(
        text, chat_id, message_id, keyboard, client_id,
    )
    .await
}

/// 发送当前授权输入方式的提示和原生键盘。
///
/// 根据用户选择的输入模式（原生用户选择器或手动输入 ID），发送相应的提示消息和专属键盘（如 ForceReply 或 UserRequest 按钮）。
///
/// # 参数
/// - `input`: 等待的授权输入类型（UserPicker 或 ManualId）
/// - `key`: (chat_id, user_id) 复合索引
/// - `client_id`: TDLib 客户端 ID
/// - `note`: 补充提示说明内容
///
/// # 返回值
/// - `anyhow::Result<()>`: 发送成功返回 Ok(())
async fn send_auth_input_prompt(
    input: PendingAuthInput,
    key: AuthInputKey,
    client_id: i32,
    note: &str,
) -> anyhow::Result<()> {
    // 格式化不同输入模式的卡片提示文本
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
    // 发送带有特殊交互键盘的卡片消息
    let sent = match input {
        // 用户选择器模式：发送带有原生请求用户按钮的普通键盘
        PendingAuthInput::UserPicker => {
            crate::tgbot::send::send_card_message_with_user_request_keyboard_returning(
                text,
                key.0,
                AUTH_USER_REQUEST_BUTTON_ID,
                client_id,
            )
            .await?
        }
        // 手动 ID 模式：发送带有 ForceReply 的卡片消息强制呼出输入框
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
    // 记录本次发送的提示消息 ID，若之前有历史提示消息则将其删除，避免界面残留无效键盘
    if let Some(previous_id) = remember_auth_prompt(key, sent.id)
        && previous_id != sent.id
    {
        delete_auth_prompt(key, previous_id, client_id).await;
    }
    Ok(())
}

/// 数据库写入失败时保留当前步骤，并重新显示可操作的输入控件。
///
/// # 参数
/// - `key`: (chat_id, user_id) 会话与操作者键
/// - `input`: 当前挂起的输入状态
/// - `client_id`: TDLib 客户端 ID
/// - `error`: 数据库或其他操作失败的错误信息
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功重新提示返回 Ok(())
async fn report_auth_add_failure(
    key: AuthInputKey,
    input: PendingAuthInput,
    client_id: i32,
    error: anyhow::Error,
) -> anyhow::Result<()> {
    // 记录错误日志
    tracing::error!(error = %error, user_id = key.1, "dynamic authorization persistence failed");
    // 保留当前的挂起输入状态
    set_pending_auth_input(key, input);
    // 重新发送带有错误提示的输入提示卡片
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

/// 处理授权管理内联键盘回调查询（Inline Keyboard Callback Query）。
///
/// 负责响应授权面板上的所有内联按钮点击事件（如添加、选择模式、删除、查看命令、刷新、取消等）。
/// 包含严格的操作权限校验（仅限 Owner）。
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `update`: TDLib 回调查询事件
/// - `config`: Bot 配置引用
/// - `actor`: 当前请求的操作者信息
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功处理返回 Ok(())
pub(in crate::tgbot) async fn auth_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: std::sync::Arc<crate::config::BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    // 提取回调数据载荷
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
    // 解析具体的授权操作动作
    let Some(action) = parse_auth_callback_data(&data.data) else {
        crate::tgbot::send::answer_callback_query(update.id, Some("授权按钮参数无效"), client_id)
            .await?;
        return Ok(());
    };

    // 校验权限：仅 Owner 允许操作授权管理面板
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

    // 根据解析出的具体动作分发处理
    match action {
        // 点击“添加管理员”：展示方式选择卡片（选择 Telegram 用户 vs 手动输入 ID）
        AuthCallbackAction::Add => {
            crate::tgbot::send::answer_callback_query(update.id, Some("选择添加方式"), client_id)
                .await?;
            clear_pending_auth_input_silently((update.chat_id, update.sender_user_id), client_id)
                .await;
            edit_auth_add_options_on(update.chat_id, update.message_id, client_id).await
        }
        // 点击“选择 Telegram 用户”：切换到 UserPicker 模式并发送原生选择器键盘
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
            // 新的原生用户选择器已经承载当前步骤，旧的“选择添加方式”卡片不再可用，将其删除
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
        // 点击“输入用户 ID”：切换到 ManualId 模式并发送 ForceReply 提示卡片
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
            // ForceReply 已经成为唯一输入入口，删除旧选项卡避免用户重复点击旧按钮
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
        // 点击“查看命令”或“隐藏命令”：切换文本命令的折叠/展开显示
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
        // 点击“刷新”：重新拉取数据库记录并就地编辑卡片
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
        // 点击“取消”：终止当前添加操作，清理挂起状态并恢复列表卡片
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
        // 点击具体管理员行旁的“删除”按钮：撤销该动态管理员授权
        AuthCallbackAction::Delete(user_id) => {
            // 静态配置文件中定义的管理员与 Owner 不允许在此处删除
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
            // 从数据库中删除记录
            let removed = crate::access::revoke_authorized_user_on(db_conn, user_id).await?;
            // 同步从内存白名单中撤销
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
            // 刷新授权主面板并附带结果提示
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

/// 执行并完成授权添加流程。
///
/// 写入数据库动态授权表、更新内存访问控制策略、清理交互提示并发送最新列表。
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `config`: Bot 配置引用
/// - `user_id`: 被授权目标用户的 ID
/// - `profile`: 目标用户的快照画像（昵称与用户名）
/// - `input`: 当前挂起的输入类型
/// - `key`: (chat_id, user_id) 复合标识键
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功执行返回 Ok(())
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
    // 校验 ID 合法性：必须大于 0
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
    // 获取数据库连接
    let db_conn = match crate::db::get_db().await {
        Ok(db_conn) => db_conn,
        Err(error) => return report_auth_add_failure(key, input, client_id, error).await,
    };
    // 写入数据库授权表并更新内存白名单
    let detail = match grant_authorized_user_with_profile_on(
        db_conn, app, config, user_id, &profile,
    )
    .await
    {
        Ok((_, detail)) => detail,
        Err(error) => return report_auth_add_failure(key, input, client_id, error).await,
    };
    // 清理挂起的输入状态
    clear_pending_auth_input(key);
    // 删除输入提示卡片（如带有 ForceReply 或 UserRequest 的消息）
    if let Some(message_id) = take_auth_prompt(key) {
        delete_auth_prompt(key, message_id, client_id).await;
    }
    // 管理员列表已经包含本次授权结果提示；只发送这一张刷新后的列表，
    // 避免“成功卡片 + 列表卡片”连续出现两条重复回复。
    send_auth_panel_on(db_conn, config, request_chat_id, client_id, Some(detail)).await
}

/// 处理授权向导中的普通文本输入；返回 true 表示消息已被授权流程消费。
///
/// 当用户处于 `ManualId` 或 `UserPicker` 阶段时，发送的纯文本（用户 ID、取消等）在此拦截处理。
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `text`: 接收到的文本消息
/// - `config`: Bot 配置引用
/// - `actor`: 发送消息的用户与会话信息
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<bool>`: 若消息被授权流程拦截消费则返回 Ok(true)，否则返回 Ok(false)
pub(in crate::tgbot) async fn handle_auth_text_input_on(
    app: &crate::app_context::AppContext,
    text: &str,
    config: std::sync::Arc<crate::config::BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<bool> {
    let key = (actor.request_chat_id, actor.user_id);
    // 判断当前用户是否处于授权输入等待状态
    let Some(input) = pending_auth_input(key) else {
        return Ok(false);
    };
    // 用户输入取消指令
    if text.trim().eq_ignore_ascii_case("/cancel") || text.trim() == "取消" {
        clear_pending_auth_input_and_remove_keyboard(key, client_id, "授权添加已取消。").await?;
        return Ok(true);
    }
    // 其它以 / 开头的斜杠命令不当作用户 ID 处理，交由通用命令分发器
    if text.trim().starts_with('/') {
        return Ok(false);
    }
    // 解析用户输入的数字 ID
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
    // 尝试查询该用户的基础画像信息（昵称和用户名）
    let profile = if matches!(
        input,
        PendingAuthInput::ManualId | PendingAuthInput::UserPicker
    ) {
        lookup_user_profile(user_id, client_id).await
    } else {
        UserProfileSnapshot::default()
    };
    // 完成授权添加
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
///
/// 当用户通过原生用户选择器按钮分享了一个或多个用户时，在此接收并处理首个用户。
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `shared`: Telegram 原生分享的用户结构体
/// - `config`: Bot 配置引用
/// - `request_chat_id`: 会话 ID
/// - `sender_user_id`: 操作者用户 ID
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<bool>`: 若事件被消费返回 Ok(true)，否则返回 Ok(false)
pub(in crate::tgbot) async fn handle_auth_shared_user_input(
    app: &crate::app_context::AppContext,
    shared: &tdlib_rs::types::MessageUsersShared,
    config: std::sync::Arc<crate::config::BotConfig>,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<bool> {
    // 校验按钮 ID 是否匹配本授权流程所注册的特殊按钮 ID
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
        // 迟到的选择结果不能清掉当前手动 ID 输入流程
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
    // 成功收到选择后先消费等待态，避免重复 update 重复授权；数据库失败时由完成函数恢复
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

/// 取消当前用户的授权向导输入流程并移除键盘。
///
/// 用于响应 `/cancel` 指令。
///
/// # 参数
/// - `request_chat_id`: 会话 ID
/// - `sender_user_id`: 操作者用户 ID
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<bool>`: 成功取消返回 Ok(bool)
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

/// 当用户触发了其它新命令时，自动放弃当前授权向导输入状态。
///
/// 避免旧的授权原生选择键盘继续拦截用户的后续输入。
///
/// # 参数
/// - `request_chat_id`: 会话 ID
/// - `sender_user_id`: 操作者用户 ID
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<bool>`: 若之前存在挂起输入且已被放弃返回 Ok(true)
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

/// 确保操作者具备 Owner 权限。
///
/// # 参数
/// - `config`: Bot 配置引用
/// - `actor`: 当前请求操作者
///
/// # 返回值
/// - `anyhow::Result<()>`: 若不是 Owner 则抛出错误
fn ensure_owner(
    config: &crate::config::BotConfig,
    actor: crate::config::RequestActor,
) -> anyhow::Result<()> {
    if actor.user_id != config.owner_user_id {
        anyhow::bail!("仅 owner 可管理授权");
    }
    Ok(())
}

/// 解析命令行参数中的目标用户 ID。
///
/// # 参数
/// - `text`: 分词后的命令行参数片段数组
/// - `usage`: 错误时提示的命令格式说明
///
/// # 返回值
/// - `anyhow::Result<i64>`: 解析出的合法正整数用户 ID
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

/// 格式化授权操作结果的卡片文本。
///
/// # 参数
/// - `status`: 状态标题（如 success / failed）
/// - `user_id`: 目标用户 ID
/// - `detail`: 详细说明或备注
///
/// # 返回值
/// - `String`: 格式化后的卡片文本
fn format_auth_result(status: &str, user_id: i64, detail: &str) -> String {
    [
        "授权管理".to_owned(),
        card::field_pair("状态", status, "用户", user_id),
        card::DIVIDER.to_owned(),
        card::note(detail),
    ]
    .join("\n")
}

/// 格式化文本命令形式的授权列表（`/auth list` 响应）。
///
/// # 参数
/// - `config`: Bot 配置引用
/// - `dynamic_user_ids`: 动态授权用户 ID 集合
///
/// # 返回值
/// - `String`: 格式化后的授权列表文本
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

    /// 测试授权内联按钮回调数据能够正确路由到对应的交互动作。
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

    /// 测试授权帮助卡片文本包含快捷回复授权的说明。
    #[test]
    fn test_auth_help_mentions_reply_shortcut() {
        let text = super::build_auth_help_detail_text();

        assert!(text.contains("回复目标用户的消息"));
        assert!(text.contains("/auth"));
    }

    /// 测试用户画像格式化优先使用姓名并保留用户名。
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

    /// 测试授权面板正确展示用户姓名、ID，且仅对动态管理员提供删除按钮。
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

    /// 测试挂起的授权输入状态按私聊操作者作用域严格隔离。
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

    /// 测试回复消息授权仅接受个人用户消息，拒绝自身与群组频道匿名发送者。
    #[test]
    fn test_reply_authorization_accepts_only_incoming_user_messages() {
        let user = tdlib_rs::enums::MessageSender::User(Box::new(
            tdlib_rs::types::MessageSenderUser {
                user_id: 123456,
            },
        ));
        let anonymous = tdlib_rs::enums::MessageSender::Chat(Box::new(
            tdlib_rs::types::MessageSenderChat {
                chat_id: -100123,
            },
        ));

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

    /// 测试在群组中回复授权时，校验 Owner 权限只认 user_id，忽略负数群聊 chat_id。
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
