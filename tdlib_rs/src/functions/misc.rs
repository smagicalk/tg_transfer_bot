//!
//! TDLib `misc` domain functions.
//!
//! Miscellaneous types, enums, network settings, and core TDLib options.
//!

#[allow(clippy::all)]
use serde_json::json;
use crate::send_request;

/// Returns the current authorization state. This is an offline method. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state. Can be called before initialization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AuthorizationState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_authorization_state(client_id: i32) -> Result<crate::enums::AuthorizationState, crate::types::Error> {
    let request = json!({
        "@type": "getAuthorizationState",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
///
/// # Arguments
///
/// * `use_test_dc` - Pass true to use Telegram test environment instead of the production environment
/// * `database_directory` - The path to the directory for the persistent database; if empty, the current working directory will be used
/// * `files_directory` - The path to the directory for storing files; if empty, database_directory will be used
/// * `database_encryption_key` - Encryption key for the database. If the encryption key is invalid, then an error with code 401 will be returned
/// * `use_file_database` - Pass true to keep information about downloaded and uploaded files between application restarts
/// * `use_chat_info_database` - Pass true to keep cache of users, basic groups, supergroups, channels and secret chats between restarts. Implies use_file_database
/// * `use_message_database` - Pass true to keep cache of chats and messages between restarts. Implies use_chat_info_database
/// * `use_secret_chats` - Pass true to enable support for secret chats
/// * `api_id` - Application identifier for Telegram API access, which can be obtained at https:my.telegram.org
/// * `api_hash` - Application identifier hash for Telegram API access, which can be obtained at https:my.telegram.org
/// * `system_language_code` - IETF language tag of the user's operating system language; must be non-empty
/// * `device_model` - Model of the device the application is being run on; must be non-empty
/// * `system_version` - Version of the operating system the application is being run on. If empty, the version is automatically detected by TDLib
/// * `application_version` - Application version; must be non-empty
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_tdlib_parameters(use_test_dc: bool, database_directory: String, files_directory: String, database_encryption_key: String, use_file_database: bool, use_chat_info_database: bool, use_message_database: bool, use_secret_chats: bool, api_id: i32, api_hash: String, system_language_code: String, device_model: String, system_version: String, application_version: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setTdlibParameters",
        "use_test_dc": use_test_dc,
        "database_directory": database_directory,
        "files_directory": files_directory,
        "database_encryption_key": database_encryption_key,
        "use_file_database": use_file_database,
        "use_chat_info_database": use_chat_info_database,
        "use_message_database": use_message_database,
        "use_secret_chats": use_secret_chats,
        "api_id": api_id,
        "api_hash": api_hash,
        "system_language_code": system_language_code,
        "device_model": device_model,
        "system_version": system_version,
        "application_version": application_version,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitPhoneNumber,
/// or if there is no pending authentication query and the current authorization state is authorizationStateWaitPremiumPurchase, authorizationStateWaitEmailAddress,
/// authorizationStateWaitEmailCode, authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
///
/// # Arguments
///
/// * `phone_number` - The phone number of the user, in international format
/// * `settings` - Settings for the authentication of the user's phone number; pass null to use default settings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_authentication_phone_number(phone_number: String, settings: Option<crate::types::PhoneNumberAuthenticationSettings>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAuthenticationPhoneNumber",
        "phone_number": phone_number,
        "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets the email address of the user and sends an authentication code to the email address. Works only when the current authorization state is authorizationStateWaitEmailAddress
///
/// # Arguments
///
/// * `email_address` - The email address of the user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_authentication_email_address(email_address: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAuthenticationEmailAddress",
        "email_address": email_address,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Resends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode, the next_code_type of the result is not null
/// and the server-specified timeout has passed, or when the current authorization state is authorizationStateWaitEmailCode
///
/// # Arguments
///
/// * `reason` - Reason of code resending; pass null if unknown
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn resend_authentication_code(reason: Option<crate::enums::ResendCodeReason>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "resendAuthenticationCode",
        "reason": reason,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks the authentication of an email address. Works only when the current authorization state is authorizationStateWaitEmailCode
///
/// # Arguments
///
/// * `code` - Email address authentication to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_email_code(code: crate::enums::EmailAddressAuthentication, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkAuthenticationEmailCode",
        "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode
///
/// # Arguments
///
/// * `code` - Authentication code to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_code(code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkAuthenticationCode",
        "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Requests QR code authentication by scanning a QR code on another logged in device. Works only when the current authorization state is authorizationStateWaitPhoneNumber,
/// or if there is no pending authentication query and the current authorization state is authorizationStateWaitPremiumPurchase, authorizationStateWaitEmailAddress,
/// authorizationStateWaitEmailCode, authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
///
/// # Arguments
///
/// * `other_user_ids` - List of user identifiers of other users currently using the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn request_qr_code_authentication(other_user_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "requestQrCodeAuthentication",
        "other_user_ids": other_user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns parameters for authentication using a passkey as JSON-serialized string
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_authentication_passkey_parameters(client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getAuthenticationPasskeyParameters",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks a passkey to log in to the corresponding account. Call getAuthenticationPasskeyParameters to get parameters for the passkey. Works only when the current authorization state is
/// authorizationStateWaitPhoneNumber or authorizationStateWaitOtherDeviceConfirmation, or if there is no pending authentication query and the current authorization state is
/// authorizationStateWaitPremiumPurchase, authorizationStateWaitEmailAddress, authorizationStateWaitEmailCode, authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
///
/// # Arguments
///
/// * `credential_id` - Base64url-encoded identifier of the credential
/// * `client_data` - JSON-encoded client data
/// * `authenticator_data` - Authenticator data of the application that created the credential
/// * `signature` - Cryptographic signature of the credential
/// * `user_handle` - User handle of the passkey
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_passkey(credential_id: String, client_data: String, authenticator_data: String, signature: String, user_handle: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkAuthenticationPasskey",
        "credential_id": credential_id,
        "client_data": client_data,
        "authenticator_data": authenticator_data,
        "signature": signature,
        "user_handle": user_handle,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks a web token to log in to the corresponding account; for official Telegram apps only. Works only when the current authorization state is
/// authorizationStateWaitPhoneNumber or authorizationStateWaitOtherDeviceConfirmation
///
/// # Arguments
///
/// * `token` - The token to check
/// * `dc_id` - Identifier of the datacenter of the user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_web_token(token: String, dc_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkAuthenticationWebToken",
        "token": token,
        "dc_id": dc_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Resets the login email address. May return an error with a message "TASK_ALREADY_EXISTS" if reset is still pending.
/// Works only when the current authorization state is authorizationStateWaitEmailCode and authorization_state.can_reset_email_address == true
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reset_authentication_email_address(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "resetAuthenticationEmailAddress",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks the 2-step verification password for correctness. Works only when the current authorization state is authorizationStateWaitPassword
///
/// # Arguments
///
/// * `password` - The 2-step verification password to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_password(password: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkAuthenticationPassword",
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Requests to send a 2-step verification password recovery code to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn request_authentication_password_recovery(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "requestAuthenticationPasswordRecovery",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks whether a 2-step verification password recovery code sent to an email address is valid. Works only when the current authorization state is authorizationStateWaitPassword
///
/// # Arguments
///
/// * `recovery_code` - Recovery code to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_password_recovery_code(recovery_code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkAuthenticationPasswordRecoveryCode",
        "recovery_code": recovery_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Recovers the 2-step verification password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
///
/// # Arguments
///
/// * `recovery_code` - Recovery code to check
/// * `new_password` - New 2-step verification password of the user; may be empty to remove the password
/// * `new_hint` - New password hint; may be empty
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn recover_authentication_password(recovery_code: String, new_password: String, new_hint: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "recoverAuthenticationPassword",
        "recovery_code": recovery_code,
        "new_password": new_password,
        "new_hint": new_hint,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends Firebase Authentication SMS to the phone number of the user. Works only when the current authorization state is authorizationStateWaitCode and the server returned code of the type authenticationCodeTypeFirebaseAndroid or authenticationCodeTypeFirebaseIos
///
/// # Arguments
///
/// * `token` - Play Integrity API or SafetyNet Attestation API token for the Android application, or secret from push notification for the iOS application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_authentication_firebase_sms(token: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendAuthenticationFirebaseSms",
        "token": token,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reports that authentication code wasn't delivered via SMS; for official mobile applications only. Works only when the current authorization state is authorizationStateWaitCode
///
/// # Arguments
///
/// * `mobile_network_code` - Current mobile network code
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_authentication_code_missing(mobile_network_code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportAuthenticationCodeMissing",
        "mobile_network_code": mobile_network_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, updateAuthorizationState with authorizationStateClosed will be sent
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn log_out(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "logOut",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes, updateAuthorizationState with authorizationStateClosed will be sent. Can be called before initialization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn close(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "close",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed.
/// After the destruction completes updateAuthorizationState with authorizationStateClosed will be sent. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn destroy(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "destroy",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Confirms QR code authentication on another device. Returns created session on success
///
/// # Arguments
///
/// * `link` - A link from a QR code. The link must be scanned by the in-app camera
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Session)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn confirm_qr_code_authentication(link: String, client_id: i32) -> Result<crate::enums::Session, crate::types::Error> {
    let request = json!({
        "@type": "confirmQrCodeAuthentication",
        "link": link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns all updates needed to restore current TDLib state, i.e. all actual updateAuthorizationState/updateUser/updateNewChat and others. This is especially useful if TDLib is run in a separate process. Can be called before initialization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Updates)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_current_state(client_id: i32) -> Result<crate::enums::Updates, crate::types::Error> {
    let request = json!({
        "@type": "getCurrentState",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain
///
/// # Arguments
///
/// * `new_encryption_key` - New encryption key
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_database_encryption_key(new_encryption_key: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setDatabaseEncryptionKey",
        "new_encryption_key": new_encryption_key,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the current state of 2-step verification
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PasswordState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_password_state(client_id: i32) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
        "@type": "getPasswordState",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the 2-step verification password for the current user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed
///
/// # Arguments
///
/// * `old_password` - Previous 2-step verification password of the user
/// * `new_password` - New 2-step verification password of the user; may be empty to remove the password
/// * `new_hint` - New password hint; may be empty
/// * `set_recovery_email_address` - Pass true to change also the recovery email address
/// * `new_recovery_email_address` - New recovery email address; may be empty
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PasswordState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_password(old_password: String, new_password: String, new_hint: String, set_recovery_email_address: bool, new_recovery_email_address: String, client_id: i32) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
        "@type": "setPassword",
        "old_password": old_password,
        "new_password": new_password,
        "new_hint": new_hint,
        "set_recovery_email_address": set_recovery_email_address,
        "new_recovery_email_address": new_recovery_email_address,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether the current user is required to set login email address
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn is_login_email_address_required(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "isLoginEmailAddressRequired",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the login email address of the user. The email address can be changed only if the current user already has login email and passwordState.login_email_address_pattern is non-empty,
/// or the user received suggestedActionSetLoginEmailAddress and isLoginEmailAddressRequired succeeds. The change will not be applied until the new login email address is confirmed with checkLoginEmailAddressCode.
/// To use Apple ID/Google ID instead of an email address, call checkLoginEmailAddressCode directly
///
/// # Arguments
///
/// * `new_login_email_address` - New login email address
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmailAddressAuthenticationCodeInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_login_email_address(new_login_email_address: String, client_id: i32) -> Result<crate::enums::EmailAddressAuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "setLoginEmailAddress",
        "new_login_email_address": new_login_email_address,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Resends the login email address verification code
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmailAddressAuthenticationCodeInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn resend_login_email_address_code(client_id: i32) -> Result<crate::enums::EmailAddressAuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "resendLoginEmailAddressCode",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks the login email address authentication
///
/// # Arguments
///
/// * `code` - Email address authentication to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_login_email_address_code(code: crate::enums::EmailAddressAuthentication, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkLoginEmailAddressCode",
        "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user
///
/// # Arguments
///
/// * `password` - The 2-step verification password for the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::RecoveryEmailAddress)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_recovery_email_address(password: String, client_id: i32) -> Result<crate::enums::RecoveryEmailAddress, crate::types::Error> {
    let request = json!({
        "@type": "getRecoveryEmailAddress",
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed.
/// If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation
///
/// # Arguments
///
/// * `password` - The 2-step verification password of the current user
/// * `new_recovery_email_address` - New recovery email address
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PasswordState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_recovery_email_address(password: String, new_recovery_email_address: String, client_id: i32) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
        "@type": "setRecoveryEmailAddress",
        "password": password,
        "new_recovery_email_address": new_recovery_email_address,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks the 2-step verification recovery email address verification code
///
/// # Arguments
///
/// * `code` - Verification code to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PasswordState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_recovery_email_address_code(code: String, client_id: i32) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
        "@type": "checkRecoveryEmailAddressCode",
        "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Resends the 2-step verification recovery email address verification code
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PasswordState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn resend_recovery_email_address_code(client_id: i32) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
        "@type": "resendRecoveryEmailAddressCode",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Cancels verification of the 2-step verification recovery email address
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PasswordState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn cancel_recovery_email_address_verification(client_id: i32) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
        "@type": "cancelRecoveryEmailAddressVerification",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Requests to send a 2-step verification password recovery code to an email address that was previously set up
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmailAddressAuthenticationCodeInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn request_password_recovery(client_id: i32) -> Result<crate::enums::EmailAddressAuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "requestPasswordRecovery",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether a 2-step verification password recovery code sent to an email address is valid
///
/// # Arguments
///
/// * `recovery_code` - Recovery code to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_password_recovery_code(recovery_code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkPasswordRecoveryCode",
        "recovery_code": recovery_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Recovers the 2-step verification password using a recovery code sent to an email address that was previously set up
///
/// # Arguments
///
/// * `recovery_code` - Recovery code to check
/// * `new_password` - New 2-step verification password of the user; may be empty to remove the password
/// * `new_hint` - New password hint; may be empty
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PasswordState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn recover_password(recovery_code: String, new_password: String, new_hint: String, client_id: i32) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
        "@type": "recoverPassword",
        "recovery_code": recovery_code,
        "new_password": new_password,
        "new_hint": new_hint,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes 2-step verification password without previous password and access to recovery email address. The password can't be reset immediately and the request needs to be repeated after the specified time
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ResetPasswordResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reset_password(client_id: i32) -> Result<crate::enums::ResetPasswordResult, crate::types::Error> {
    let request = json!({
        "@type": "resetPassword",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Cancels reset of 2-step verification password. The method can be called if passwordState.pending_reset_date > 0
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn cancel_password_reset(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "cancelPasswordReset",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Creates a new temporary password for processing payments
///
/// # Arguments
///
/// * `password` - The 2-step verification password of the current user
/// * `valid_for` - Time during which the temporary password will be valid, in seconds; must be between 60 and 86400
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TemporaryPasswordState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_temporary_password(password: String, valid_for: i32, client_id: i32) -> Result<crate::enums::TemporaryPasswordState, crate::types::Error> {
    let request = json!({
        "@type": "createTemporaryPassword",
        "password": password,
        "valid_for": valid_for,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about the current temporary password
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TemporaryPasswordState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_temporary_password_state(client_id: i32) -> Result<crate::enums::TemporaryPasswordState, crate::types::Error> {
    let request = json!({
        "@type": "getTemporaryPasswordState",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns full information about a community. The data will be sent through update.
///
/// # Arguments
///
/// * `community_id` - Community identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn load_community_full_info(community_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadCommunityFullInfo",
        "community_id": community_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Creates a new community for the given chat. Returns identifier of the created community
///
/// # Arguments
///
/// * `name` - Name of the new community
/// * `chat_id` - Identifier of the chat in the community; only chats with owned bots and owned basic group, supergroup and channel chats are allowed;
/// basic group chats will be automatically upgraded to supergroup chats
/// * `is_chat_hidden` - Pass true if the chat will be visible only to administrators of the community
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CommunityId)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_community(name: String, chat_id: i64, is_chat_hidden: bool, client_id: i32) -> Result<crate::enums::CommunityId, crate::types::Error> {
    let request = json!({
        "@type": "createCommunity",
        "name": name,
        "chat_id": chat_id,
        "is_chat_hidden": is_chat_hidden,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes name of the given community; requires can_change_info administrator right in the community
///
/// # Arguments
///
/// * `community_id` - Identifier of the community
/// * `name` - New name of the community
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_community_name(community_id: i64, name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setCommunityName",
        "community_id": community_id,
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a list of common group chats with a given user. Chats are sorted by their type and creation date
///
/// # Arguments
///
/// * `user_id` - User identifier
/// * `offset_chat_id` - Chat identifier starting from which to return chats; use 0 for the first request
/// * `limit` - The maximum number of chats to be returned; up to 100
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Chats)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_groups_in_common(user_id: i64, offset_chat_id: i64, limit: i32, client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getGroupsInCommon",
        "user_id": user_id,
        "offset_chat_id": offset_chat_id,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks public post search limits without actually performing the search
///
/// # Arguments
///
/// * `query` - Query that will be searched for
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PublicPostSearchLimits)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_public_post_search_limits(query: String, client_id: i32) -> Result<crate::enums::PublicPostSearchLimits, crate::types::Error> {
    let request = json!({
        "@type": "getPublicPostSearchLimits",
        "query": query,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for public channel posts using the given query. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
///
/// # Arguments
///
/// * `query` - Query to search for
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages to be returned; up to 100. For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `star_count` - The Telegram Star amount the user agreed to pay for the search; pass 0 for free searches
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundPublicPosts)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_public_posts(query: String, offset: String, limit: i32, star_count: i64, client_id: i32) -> Result<crate::enums::FoundPublicPosts, crate::types::Error> {
    let request = json!({
        "@type": "searchPublicPosts",
        "query": query,
        "offset": offset,
        "limit": limit,
        "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns recently searched for hashtags or cashtags by their prefix
///
/// # Arguments
///
/// * `tag_prefix` - Prefix of hashtags or cashtags to return
/// * `limit` - The maximum number of items to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Hashtags)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_searched_for_tags(tag_prefix: String, limit: i32, client_id: i32) -> Result<crate::enums::Hashtags, crate::types::Error> {
    let request = json!({
        "@type": "getSearchedForTags",
        "tag_prefix": tag_prefix,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes a hashtag or a cashtag from the list of recently searched for hashtags or cashtags
///
/// # Arguments
///
/// * `tag` - Hashtag or cashtag to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_searched_for_tag(tag: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeSearchedForTag",
        "tag": tag,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Clears the list of recently searched for hashtags or cashtags
///
/// # Arguments
///
/// * `clear_cashtags` - Pass true to clear the list of recently searched for cashtags; otherwise, the list of recently searched for hashtags will be cleared
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn clear_searched_for_tags(clear_cashtags: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearSearchedForTags",
        "clear_cashtags": clear_cashtags,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Recognizes speech in a video note or a voice note message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message. Use messageProperties.can_recognize_speech to check whether the message is suitable
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn recognize_speech(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "recognizeSpeech",
        "chat_id": chat_id,
        "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Rates recognized speech in a video note or a voice note message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `is_good` - Pass true if the speech recognition is good
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn rate_speech_recognition(chat_id: i64, message_id: i64, is_good: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "rateSpeechRecognition",
        "chat_id": chat_id,
        "message_id": message_id,
        "is_good": is_good,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks validness of a name for a quick reply shortcut. Can be called synchronously
///
/// # Arguments
///
/// * `name` - The name of the shortcut; 1-32 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_quick_reply_shortcut_name(name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkQuickReplyShortcutName",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Loads quick reply shortcuts created by the current user. The loaded data will be sent through updateQuickReplyShortcut and updateQuickReplyShortcuts
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn load_quick_reply_shortcuts(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadQuickReplyShortcuts",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes name of a quick reply shortcut
///
/// # Arguments
///
/// * `shortcut_id` - Unique identifier of the quick reply shortcut
/// * `name` - New name for the shortcut. Use checkQuickReplyShortcutName to check its validness
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_quick_reply_shortcut_name(shortcut_id: i32, name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setQuickReplyShortcutName",
        "shortcut_id": shortcut_id,
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes a quick reply shortcut
///
/// # Arguments
///
/// * `shortcut_id` - Unique identifier of the quick reply shortcut
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_quick_reply_shortcut(shortcut_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteQuickReplyShortcut",
        "shortcut_id": shortcut_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the order of quick reply shortcuts
///
/// # Arguments
///
/// * `shortcut_ids` - The new order of quick reply shortcuts
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_quick_reply_shortcuts(shortcut_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderQuickReplyShortcuts",
        "shortcut_ids": shortcut_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns parameters for creating of a new passkey as JSON-serialized string
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_passkey_parameters(client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getPasskeyParameters",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a passkey allowed to be used for the login by the current user and returns the added passkey. Call getPasskeyParameters to get parameters for creating of the passkey
///
/// # Arguments
///
/// * `client_data` - JSON-encoded client data
/// * `attestation_object` - Passkey attestation object
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Passkey)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_login_passkey(client_data: String, attestation_object: String, client_id: i32) -> Result<crate::enums::Passkey, crate::types::Error> {
    let request = json!({
        "@type": "addLoginPasskey",
        "client_data": client_data,
        "attestation_object": attestation_object,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of passkeys allowed to be used for the login by the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Passkeys)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_login_passkeys(client_id: i32) -> Result<crate::enums::Passkeys, crate::types::Error> {
    let request = json!({
        "@type": "getLoginPasskeys",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes a passkey from the list of passkeys allowed to be used for the login by the current user
///
/// # Arguments
///
/// * `passkey_id` - Unique identifier of the passkey to remove
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_login_passkey(passkey_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeLoginPasskey",
        "passkey_id": passkey_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Searches for a given quote in a text. Returns found quote start position in UTF-16 code units. Returns a 404 error if the quote is not found. Can be called synchronously
///
/// # Arguments
///
/// * `text` - Text in which to search for the quote
/// * `quote` - Quote to search for
/// * `quote_position` - Approximate quote position in UTF-16 code units
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundPosition)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_quote(text: crate::types::FormattedText, quote: crate::types::FormattedText, quote_position: i32, client_id: i32) -> Result<crate::enums::FoundPosition, crate::types::Error> {
    let request = json!({
        "@type": "searchQuote",
        "text": text,
        "quote": quote,
        "quote_position": quote_position,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Parses Markdown entities in a human-friendly format, ignoring markup errors. Can be called synchronously
///
/// # Arguments
///
/// * `text` - The text to parse. For example, "__italic__ ~~strikethrough~~ ||spoiler|| **bold** `code` ```pre``` __[italic__ text_url](telegram.org) __italic**bold italic__bold**"
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FormattedText)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn parse_markdown(text: crate::types::FormattedText, client_id: i32) -> Result<crate::enums::FormattedText, crate::types::Error> {
    let request = json!({
        "@type": "parseMarkdown",
        "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. Can be called synchronously
///
/// # Arguments
///
/// * `language_pack_database_path` - Path to the language pack database in which strings are stored
/// * `localization_target` - Localization target to which the language pack belongs
/// * `language_pack_id` - Language pack identifier
/// * `key` - Language pack key of the string to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LanguagePackStringValue)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_language_pack_string(language_pack_database_path: String, localization_target: String, language_pack_id: String, key: String, client_id: i32) -> Result<crate::enums::LanguagePackStringValue, crate::types::Error> {
    let request = json!({
        "@type": "getLanguagePackString",
        "language_pack_database_path": language_pack_database_path,
        "localization_target": localization_target,
        "language_pack_id": language_pack_id,
        "key": key,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Converts a JSON-serialized string to corresponding JsonValue object. Can be called synchronously
///
/// # Arguments
///
/// * `json` - The JSON-serialized string
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::JsonValue)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_json_value(json: String, client_id: i32) -> Result<crate::enums::JsonValue, crate::types::Error> {
    let request = json!({
        "@type": "getJsonValue",
        "json": json,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Converts a JsonValue object to corresponding JSON-serialized string. Can be called synchronously
///
/// # Arguments
///
/// * `json_value` - The JsonValue object
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_json_string(json_value: crate::enums::JsonValue, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getJsonString",
        "json_value": json_value,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Converts a themeParameters object to corresponding JSON-serialized string. Can be called synchronously
///
/// # Arguments
///
/// * `theme` - Theme parameters to convert to JSON
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_theme_parameters_json_string(theme: crate::types::ThemeParameters, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getThemeParametersJsonString",
        "theme": theme,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds tasks to a checklist in a message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message containing the checklist. Use messageProperties.can_add_tasks to check whether the tasks can be added
/// * `tasks` - List of added tasks
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_checklist_tasks(chat_id: i64, message_id: i64, tasks: Vec<crate::types::InputChecklistTask>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addChecklistTasks",
        "chat_id": chat_id,
        "message_id": message_id,
        "tasks": tasks,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds tasks of a checklist in a message as done or not done
///
/// # Arguments
///
/// * `chat_id` - Identifier of the chat with the message
/// * `message_id` - Identifier of the message containing the checklist. Use messageProperties.can_mark_tasks_as_done to check whether the tasks can be marked as done or not done
/// * `marked_as_done_task_ids` - Identifiers of tasks that were marked as done
/// * `marked_as_not_done_task_ids` - Identifiers of tasks that were marked as not done
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn mark_checklist_tasks_as_done(chat_id: i64, message_id: i64, marked_as_done_task_ids: Vec<i32>, marked_as_not_done_task_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "markChecklistTasksAsDone",
        "chat_id": chat_id,
        "message_id": message_id,
        "marked_as_done_task_ids": marked_as_done_task_ids,
        "marked_as_not_done_task_ids": marked_as_not_done_task_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Hides a suggested action
///
/// # Arguments
///
/// * `action` - Suggested action to hide
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn hide_suggested_action(action: crate::enums::SuggestedAction, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "hideSuggestedAction",
        "action": action,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the message with the button
/// * `message_id` - Message identifier of the message with the button. The message must not be scheduled
/// * `button_id` - Button identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LoginUrlInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_login_url_info(chat_id: i64, message_id: i64, button_id: i64, client_id: i32) -> Result<crate::enums::LoginUrlInfo, crate::types::Error> {
    let request = json!({
        "@type": "getLoginUrlInfo",
        "chat_id": chat_id,
        "message_id": message_id,
        "button_id": button_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTP URL which can be used to automatically authorize the user on a website after clicking an inline button of type inlineKeyboardButtonTypeLoginUrl.
/// Use the method getLoginUrlInfo to find whether a prior user confirmation is needed. If an error is returned, then the button must be handled as an ordinary URL button
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the message with the button
/// * `message_id` - Message identifier of the message with the button
/// * `button_id` - Button identifier
/// * `allow_write_access` - Pass true to allow the bot to send messages to the current user. Phone number access can't be requested using the button
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_login_url(chat_id: i64, message_id: i64, button_id: i64, allow_write_access: bool, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getLoginUrl",
        "chat_id": chat_id,
        "message_id": message_id,
        "button_id": button_id,
        "allow_write_access": allow_write_access,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the result of a guest query; for bots only
///
/// # Arguments
///
/// * `guest_query_id` - Identifier of the guest query
/// * `result` - The result of the query
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::InlineMessageId)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn answer_guest_query(guest_query_id: i64, result: crate::enums::InputInlineQueryResult, client_id: i32) -> Result<crate::enums::InlineMessageId, crate::types::Error> {
    let request = json!({
        "@type": "answerGuestQuery",
        "guest_query_id": guest_query_id,
        "result": result,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Saves a keyboard button to be shown to the given user; for bots only
///
/// # Arguments
///
/// * `user_id` - Identifier of the user
/// * `button` - The button; must be of the type keyboardButtonTypeRequestUsers, keyboardButtonTypeRequestChat, or keyboardButtonTypeRequestManagedBot
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn save_prepared_keyboard_button(user_id: i64, button: crate::types::KeyboardButton, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "savePreparedKeyboardButton",
        "user_id": user_id,
        "button": button,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a keyboard button prepared by the bot for the user. The button will be of the type keyboardButtonTypeRequestUsers, keyboardButtonTypeRequestChat, or keyboardButtonTypeRequestManagedBot
///
/// # Arguments
///
/// * `bot_user_id` - Identifier of the bot that created the button
/// * `prepared_button_id` - Identifier of the prepared button
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::KeyboardButton)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_prepared_keyboard_button(bot_user_id: i64, prepared_button_id: String, client_id: i32) -> Result<crate::enums::KeyboardButton, crate::types::Error> {
    let request = json!({
        "@type": "getPreparedKeyboardButton",
        "bot_user_id": bot_user_id,
        "prepared_button_id": prepared_button_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Updates the game score of the specified user in the game; for bots only
///
/// # Arguments
///
/// * `chat_id` - The chat to which the message with the game belongs
/// * `message_id` - Identifier of the message
/// * `edit_message` - Pass true to edit the game message to include the current scoreboard
/// * `user_id` - User identifier
/// * `score` - The new score
/// * `force` - Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_game_score(chat_id: i64, message_id: i64, edit_message: bool, user_id: i64, score: i32, force: bool, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "setGameScore",
        "chat_id": chat_id,
        "message_id": message_id,
        "edit_message": edit_message,
        "user_id": user_id,
        "score": score,
        "force": force,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Updates the game score of the specified user in a game; for bots only
///
/// # Arguments
///
/// * `inline_message_id` - Inline message identifier
/// * `edit_message` - Pass true to edit the game message to include the current scoreboard
/// * `user_id` - User identifier
/// * `score` - The new score
/// * `force` - Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_inline_game_score(inline_message_id: String, edit_message: bool, user_id: i64, score: i32, force: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setInlineGameScore",
        "inline_message_id": inline_message_id,
        "edit_message": edit_message,
        "user_id": user_id,
        "score": score,
        "force": force,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only
///
/// # Arguments
///
/// * `chat_id` - The chat that contains the message with the game
/// * `message_id` - Identifier of the message
/// * `user_id` - User identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GameHighScores)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_game_high_scores(chat_id: i64, message_id: i64, user_id: i64, client_id: i32) -> Result<crate::enums::GameHighScores, crate::types::Error> {
    let request = json!({
        "@type": "getGameHighScores",
        "chat_id": chat_id,
        "message_id": message_id,
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns game high scores and some part of the high score table in the range of the specified user; for bots only
///
/// # Arguments
///
/// * `inline_message_id` - Inline message identifier
/// * `user_id` - User identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GameHighScores)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_inline_game_high_scores(inline_message_id: String, user_id: i64, client_id: i32) -> Result<crate::enums::GameHighScores, crate::types::Error> {
    let request = json!({
        "@type": "getInlineGameHighScores",
        "inline_message_id": inline_message_id,
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTPS or a tg: link with the given type. Can be called before authorization
///
/// # Arguments
///
/// * `r#type` - Expected type of the link
/// * `is_http` - Pass true to create an HTTPS link (only available for some link types); pass false to create a tg: link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_internal_link(r#type: crate::enums::InternalLinkType, is_http: bool, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getInternalLink",
        "type": r#type,
        "is_http": is_http,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about the type of internal link. Returns a 404 error if the link is not internal. Can be called before authorization
///
/// # Arguments
///
/// * `link` - The link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::InternalLinkType)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_internal_link_type(link: String, client_id: i32) -> Result<crate::enums::InternalLinkType, crate::types::Error> {
    let request = json!({
        "@type": "getInternalLinkType",
        "link": link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about an action to be done when the current user clicks an external link. Don't use this method for links from secret chats
/// if link preview is disabled in secret chats, and use directly getLinkWebBrowserType
///
/// # Arguments
///
/// * `link` - The link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LoginUrlInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_external_link_info(link: String, client_id: i32) -> Result<crate::enums::LoginUrlInfo, crate::types::Error> {
    let request = json!({
        "@type": "getExternalLinkInfo",
        "link": link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an HTTP URL which can be used to automatically authorize the current user on a website after clicking an HTTP link.
/// Use the method getExternalLinkInfo to find whether a prior user confirmation is needed. May return an empty link if just a toast about successful login has to be shown
///
/// # Arguments
///
/// * `link` - The HTTP link
/// * `allow_write_access` - Pass true if the current user allowed the bot that was returned in getExternalLinkInfo, to send them messages
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_external_link(link: String, allow_write_access: bool, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getExternalLink",
        "link": link,
        "allow_write_access": allow_write_access,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a type of the web browser which must be used to open the link
///
/// # Arguments
///
/// * `link` - The HTTP link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::WebBrowserType)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_link_web_browser_type(link: String, client_id: i32) -> Result<crate::enums::WebBrowserType, crate::types::Error> {
    let request = json!({
        "@type": "getLinkWebBrowserType",
        "link": link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about an OAuth deep link. Use checkOauthRequestMatchCode, acceptOauthRequest or declineOauthRequest to process the link
///
/// # Arguments
///
/// * `url` - URL of the link
/// * `in_app_origin` - Origin of the OAuth request if the request was received from the in-app browser; pass an empty string otherwise
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::OauthLinkInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_oauth_link_info(url: String, in_app_origin: String, client_id: i32) -> Result<crate::enums::OauthLinkInfo, crate::types::Error> {
    let request = json!({
        "@type": "getOauthLinkInfo",
        "url": url,
        "in_app_origin": in_app_origin,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks a match-code for an OAuth authorization request. If fails, then the authorization request has failed. Otherwise,
/// authorization confirmation dialog must be shown and the link must be processed using acceptOauthRequest or declineOauthRequest
///
/// # Arguments
///
/// * `url` - URL of the OAuth deep link
/// * `match_code` - The matching code chosen by the user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_oauth_request_match_code(url: String, match_code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkOauthRequestMatchCode",
        "url": url,
        "match_code": match_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Accepts an OAuth authorization request. Returns an HTTP URL to open after successful authorization.
/// May return an empty link if just a toast about successful login has to be shown
///
/// # Arguments
///
/// * `url` - URL of the OAuth deep link
/// * `match_code` - The matching code chosen by the user
/// * `allow_write_access` - Pass true if the current user allowed the bot that was returned in getOauthLinkInfo, to send them messages
/// * `allow_phone_number_access` - Pass true if the current user allowed the bot that was returned in getOauthLinkInfo, to access their phone number
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn accept_oauth_request(url: String, match_code: String, allow_write_access: bool, allow_phone_number_access: bool, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "acceptOauthRequest",
        "url": url,
        "match_code": match_code,
        "allow_write_access": allow_write_access,
        "allow_phone_number_access": allow_phone_number_access,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Declines an OAuth authorization request
///
/// # Arguments
///
/// * `url` - URL of the OAuth deep link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn decline_oauth_request(url: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "declineOauthRequest",
        "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks whether the current session can be used to transfer a chat ownership to another user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CanTransferOwnershipResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn can_transfer_ownership(client_id: i32) -> Result<crate::enums::CanTransferOwnershipResult, crate::types::Error> {
    let request = json!({
        "@type": "canTransferOwnership",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the current weather in the given location
///
/// # Arguments
///
/// * `location` - The location
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CurrentWeather)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_current_weather(location: crate::types::Location, client_id: i32) -> Result<crate::enums::CurrentWeather, crate::types::Error> {
    let request = json!({
        "@type": "getCurrentWeather",
        "location": location,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes pause state of a file in the file download list
///
/// # Arguments
///
/// * `file_id` - Identifier of the downloaded file
/// * `is_paused` - Pass true if the download is paused
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_download_is_paused(file_id: i32, is_paused: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleDownloadIsPaused",
        "file_id": file_id,
        "is_paused": is_paused,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes pause state of all files in the file download list
///
/// # Arguments
///
/// * `are_paused` - Pass true to pause all downloads; pass false to unpause them
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_all_downloads_are_paused(are_paused: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleAllDownloadsArePaused",
        "are_paused": are_paused,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that application or reCAPTCHA verification has been completed. Can be called before authorization
///
/// # Arguments
///
/// * `verification_id` - Unique identifier for the verification process as received from updateApplicationVerificationRequired or updateApplicationRecaptchaVerificationRequired
/// * `token` - Play Integrity API token for the Android application, or secret from push notification for the iOS application for application verification, or reCAPTCHA token for reCAPTCHA verifications;
/// pass an empty string to abort verification and receive the error "VERIFICATION_FAILED" for the request
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_application_verification_token(verification_id: i64, token: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setApplicationVerificationToken",
        "verification_id": verification_id,
        "token": token,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Approves a suggested post in a channel direct messages chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `message_id` - Identifier of the message with the suggested post. Use messageProperties.can_be_approved to check whether the suggested post can be approved
/// * `send_date` - Point in time (Unix timestamp) when the post is expected to be published; pass 0 if the date has already been chosen. If specified,
/// then the date must be in the future, but at most getOption("suggested_post_send_delay_max") seconds in the future
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn approve_suggested_post(chat_id: i64, message_id: i64, send_date: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "approveSuggestedPost",
        "chat_id": chat_id,
        "message_id": message_id,
        "send_date": send_date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Declines a suggested post in a channel direct messages chat
///
/// # Arguments
///
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `message_id` - Identifier of the message with the suggested post. Use messageProperties.can_be_declined to check whether the suggested post can be declined
/// * `comment` - Comment for the creator of the suggested post; 0-128 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn decline_suggested_post(chat_id: i64, message_id: i64, comment: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "declineSuggestedPost",
        "chat_id": chat_id,
        "message_id": message_id,
        "comment": comment,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a suggested post based on a previously sent message in a channel direct messages chat. Can be also used to suggest price or time change for an existing suggested post.
/// Returns the sent message
///
/// # Arguments
///
/// * `chat_id` - Identifier of the channel direct messages chat
/// * `message_id` - Identifier of the message in the chat which will be sent as suggested post. Use messageProperties.can_add_offer to check whether an offer can be added
/// or messageProperties.can_edit_suggested_post_info to check whether price or time of sending of the post can be changed
/// * `options` - Options to be used to send the message. New information about the suggested post must always be specified
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Message)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_offer(chat_id: i64, message_id: i64, options: crate::types::MessageSendOptions, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "addOffer",
        "chat_id": chat_id,
        "message_id": message_id,
        "options": options,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes the list of close friends of the current user
///
/// # Arguments
///
/// * `user_ids` - User identifiers of close friends; the users must be contacts of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_close_friends(user_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setCloseFriends",
        "user_ids": user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns all close friends of the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Users)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_close_friends(client_id: i32) -> Result<crate::enums::Users, crate::types::Error> {
    let request = json!({
        "@type": "getCloseFriends",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Shares the phone number of the current user with a mutual contact. Supposed to be called when the user clicks on chatActionBarSharePhoneNumber
///
/// # Arguments
///
/// * `user_id` - Identifier of the user with whom to share the phone number. The user must be a mutual contact
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn share_phone_number(user_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sharePhoneNumber",
        "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Searches for recently used hashtags by their prefix
///
/// # Arguments
///
/// * `prefix` - Hashtag prefix to search for
/// * `limit` - The maximum number of hashtags to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Hashtags)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_hashtags(prefix: String, limit: i32, client_id: i32) -> Result<crate::enums::Hashtags, crate::types::Error> {
    let request = json!({
        "@type": "searchHashtags",
        "prefix": prefix,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes a hashtag from the list of recently used hashtags
///
/// # Arguments
///
/// * `hashtag` - Hashtag to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_recent_hashtag(hashtag: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeRecentHashtag",
        "hashtag": hashtag,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns a link preview by the text of a message. Do not call this function too often. Returns a 404 error if the text has no link preview
///
/// # Arguments
///
/// * `text` - Message text with formatting
/// * `link_preview_options` - Options to be used for generation of the link preview; pass null to use default link preview options
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LinkPreview)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_link_preview(text: crate::types::FormattedText, link_preview_options: Option<crate::types::LinkPreviewOptions>, client_id: i32) -> Result<crate::enums::LinkPreview, crate::types::Error> {
    let request = json!({
        "@type": "getLinkPreview",
        "text": text,
        "link_preview_options": link_preview_options,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns an instant view version of a web page if available. This is an offline method if only_local is true. Returns a 404 error if the web page has no instant view page
///
/// # Arguments
///
/// * `url` - The web page URL
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::WebPageInstantView)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_web_page_instant_view(url: String, only_local: bool, client_id: i32) -> Result<crate::enums::WebPageInstantView, crate::types::Error> {
    let request = json!({
        "@type": "getWebPageInstantView",
        "url": url,
        "only_local": only_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes accent color and background custom emoji for the current user; for Telegram Premium users only
///
/// # Arguments
///
/// * `accent_color_id` - Identifier of the accent color to use
/// * `background_custom_emoji_id` - Identifier of a custom emoji to be shown on the reply header and link preview background; 0 if none
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_accent_color(accent_color_id: i32, background_custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAccentColor",
        "accent_color_id": accent_color_id,
        "background_custom_emoji_id": background_custom_emoji_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes color scheme for the current user based on an owned or a hosted upgraded gift; for Telegram Premium users only
///
/// # Arguments
///
/// * `upgraded_gift_colors_id` - Identifier of the upgradedGiftColors scheme to use
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_upgraded_gift_colors(upgraded_gift_colors_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setUpgradedGiftColors",
        "upgraded_gift_colors_id": upgraded_gift_colors_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the first and last name of the current user
///
/// # Arguments
///
/// * `first_name` - The new value of the first name for the current user; 1-64 characters
/// * `last_name` - The new value of the optional last name for the current user; 0-64 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_name(first_name: String, last_name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setName",
        "first_name": first_name,
        "last_name": last_name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the bio of the current user
///
/// # Arguments
///
/// * `bio` - The new value of the user bio; 0-getOption("bio_length_max") characters without line feeds
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_bio(bio: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBio",
        "bio": bio,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the birthdate of the current user
///
/// # Arguments
///
/// * `birthdate` - The new value of the current user's birthdate; pass null to remove the birthdate
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_birthdate(birthdate: Option<crate::types::Birthdate>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBirthdate",
        "birthdate": birthdate,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a code to the specified phone number. Aborts previous phone number verification if there was one. On success, returns information about the sent code
///
/// # Arguments
///
/// * `phone_number` - The phone number, in international format
/// * `settings` - Settings for the authentication of the user's phone number; pass null to use default settings
/// * `r#type` - Type of the request for which the code is sent
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AuthenticationCodeInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_phone_number_code(phone_number: String, settings: Option<crate::types::PhoneNumberAuthenticationSettings>, r#type: crate::enums::PhoneNumberCodeType, client_id: i32) -> Result<crate::enums::AuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "sendPhoneNumberCode",
        "phone_number": phone_number,
        "settings": settings,
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends Firebase Authentication SMS to the specified phone number. Works only when received a code of the type authenticationCodeTypeFirebaseAndroid or authenticationCodeTypeFirebaseIos
///
/// # Arguments
///
/// * `token` - Play Integrity API or SafetyNet Attestation API token for the Android application, or secret from push notification for the iOS application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_phone_number_firebase_sms(token: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendPhoneNumberFirebaseSms",
        "token": token,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Reports that authentication code wasn't delivered via SMS to the specified phone number; for official mobile applications only
///
/// # Arguments
///
/// * `mobile_network_code` - Current mobile network code
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn report_phone_number_code_missing(mobile_network_code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportPhoneNumberCodeMissing",
        "mobile_network_code": mobile_network_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Resends the authentication code sent to a phone number. Works only if the previously received authenticationCodeInfo next_code_type was not null and the server-specified timeout has passed
///
/// # Arguments
///
/// * `reason` - Reason of code resending; pass null if unknown
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AuthenticationCodeInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn resend_phone_number_code(reason: Option<crate::enums::ResendCodeReason>, client_id: i32) -> Result<crate::enums::AuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "resendPhoneNumberCode",
        "reason": reason,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks the authentication code and completes the request for which the code was sent if appropriate
///
/// # Arguments
///
/// * `code` - Authentication code to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_phone_number_code(code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkPhoneNumberCode",
        "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sets the list of commands supported by the bot for the given user scope and language; for bots only
///
/// # Arguments
///
/// * `scope` - The scope to which the commands are relevant; pass null to change commands in the default bot command scope
/// * `language_code` - A two-letter ISO 639-1 language code. If empty, the commands will be applied to all users from the given scope, for which language there are no dedicated commands
/// * `commands` - List of the bot's commands
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_commands(scope: Option<crate::enums::BotCommandScope>, language_code: String, commands: Vec<crate::types::BotCommand>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setCommands",
        "scope": scope,
        "language_code": language_code,
        "commands": commands,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes commands supported by the bot for the given user scope and language; for bots only
///
/// # Arguments
///
/// * `scope` - The scope to which the commands are relevant; pass null to delete commands in the default bot command scope
/// * `language_code` - A two-letter ISO 639-1 language code or an empty string
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_commands(scope: Option<crate::enums::BotCommandScope>, language_code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteCommands",
        "scope": scope,
        "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of commands supported by the bot for the given user scope and language; for bots only
///
/// # Arguments
///
/// * `scope` - The scope to which the commands are relevant; pass null to get commands in the default bot command scope
/// * `language_code` - A two-letter ISO 639-1 language code or an empty string
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::BotCommands)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_commands(scope: Option<crate::enums::BotCommandScope>, language_code: String, client_id: i32) -> Result<crate::enums::BotCommands, crate::types::Error> {
    let request = json!({
        "@type": "getCommands",
        "scope": scope,
        "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets default administrator rights for adding the bot to basic group and supergroup chats; for bots only
///
/// # Arguments
///
/// * `default_group_administrator_rights` - Default administrator rights for adding the bot to basic group and supergroup chats; pass null to remove default rights
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_default_group_administrator_rights(default_group_administrator_rights: Option<crate::types::ChatAdministratorRights>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setDefaultGroupAdministratorRights",
        "default_group_administrator_rights": default_group_administrator_rights,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns all active sessions of the current user. Additionally, getBusinessConnectedBot must be used to show the bot on top of active sessions
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Sessions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_active_sessions(client_id: i32) -> Result<crate::enums::Sessions, crate::types::Error> {
    let request = json!({
        "@type": "getActiveSessions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Terminates a session of the current user
///
/// # Arguments
///
/// * `session_id` - Session identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn terminate_session(session_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "terminateSession",
        "session_id": session_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Terminates all other sessions of the current user. Additionally, the user must be suggested to delete the connected business bot using deleteBusinessConnectedBot if there is any
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn terminate_all_other_sessions(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "terminateAllOtherSessions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Confirms an unconfirmed session of the current user from another device
///
/// # Arguments
///
/// * `session_id` - Session identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn confirm_session(session_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "confirmSession",
        "session_id": session_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the period of inactivity after which sessions will automatically be terminated
///
/// # Arguments
///
/// * `inactive_session_ttl_days` - New number of days of inactivity before sessions will be automatically terminated; 1-366 days
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_inactive_session_ttl(inactive_session_ttl_days: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setInactiveSessionTtl",
        "inactive_session_ttl_days": inactive_session_ttl_days,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns all website where the current user used Telegram to log in
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ConnectedWebsites)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_connected_websites(client_id: i32) -> Result<crate::enums::ConnectedWebsites, crate::types::Error> {
    let request = json!({
        "@type": "getConnectedWebsites",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Disconnects website from the current user's Telegram account
///
/// # Arguments
///
/// * `website_id` - Website identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn disconnect_website(website_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "disconnectWebsite",
        "website_id": website_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Disconnects all websites from the current user's Telegram account
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn disconnect_all_websites(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "disconnectAllWebsites",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of supported time zones
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TimeZones)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_time_zones(client_id: i32) -> Result<crate::enums::TimeZones, crate::types::Error> {
    let request = json!({
        "@type": "getTimeZones",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes saved credentials for all payment provider bots
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_saved_credentials(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteSavedCredentials",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes settings for gift receiving for the current user
///
/// # Arguments
///
/// * `settings` - The new settings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_gift_settings(settings: crate::types::GiftSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setGiftSettings",
        "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns gifts that can be sent to other users and channel chats
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AvailableGifts)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_available_gifts(client_id: i32) -> Result<crate::enums::AvailableGifts, crate::types::Error> {
    let request = json!({
        "@type": "getAvailableGifts",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks whether a gift with next_send_date in the future can be sent already
///
/// # Arguments
///
/// * `gift_id` - Identifier of the gift to send
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CanSendGiftResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn can_send_gift(gift_id: i64, client_id: i32) -> Result<crate::enums::CanSendGiftResult, crate::types::Error> {
    let request = json!({
        "@type": "canSendGift",
        "gift_id": gift_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends a gift to another user or channel chat. May return an error with a message "STARGIFT_USAGE_LIMITED" if the gift was sold out
///
/// # Arguments
///
/// * `gift_id` - Identifier of the gift to send
/// * `owner_id` - Identifier of the user or the channel chat that will receive the gift; limited gifts can't be sent to channel chats
/// * `text` - Text to show along with the gift; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed.
/// Must be empty if the receiver enabled paid messages
/// * `is_private` - Pass true to show gift text and sender only to the gift receiver; otherwise, everyone will be able to see them
/// * `pay_for_upgrade` - Pass true to additionally pay for the gift upgrade and allow the receiver to upgrade it for free
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_gift(gift_id: i64, owner_id: crate::enums::MessageSender, text: crate::types::FormattedText, is_private: bool, pay_for_upgrade: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendGift",
        "gift_id": gift_id,
        "owner_id": owner_id,
        "text": text,
        "is_private": is_private,
        "pay_for_upgrade": pay_for_upgrade,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns auction state for a gift
///
/// # Arguments
///
/// * `auction_id` - Unique identifier of the auction
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftAuctionState)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_auction_state(auction_id: String, client_id: i32) -> Result<crate::enums::GiftAuctionState, crate::types::Error> {
    let request = json!({
        "@type": "getGiftAuctionState",
        "auction_id": auction_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the gifts that were acquired by the current user on a gift auction
///
/// # Arguments
///
/// * `gift_id` - Identifier of the auctioned gift
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftAuctionAcquiredGifts)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_auction_acquired_gifts(gift_id: i64, client_id: i32) -> Result<crate::enums::GiftAuctionAcquiredGifts, crate::types::Error> {
    let request = json!({
        "@type": "getGiftAuctionAcquiredGifts",
        "gift_id": gift_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Informs TDLib that a gift auction was opened by the user
///
/// # Arguments
///
/// * `gift_id` - Identifier of the gift, which auction was opened
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn open_gift_auction(gift_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "openGiftAuction",
        "gift_id": gift_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs TDLib that a gift auction was closed by the user
///
/// # Arguments
///
/// * `gift_id` - Identifier of the gift, which auction was closed
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn close_gift_auction(gift_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "closeGiftAuction",
        "gift_id": gift_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Places a bid on an auction gift
///
/// # Arguments
///
/// * `gift_id` - Identifier of the gift to place the bid on
/// * `star_count` - The number of Telegram Stars to place in the bid
/// * `user_id` - Identifier of the user who will receive the gift
/// * `text` - Text to show along with the gift; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed.
/// Must be empty if the receiver enabled paid messages
/// * `is_private` - Pass true to show gift text and sender only to the gift receiver; otherwise, everyone will be able to see them
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn place_gift_auction_bid(gift_id: i64, star_count: i64, user_id: i64, text: crate::types::FormattedText, is_private: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "placeGiftAuctionBid",
        "gift_id": gift_id,
        "star_count": star_count,
        "user_id": user_id,
        "text": text,
        "is_private": is_private,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Increases a bid for an auction gift without changing gift text and receiver
///
/// # Arguments
///
/// * `gift_id` - Identifier of the gift to put the bid on
/// * `star_count` - The number of Telegram Stars to put in the bid
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn increase_gift_auction_bid(gift_id: i64, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "increaseGiftAuctionBid",
        "gift_id": gift_id,
        "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sells a gift for Telegram Stars; requires owner privileges for gifts owned by a chat
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request; for bots only
/// * `received_gift_id` - Identifier of the gift
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn sell_gift(business_connection_id: String, received_gift_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sellGift",
        "business_connection_id": business_connection_id,
        "received_gift_id": received_gift_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Toggles whether a gift is shown on the current user's or the channel's profile page; requires can_post_messages administrator right in the channel chat
///
/// # Arguments
///
/// * `received_gift_id` - Identifier of the gift
/// * `is_saved` - Pass true to display the gift on the user's or the channel's profile page; pass false to remove it from the profile page
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn toggle_gift_is_saved(received_gift_id: String, is_saved: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGiftIsSaved",
        "received_gift_id": received_gift_id,
        "is_saved": is_saved,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the list of pinned gifts on the current user's or the channel's profile page; requires can_post_messages administrator right in the channel chat
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that received the gifts
/// * `received_gift_ids` - New list of pinned gifts. All gifts must be upgraded and saved on the profile page first. There can be up to getOption("pinned_gift_count_max") pinned gifts
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_pinned_gifts(owner_id: crate::enums::MessageSender, received_gift_ids: Vec<String>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setPinnedGifts",
        "owner_id": owner_id,
        "received_gift_ids": received_gift_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns examples of possible upgraded gifts for a regular gift
///
/// # Arguments
///
/// * `regular_gift_id` - Identifier of the regular gift
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftUpgradePreview)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_upgrade_preview(regular_gift_id: i64, client_id: i32) -> Result<crate::enums::GiftUpgradePreview, crate::types::Error> {
    let request = json!({
        "@type": "getGiftUpgradePreview",
        "regular_gift_id": regular_gift_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns all possible variants of upgraded gifts for a regular gift
///
/// # Arguments
///
/// * `regular_gift_id` - Identifier of the regular gift
/// * `return_upgrade_models` - Pass true to get models that can be obtained by upgrading a regular gift
/// * `return_craft_models` - Pass true to get models that can be obtained by crafting a gift from upgraded gifts
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftUpgradeVariants)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_upgraded_gift_variants(regular_gift_id: i64, return_upgrade_models: bool, return_craft_models: bool, client_id: i32) -> Result<crate::enums::GiftUpgradeVariants, crate::types::Error> {
    let request = json!({
        "@type": "getUpgradedGiftVariants",
        "regular_gift_id": regular_gift_id,
        "return_upgrade_models": return_upgrade_models,
        "return_craft_models": return_craft_models,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Upgrades a regular gift
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request; for bots only
/// * `received_gift_id` - Identifier of the gift
/// * `keep_original_details` - Pass true to keep the original gift text, sender and receiver in the upgraded gift
/// * `star_count` - The Telegram Star amount required to pay for the upgrade. If the gift has prepaid_upgrade_star_count > 0, then pass 0, otherwise, pass gift.upgrade_star_count
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::UpgradeGiftResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn upgrade_gift(business_connection_id: String, received_gift_id: String, keep_original_details: bool, star_count: i64, client_id: i32) -> Result<crate::enums::UpgradeGiftResult, crate::types::Error> {
    let request = json!({
        "@type": "upgradeGift",
        "business_connection_id": business_connection_id,
        "received_gift_id": received_gift_id,
        "keep_original_details": keep_original_details,
        "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Pays for upgrade of a regular gift that is owned by another user or channel chat
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that owns the gift
/// * `prepaid_upgrade_hash` - Prepaid upgrade hash as received along with the gift
/// * `star_count` - The Telegram Star amount the user agreed to pay for the upgrade; must be equal to gift.upgrade_star_count
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn buy_gift_upgrade(owner_id: crate::enums::MessageSender, prepaid_upgrade_hash: String, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "buyGiftUpgrade",
        "owner_id": owner_id,
        "prepaid_upgrade_hash": prepaid_upgrade_hash,
        "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Crafts a new gift from other gifts that will be permanently lost
///
/// # Arguments
///
/// * `received_gift_ids` - Identifier of the gifts to use for crafting. In the case of a successful craft, the resulting gift will have the number of the first gift.
/// Consequently, the first gift must not have been withdrawn to the TON blockchain as an NFT and must have an empty gift_address
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CraftGiftResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn craft_gift(received_gift_ids: Vec<String>, client_id: i32) -> Result<crate::enums::CraftGiftResult, crate::types::Error> {
    let request = json!({
        "@type": "craftGift",
        "received_gift_ids": received_gift_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends an upgraded gift to another user or channel chat
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request; for bots only
/// * `received_gift_id` - Identifier of the gift
/// * `new_owner_id` - Identifier of the user or the channel chat that will receive the gift
/// * `star_count` - The Telegram Star amount required to pay for the transfer
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn transfer_gift(business_connection_id: String, received_gift_id: String, new_owner_id: crate::enums::MessageSender, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "transferGift",
        "business_connection_id": business_connection_id,
        "received_gift_id": received_gift_id,
        "new_owner_id": new_owner_id,
        "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Drops original details for an upgraded gift
///
/// # Arguments
///
/// * `received_gift_id` - Identifier of the gift
/// * `star_count` - The Telegram Star amount required to pay for the operation
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn drop_gift_original_details(received_gift_id: String, star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "dropGiftOriginalDetails",
        "received_gift_id": received_gift_id,
        "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends an upgraded gift that is available for resale to another user or channel chat; gifts already owned by the current user
/// must be transferred using transferGift and can't be passed to the method
///
/// # Arguments
///
/// * `gift_name` - Name of the upgraded gift to send
/// * `owner_id` - Identifier of the user or the channel chat that will receive the gift
/// * `price` - The price that the user agreed to pay for the gift
/// * `text` - Text to show along with the gift; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, CustomEmoji, and DateTime entities are allowed.
/// Must be empty if the receiver enabled paid messages and the price of the gift is less than the price of a paid message to the user
/// * `is_private` - Pass true to show gift text and sender only to the gift receiver; otherwise, everyone will be able to see them
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftResaleResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_resold_gift(gift_name: String, owner_id: crate::enums::MessageSender, price: crate::enums::GiftResalePrice, text: crate::types::FormattedText, is_private: bool, client_id: i32) -> Result<crate::enums::GiftResaleResult, crate::types::Error> {
    let request = json!({
        "@type": "sendResoldGift",
        "gift_name": gift_name,
        "owner_id": owner_id,
        "price": price,
        "text": text,
        "is_private": is_private,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends an offer to purchase an upgraded gift
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that currently owns the gift and will receive the offer
/// * `gift_name` - Name of the upgraded gift
/// * `price` - The price that the user agreed to pay for the gift
/// * `duration` - Duration of the offer, in seconds; must be one of 21600, 43200, 86400, 129600, 172800, or 259200. Can also be 120 if Telegram test environment is used
/// * `paid_message_star_count` - The number of Telegram Stars the user agreed to pay additionally for sending of the offer message to the current gift owner; pass userFullInfo.outgoing_paid_message_star_count for users and 0 otherwise
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_gift_purchase_offer(owner_id: crate::enums::MessageSender, gift_name: String, price: crate::enums::GiftResalePrice, duration: i32, paid_message_star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendGiftPurchaseOffer",
        "owner_id": owner_id,
        "gift_name": gift_name,
        "price": price,
        "duration": duration,
        "paid_message_star_count": paid_message_star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Handles a pending gift purchase offer
///
/// # Arguments
///
/// * `message_id` - Identifier of the message with the gift purchase offer
/// * `accept` - Pass true to accept the request; pass false to reject it
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn process_gift_purchase_offer(message_id: i64, accept: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "processGiftPurchaseOffer",
        "message_id": message_id,
        "accept": accept,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns gifts received by the given user or chat
///
/// # Arguments
///
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request; for bots only
/// * `owner_id` - Identifier of the gift receiver
/// * `collection_id` - Pass collection identifier to get gifts only from the specified collection; pass 0 to get gifts regardless of collections
/// * `exclude_unsaved` - Pass true to exclude gifts that aren't saved to the chat's profile page. Always true for gifts received by other users and channel chats without can_post_messages administrator right
/// * `exclude_saved` - Pass true to exclude gifts that are saved to the chat's profile page. Always false for gifts received by other users and channel chats without can_post_messages administrator right
/// * `exclude_unlimited` - Pass true to exclude gifts that can be purchased unlimited number of times
/// * `exclude_upgradable` - Pass true to exclude gifts that can be purchased limited number of times and can be upgraded
/// * `exclude_non_upgradable` - Pass true to exclude gifts that can be purchased limited number of times and can't be upgraded
/// * `exclude_upgraded` - Pass true to exclude upgraded gifts
/// * `exclude_without_colors` - Pass true to exclude gifts that can't be used in setUpgradedGiftColors
/// * `exclude_hosted` - Pass true to exclude gifts that are just hosted and are not owned by the owner
/// * `sort_by_price` - Pass true to sort results by gift price instead of send date
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of gifts to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ReceivedGifts)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_received_gifts(business_connection_id: String, owner_id: crate::enums::MessageSender, collection_id: i32, exclude_unsaved: bool, exclude_saved: bool, exclude_unlimited: bool, exclude_upgradable: bool, exclude_non_upgradable: bool, exclude_upgraded: bool, exclude_without_colors: bool, exclude_hosted: bool, sort_by_price: bool, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::ReceivedGifts, crate::types::Error> {
    let request = json!({
        "@type": "getReceivedGifts",
        "business_connection_id": business_connection_id,
        "owner_id": owner_id,
        "collection_id": collection_id,
        "exclude_unsaved": exclude_unsaved,
        "exclude_saved": exclude_saved,
        "exclude_unlimited": exclude_unlimited,
        "exclude_upgradable": exclude_upgradable,
        "exclude_non_upgradable": exclude_non_upgradable,
        "exclude_upgraded": exclude_upgraded,
        "exclude_without_colors": exclude_without_colors,
        "exclude_hosted": exclude_hosted,
        "sort_by_price": sort_by_price,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a received gift
///
/// # Arguments
///
/// * `received_gift_id` - Identifier of the gift
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ReceivedGift)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_received_gift(received_gift_id: String, client_id: i32) -> Result<crate::enums::ReceivedGift, crate::types::Error> {
    let request = json!({
        "@type": "getReceivedGift",
        "received_gift_id": received_gift_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns upgraded gifts of the current user who can be used to craft another gifts
///
/// # Arguments
///
/// * `regular_gift_id` - Identifier of the regular gift that will be used for crafting
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of gifts to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftsForCrafting)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_gifts_for_crafting(regular_gift_id: i64, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::GiftsForCrafting, crate::types::Error> {
    let request = json!({
        "@type": "getGiftsForCrafting",
        "regular_gift_id": regular_gift_id,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about an upgraded gift by its name
///
/// # Arguments
///
/// * `name` - Unique name of the upgraded gift
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::UpgradedGift)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_upgraded_gift(name: String, client_id: i32) -> Result<crate::enums::UpgradedGift, crate::types::Error> {
    let request = json!({
        "@type": "getUpgradedGift",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about value of an upgraded gift by its name
///
/// # Arguments
///
/// * `name` - Unique name of the upgraded gift
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::UpgradedGiftValueInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_upgraded_gift_value_info(name: String, client_id: i32) -> Result<crate::enums::UpgradedGiftValueInfo, crate::types::Error> {
    let request = json!({
        "@type": "getUpgradedGiftValueInfo",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a URL for upgraded gift withdrawal in the TON blockchain as an NFT; requires owner privileges for gifts owned by a chat
///
/// # Arguments
///
/// * `received_gift_id` - Identifier of the gift
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_upgraded_gift_withdrawal_url(received_gift_id: String, password: String, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getUpgradedGiftWithdrawalUrl",
        "received_gift_id": received_gift_id,
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes resale price of a unique gift owned by the current user
///
/// # Arguments
///
/// * `received_gift_id` - Identifier of the unique gift
/// * `price` - The new price for the unique gift; pass null to disallow gift resale. The current user will receive
/// getOption("gift_resale_star_earnings_per_mille") Telegram Stars for each 1000 Telegram Stars paid for the gift if the gift price is in Telegram Stars or
/// getOption("gift_resale_gram_earnings_per_mille") TON Grams for each 1000 Grams paid for the gift if the gift price is in Grams
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_gift_resale_price(received_gift_id: String, price: Option<crate::enums::GiftResalePrice>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setGiftResalePrice",
        "received_gift_id": received_gift_id,
        "price": price,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns upgraded gifts that can be bought from other owners using sendResoldGift
///
/// # Arguments
///
/// * `gift_id` - Identifier of the regular gift that was upgraded to a unique gift
/// * `order` - Order in which the results will be sorted
/// * `for_crafting` - Pass true to get only gifts suitable for crafting
/// * `for_stars` - Pass true to get only gifts that can be bought using Telegram Stars
/// * `attributes` - Attributes used to filter received gifts. If multiple attributes of the same type are specified, then all of them are allowed.
/// If none attributes of specific type are specified, then all values for this attribute type are allowed
/// * `offset` - Offset of the first entry to return as received from the previous request with the same order and attributes; use empty string to get the first chunk of results
/// * `limit` - The maximum number of gifts to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftsForResale)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_gifts_for_resale(gift_id: i64, order: crate::enums::GiftForResaleOrder, for_crafting: bool, for_stars: bool, attributes: Vec<crate::enums::UpgradedGiftAttributeId>, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::GiftsForResale, crate::types::Error> {
    let request = json!({
        "@type": "searchGiftsForResale",
        "gift_id": gift_id,
        "order": order,
        "for_crafting": for_crafting,
        "for_stars": for_stars,
        "attributes": attributes,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns collections of gifts owned by the given user or chat
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that received the gifts
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftCollections)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_collections(owner_id: crate::enums::MessageSender, client_id: i32) -> Result<crate::enums::GiftCollections, crate::types::Error> {
    let request = json!({
        "@type": "getGiftCollections",
        "owner_id": owner_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Creates a collection from gifts on the current user's or a channel's profile page; requires can_post_messages administrator right in the channel chat.
/// An owner can have up to getOption("gift_collection_count_max") gift collections. The new collection will be added to the end of the gift collection list of the owner. Returns the created collection
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that received the gifts
/// * `name` - Name of the collection; 1-12 characters
/// * `received_gift_ids` - Identifier of the gifts to add to the collection; 0-getOption("gift_collection_size_max") identifiers
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftCollection)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn create_gift_collection(owner_id: crate::enums::MessageSender, name: String, received_gift_ids: Vec<String>, client_id: i32) -> Result<crate::enums::GiftCollection, crate::types::Error> {
    let request = json!({
        "@type": "createGiftCollection",
        "owner_id": owner_id,
        "name": name,
        "received_gift_ids": received_gift_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes order of gift collections. If the collections are owned by a channel chat, then requires can_post_messages administrator right in the channel chat
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_ids` - New order of gift collections
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_gift_collections(owner_id: crate::enums::MessageSender, collection_ids: Vec<i32>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderGiftCollections",
        "owner_id": owner_id,
        "collection_ids": collection_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes a gift collection. If the collection is owned by a channel chat, then requires can_post_messages administrator right in the channel chat
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_id` - Identifier of the gift collection
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_gift_collection(owner_id: crate::enums::MessageSender, collection_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteGiftCollection",
        "owner_id": owner_id,
        "collection_id": collection_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes name of a gift collection. If the collection is owned by a channel chat, then requires can_post_messages administrator right in the channel chat. Returns the changed collection
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_id` - Identifier of the gift collection
/// * `name` - New name of the collection; 1-12 characters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftCollection)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_gift_collection_name(owner_id: crate::enums::MessageSender, collection_id: i32, name: String, client_id: i32) -> Result<crate::enums::GiftCollection, crate::types::Error> {
    let request = json!({
        "@type": "setGiftCollectionName",
        "owner_id": owner_id,
        "collection_id": collection_id,
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds gifts to the beginning of a previously created collection. If the collection is owned by a channel chat, then requires can_post_messages administrator right in the channel chat. Returns the changed collection
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_id` - Identifier of the gift collection
/// * `received_gift_ids` - Identifier of the gifts to add to the collection; 1-getOption("gift_collection_size_max") identifiers.
/// If after addition the collection has more than getOption("gift_collection_size_max") gifts, then the last one are removed from the collection
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftCollection)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_gift_collection_gifts(owner_id: crate::enums::MessageSender, collection_id: i32, received_gift_ids: Vec<String>, client_id: i32) -> Result<crate::enums::GiftCollection, crate::types::Error> {
    let request = json!({
        "@type": "addGiftCollectionGifts",
        "owner_id": owner_id,
        "collection_id": collection_id,
        "received_gift_ids": received_gift_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes gifts from a collection. If the collection is owned by a channel chat, then requires can_post_messages administrator right in the channel chat. Returns the changed collection
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_id` - Identifier of the gift collection
/// * `received_gift_ids` - Identifier of the gifts to remove from the collection
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftCollection)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_gift_collection_gifts(owner_id: crate::enums::MessageSender, collection_id: i32, received_gift_ids: Vec<String>, client_id: i32) -> Result<crate::enums::GiftCollection, crate::types::Error> {
    let request = json!({
        "@type": "removeGiftCollectionGifts",
        "owner_id": owner_id,
        "collection_id": collection_id,
        "received_gift_ids": received_gift_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes order of gifts in a collection. If the collection is owned by a channel chat, then requires can_post_messages administrator right in the channel chat. Returns the changed collection
///
/// # Arguments
///
/// * `owner_id` - Identifier of the user or the channel chat that owns the collection
/// * `collection_id` - Identifier of the gift collection
/// * `received_gift_ids` - Identifier of the gifts to move to the beginning of the collection. All other gifts are placed in the current order after the specified gifts
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::GiftCollection)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reorder_gift_collection_gifts(owner_id: crate::enums::MessageSender, collection_id: i32, received_gift_ids: Vec<String>, client_id: i32) -> Result<crate::enums::GiftCollection, crate::types::Error> {
    let request = json!({
        "@type": "reorderGiftCollectionGifts",
        "owner_id": owner_id,
        "collection_id": collection_id,
        "received_gift_ids": received_gift_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Constructs a persistent HTTP URL for a background
///
/// # Arguments
///
/// * `name` - Background name
/// * `r#type` - Background type; backgroundTypeChatTheme isn't supported
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_background_url(name: String, r#type: crate::enums::BackgroundType, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getBackgroundUrl",
        "name": name,
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Searches for a background by its name
///
/// # Arguments
///
/// * `name` - The name of the background
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Background)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_background(name: String, client_id: i32) -> Result<crate::enums::Background, crate::types::Error> {
    let request = json!({
        "@type": "searchBackground",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets default background for chats; adds the background to the list of installed backgrounds
///
/// # Arguments
///
/// * `background` - The input background to use; pass null to create a new filled background
/// * `r#type` - Background type; pass null to use the default type of the remote background; backgroundTypeChatTheme isn't supported
/// * `for_dark_theme` - Pass true if the background is set for a dark theme
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Background)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_default_background(background: Option<crate::enums::InputBackground>, r#type: Option<crate::enums::BackgroundType>, for_dark_theme: bool, client_id: i32) -> Result<crate::enums::Background, crate::types::Error> {
    let request = json!({
        "@type": "setDefaultBackground",
        "background": background,
        "type": r#type,
        "for_dark_theme": for_dark_theme,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes default background for chats
///
/// # Arguments
///
/// * `for_dark_theme` - Pass true if the background is deleted for a dark theme
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_default_background(for_dark_theme: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteDefaultBackground",
        "for_dark_theme": for_dark_theme,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns backgrounds installed by the user
///
/// # Arguments
///
/// * `for_dark_theme` - Pass true to order returned backgrounds for a dark theme
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Backgrounds)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_installed_backgrounds(for_dark_theme: bool, client_id: i32) -> Result<crate::enums::Backgrounds, crate::types::Error> {
    let request = json!({
        "@type": "getInstalledBackgrounds",
        "for_dark_theme": for_dark_theme,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Removes background from the list of installed backgrounds
///
/// # Arguments
///
/// * `background_id` - The background identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_installed_background(background_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeInstalledBackground",
        "background_id": background_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Resets list of installed backgrounds to its default value
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reset_installed_backgrounds(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "resetInstalledBackgrounds",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about the current localization target. This is an offline method if only_local is true. Can be called before authorization
///
/// # Arguments
///
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LocalizationTargetInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_localization_target_info(only_local: bool, client_id: i32) -> Result<crate::enums::LocalizationTargetInfo, crate::types::Error> {
    let request = json!({
        "@type": "getLocalizationTargetInfo",
        "only_local": only_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization
///
/// # Arguments
///
/// * `language_pack_id` - Language pack identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LanguagePackInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_language_pack_info(language_pack_id: String, client_id: i32) -> Result<crate::enums::LanguagePackInfo, crate::types::Error> {
    let request = json!({
        "@type": "getLanguagePackInfo",
        "language_pack_id": language_pack_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns strings from a language pack in the current localization target by their keys. Can be called before authorization
///
/// # Arguments
///
/// * `language_pack_id` - Language pack identifier of the strings to be returned
/// * `keys` - Language pack keys of the strings to be returned; leave empty to request all available strings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LanguagePackStrings)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_language_pack_strings(language_pack_id: String, keys: Vec<String>, client_id: i32) -> Result<crate::enums::LanguagePackStrings, crate::types::Error> {
    let request = json!({
        "@type": "getLanguagePackStrings",
        "language_pack_id": language_pack_id,
        "keys": keys,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Fetches the latest versions of all strings from a language pack in the current localization target from the server.
/// This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization
///
/// # Arguments
///
/// * `language_pack_id` - Language pack identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn synchronize_language_pack(language_pack_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "synchronizeLanguagePack",
        "language_pack_id": language_pack_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization
///
/// # Arguments
///
/// * `language_pack_id` - Identifier of a language pack to be added
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_custom_server_language_pack(language_pack_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addCustomServerLanguagePack",
        "language_pack_id": language_pack_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds or changes a custom local language pack to the current localization target
///
/// # Arguments
///
/// * `info` - Information about the language pack. Language pack identifier must start with 'X', consist only of English letters, digits and hyphens, and must not exceed 64 characters. Can be called before authorization
/// * `strings` - Strings of the new language pack
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_custom_language_pack(info: crate::types::LanguagePackInfo, strings: Vec<crate::types::LanguagePackString>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setCustomLanguagePack",
        "info": info,
        "strings": strings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Edits information about a custom local language pack in the current localization target. Can be called before authorization
///
/// # Arguments
///
/// * `info` - New information about the custom local language pack
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_custom_language_pack_info(info: crate::types::LanguagePackInfo, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editCustomLanguagePackInfo",
        "info": info,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds, edits or deletes a string in a custom local language pack. Can be called before authorization
///
/// # Arguments
///
/// * `language_pack_id` - Identifier of a previously added custom local language pack in the current localization target
/// * `new_string` - New language pack string
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_custom_language_pack_string(language_pack_id: String, new_string: crate::types::LanguagePackString, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setCustomLanguagePackString",
        "language_pack_id": language_pack_id,
        "new_string": new_string,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted.
/// Can be called before authorization
///
/// # Arguments
///
/// * `language_pack_id` - Identifier of the language pack to delete
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_language_pack(language_pack_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteLanguagePack",
        "language_pack_id": language_pack_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription
///
/// # Arguments
///
/// * `device_token` - Device token
/// * `other_user_ids` - List of user identifiers of other users currently using the application
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PushReceiverId)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn register_device(device_token: crate::enums::DeviceToken, other_user_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::PushReceiverId, crate::types::Error> {
    let request = json!({
        "@type": "registerDevice",
        "device_token": device_token,
        "other_user_ids": other_user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns t.me URLs recently visited by a newly registered user
///
/// # Arguments
///
/// * `referrer` - Google Play referrer to identify the user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TmeUrls)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_recently_visited_t_me_urls(referrer: String, client_id: i32) -> Result<crate::enums::TmeUrls, crate::types::Error> {
    let request = json!({
        "@type": "getRecentlyVisitedTMeUrls",
        "referrer": referrer,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Changes privacy settings for message read date
///
/// # Arguments
///
/// * `settings` - New settings
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_read_date_privacy_settings(settings: crate::types::ReadDatePrivacySettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setReadDatePrivacySettings",
        "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns privacy settings for message read date
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::ReadDatePrivacySettings)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_read_date_privacy_settings(client_id: i32) -> Result<crate::enums::ReadDatePrivacySettings, crate::types::Error> {
    let request = json!({
        "@type": "getReadDatePrivacySettings",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the value of an option by its name. (Check the list of available options on https:core.telegram.org/tdlib/options.) Can be called before authorization. Can be called synchronously for options "version" and "commit_hash"
///
/// # Arguments
///
/// * `name` - The name of the option
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::OptionValue)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_option(name: String, client_id: i32) -> Result<crate::enums::OptionValue, crate::types::Error> {
    let request = json!({
        "@type": "getOption",
        "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the value of an option. (Check the list of available options on https:core.telegram.org/tdlib/options.) Only writable options can be set. Can be called before authorization
///
/// # Arguments
///
/// * `name` - The name of the option
/// * `value` - The new value of the option; pass null to reset option value to a default value
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_option(name: String, value: Option<crate::enums::OptionValue>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setOption",
        "name": name,
        "value": value,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes the period of inactivity after which the account of the current user will automatically be deleted
///
/// # Arguments
///
/// * `ttl` - New account TTL
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_account_ttl(ttl: crate::types::AccountTtl, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAccountTtl",
        "ttl": ttl,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the period of inactivity after which the account of the current user will automatically be deleted
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AccountTtl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_account_ttl(client_id: i32) -> Result<crate::enums::AccountTtl, crate::types::Error> {
    let request = json!({
        "@type": "getAccountTtl",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account.
/// Can be called before authorization when the current authorization state is authorizationStateWaitPassword
///
/// # Arguments
///
/// * `reason` - The reason why the account was deleted; optional
/// * `password` - The 2-step verification password of the current user. If the current user isn't authorized, then an empty string can be passed and account deletion can be canceled within one week
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn delete_account(reason: String, password: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteAccount",
        "reason": reason,
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of TON blockchain transactions of the current user
///
/// # Arguments
///
/// * `direction` - Direction of the transactions to receive; pass null to get all transactions
/// * `offset` - Offset of the first transaction to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of transactions to return
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TonTransactions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_ton_transactions(direction: Option<crate::enums::TransactionDirection>, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::TonTransactions, crate::types::Error> {
    let request = json!({
        "@type": "getTonTransactions",
        "direction": direction,
        "offset": offset,
        "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns a URL for TON Gram withdrawal from the current user's account. The user must have at least 10 Grams to withdraw
/// and can withdraw up to 100000 Grams in one transaction
///
/// # Arguments
///
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_gram_withdrawal_url(password: String, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getGramWithdrawalUrl",
        "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Loads an asynchronous or a zoomed in statistical graph
///
/// # Arguments
///
/// * `chat_id` - Chat identifier
/// * `token` - The token for graph loading
/// * `x` - X-value for zoomed in graph or 0 otherwise
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StatisticalGraph)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_statistical_graph(chat_id: i64, token: String, x: i64, client_id: i32) -> Result<crate::enums::StatisticalGraph, crate::types::Error> {
    let request = json!({
        "@type": "getStatisticalGraph",
        "chat_id": chat_id,
        "token": token,
        "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns storage usage statistics. Can be called before authorization
///
/// # Arguments
///
/// * `chat_limit` - The maximum number of chats with the largest storage usage for which separate statistics need to be returned. All other chats will be grouped in entries with chat_id == 0. If the chat info database is not used, the chat_limit is ignored and is always set to 0
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StorageStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_storage_statistics(chat_limit: i32, client_id: i32) -> Result<crate::enums::StorageStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getStorageStatistics",
        "chat_limit": chat_limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Quickly returns approximate storage usage statistics. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StorageStatisticsFast)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_storage_statistics_fast(client_id: i32) -> Result<crate::enums::StorageStatisticsFast, crate::types::Error> {
    let request = json!({
        "@type": "getStorageStatisticsFast",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns database statistics
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::DatabaseStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_database_statistics(client_id: i32) -> Result<crate::enums::DatabaseStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getDatabaseStatistics",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted
///
/// # Arguments
///
/// * `size` - Limit on the total size of files after deletion, in bytes. Pass -1 to use the default limit
/// * `ttl` - Limit on the time that has passed since the last time a file was accessed (or creation time for some filesystems). Pass -1 to use the default limit
/// * `count` - Limit on the total number of files after deletion. Pass -1 to use the default limit
/// * `immunity_delay` - The amount of time after the creation of a file during which it can't be deleted, in seconds. Pass -1 to use the default value
/// * `file_types` - If non-empty, only files with the given types are considered. By default, all types except thumbnails, profile photos, stickers and wallpapers are deleted
/// * `chat_ids` - If non-empty, only files from the given chats are considered. Use 0 as chat identifier to delete files not belonging to any chat (e.g., profile photos)
/// * `exclude_chat_ids` - If non-empty, files from the given chats are excluded. Use 0 as chat identifier to exclude all files not belonging to any chat (e.g., profile photos)
/// * `return_deleted_file_statistics` - Pass true if statistics about the files that were deleted must be returned instead of the whole storage usage statistics. Affects only returned statistics
/// * `chat_limit` - Same as in getStorageStatistics. Affects only returned statistics
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::StorageStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn optimize_storage(size: i64, ttl: i32, count: i32, immunity_delay: i32, file_types: Vec<crate::enums::FileType>, chat_ids: Vec<i64>, exclude_chat_ids: Vec<i64>, return_deleted_file_statistics: bool, chat_limit: i32, client_id: i32) -> Result<crate::enums::StorageStatistics, crate::types::Error> {
    let request = json!({
        "@type": "optimizeStorage",
        "size": size,
        "ttl": ttl,
        "count": count,
        "immunity_delay": immunity_delay,
        "file_types": file_types,
        "chat_ids": chat_ids,
        "exclude_chat_ids": exclude_chat_ids,
        "return_deleted_file_statistics": return_deleted_file_statistics,
        "chat_limit": chat_limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks,
/// so it must be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics
///
/// # Arguments
///
/// * `r#type` - The new network type; pass null to set network type to networkTypeOther
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_network_type(r#type: Option<crate::enums::NetworkType>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setNetworkType",
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns network data usage statistics. Can be called before authorization
///
/// # Arguments
///
/// * `only_current` - Pass true to get statistics only for the current library launch
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::NetworkStatistics)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_network_statistics(only_current: bool, client_id: i32) -> Result<crate::enums::NetworkStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getNetworkStatistics",
        "only_current": only_current,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds the specified data to data usage statistics. Can be called before authorization
///
/// # Arguments
///
/// * `entry` - The network statistics entry with the data to be added to statistics
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_network_statistics(entry: crate::enums::NetworkStatisticsEntry, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addNetworkStatistics",
        "entry": entry,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Resets all network data usage statistics to zero. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn reset_network_statistics(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "resetNetworkStatistics",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns auto-download settings presets for the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AutoDownloadSettingsPresets)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_auto_download_settings_presets(client_id: i32) -> Result<crate::enums::AutoDownloadSettingsPresets, crate::types::Error> {
    let request = json!({
        "@type": "getAutoDownloadSettingsPresets",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets auto-download settings
///
/// # Arguments
///
/// * `settings` - New user auto-download settings
/// * `r#type` - Type of the network for which the new settings are relevant
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_auto_download_settings(settings: crate::types::AutoDownloadSettings, r#type: crate::enums::NetworkType, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAutoDownloadSettings",
        "settings": settings,
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns autosave settings for the current user
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AutosaveSettings)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_autosave_settings(client_id: i32) -> Result<crate::enums::AutosaveSettings, crate::types::Error> {
    let request = json!({
        "@type": "getAutosaveSettings",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets autosave settings for the given scope. The method is guaranteed to work only after at least one call to getAutosaveSettings
///
/// # Arguments
///
/// * `scope` - Autosave settings scope
/// * `settings` - New autosave settings for the scope; pass null to set autosave settings to default
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_autosave_settings(scope: crate::enums::AutosaveSettingsScope, settings: Option<crate::types::ScopeAutosaveSettings>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAutosaveSettings",
        "scope": scope,
        "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Clears the list of all autosave settings exceptions. The method is guaranteed to work only after at least one call to getAutosaveSettings
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn clear_autosave_settings_exceptions(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clearAutosaveSettingsExceptions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Changes web browser settings
///
/// # Arguments
///
/// * `open_external_browser` - Pass true if links must be opened in an external browser by default
/// * `display_close_button` - Pass true if a close button must be shown in the in-app browser; for Android app only
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn change_web_browser_settings(open_external_browser: bool, display_close_button: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "changeWebBrowserSettings",
        "open_external_browser": open_external_browser,
        "display_close_button": display_close_button,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Adds a special handling for the opening of the specified URL
///
/// # Arguments
///
/// * `open_external_browser` - Pass true if the specified website must be opened in an external browser; pass false to open it in the in-app browser. There can be at most 100 exceptions in each list of the exceptions
/// * `url` - URL of the website
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_web_browser_settings_exception(open_external_browser: bool, url: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addWebBrowserSettingsException",
        "open_external_browser": open_external_browser,
        "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes a special handling for the opening of the specified URL
///
/// # Arguments
///
/// * `url` - URL of the website
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_web_browser_settings_exception(url: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeWebBrowserSettingsException",
        "url": url,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes special handling for the opening of all links
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_all_web_browser_settings_exceptions(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeAllWebBrowserSettingsExceptions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns an IETF language tag of the language preferred in the country, which must be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown
///
/// # Arguments
///
/// * `country_code` - A two-letter ISO 3166-1 alpha-2 country code
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_preferred_country_language(country_code: String, client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getPreferredCountryLanguage",
        "country_code": country_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends a code to verify an email address to be added to a user's Telegram Passport
///
/// # Arguments
///
/// * `email_address` - Email address
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmailAddressAuthenticationCodeInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_email_address_verification_code(email_address: String, client_id: i32) -> Result<crate::enums::EmailAddressAuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "sendEmailAddressVerificationCode",
        "email_address": email_address,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Resends the code to verify an email address to be added to a user's Telegram Passport
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::EmailAddressAuthenticationCodeInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn resend_email_address_verification_code(client_id: i32) -> Result<crate::enums::EmailAddressAuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "resendEmailAddressVerificationCode",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Checks the email address verification code for Telegram Passport
///
/// # Arguments
///
/// * `code` - Verification code to check
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn check_email_address_verification_code(code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkEmailAddressVerificationCode",
        "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Checks whether an in-store purchase is possible. Must be called before any in-store purchase. For official applications only
///
/// # Arguments
///
/// * `purpose` - Transaction purpose
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn can_purchase_from_store(purpose: crate::enums::StorePaymentPurpose, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "canPurchaseFromStore",
        "purpose": purpose,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Informs server about an in-store purchase. For official applications only
///
/// # Arguments
///
/// * `transaction` - Information about the transaction
/// * `purpose` - Transaction purpose
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn assign_store_transaction(transaction: crate::enums::StoreTransaction, purpose: crate::enums::StorePaymentPurpose, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "assignStoreTransaction",
        "transaction": transaction,
        "purpose": purpose,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Accepts Telegram terms of service
///
/// # Arguments
///
/// * `terms_of_service_id` - Terms of service identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn accept_terms_of_service(terms_of_service_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "acceptTermsOfService",
        "terms_of_service_id": terms_of_service_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Searches specified query by word prefixes in the provided strings. Returns 0-based positions of strings that matched. Can be called synchronously
///
/// # Arguments
///
/// * `strings` - The strings to search in for the query
/// * `query` - Query to search for
/// * `limit` - The maximum number of objects to return
/// * `return_none_for_empty_query` - Pass true to receive no results for an empty query
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::FoundPositions)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn search_strings_by_prefix(strings: Vec<String>, query: String, limit: i32, return_none_for_empty_query: bool, client_id: i32) -> Result<crate::enums::FoundPositions, crate::types::Error> {
    let request = json!({
        "@type": "searchStringsByPrefix",
        "strings": strings,
        "query": query,
        "limit": limit,
        "return_none_for_empty_query": return_none_for_empty_query,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends a custom request; for bots only
///
/// # Arguments
///
/// * `method` - The method name
/// * `parameters` - JSON-serialized method parameters
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CustomRequestResult)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn send_custom_request(method: String, parameters: String, client_id: i32) -> Result<crate::enums::CustomRequestResult, crate::types::Error> {
    let request = json!({
        "@type": "sendCustomRequest",
        "method": method,
        "parameters": parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Answers a custom query; for bots only
///
/// # Arguments
///
/// * `custom_query_id` - Identifier of a custom query
/// * `data` - JSON-serialized answer to the query
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn answer_custom_query(custom_query_id: i64, data: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "answerCustomQuery",
        "custom_query_id": custom_query_id,
        "data": data,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Succeeds after a specified amount of time has passed. Can be called before initialization
///
/// # Arguments
///
/// * `seconds` - Number of seconds before the function returns
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_alarm(seconds: f64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setAlarm",
        "seconds": seconds,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about existing countries. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Countries)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_countries(client_id: i32) -> Result<crate::enums::Countries, crate::types::Error> {
    let request = json!({
        "@type": "getCountries",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about an existing country. Can be called before authorization
///
/// # Arguments
///
/// * `country_code` - A two-letter ISO 3166-1 alpha-2 country code
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CountryInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_country(country_code: String, client_id: i32) -> Result<crate::enums::CountryInfo, crate::types::Error> {
    let request = json!({
        "@type": "getCountry",
        "country_code": country_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Uses the current IP address to find the current country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_country_code(client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getCountryCode",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a phone number by its prefix. Can be called before authorization
///
/// # Arguments
///
/// * `phone_number_prefix` - The phone number prefix
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PhoneNumberInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_phone_number_info(phone_number_prefix: String, client_id: i32) -> Result<crate::enums::PhoneNumberInfo, crate::types::Error> {
    let request = json!({
        "@type": "getPhoneNumberInfo",
        "phone_number_prefix": phone_number_prefix,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a phone number by its prefix synchronously. getCountries must be called at least once after changing localization to the specified language if properly localized country information is expected. Can be called synchronously
///
/// # Arguments
///
/// * `language_code` - A two-letter ISO 639-1 language code for country information localization
/// * `phone_number_prefix` - The phone number prefix
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::PhoneNumberInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_phone_number_info_sync(language_code: String, phone_number_prefix: String, client_id: i32) -> Result<crate::enums::PhoneNumberInfo, crate::types::Error> {
    let request = json!({
        "@type": "getPhoneNumberInfoSync",
        "language_code": language_code,
        "phone_number_prefix": phone_number_prefix,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a given collectible item that was purchased at https:fragment.com
///
/// # Arguments
///
/// * `r#type` - Type of the collectible item. The item must be used by a user and must be visible to the current user
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::CollectibleItemInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_collectible_item_info(r#type: crate::enums::CollectibleItemType, client_id: i32) -> Result<crate::enums::CollectibleItemInfo, crate::types::Error> {
    let request = json!({
        "@type": "getCollectibleItemInfo",
        "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns information about a tg: deep link. Use "tg:need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization
///
/// # Arguments
///
/// * `link` - The link
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::DeepLinkInfo)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_deep_link_info(link: String, client_id: i32) -> Result<crate::enums::DeepLinkInfo, crate::types::Error> {
    let request = json!({
        "@type": "getDeepLinkInfo",
        "link": link,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns application config, provided by the server. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::JsonValue)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_application_config(client_id: i32) -> Result<crate::enums::JsonValue, crate::types::Error> {
    let request = json!({
        "@type": "getApplicationConfig",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Saves application log event on the server. Can be called before authorization
///
/// # Arguments
///
/// * `r#type` - Event type
/// * `chat_id` - Optional chat identifier, associated with the event
/// * `data` - The log event data
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn save_application_log_event(r#type: String, chat_id: i64, data: crate::enums::JsonValue, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "saveApplicationLogEvent",
        "type": r#type,
        "chat_id": chat_id,
        "data": data,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the link for downloading official Telegram application to be used when the current user invites friends to Telegram
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::HttpUrl)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_application_download_link(client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getApplicationDownloadLink",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Adds a proxy server for network requests. Can be called before authorization
///
/// # Arguments
///
/// * `proxy` - The proxy to add
/// * `enable` - Pass true to immediately enable the proxy
/// * `comment` - Comment to set for the proxy
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AddedProxy)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn add_proxy(proxy: crate::types::Proxy, enable: bool, comment: String, client_id: i32) -> Result<crate::enums::AddedProxy, crate::types::Error> {
    let request = json!({
        "@type": "addProxy",
        "proxy": proxy,
        "enable": enable,
        "comment": comment,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Edits an existing proxy server for network requests. Can be called before authorization
///
/// # Arguments
///
/// * `proxy_id` - Proxy identifier
/// * `proxy` - The new information about the proxy
/// * `enable` - Pass true to immediately enable the proxy
/// * `comment` - New comment for the proxy
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AddedProxy)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn edit_proxy(proxy_id: i32, proxy: crate::types::Proxy, enable: bool, comment: String, client_id: i32) -> Result<crate::enums::AddedProxy, crate::types::Error> {
    let request = json!({
        "@type": "editProxy",
        "proxy_id": proxy_id,
        "proxy": proxy,
        "enable": enable,
        "comment": comment,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization
///
/// # Arguments
///
/// * `proxy_id` - Proxy identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn enable_proxy(proxy_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "enableProxy",
        "proxy_id": proxy_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Disables the currently enabled proxy. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn disable_proxy(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "disableProxy",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Removes a proxy server. Can be called before authorization
///
/// # Arguments
///
/// * `proxy_id` - Proxy identifier
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn remove_proxy(proxy_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removeProxy",
        "proxy_id": proxy_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns the list of proxies that are currently set up. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::AddedProxies)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_proxies(client_id: i32) -> Result<crate::enums::AddedProxies, crate::types::Error> {
    let request = json!({
        "@type": "getProxies",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization
///
/// # Arguments
///
/// * `proxy` - The proxy to test; pass null to ping a Telegram server without a proxy
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Seconds)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn ping_proxy(proxy: Option<crate::types::Proxy>, client_id: i32) -> Result<crate::enums::Seconds, crate::types::Error> {
    let request = json!({
        "@type": "pingProxy",
        "proxy": proxy,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets new log stream for internal logging of TDLib. Can be called synchronously
///
/// # Arguments
///
/// * `log_stream` - New log stream
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_log_stream(log_stream: crate::enums::LogStream, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setLogStream",
        "log_stream": log_stream,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns information about currently used log stream for internal logging of TDLib. Can be called synchronously
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LogStream)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_log_stream(client_id: i32) -> Result<crate::enums::LogStream, crate::types::Error> {
    let request = json!({
        "@type": "getLogStream",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the verbosity level of the internal logging of TDLib. Can be called synchronously
///
/// # Arguments
///
/// * `new_verbosity_level` - New value of the verbosity level for logging. Value 0 corresponds to fatal errors, value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings,
/// value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds to verbose debug, value greater than 5 and up to 1023 can be used to enable even more logging
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_log_verbosity_level(new_verbosity_level: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setLogVerbosityLevel",
        "new_verbosity_level": new_verbosity_level,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns current verbosity level of the internal logging of TDLib. Can be called synchronously
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LogVerbosityLevel)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_log_verbosity_level(client_id: i32) -> Result<crate::enums::LogVerbosityLevel, crate::types::Error> {
    let request = json!({
        "@type": "getLogVerbosityLevel",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the list of available TDLib internal log tags, for example, ["actor", "binlog", "connections", "notifications", "proxy"]. Can be called synchronously
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LogTags)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_log_tags(client_id: i32) -> Result<crate::enums::LogTags, crate::types::Error> {
    let request = json!({
        "@type": "getLogTags",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sets the verbosity level for a specified TDLib internal log tag. Can be called synchronously
///
/// # Arguments
///
/// * `tag` - Logging tag to change verbosity level
/// * `new_verbosity_level` - New verbosity level; 1-1024
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn set_log_tag_verbosity_level(tag: String, new_verbosity_level: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setLogTagVerbosityLevel",
        "tag": tag,
        "new_verbosity_level": new_verbosity_level,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Returns current verbosity level for a specified TDLib internal log tag. Can be called synchronously
///
/// # Arguments
///
/// * `tag` - Logging tag to change verbosity level
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::LogVerbosityLevel)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_log_tag_verbosity_level(tag: String, client_id: i32) -> Result<crate::enums::LogVerbosityLevel, crate::types::Error> {
    let request = json!({
        "@type": "getLogTagVerbosityLevel",
        "tag": tag,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns localized name of the Telegram support user; for Telegram support only
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Text)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn get_support_name(client_id: i32) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
        "@type": "getSupportName",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the squared received number; for testing only. This is an offline method. Can be called before authorization
///
/// # Arguments
///
/// * `x` - Number to square
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::TestInt)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_square_int(x: i32, client_id: i32) -> Result<crate::enums::TestInt, crate::types::Error> {
    let request = json!({
        "@type": "testSquareInt",
        "x": x,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_network(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "testNetwork",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Sends a simple network request to the Telegram servers via proxy; for testing only. Can be called before authorization
///
/// # Arguments
///
/// * `proxy` - The proxy to test
/// * `dc_id` - Identifier of a datacenter with which to test connection
/// * `timeout` - The maximum overall timeout for the request
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_proxy(proxy: crate::types::Proxy, dc_id: i32, timeout: f64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "testProxy",
        "proxy": proxy,
        "dc_id": dc_id,
        "timeout": timeout,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Forces an updates.getDifference call to the Telegram servers; for testing only
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(())` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_get_difference(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "testGetDifference",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}

/// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
///
/// # Arguments
///
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Update)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_use_update(client_id: i32) -> Result<crate::enums::Update, crate::types::Error> {
    let request = json!({
        "@type": "testUseUpdate",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

/// Returns the specified error and ensures that the Error object is used; for testing only. Can be called synchronously
///
/// # Arguments
///
/// * `error` - The error to be returned
/// * `client_id` - The numeric client identifier to send the request to.
///
/// # Returns
///
/// * `Ok(crate::enums::Error)` - On success.
/// * `Err(crate::types::Error)` - On failure, returns a TDLib error.
#[allow(clippy::too_many_arguments)]
pub async fn test_return_error(error: crate::types::Error, client_id: i32) -> Result<crate::enums::Error, crate::types::Error> {
    let request = json!({
        "@type": "testReturnError",
        "error": error,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}

