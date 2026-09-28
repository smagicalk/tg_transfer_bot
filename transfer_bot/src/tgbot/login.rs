//! TDLib 客户端授权状态机与登录流转模块。
//!
//! # 核心职责
//! 1. **状态机流转（AuthorizationState）**：接收并响应 TDLib 抛出的各阶段授权状态更新（参数配置、手机号/Token 提交、扫码确认、二次密码验证、登录就绪、登出关闭）。
//! 2. **多角色登录隔离**：区分 Bot 与 User 角色：
//!    - Bot 角色通过 Token 自动完成登录，并在首次 Ready 时注册 Telegram 原生斜杠命令菜单。
//!    - User 角色（执行器）通过二维码与二次密码以交互方式登录，不阻塞进程主启动流。
//! 3. **安全凭据加密与兼容**：
//!    - TDLib 数据库加密密钥（`database_encryption_key`）以 Base64 进行 JSON 通信编码。
//!    - 自动兼容旧版直接传入 Base64 字符串的遗留测试库。

use crate::config::{ClientRole, LoginInfo};
use crate::tgbot::TdError;
use base64::{Engine as _, engine::general_purpose};
use once_cell::sync::Lazy;
use std::collections::BTreeSet;
use std::process::exit;
use tdlib_rs::enums::AuthorizationState;
use tokio::sync::Mutex;

/// 已经成功注册过斜杠命令的 Bot 客户端集合。
///
/// TDLib 可能在会话恢复或网络重连时多次上报 `AuthorizationState::Ready`；
/// 这里按 `client_id` 去重记录，避免每次重连时重复调用 `setCommands` 请求。
static REGISTERED_BOT_COMMAND_CLIENTS: Lazy<Mutex<BTreeSet<i32>>> =
    Lazy::new(|| Mutex::new(BTreeSet::new()));

