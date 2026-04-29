#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes active state for a username of a bot. The editable username can be disabled only if there are other active usernames.
/// May return an error with a message "USERNAMES_ACTIVE_TOO_MUCH" if the maximum number of active usernames has been reached. Can be called only if userTypeBot.can_be_edited == true
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `username` - The username to change
/// * `is_active` - Pass true to activate the username; pass false to disable it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_bot_username_is_active(bot_user_id: i64, username: String, is_active: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleBotUsernameIsActive",
        "bot_user_id": bot_user_id,
        "username": username,
        "is_active": is_active,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
