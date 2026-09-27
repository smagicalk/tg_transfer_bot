//! 用户执行器（User Executor）的 Owner 专属交互控制面板。
//!
//! # 核心职责
//! - **按需动态拉起 User Client**：默认情况下 Bot 作为主执行端；仅当需要转存私有频道或遇到权限限制时，由 Owner 在 Bot 私聊中手动登录 User TDLib Client。
//! - **安全登录流转**：通过二维码扫描登录（生成二维码发送至 Owner 私聊）以及两步验证密码（ForceReply 输入后立即从 Telegram 删除消息）。
//! - **执行器面板与生命周期管理**：在 Telegram 卡片中直观展示执行器登录状态（未登录、扫码中、已登录、下线排空中等），支持一键登出与优雅下线。
//! - **隐私与安全性保障**：登录凭据与二维码临时文件绝不落盘、不记入日志，退出后立即清理。

use std::sync::Arc;

use crate::app_context::{ExecutorIdentity, ExecutorPhase, ExecutorRuntimeState};
use crate::config::{BotConfig, ClientRole, RequestActor};
use crate::tgbot::send;

/// 执行器相关内联键盘 Callback Data 前缀。
const EXECUTOR_CALLBACK_PREFIX: &str = "ex:";

/// 判断指定的 Callback Data 是否属于执行器控制面板的操作指令。
///
/// # 参数
/// * `data` - 内联键盘按钮回调数据字符串
pub(crate) fn is_executor_callback_data(data: &str) -> bool {
    data.starts_with(EXECUTOR_CALLBACK_PREFIX)
}

/// 构建打开执行器控制面板的 Callback Data。
pub(crate) fn build_executor_panel_callback_data() -> String {
    format!("{EXECUTOR_CALLBACK_PREFIX}open")
}

/// 构建执行器指定动作的 Callback Data。
///
/// # 参数
/// * `action` - 动作标识（如 "login", "logout", "cancel"）
fn build_executor_callback_data(action: &str) -> String {
    format!("{EXECUTOR_CALLBACK_PREFIX}{action}")
}

/// 从 User TDLib 收到二维码登录链接后生成图片并发送给 Owner 私聊。
///
/// 若已有历史二维码消息，则直接编辑更新图片；否则发送新图片并保存其消息 ID 供后续刷新。
///
/// # 参数
/// * `app` - 全局应用上下文引用
/// * `user_client_id` - User 执行器客户端标识
/// * `qr_link` - TDLib 返回的 `tg://login?token=...` 二维码登录链接
/// * `bot_client_id` - 发送消息所使用的 Bot 客户端标识
///
/// # 返回
/// 成功返回 `Ok(())`，失败返回相应错误
pub(crate) async fn send_qr_code_to_owner(
    app: &crate::app_context::AppContext,
    user_client_id: i32,
    qr_link: String,
    bot_client_id: i32,
) -> anyhow::Result<()> {
    let Some(owner_chat_id) = app.executor_runtime.owner_chat_id() else {
        anyhow::bail!("executor login owner chat is not available");
    };
    if app.executor_runtime.user_client_id() != Some(user_client_id) {
        anyhow::bail!("executor QR belongs to an inactive client");
    }

    // 生成二维码图像并保存到系统临时文件
    let code = qrcode::QrCode::with_error_correction_level(qr_link.as_bytes(), qrcode::EcLevel::Q)?;
    let image = code.render::<image::Luma<u8>>().quiet_zone(true).build();
    let path =
        std::env::temp_dir().join(format!("tg-transfer-bot-executor-qr-{user_client_id}.png"));
    image.save(&path)?;
    let caption = "请在 Telegram 已登录设备中扫描此二维码登录执行器。二维码会自动失效。";
    if let Some(message_id) = app.executor_runtime.qr_message_id() {
        // 原地编辑已有二维码消息
        send::edit_local_photo(
            &path.to_string_lossy(),
            caption,
            owner_chat_id,
            message_id,
            bot_client_id,
        )
        .await?;
    } else {
        // 发送全新二维码图片消息
        let receipt = send::send_local_photo_returning(
            &path.to_string_lossy(),
            caption,
            owner_chat_id,
            bot_client_id,
        )
        .await?;
        app.executor_runtime.replace_qr_message_id(receipt.id);
    }
    // 清理被替换的旧二维码临时文件
    if let Some(old_path) = app.executor_runtime.replace_qr_image_path(path.clone())
        && old_path != path
    {
        let _ = std::fs::remove_file(old_path);
    }
    Ok(())
}

