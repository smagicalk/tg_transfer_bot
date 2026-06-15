// 登录状态处理模块。
// 负责根据 TDLib 返回的 AuthorizationState 推进登录流程。
use crate::config::{ClientRole, LoginInfo};
use crate::tgbot::TdError;
use base64::{Engine as _, engine::general_purpose};
use once_cell::sync::Lazy;
use std::collections::BTreeSet;
use std::process::exit;
use tdlib_rs::enums::AuthorizationState;
use tokio::sync::Mutex;

/// 已经注册过命令的 bot client。
///
/// TDLib 可能在恢复会话或重连时再次报告 `AuthorizationState::Ready`；
/// 这里按 client_id 去重，避免每次 Ready 都重复调用 `setCommands`。
static REGISTERED_BOT_COMMAND_CLIENTS: Lazy<Mutex<BTreeSet<i32>>> =
    Lazy::new(|| Mutex::new(BTreeSet::new()));

// 根据授权状态执行下一步动作。
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
        // 初始化 TDLib 参数。
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

        // 进入手机 / Token / OCR 登录分支。
        AuthorizationState::WaitPhoneNumber => match &login_info {
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
        },

        // 以下状态暂未实现：返回可控错误，避免 `todo!` 触发 panic。
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

        // 输入短信验证码。
        AuthorizationState::WaitCode(authorization_state_wait_code) => {
            let phone_number = authorization_state_wait_code.code_info.phone_number.clone();
            tracing::info!(client_id, "waiting for phone login code");
            let code_result =
                inquire::Text::new(format!("请输入 {} 的验证码", phone_number).as_str())
                    .with_placeholder("验证码")
                    .with_help_message(
                        format!("请输入 {} 在其他设备收到的验证码", phone_number).as_str(),
                    )
                    .with_validator(inquire::validator::MinLengthValidator::new(5))
                    .prompt()
                    .map_err(anyhow::Error::new)?;
            tdlib_rs::functions::check_authentication_code(code_result, client_id)
                .await
                .map_err(|e| anyhow::Error::new(TdError(e)))
        }

        // 输出扫码登录二维码。
        AuthorizationState::WaitOtherDeviceConfirmation(
            authorization_state_wait_other_device_confirmation,
        ) => {
            tracing::info!(client_id, "qr login confirmation requested");
            let link = authorization_state_wait_other_device_confirmation.link;
            let code =
                qrcode::QrCode::with_error_correction_level(link.as_bytes(), qrcode::EcLevel::Q)?;
            let qr = code
                .render::<qrcode::render::unicode::Dense1x2>()
                .quiet_zone(true)
                .build();

            println!("请使用 Telegram 扫描下面的登录二维码：");
            println!("{}", qr);
            println!("如果二维码无法识别，可在可信环境打开临时链接：{}", link);
            Ok(())
        }

        AuthorizationState::WaitRegistration(_) => {
            tracing::warn!(
                client_id,
                "tdlib authorization waits for account registration"
            );
            anyhow::bail!("WaitRegistration 未实现")
        }

        // 输入二次密码。
        AuthorizationState::WaitPassword(authorization_state_wait_password) => {
            tracing::info!(client_id, "waiting for two-factor password");
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

        // 登录完成。
        AuthorizationState::Ready => {
            let login_mode = match &login_info {
                LoginInfo::Phone(_) => "phone",
                LoginInfo::Token(_) => "token",
                LoginInfo::Ocr => "ocr",
            };
            // 登录凭证属于敏感信息，日志只记录登录方式，不记录手机号或 token。
            tracing::info!(
                client_id,
                role = role.as_str(),
                login_mode,
                "tdlib authorization ready"
            );
            if role == ClientRole::Bot {
                register_bot_commands_once(client_id).await;
            }
            let mut ready_roles = ready_roles.lock().await;
            ready_roles.insert(role);
            if config.all_required_clients_ready(&ready_roles) {
                let transfer_clients = config.transfer_client_ids()?;
                drop(ready_roles);
                crate::tgbot::transfer::on_clients_ready(app_context, transfer_clients);
            }
            Ok(())
        }

        // 生命周期终止状态：直接退出进程。
        AuthorizationState::LoggingOut => {
            tracing::info!(client_id, "tdlib logging out");
            exit(0)
        }
        AuthorizationState::Closing => {
            tracing::info!(client_id, "tdlib closing");
            exit(0)
        }
        AuthorizationState::Closed => {
            tracing::info!(client_id, "tdlib closed");
            exit(0)
        }
    }
}

