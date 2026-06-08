use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Adds a new media preview to the beginning of the list of media previews of a bot. Returns the added preview after addition is completed server-side. The total number of previews must not exceed getOption("bot_media_preview_count_max") for the given language
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot. The bot must be owned and must have the main Web App
/// * `language_code` - A two-letter ISO 639-1 language code for which preview is added. If empty, then the preview will be shown to all users for whose languages there are no dedicated previews.
/// If non-empty, then there must be an official language pack of the same name, which is returned by getLocalizationTargetInfo
/// * `content` - Content of the added preview
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_bot_media_preview(
    bot_user_id: i64,
    language_code: String,
    content: crate::enums::InputStoryContent,
    client_id: i32,
) -> Result<crate::enums::BotMediaPreview, crate::types::Error> {
    let request = json!({
    "@type": "addBotMediaPreview",
    "bot_user_id": bot_user_id,
    "language_code": language_code,
    "content": content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
