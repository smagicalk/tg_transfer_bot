#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sets the text shown in the chat with a bot if the chat is empty. Can be called only if userTypeBot.can_be_edited == true
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot
/// * `language_code` - A two-letter ISO 639-1 language code. If empty, the description will be shown to all users for whose languages there is no dedicated description
/// * `description` - New bot's description on the specified language
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_bot_info_description(bot_user_id: i64, language_code: String, description: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setBotInfoDescription",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "description": description,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
