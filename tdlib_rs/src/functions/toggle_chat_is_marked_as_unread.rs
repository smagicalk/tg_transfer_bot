use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the marked as unread state of a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `is_marked_as_unread` - New value of is_marked_as_unread
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_chat_is_marked_as_unread(
    chat_id: i64,
    is_marked_as_unread: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleChatIsMarkedAsUnread",
    "chat_id": chat_id,
    "is_marked_as_unread": is_marked_as_unread,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
