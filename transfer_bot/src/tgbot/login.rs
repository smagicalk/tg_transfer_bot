// 登录状态处理模块。
// 负责根据 TDLib 返回的 AuthorizationState 推进登录流程。
use crate::config::LoginInfo;
use crate::tgbot::TdError;
use std::process::exit;
use tdlib_rs::enums::AuthorizationState;

// 根据授权状态执行下一步动作。
pub async fn handle_authorization(
    authorization_state: tdlib_rs::enums::AuthorizationState,
    config: std::sync::Arc<crate::config::BotConfig>,
) -> anyhow::Result<()> {
    match config.client_id {
        None => {
            tracing::error!("tdlib authorization received before client_id initialized");
            anyhow::bail!("Client ID not set");
        }
        Some(client_id) => {
            tracing::debug!(
                client_id,
                auth_state = authorization_state_kind(&authorization_state),
                "tdlib authorization state received"
            );

            match authorization_state {
                // 初始化 TDLib 参数。
                AuthorizationState::WaitTdlibParameters => {
                    tracing::info!(client_id, "setting tdlib parameters");
                    tokio::fs::create_dir_all(&config.tdlib_config.files_directory).await?;
                    tokio::fs::create_dir_all(&config.tdlib_config.database_directory).await?;
                    tracing::debug!(
                        client_id,
                        use_test_dc = config.tdlib_config.use_test_dc,
                        use_file_database = config.tdlib_config.use_file_database,
                        use_chat_info_database = config.tdlib_config.use_chat_info_database,
                        use_message_database = config.tdlib_config.use_message_database,
                        use_secret_chats = config.tdlib_config.use_secret_chats,
                        "tdlib local directories prepared"
                    );

                    tdlib_rs::functions::set_tdlib_parameters(
                        config.tdlib_config.use_test_dc,
                        config.tdlib_config.database_directory.clone(),
                        config.tdlib_config.files_directory.clone(),
                        config.tdlib_config.database_encryption_key.clone(),
                        config.tdlib_config.use_file_database,
                        config.tdlib_config.use_chat_info_database,
                        config.tdlib_config.use_message_database,
                        config.tdlib_config.use_secret_chats,
                        config.tdlib_config.api_id,
                        config.tdlib_config.api_hash.clone(),
                        config.tdlib_config.system_language_code.clone(),
                        config.tdlib_config.device_model.clone(),
                        config.tdlib_config.system_version.clone(),
                        config.tdlib_config.application_version.clone(),
                        client_id,
                    )
                    .await
                    .map_err(|e| anyhow::Error::new(TdError(e)))
                }

                // 进入手机 / Token / OCR 登录分支。
                AuthorizationState::WaitPhoneNumber => match &config.login_info {
                    LoginInfo::Phone(phone) => {
                        tracing::info!(client_id, "submitting phone login request");
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
                        tracing::info!(client_id, "submitting bot token login request");
                        tdlib_rs::functions::check_authentication_bot_token(
                            token.clone(),
                            client_id,
                        )
                        .await
                        .map_err(|e| anyhow::Error::new(TdError(e)))
                    }
                    LoginInfo::Ocr => {
                        tracing::info!(client_id, "requesting qr login");
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
                    let code = qrcode::QrCode::with_error_correction_level(
                        link.as_bytes(),
                        qrcode::EcLevel::Q,
                    )?;
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
                    let password = inquire::Password::new(
                        authorization_state_wait_password.password_hint.as_str(),
                    )
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
                    let login_mode = match &config.login_info {
                        LoginInfo::Phone(_) => "phone",
                        LoginInfo::Token(_) => "token",
                        LoginInfo::Ocr => "ocr",
                    };
                    // 登录凭证属于敏感信息，日志只记录登录方式，不记录手机号或 token。
                    tracing::info!(client_id, login_mode, "tdlib authorization ready");
                    // 登录完成后启动转存后台任务：
                    // 1) 恢复未完成任务
                    // 2) 启动文件延迟删除队列
                    crate::tgbot::transfer::on_client_ready(client_id);
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
