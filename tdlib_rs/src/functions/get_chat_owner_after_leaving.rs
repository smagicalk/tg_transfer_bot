#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the user who will become the owner of the chat after 7 days if the current user does not return to the supergroup or channel during that period or immediately for basic groups; requires owner privileges in the chat.
/// Available only for supergroups and channel chats
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_owner_after_leaving(chat_id: i64, client_id: i32) -> Result<crate::enums::User, crate::types::Error> {
    let request = json!({
        "@type": "getChatOwnerAfterLeaving",
        "chat_id": chat_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
