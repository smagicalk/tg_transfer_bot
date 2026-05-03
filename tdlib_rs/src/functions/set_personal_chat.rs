use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the personal chat of the current user
/// # Arguments
/// * `chat_id` - Identifier of the new personal chat; pass 0 to remove the chat. Use getSuitablePersonalChats to get suitable chats
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_personal_chat(chat_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setPersonalChat",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