/// 读取并刷新已登录执行器的账号摘要信息。
///
/// 面板仅展示用户 ID、显示名称（名字+姓氏）和用户名，严格不记录或显示手机号等敏感信息。
///
/// # 参数
/// * `app` - 全局应用上下文引用
/// * `user_client_id` - User 执行器客户端标识
pub(crate) async fn refresh_executor_identity(
    app: &crate::app_context::AppContext,
    user_client_id: i32,
) -> anyhow::Result<()> {
    // 调用 TDLib getMe 查询当前执行端个人资料
    let tdlib_rs::enums::User::User(user) = tdlib_rs::functions::get_me(user_client_id)
        .await
        .map_err(|error| anyhow::Error::new(crate::tgbot::TdError(error)))?;
    let display_name = [user.first_name.trim(), user.last_name.trim()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let username = user
        .usernames
        .as_ref()
        .and_then(|names| names.active_usernames.first())
        .cloned()
        .filter(|name| !name.trim().is_empty());
    let display_name = if display_name.is_empty() {
        username.clone().unwrap_or_else(|| "未设置名称".to_owned())
    } else {
        display_name
    };
    let identity = ExecutorIdentity {
        user_id: user.id,
        display_name,
        username,
    };
    if app
        .executor_runtime
        .set_identity_if_ready(user_client_id, identity)
    {
        tracing::info!(user_client_id, "executor account identity refreshed");
    }
    Ok(())
}

/// 当 User TDLib 要求二次验证（2FA）密码时，在 Bot 私聊中使用 ForceReply 引导 Owner 输入密码。
///
/// # 参数
/// * `app` - 全局应用上下文引用
/// * `user_client_id` - User 客户端标识
/// * `password_hint` - 密码提示文本
/// * `bot_client_id` - 交互使用的 Bot 客户端标识
pub(crate) async fn request_two_factor_password(
    app: &crate::app_context::AppContext,
    user_client_id: i32,
    password_hint: &str,
    bot_client_id: i32,
) -> anyhow::Result<()> {
    if !app.executor_runtime.set_waiting_password(user_client_id) {
        return Ok(());
    }
    let Some(owner_chat_id) = app.executor_runtime.owner_chat_id() else {
        anyhow::bail!("executor login owner chat is not available");
    };
    let hint = if password_hint.trim().is_empty() {
        "无".to_owned()
    } else {
        password_hint.to_owned()
    };
    let prompt = send::send_card_message_with_force_reply_returning(
        format!(
            "执行器登录\n\n需要二次验证密码。\n提示：{hint}\n\n回复本消息输入密码；密码消息会在提交后删除。"
        ),
        owner_chat_id,
        "输入二次验证密码",
        bot_client_id,
    )
    .await?;
    if let Some(previous_prompt_id) = app
        .executor_runtime
        .replace_password_prompt_message_id(prompt.id)
    {
        let _ = send::delete_message(owner_chat_id, previous_prompt_id, bot_client_id).await;
    }
    Ok(())
}

/// 提交二次验证密码至 User TDLib。
///
/// 校验当前是否处于等待密码状态、发送者是否为 Owner 以及回复的目标消息是否匹配。
///
/// # 参数
/// * `app` - 全局应用上下文引用
/// * `sender_user_id` - 消息发送者 ID
/// * `reply_message_id` - 回复的消息 ID
/// * `password` - 输入的密码字符串
///
/// # 返回
/// 成功消费并校验密码返回 `Ok(true)`，状态不匹配返回 `Ok(false)`，密码错误返回 Err
pub(crate) async fn submit_two_factor_password(
    app: &crate::app_context::AppContext,
    sender_user_id: i64,
    reply_message_id: Option<i64>,
    password: String,
) -> anyhow::Result<bool> {
    if app.executor_runtime.phase() != ExecutorPhase::WaitingPassword {
        return Ok(false);
    }
    let Some(client_id) = app.executor_runtime.user_client_id() else {
        return Ok(false);
    };
    if app.executor_runtime.owner_chat_id() != Some(sender_user_id) {
        return Ok(false);
    }
    if app.executor_runtime.password_prompt_message_id() != reply_message_id {
        return Ok(false);
    }
    tdlib_rs::functions::check_authentication_password(password, client_id)
        .await
        .map_err(|error| {
            anyhow::anyhow!("executor password verification failed: {}", error.message)
        })?;
    Ok(true)
}

/// 执行器面板专属 Callback Query 路由分发器。
///
/// 处理面板开启、登录发起、登出排空及取消登出等交互指令，严格限制仅 Owner 可操作。
///
/// # 参数
/// * `app` - 全局应用上下文引用
/// * `update` - TDLib 回调更新对象
/// * `config` - 机器人全局配置
/// * `actor` - 请求操作者信息
/// * `client_id` - 当前响应使用的 Bot 客户端标识
pub(crate) async fn executor_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: Arc<BotConfig>,
    actor: RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let tdlib_rs::enums::CallbackQueryPayload::Data(data) = update.payload else {
        send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
        return Ok(());
    };
    if actor.user_id != config.owner_user_id {
        send::answer_callback_query(update.id, Some("仅 owner 可管理执行器"), client_id).await?;
        return Ok(());
    }

    match data.data.as_str() {
        "ex:open" => {
            send::answer_callback_query(update.id, Some("执行器"), client_id).await?;
            edit_executor_panel(app, update.chat_id, update.message_id, client_id).await
        }
        "ex:login" => {
            if app.executor_runtime.phase() != ExecutorPhase::Offline {
                send::answer_callback_query(update.id, Some("执行器已在登录或在线"), client_id)
                    .await?;
                return Ok(());
            }
            let user_client_id = create_user_client(Arc::new(app.clone()), config.as_ref()).await?;
            app.executor_runtime
                .begin_login(user_client_id, update.chat_id);
            send::answer_callback_query(update.id, Some("正在申请二维码"), client_id).await?;
            edit_executor_panel(app, update.chat_id, update.message_id, client_id).await
        }

        "ex:logout" => {
            let Some(user_client_id) = app.executor_runtime.user_client_id() else {
                send::answer_callback_query(update.id, Some("执行器未登录"), client_id).await?;
                return Ok(());
            };
            if !app.executor_runtime.begin_draining(user_client_id) {
                send::answer_callback_query(update.id, Some("执行器当前不能退出"), client_id)
                    .await?;
                return Ok(());
            }
            app.transfer_runtime.begin_transfer_drain();
            spawn_executor_logout_after_drain(
                app.transfer_runtime.clone(),
                app.executor_runtime.clone(),
                user_client_id,
            );
            send::answer_callback_query(update.id, Some("等待现有任务结束"), client_id).await?;
            edit_executor_panel(app, update.chat_id, update.message_id, client_id).await
        }
        "ex:cancel" => {
            let Some(user_client_id) = app.executor_runtime.user_client_id() else {
                send::answer_callback_query(update.id, Some("执行器未登录"), client_id).await?;
                return Ok(());
            };
            if !app.executor_runtime.cancel_draining(user_client_id) {
                send::answer_callback_query(update.id, Some("当前没有可取消的退出操作"), client_id)
                    .await?;
                return Ok(());
            }
            app.transfer_runtime.cancel_transfer_drain();
            send::answer_callback_query(update.id, Some("已继续接收新任务"), client_id).await?;
            edit_executor_panel(app, update.chat_id, update.message_id, client_id).await
        }
        _ => {
            send::answer_callback_query(update.id, Some("执行器按钮参数无效"), client_id).await?;
            Ok(())
        }
    }
}

