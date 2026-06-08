use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Declines a suggested post in a channel direct messages chat
/// # Arguments
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `message_id` - Identifier of the message with the suggested post. Use messageProperties.can_be_declined to check whether the suggested post can be declined
/// * `comment` - Comment for the creator of the suggested post; 0-128 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn decline_suggested_post(
    chat_id: i64,
    message_id: i64,
    comment: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "declineSuggestedPost",
    "chat_id": chat_id,
    "message_id": message_id,
    "comment": comment,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
