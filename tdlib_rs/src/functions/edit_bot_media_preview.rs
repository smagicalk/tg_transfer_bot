#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Replaces media preview in the list of media previews of a bot. Returns the new preview after edit is completed server-side
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot. The bot must be owned and must have the main Web App
/// * `language_code` - Language code of the media preview to edit
/// * `file_id` - File identifier of the media to replace
/// * `content` - Content of the new preview
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_bot_media_preview(bot_user_id: i64, language_code: String, file_id: i32, content: crate::enums::InputStoryContent, client_id: i32) -> Result<crate::enums::BotMediaPreview, crate::types::Error> {
    let request = json!({
        "@type": "editBotMediaPreview",
        "bot_user_id": bot_user_id,
        "language_code": language_code,
        "file_id": file_id,
        "content": content,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
