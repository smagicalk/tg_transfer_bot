#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the discussion group of a channel chat; requires can_change_info administrator right in the channel if it is specified
/// # Arguments
/// * `chat_id` - Identifier of the channel chat. Pass 0 to remove a link from the supergroup passed in the second argument to a linked channel chat (requires can_pin_messages member right in the supergroup)
/// * `discussion_chat_id` - Identifier of a new channel's discussion group. Use 0 to remove the discussion group. Use the method getSuitableDiscussionChats to find all suitable groups.
    /// Basic group chats must be first upgraded to supergroup chats. If new chat members don't have access to old messages in the supergroup, then toggleSupergroupIsAllHistoryAvailable must be used first to change that
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_discussion_group(chat_id: i64, discussion_chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatDiscussionGroup",
        "chat_id": chat_id,
        "discussion_chat_id": discussion_chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
