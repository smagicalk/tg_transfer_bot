use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of boosts applied to a chat by a given user; requires administrator rights in the chat; for bots only
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `user_id` - Identifier of the user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_user_chat_boosts(
    chat_id: i64,
    user_id: i64,
    client_id: i32,
) -> Result<crate::enums::FoundChatBoosts, crate::types::Error> {
    let request = json!({
    "@type": "getUserChatBoosts",
    "chat_id": chat_id,
    "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
