use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns an HTTPS link to boost the specified supergroup or channel chat
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_boost_link(
    chat_id: i64,
    client_id: i32,
) -> Result<crate::enums::ChatBoostLink, crate::types::Error> {
    let request = json!({
    "@type": "getChatBoostLink",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
