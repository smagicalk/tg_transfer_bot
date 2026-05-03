use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Pauses or resumes the connected business bot in a specific chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `is_paused` - Pass true to pause the connected bot in the chat; pass false to resume the bot
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_business_connected_bot_chat_is_paused(
    chat_id: i64,
    is_paused: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleBusinessConnectedBotChatIsPaused",
    "chat_id": chat_id,
    "is_paused": is_paused,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
