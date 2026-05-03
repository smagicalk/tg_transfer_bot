use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns chat lists to which the chat can be added. This is an offline method
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_lists_to_add_chat(
    chat_id: i64,
    client_id: i32,
) -> Result<crate::enums::ChatLists, crate::types::Error> {
    let request = json!({
    "@type": "getChatListsToAddChat",
    "chat_id": chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