/// 根据 TDLib 返回的 `AuthorizationState` 状态推进登录流程。
///
/// # 参数
/// * `app_context` - 全局应用上下文引用
/// * `authorization_state` - TDLib 授权状态枚举
/// * `role` - 当前客户端角色（`Bot` 或 `User`）
/// * `client_id` - 当前客户端标识
/// * `config` - 机器人全局配置
/// * `ready_roles` - 已就绪客户端角色集合互斥锁
///
/// # 返回
/// 推进成功返回 `Ok(())`，遇到未实现状态或异常报错返回 Err
pub async fn handle_authorization(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    authorization_state: tdlib_rs::enums::AuthorizationState,
    role: ClientRole,
    client_id: i32,
    config: std::sync::Arc<crate::config::BotConfig>,
    ready_roles: std::sync::Arc<tokio::sync::Mutex<BTreeSet<ClientRole>>>,
) -> anyhow::Result<()> {
    let runtime_client = config.runtime_client(role)?.clone();
    let login_info = runtime_client.login_info;
    let tdlib_config = runtime_client.tdlib_config;

    tracing::debug!(
        client_id,
        role = role.as_str(),
        auth_state = authorization_state_kind(&authorization_state),
        "tdlib authorization state received"
    );

    match authorization_state {
        // 第一阶段：初始化 TDLib 运行参数（本地目录、API 凭证、加密密钥等）
        AuthorizationState::WaitTdlibParameters => {
            tracing::info!(client_id, role = role.as_str(), "setting tdlib parameters");
            tokio::fs::create_dir_all(&tdlib_config.files_directory).await?;
            tokio::fs::create_dir_all(&tdlib_config.database_directory).await?;
            tracing::debug!(
                client_id,
                role = role.as_str(),
                use_test_dc = tdlib_config.use_test_dc,
                use_file_database = tdlib_config.use_file_database,
                use_chat_info_database = tdlib_config.use_chat_info_database,
                use_message_database = tdlib_config.use_message_database,
                use_secret_chats = tdlib_config.use_secret_chats,
                "tdlib local directories prepared"
            );

            set_tdlib_parameters_with_key_compat(&tdlib_config, role, client_id).await
        }

        // 第二阶段：进入手机号 / Bot Token / 扫码登录分支
        AuthorizationState::WaitPhoneNumber => {
            // 用户执行器只能由 Owner 在 Bot 私聊面板中显式发起。
            // 严禁在进程启动或 TDLib 意外重连时自动弹出终端二维码，避免阻塞主流程。
            if role == ClientRole::User {
                if app_context
                    .executor_runtime
                    .request_qr_if_starting(client_id)
                {
                    tracing::info!(client_id, "requesting executor qr login");
                    return tdlib_rs::functions::request_qr_code_authentication(vec![], client_id)
                        .await
                        .map_err(|error| anyhow::Error::new(TdError(error)));
                }
                tracing::debug!(
                    client_id,
                    phase = ?app_context.executor_runtime.phase(),
                    "executor user client is waiting for an owner login request"
                );
                return Ok(());
            }

            match &login_info {
                LoginInfo::Phone(phone) => {
                    tracing::info!(
                        client_id,
                        role = role.as_str(),
                        "submitting phone login request"
                    );
                    let phone_number_authentication_settings =
                        tdlib_rs::types::PhoneNumberAuthenticationSettings {
                            allow_flash_call: false,
                            allow_missed_call: false,
                            is_current_phone_number: true,
                            has_unknown_phone_number: false,
                            allow_sms_retriever_api: false,
                            firebase_authentication_settings: None,
                            authentication_tokens: vec![],
                        };

                    tdlib_rs::functions::set_authentication_phone_number(
                        phone.clone(),
                        Some(phone_number_authentication_settings),
                        client_id,
                    )
                    .await
                    .map_err(|e| anyhow::Error::new(TdError(e)))
                }
                LoginInfo::Token(token) => {
                    tracing::info!(
                        client_id,
                        role = role.as_str(),
                        "submitting bot token login request"
                    );
                    tdlib_rs::functions::check_authentication_bot_token(token.clone(), client_id)
                        .await
                        .map_err(|e| anyhow::Error::new(TdError(e)))
                }
                LoginInfo::Ocr => {
                    tracing::info!(client_id, role = role.as_str(), "requesting qr login");
                    tdlib_rs::functions::request_qr_code_authentication(vec![], client_id)
                        .await
                        .map_err(|e| anyhow::Error::new(TdError(e)))
                }
            }
        }

        // 暂未实现的认证状态：返回显式受控错误，避免 todo! 触发程序崩溃
        AuthorizationState::WaitPremiumPurchase(_) => {
            tracing::warn!(client_id, "tdlib authorization waits for premium purchase");
            anyhow::bail!("WaitPremiumPurchase 未实现")
        }
        AuthorizationState::WaitEmailAddress(_) => {
            tracing::warn!(client_id, "tdlib authorization waits for email address");
            anyhow::bail!("WaitEmailAddress 未实现")
        }
        AuthorizationState::WaitEmailCode(_) => {
            tracing::warn!(client_id, "tdlib authorization waits for email code");
            anyhow::bail!("WaitEmailCode 未实现")
        }

        // 第三阶段：输入手机验证码
        AuthorizationState::WaitCode(authorization_state_wait_code) => {
            let phone_number = authorization_state_wait_code.code_info.phone_number.clone();
            tracing::info!(client_id, "waiting for phone login code");
            let code_result =
                inquire::Text::new(format!("请输入 {phone_number} 的验证码").as_str())
                    .with_placeholder("验证码")
                    .with_help_message(
                        format!("请输入 {phone_number} 在其他设备收到的验证码").as_str(),
                    )
                    .with_validator(inquire::validator::MinLengthValidator::new(5))
                    .prompt()
                    .map_err(anyhow::Error::new)?;
            tdlib_rs::functions::check_authentication_code(code_result, client_id)
                .await
                .map_err(|e| anyhow::Error::new(TdError(e)))
        }

        // 第四阶段：等待其他设备扫码确认
        AuthorizationState::WaitOtherDeviceConfirmation(
            authorization_state_wait_other_device_confirmation,
        ) => {
            tracing::info!(client_id, "qr login confirmation requested");
            let link = authorization_state_wait_other_device_confirmation.link;
            // 若为 User 客户端，向 Owner 私聊发送二维码图片卡片
            if role == ClientRole::User {
                return crate::tgbot::executor::send_qr_code_to_owner(
                    app_context.as_ref(),
                    client_id,
                    link,
                    config.interaction_client_id()?,
                )
                .await;
            }
            // 终端环境渲染 ANSI 二维码
            let code =
                qrcode::QrCode::with_error_correction_level(link.as_bytes(), qrcode::EcLevel::Q)?;
            let qr = code
                .render::<qrcode::render::unicode::Dense1x2>()
                .quiet_zone(true)
                .build();

            println!("请使用 Telegram 扫描下面的登录二维码：");
            println!("{qr}");
            println!("如果二维码无法识别，可在可信环境打开临时链接：{link}");
            Ok(())
        }

        AuthorizationState::WaitRegistration(_) => {
            tracing::warn!(
                client_id,
                "tdlib authorization waits for account registration"
            );
            anyhow::bail!("WaitRegistration 未实现")
        }

        // 第五阶段：输入二次验证（两步验证）密码
        AuthorizationState::WaitPassword(authorization_state_wait_password) => {
            tracing::info!(client_id, "waiting for two-factor password");
            // 若为 User 客户端，通过 Bot 私聊使用 ForceReply 交互输入密码
            if role == ClientRole::User {
                return crate::tgbot::executor::request_two_factor_password(
                    app_context.as_ref(),
                    client_id,
                    authorization_state_wait_password.password_hint.as_str(),
                    config.interaction_client_id()?,
                )
                .await;
            }
            // 终端环境掩码输入密码
            let password =
                inquire::Password::new(authorization_state_wait_password.password_hint.as_str())
                    .with_help_message("请输入密码")
                    .with_display_mode(inquire::PasswordDisplayMode::Masked)
                    .prompt()
                    .map_err(anyhow::Error::new)?;
            tdlib_rs::functions::check_authentication_password(password, client_id)
                .await
                .map_err(|e| anyhow::Error::new(TdError(e)))
        }

        // 第六阶段：登录完成，会话就绪
        AuthorizationState::Ready => {
            let login_mode = match &login_info {
                LoginInfo::Phone(_) => "phone",
                LoginInfo::Token(_) => "token",
                LoginInfo::Ocr => "ocr",
            };
            // 登录凭证属于敏感信息，日志只记录登录方式，不记录手机号或 token
            tracing::info!(
                client_id,
                role = role.as_str(),
                login_mode,
                "tdlib authorization ready"
            );
            // Bot 首次就绪时注册斜杠命令菜单
            if role == ClientRole::Bot {
                register_bot_commands_once(client_id).await;
            }
            // User 客户端就绪时更新运行时身份快照并清理临时二维码图片与密码提示
            if role == ClientRole::User && app_context.executor_runtime.mark_ready(client_id) {
                if let Err(error) = crate::tgbot::executor::refresh_executor_identity(
                    app_context.as_ref(),
                    client_id,
                )
                .await
                {
                    // 账号摘要仅用于面板展示；读取失败不能阻止已完成的执行器登录
                    tracing::warn!(client_id, error = %error, "load executor account identity failed");
                }
                if let Some(path) = app_context.executor_runtime.take_qr_image_path() {
                    let _ = std::fs::remove_file(path);
                }
                if let Some(prompt_id) = app_context
                    .executor_runtime
                    .take_password_prompt_message_id()
                {
                    let _ = crate::tgbot::send::delete_message(
                        config.owner_user_id,
                        prompt_id,
                        config.interaction_client_id()?,
                    )
                    .await;
                }
            }
            let mut ready_roles = ready_roles.lock().await;
            ready_roles.insert(role);
            // 当所有必需客户端均就绪时，触发转存运行时就绪回调并初始化后台工作流
            if config.all_required_clients_ready(&ready_roles) {
                let mut transfer_clients =
                    config.transfer_client_ids_for_ready_roles(&ready_roles)?;
                transfer_clients.user = app_context.executor_runtime.user_client_id();
                drop(ready_roles);
                crate::tgbot::transfer::on_clients_ready(app_context, transfer_clients);
            }
            Ok(())
        }

        // 第七阶段：正在注销
        // Bot 生命周期终止才退出进程；用户执行器可以独立退出或重新登录
        AuthorizationState::LoggingOut => {
            tracing::info!(client_id, "tdlib logging out");
            if role == ClientRole::User {
                app_context.executor_runtime.mark_logging_out(client_id);
                return Ok(());
            }
            exit(0)
        }
        // 第八阶段：正在关闭
        AuthorizationState::Closing => {
            tracing::info!(client_id, "tdlib closing");
            if role == ClientRole::User {
                return Ok(());
            }
            exit(0)
        }
        // 第九阶段：已完全关闭
        AuthorizationState::Closed => {
            tracing::info!(client_id, "tdlib closed");
            if role == ClientRole::User {
                ready_roles.lock().await.remove(&ClientRole::User);
                if app_context.executor_runtime.clear_user_client_if(client_id) {
                    if let Some(path) = app_context.executor_runtime.take_qr_image_path() {
                        let _ = std::fs::remove_file(path);
                    }
                    if let Some(prompt_id) = app_context
                        .executor_runtime
                        .take_password_prompt_message_id()
                    {
                        let _ = crate::tgbot::send::delete_message(
                            config.owner_user_id,
                            prompt_id,
                            config.interaction_client_id()?,
                        )
                        .await;
                    }
                    if let Some(mut transfer_clients) =
                        app_context.transfer_runtime.transfer_client_ids()
                    {
                        transfer_clients.user = None;
                        app_context
                            .transfer_runtime
                            .set_transfer_client_ids(transfer_clients);
                    }
                }
                return Ok(());
            }
            exit(0)
        }
    }
}

