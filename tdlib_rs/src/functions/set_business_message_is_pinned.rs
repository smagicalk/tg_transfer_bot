use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Pins or unpins a message sent on behalf of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection on behalf of which the message was sent
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `is_pinned` - Pass true to pin the message, pass false to unpin it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_message_is_pinned(
    business_connection_id: String,
    chat_id: i64,
    message_id: i64,
    is_pinned: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBusinessMessageIsPinned",
    "business_connection_id": business_connection_id,
    "chat_id": chat_id,
    "message_id": message_id,
    "is_pinned": is_pinned,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
