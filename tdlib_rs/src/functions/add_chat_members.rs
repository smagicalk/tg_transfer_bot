#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds multiple new members to a chat; requires can_invite_users member right. Currently, this method is only available for supergroups and channels.
/// This method can't be used to join a chat. Members can't be added to a channel if it has more than 200 members. Returns information about members that weren't added
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `user_ids` - Identifiers of the users to be added to the chat. The maximum number of added users is 20 for supergroups and 100 for channels
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_chat_members(chat_id: i64, user_ids: Vec<i64>, client_id: i32) -> Result<crate::enums::FailedToAddMembers, crate::types::Error> {
    let request = json!({
        "@type": "addChatMembers",
        "chat_id": chat_id,
        "user_ids": user_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