/// 为 Bot 客户端向 Telegram 注册斜杠命令菜单。
///
/// 注册失败不阻塞机器人主流程：斜杠命令菜单属于原生客户端输入提示的体验增强，
/// 所有的转存与管理命令仍可通过输入文本正常执行。
///
/// # 参数
/// * `client_id` - Bot TDLib 客户端标识
async fn register_bot_commands_once(client_id: i32) {
    {
        let mut registered = REGISTERED_BOT_COMMAND_CLIENTS.lock().await;
        // 幂等去重：若当前 client_id 已注册过则直接跳过
        if !registered.insert(client_id) {
            tracing::trace!(client_id, "bot commands already registered for client");
            return;
        }
    }

    let commands = bot_command_definitions();
    let command_count = commands.len();
    tracing::info!(client_id, command_count, "registering bot commands");
    if let Err(err) = tdlib_rs::functions::set_commands(None, String::new(), commands, client_id)
        .await
        .map_err(|e| anyhow::Error::new(TdError(e)))
    {
        // 若注册失败，移除标记以允许重试
        REGISTERED_BOT_COMMAND_CLIENTS
            .lock()
            .await
            .remove(&client_id);
        tracing::warn!(
            client_id,
            error = %err,
            "register bot commands failed"
        );
        return;
    }
    tracing::info!(client_id, command_count, "bot commands registered");
}

