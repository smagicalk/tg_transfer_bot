use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a list of common group chats with a given user. Chats are sorted by their type and creation date
/// # Arguments
/// * `user_id` - User identifier
/// * `offset_chat_id` - Chat identifier starting from which to return chats; use 0 for the first request
/// * `limit` - The maximum number of chats to be returned; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_groups_in_common(
    user_id: i64,
    offset_chat_id: i64,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
    "@type": "getGroupsInCommon",
    "user_id": user_id,
    "offset_chat_id": offset_chat_id,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
