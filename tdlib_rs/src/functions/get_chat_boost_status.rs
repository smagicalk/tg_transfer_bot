use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the current boost status for a supergroup or a channel chat
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_status(
    chat_id: i64,
    client_id: i32,
) -> Result<crate::enums::ChatBoostStatus, crate::types::Error> {
    let request = json!({
    "@type": "getChatBoostStatus",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
