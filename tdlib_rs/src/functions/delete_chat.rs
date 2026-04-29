#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes a chat along with all messages in the corresponding chat for all chat members. For group chats this will release the usernames and remove all members.
/// Use the field chat.can_be_deleted_for_all_users to find whether the method can be applied to the chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChat",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
