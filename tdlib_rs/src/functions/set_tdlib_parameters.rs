use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
/// # Arguments
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
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_tdlib_parameters(
    use_test_dc: bool,
    database_directory: String,
    files_directory: String,
    database_encryption_key: String,
    use_file_database: bool,
    use_chat_info_database: bool,
    use_message_database: bool,
    use_secret_chats: bool,
    api_id: i32,
    api_hash: String,
    system_language_code: String,
    device_model: String,
    system_version: String,
    application_version: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
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
