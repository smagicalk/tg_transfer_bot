#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns users and chats that were blocked by the current user
/// # Arguments
/// * `block_list` - Block list from which to return users
/// * `offset` - Number of users and chats to skip in the result; must be non-negative
/// * `limit` - The maximum number of users and chats to return; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_blocked_message_senders(block_list: crate::enums::BlockList, offset: i32, limit: i32, client_id: i32) -> Result<crate::enums::MessageSenders, crate::types::Error> {
    let request = json!({
        "@type": "getBlockedMessageSenders",
        "block_list": block_list,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