/// 构造 Bot 支持的全部官方斜杠命令列表定义。
///
/// # 返回
/// 包含所有命令标识与简要中文说明的 `BotCommand` 列表
fn bot_command_definitions() -> Vec<tdlib_rs::types::BotCommand> {
    vec![
        bot_command("menu", "打开交互菜单"),
        bot_command("transfer", "转存消息或相册"),
        bot_command("lookup", "查询历史转存结果"),
        bot_command("downloads", "查看转存任务列表"),
        bot_command("job", "查看或控制指定任务"),
        bot_command("targets", "管理默认目标和别名"),
        bot_command("config", "查看或调整运行配置"),
        bot_command("health", "查看运行状态"),
        bot_command("cache", "查看文件缓存"),
        bot_command("auth", "管理授权用户（仅 owner）"),
        bot_command("help", "查看命令帮助"),
    ]
}

/// 辅助函数：构造单条 `BotCommand` 对象。
///
/// # 参数
/// * `command` - 命令字符串（不带斜杠 `/`）
/// * `description` - 命令功能简要描述
fn bot_command(command: &str, description: &str) -> tdlib_rs::types::BotCommand {
    tdlib_rs::types::BotCommand {
        command: command.to_owned(),
        description: description.to_owned(),
        is_ephemeral: false,
    }
}

