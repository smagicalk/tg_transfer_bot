use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Reads a message on behalf of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection through which the message was received
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn read_business_message(
    business_connection_id: String,
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "readBusinessMessage",
    "business_connection_id": business_connection_id,
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
