#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the tag or custom title of a chat member; requires can_manage_tags administrator right to change tag of other users; for basic groups and supergroups only
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `user_id` - Identifier of the user, which tag is changed. Chats can't have member tags
/// * `tag` - The new tag of the member in the chat; 0-16 characters without emoji
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_chat_member_tag(chat_id: i64, user_id: i64, tag: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setChatMemberTag",
        "chat_id": chat_id,
        "user_id": user_id,
        "tag": tag,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
