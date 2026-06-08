use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes a chat from the list of recently found chats
/// # Arguments
/// * `chat_id` - Identifier of the chat to be removed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_recently_found_chat(
    chat_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeRecentlyFoundChat",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
