use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes media previews from the list of media previews of a bot
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot. The bot must be owned and must have the main Web App
/// * `language_code` - Language code of the media previews to delete
/// * `file_ids` - File identifiers of the media to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_bot_media_previews(
    bot_user_id: i64,
    language_code: String,
    file_ids: Vec<i32>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteBotMediaPreviews",
    "bot_user_id": bot_user_id,
    "language_code": language_code,
    "file_ids": file_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
