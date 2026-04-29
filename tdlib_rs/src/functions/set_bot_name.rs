#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets the name of a bot. Can be called only if userTypeBot.can_be_edited == true
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `language_code` - A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose languages there is no dedicated name
/// * `name` - New bot's name on the specified language; 0-64 characters; must be non-empty if language code is empty
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_bot_name(bot_user_id: i64, language_code: String, name: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBotName",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "name": name,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
