use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Informs TDLib that a message with an animated emoji was clicked by the user. Returns a big animated sticker to be played or a 404 error if usual animation needs to be played
/// # Arguments
/// * `chat_id` - Chat identifier of the message
/// * `message_id` - Identifier of the clicked message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn click_animated_emoji_message(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<crate::enums::Sticker, crate::types::Error> {
    let request = json!({
    "@type": "clickAnimatedEmojiMessage",
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