/// 返回授权状态的字符串简述，供日志统一记录。
///
/// 避免直接打印完整的 `AuthorizationState` 导致把二维码链接或手机号打入日志。
///
/// # 参数
/// * `state` - 授权状态枚举引用
fn authorization_state_kind(state: &AuthorizationState) -> &'static str {
    match state {
        AuthorizationState::WaitTdlibParameters => "wait_tdlib_parameters",
        AuthorizationState::WaitPhoneNumber => "wait_phone_number",
        AuthorizationState::WaitPremiumPurchase(_) => "wait_premium_purchase",
        AuthorizationState::WaitEmailAddress(_) => "wait_email_address",
        AuthorizationState::WaitEmailCode(_) => "wait_email_code",
        AuthorizationState::WaitCode(_) => "wait_code",
        AuthorizationState::WaitOtherDeviceConfirmation(_) => "wait_other_device_confirmation",
        AuthorizationState::WaitRegistration(_) => "wait_registration",
        AuthorizationState::WaitPassword(_) => "wait_password",
        AuthorizationState::Ready => "ready",
        AuthorizationState::LoggingOut => "logging_out",
        AuthorizationState::Closing => "closing",
        AuthorizationState::Closed => "closed",
    }
}

/// 将数据库加密密钥编码为 TDLib JSON 协议所要求的 Base64 字符串。
///
/// TDLib JSON 协议中的 bytes 字段要求以 Base64 传输；
/// 配置文件中用户输入普通明文密码，此处在进入 TDLib 前统一编码，避免用户手动计算 Base64。
///
/// # 参数
/// * `key` - 明文密钥字符串
fn tdlib_database_encryption_key_for_json(key: &str) -> String {
    general_purpose::STANDARD.encode(key.as_bytes())
}

/// 设置 TDLib 参数，并对早期以 Base64 字符串直接初始化的数据库进行向后兼容重试。
///
/// # 参数
/// * `tdlib_config` - TDLib 客户端运行配置
/// * `role` - 客户端角色
/// * `client_id` - 客户端标识
async fn set_tdlib_parameters_with_key_compat(
    tdlib_config: &crate::config::TdlibConfig,
    role: ClientRole,
    client_id: i32,
) -> anyhow::Result<()> {
    let encoded_key = tdlib_database_encryption_key_for_json(&tdlib_config.database_encryption_key);
    match set_tdlib_parameters_with_key(tdlib_config, encoded_key, client_id).await {
        Ok(()) => Ok(()),
        Err(err)
            if should_retry_legacy_database_key(&tdlib_config.database_encryption_key, &err) =>
        {
            tracing::warn!(
                client_id,
                role = role.as_str(),
                "tdlib database key matched legacy base64 mode, retrying compatibility path"
            );
            // 兼容路径：若报错密钥错误且原值本身即为 Base64，直接以原配置值重试一次
            set_tdlib_parameters_with_key(
                tdlib_config,
                tdlib_config.database_encryption_key.clone(),
                client_id,
            )
            .await
            .map_err(|err| anyhow::Error::new(TdError(err)))
        }
        Err(err) => Err(anyhow::Error::new(TdError(err))),
    }
}