/// 启动后台异步任务：在所有转存任务排空（Drain）完成后执行执行器登出。
///
/// # 参数
/// * `transfer_runtime` - 转存运行时状态，用于等待任务排空
/// * `executor_runtime` - 执行器运行时状态，用于推进登出阶段
/// * `user_client_id` - 待退出的 User 客户端标识
fn spawn_executor_logout_after_drain(
    transfer_runtime: Arc<crate::app_context::TransferRuntimeState>,
    executor_runtime: Arc<ExecutorRuntimeState>,
    user_client_id: i32,
) {
    tokio::spawn(async move {
        // 等待所有正在执行的转存任务全部收尾完毕
        transfer_runtime.wait_for_transfer_drain().await;
        if !executor_runtime.mark_logging_out(user_client_id) {
            return;
        }
        // 调用 TDLib logOut 销毁当前会话
        if let Err(error) = tdlib_rs::functions::log_out(user_client_id).await {
            tracing::error!(
                user_client_id,
                error_code = error.code,
                error_message = %error.message,
                "executor logout failed after transfer drain"
            );
            // 若注销失败，恢复为在线状态并取消排空阻塞
            if executor_runtime.restore_ready_after_logout_failure(user_client_id) {
                transfer_runtime.cancel_transfer_drain();
            }
        }
    });
}

