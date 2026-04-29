#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes all pending paid reactions on a message
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the message belongs
/// * `message_id` - Identifier of the message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_pending_paid_message_reactions(chat_id: i64, message_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "removePendingPaidMessageReactions",
        "chat_id": chat_id,
        "message_id": message_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
