use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Stops a poll
/// # Arguments
/// * `chat_id` - Identifier of the chat to which the poll belongs
/// * `message_id` - Identifier of the message containing the poll. Use messageProperties.can_be_edited to check whether the poll can be stopped
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn stop_poll(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "stopPoll",
    "chat_id": chat_id,
    "message_id": message_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
