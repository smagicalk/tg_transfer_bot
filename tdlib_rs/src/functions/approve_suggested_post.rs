use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Approves a suggested post in a channel direct messages chat
/// # Arguments
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `message_id` - Identifier of the message with the suggested post. Use messageProperties.can_be_approved to check whether the suggested post can be approved
/// * `send_date` - Point in time (Unix timestamp) when the post is expected to be published; pass 0 if the date has already been chosen. If specified,
/// then the date must be in the future, but at most getOption("suggested_post_send_delay_max") seconds in the future
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn approve_suggested_post(
    chat_id: i64,
    message_id: i64,
    send_date: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "approveSuggestedPost",
    "chat_id": chat_id,
    "message_id": message_id,
    "send_date": send_date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
