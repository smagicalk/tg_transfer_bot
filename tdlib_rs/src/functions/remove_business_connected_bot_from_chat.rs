use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes the connected business bot from a specific chat by adding the chat to businessRecipients.excluded_chat_ids
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_business_connected_bot_from_chat(
    chat_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeBusinessConnectedBotFromChat",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
