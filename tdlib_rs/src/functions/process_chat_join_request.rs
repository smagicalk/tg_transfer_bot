use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Handles a pending join request in a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `user_id` - Identifier of the user who sent the request
/// * `approve` - Pass true to approve the request; pass false to decline it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn process_chat_join_request(
    chat_id: i64,
    user_id: i64,
    approve: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "processChatJoinRequest",
    "chat_id": chat_id,
    "user_id": user_id,
    "approve": approve,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
