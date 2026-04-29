#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a replyMarkupForceReply reply markup has been used or dismissed
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `message_id` - The message identifier of the used keyboard
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_chat_reply_markup(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteChatReplyMarkup",
        "chat_id": chat_id,
        "message_id": message_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
