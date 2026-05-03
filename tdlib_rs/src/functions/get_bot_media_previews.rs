use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of media previews of a bot
/// # Arguments
/// * `bot_user_id` - Identifier of the target bot. The bot must have the main Web App
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_bot_media_previews(
    bot_user_id: i64,
    client_id: i32,
) -> Result<crate::enums::BotMediaPreviews, crate::types::Error> {
    let request = json!({
    "@type": "getBotMediaPreviews",
    "bot_user_id": bot_user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