/// 动态创建并初始化 User 角色的 TDLib 客户端。
///
/// # 参数
/// * `app` - 全局应用上下文引用
/// * `config` - 机器人全局配置
///
/// # 返回
/// 成功返回新建客户端的 `client_id`
async fn create_user_client(
    app: Arc<crate::app_context::AppContext>,
    config: &BotConfig,
) -> anyhow::Result<i32> {
    let runtime = config.runtime_client(ClientRole::User)?.clone();
    let user_client = tdlib_rs::Client::new();
    let client_id = user_client.id();
    let log_level = runtime.log_verbosity_level;
    // 异步配置 TDLib 日志级别
    tokio::spawn(async move {
        crate::tgbot::set_log(client_id, log_level).await;
    });
    // 异步探测 TDLib 运行时版本
    tokio::spawn(async move {
        if let Err(error) = crate::tgbot::get_version(client_id).await {
            tracing::warn!(client_id, error = %error, "load executor tdlib version failed");
        }
    });
    let ready_roles = std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::BTreeSet::new()));
    let cfg_arc = Arc::new(config.clone());
    // 启动 Update 监听循环
    crate::tgbot::spawn_client_listener(
        user_client,
        ClientRole::User,
        app,
        cfg_arc,
        ready_roles,
    );
    tracing::info!(client_id, "executor user tdlib client created and listening");
    Ok(client_id)
}

/// 编辑并刷新 Telegram 中的执行器管理面板卡片消息。
///
/// # 参数
/// * `app` - 全局应用上下文引用
/// * `chat_id` - 目标会话标识
/// * `message_id` - 待编辑的消息 ID
/// * `client_id` - 响应使用的 Bot 客户端标识
async fn edit_executor_panel(
    app: &crate::app_context::AppContext,
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, rows) = build_executor_panel(app.executor_runtime.as_ref());
    let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        "执行器页面更新失败",
        "执行器状态已变更；请返回管理菜单重新打开。",
    )
    .await
}

