#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that messages are being viewed by the user. Sponsored messages must be marked as viewed only when the entire text of the message is shown on the screen (excluding the button).
/// Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels)
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `message_ids` - The identifiers of the messages being viewed
/// * `source` - Source of the message view; pass null to guess the source based on chat open state
/// * `force_read` - Pass true to mark as read the specified messages even if the chat is closed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn view_messages(chat_id: i64, message_ids: Vec<i64>, source: Option<crate::enums::MessageSource>, force_read: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "viewMessages",
        "chat_id": chat_id,
        "message_ids": message_ids,
        "source": source,
        "force_read": force_read,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
