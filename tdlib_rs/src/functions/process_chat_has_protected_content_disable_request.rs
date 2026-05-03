use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Processes request to disable has_protected_content in a chat
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `request_message_id` - Identifier of the message with the request. The message must be incoming and has content of the type messageChatHasProtectedContentDisableRequested
/// * `approve` - Pass true to approve the request; pass false to reject the request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn process_chat_has_protected_content_disable_request(
    chat_id: i64,
    request_message_id: i64,
    approve: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "processChatHasProtectedContentDisableRequest",
    "chat_id": chat_id,
    "request_message_id": request_message_id,
    "approve": approve,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