/// 根据当前执行器运行时状态构建面板卡片文本与交互按钮矩阵。
///
/// # 参数
/// * `state` - 执行器运行时状态引用
///
/// # 返回
/// 返回包含面板富文本与内联键盘行列表的元组 `(String, Vec<Vec<InlineKeyboardButton>>)`
fn build_executor_panel(
    state: &ExecutorRuntimeState,
) -> (String, Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>) {
    let (status, detail, action) = match state.phase() {
        ExecutorPhase::Offline => (
            "未登录",
            "Bot 正常可用；需要回退时再登录执行器。",
            "登录执行器",
        ),
        ExecutorPhase::Starting => ("正在启动", "正在初始化执行器并申请二维码。", "刷新"),
        ExecutorPhase::WaitingQr => (
            "等待扫码",
            "二维码已发送到本私聊，请使用已登录设备扫描。",
            "刷新",
        ),
        ExecutorPhase::WaitingPassword => ("等待二次验证", "请回复密码输入提示完成登录。", "刷新"),
        ExecutorPhase::Ready => (
            "已登录",
            "Bot 默认执行；执行器仅在需要时自动回退使用。",
            "退出执行器",
        ),
        ExecutorPhase::Draining => (
            "等待任务结束",
            "已停止接收新任务；现有任务仍可暂停、恢复或停止。",
            "取消退出",
        ),
        ExecutorPhase::LoggingOut => ("正在退出", "等待 TDLib 清理本地执行器会话。", "刷新"),
    };
    let mut rows = Vec::new();
    match state.phase() {
        ExecutorPhase::Offline => rows.push(vec![send::build_callback_button(
            action,
            &build_executor_callback_data("login"),
            tdlib_rs::enums::ButtonStyle::Primary,
        )]),
        ExecutorPhase::Ready => rows.push(vec![send::build_callback_button(
            action,
            &build_executor_callback_data("logout"),
            tdlib_rs::enums::ButtonStyle::Danger,
        )]),
        ExecutorPhase::Draining => rows.push(vec![send::build_callback_button(
            action,
            &build_executor_callback_data("cancel"),
            tdlib_rs::enums::ButtonStyle::Default,
        )]),
        _ => rows.push(vec![send::build_callback_button(
            action,
            &build_executor_panel_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        )]),
    }
    // 底部添加返回主管理菜单按钮
    rows.push(vec![send::build_callback_button(
        "返回管理",
        "m:mh",
        tdlib_rs::enums::ButtonStyle::Default,
    )]);
    // 若已登录且有已识别的账号身份，拼接账号基本信息
    let identity = state.identity().map(|identity| {
        let username = identity
            .username
            .as_deref()
            .map(|username| format!("\n用户名：@{username}"))
            .unwrap_or_default();
        format!(
            "\n\n账号\nID：{}\n名称：{}{}",
            identity.user_id, identity.display_name, username
        )
    });
    (
        format!(
            "执行器\n\n状态：{status}\n说明：{detail}{}",
            identity.unwrap_or_default()
        ),
        rows,
    )
}

/// 将管理主菜单中的 Owner 专属入口转换为跳转至执行器面板的内联按钮。
pub(crate) fn build_executor_panel_button() -> tdlib_rs::types::InlineKeyboardButton {
    send::build_callback_button(
        "执行器",
        &build_executor_panel_callback_data(),
        tdlib_rs::enums::ButtonStyle::Primary,
    )
}

#[cfg(test)]
mod tests {
    use super::build_executor_panel;
    use crate::app_context::{ExecutorIdentity, ExecutorRuntimeState};

    /// 验证执行器登录完成后，管理面板仅展示非敏感的账号身份信息（ID、名称、用户名），绝不出现手机号。
    #[test]
    fn executor_panel_shows_non_sensitive_account_identity_after_login() {
        let state = ExecutorRuntimeState::default();
        state.begin_login(71, 1001);
        assert!(state.mark_ready(71));
        assert!(state.set_identity_if_ready(
            71,
            ExecutorIdentity {
                user_id: 2002,
                display_name: "测试账号".to_owned(),
                username: Some("tester".to_owned()),
            },
        ));

        let (text, _) = build_executor_panel(&state);

        assert!(text.contains("状态：已登录"));
        assert!(text.contains("ID：2002"));
        assert!(text.contains("名称：测试账号"));
        assert!(text.contains("用户名：@tester"));
        assert!(!text.contains("手机号"));
    }
}