/// 为 bot 注册 Telegram 斜杠命令。
///
/// 注册失败不阻塞主流程：命令菜单只是交互增强，转存命令本身仍可手动输入。
async fn register_bot_commands_once(client_id: i32) {
    {
        let mut registered = REGISTERED_BOT_COMMAND_CLIENTS.lock().await;
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

/// 构造 bot 命令列表。
///
/// Telegram 命令本身不能带 `/`，这里只注册长命令，避免菜单中出现重复短别名。
fn bot_command_definitions() -> Vec<tdlib_rs::types::BotCommand> {
    vec![
        bot_command("menu", "打开交互菜单"),
        bot_command("transfer", "转存链接或回复消息"),
        bot_command("lookup", "查询历史转存结果"),
        bot_command("downloads", "查看任务列表和下载进度"),
        bot_command("job", "查看或控制任务"),
        bot_command("balance", "查看积分余额"),
        bot_command("points", "管理员调整积分"),
        bot_command("config", "查看或调整运行配置"),
        bot_command("health", "查看运行健康状态"),
        bot_command("cache", "查看文件缓存"),
        bot_command("help", "查看命令帮助"),
    ]
}

/// 构造单条 bot command。
fn bot_command(command: &str, description: &str) -> tdlib_rs::types::BotCommand {
    tdlib_rs::types::BotCommand {
        command: command.to_owned(),
        description: description.to_owned(),
    }
}

/// 返回授权状态名，用于日志判断当前是否已经登录成功。
///
/// 不直接打印完整 AuthorizationState，避免把临时二维码链接、手机号等敏感信息写入日志。
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

/// TDLib JSON 协议里的 `database_encryption_key` 是 bytes 字段，必须用 base64 字符串传输。
///
/// 配置文件仍然按普通明文 key 填写；这里在进入 TDLib 前统一编码，避免用户手动处理
/// base64，也避免生成的 `tdlib_rs` wrapper 直接透传普通字符串导致 `Wrong padding length`。
fn tdlib_database_encryption_key_for_json(key: &str) -> String {
    general_purpose::STANDARD.encode(key.as_bytes())
}

/// 设置 TDLib 参数，并兼容旧库中已经按 base64 key 打开的数据库。
///
/// 正常路径：配置中的普通字符串会先编码成 TDLib JSON bytes 需要的 base64。
/// 兼容路径：如果现有数据库返回 401 `Wrong database encryption key`，且原配置值本身就是
/// 合法 base64，则再用原值直传一次，兼容早期版本直接把 base64 字符串传给 TDLib 的库。
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

/// 使用指定 JSON key 设置 TDLib 参数。
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

/// 判断是否需要按旧版“原值已经是 base64”语义重试。
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

    // TDLib JSON bytes 字段要求 base64；空 key 编码后仍是空字符串。
    #[test]
    fn test_tdlib_database_encryption_key_for_json() {
        assert_eq!(tdlib_database_encryption_key_for_json(""), "");
        assert_eq!(
            tdlib_database_encryption_key_for_json("bot-key"),
            "Ym90LWtleQ=="
        );
    }

    // 只有“现有库 key 错误 + 原配置像合法 base64”时才走旧库兼容重试。
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

    // 注册给 Telegram 的命令不能带 `/`，且短命令和长命令都需要保留。
    #[test]
    fn test_bot_command_definitions_cover_long_commands() {
        let commands = bot_command_definitions();
        let names = commands
            .iter()
            .map(|command| command.command.as_str())
            .collect::<BTreeSet<_>>();

        for expected in [
            "menu",
            "transfer",
            "lookup",
            "downloads",
            "job",
            "balance",
            "points",
            "config",
            "health",
            "cache",
            "help",
        ] {
            assert!(names.contains(expected), "missing command {expected}");
        }

        assert_eq!(names.len(), commands.len());
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
