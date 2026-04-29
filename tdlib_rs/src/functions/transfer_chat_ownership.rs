#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the owner of a chat; for basic groups, supergroups and channel chats only; requires owner privileges in the chat. Use the method canTransferOwnership to check whether the ownership can be transferred from the current session
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `user_id` - Identifier of the user to which transfer the ownership. The ownership can't be transferred to a bot or to a deleted user
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn transfer_chat_ownership(chat_id: i64, user_id: i64, password: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "transferChatOwnership",
        "chat_id": chat_id,
        "user_id": user_id,
        "password": password,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
