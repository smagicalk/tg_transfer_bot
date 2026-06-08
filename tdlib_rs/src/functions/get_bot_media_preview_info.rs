use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of media previews for the given language and the list of languages for which the bot has dedicated previews
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot. The bot must be owned and must have the main Web App
/// * `language_code` - A two-letter ISO 639-1 language code for which to get previews. If empty, then default previews are returned
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_media_preview_info(
    bot_user_id: i64,
    language_code: String,
    client_id: i32,
) -> Result<crate::enums::BotMediaPreviewInfo, crate::types::Error> {
    let request = json!({
    "@type": "getBotMediaPreviewInfo",
    "bot_user_id": bot_user_id,
    "language_code": language_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