/// 将参数组装并调用底层 TDLib `setTdlibParameters` 接口。
///
/// # 参数
/// * `tdlib_config` - TDLib 配置
/// * `database_encryption_key_json` - JSON 传输用的加密密钥字符串
/// * `client_id` - 客户端标识
async fn set_tdlib_parameters_with_key(
    tdlib_config: &crate::config::TdlibConfig,
    database_encryption_key_json: String,
    client_id: i32,
) -> Result<(), tdlib_rs::types::Error> {
    tdlib_rs::functions::set_tdlib_parameters(
        tdlib_config.use_test_dc,
        tdlib_config.database_directory.clone(),
        tdlib_config.files_directory.clone(),
        database_encryption_key_json,
        tdlib_config.use_file_database,
        tdlib_config.use_chat_info_database,
        tdlib_config.use_message_database,
        tdlib_config.use_secret_chats,
        tdlib_config.api_id,
        tdlib_config.api_hash.clone(),
        tdlib_config.system_language_code.clone(),
        tdlib_config.device_model.clone(),
        tdlib_config.system_version.clone(),
        tdlib_config.application_version.clone(),
        client_id,
    )
    .await
}

/// 判断是否需要按照旧版“原配置值已经是 Base64”的语义进行二次重试。
///
/// 只有当：密钥非空、错误码为 401（Wrong database encryption key）、
/// 且配置字符串本身能够成功进行 Base64 解码时，才触发兼容重试。
fn should_retry_legacy_database_key(key: &str, err: &tdlib_rs::types::Error) -> bool {
    !key.is_empty()
        && err.code == 401
        && err.message.contains("Wrong database encryption key")
        && tdlib_database_encryption_key_for_json(key) != key
        && general_purpose::STANDARD.decode(key).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{
        bot_command_definitions, should_retry_legacy_database_key,
        tdlib_database_encryption_key_for_json,
    };
    use std::collections::BTreeSet;

    /// 验证数据库加密密钥 Base64 编码逻辑。
    #[test]
    fn test_tdlib_database_encryption_key_for_json() {
        assert_eq!(tdlib_database_encryption_key_for_json(""), "");
        assert_eq!(
            tdlib_database_encryption_key_for_json("bot-key"),
            "Ym90LWtleQ=="
        );
    }

    /// 验证只有“库返回 401 密钥错误 且 原配置值符合 Base64 格式”时才命中重试分支。
    #[test]
    fn test_should_retry_legacy_database_key() {
        let wrong_key = tdlib_rs::types::Error {
            code: 401,
            message: "Wrong database encryption key".to_owned(),
        };
        let wrong_padding = tdlib_rs::types::Error {
            code: 400,
            message: "Wrong padding length".to_owned(),
        };

        assert!(should_retry_legacy_database_key("dXNlci1rZXk=", &wrong_key));
        assert!(!should_retry_legacy_database_key(
            "plain-user-key",
            &wrong_key
        ));
        assert!(!should_retry_legacy_database_key("", &wrong_key));
        assert!(!should_retry_legacy_database_key(
            "dXNlci1rZXk=",
            &wrong_padding
        ));
    }

    /// 验证注册的 Bot 命令列表包含所有支持的命令，且命令名称合法、描述非空。
    #[test]
    fn test_bot_command_definitions_expose_all_supported_commands() {
        let commands = bot_command_definitions();
        let names = commands
            .iter()
            .map(|command| command.command.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec![
                "menu",
                "transfer",
                "lookup",
                "downloads",
                "job",
                "targets",
                "config",
                "health",
                "cache",
                "auth",
                "help",
            ]
        );
        assert_eq!(
            names.iter().copied().collect::<BTreeSet<_>>().len(),
            names.len()
        );
        for command in commands {
            assert!(!command.command.starts_with('/'));
            assert!(
                command
                    .command
                    .chars()
                    .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
            );
            assert!(!command.description.trim().is_empty());
        }
    }
}
