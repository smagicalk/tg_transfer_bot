use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits the time when a scheduled message will be sent. Scheduling state of all messages in the same album or forwarded together with the message will be also changed
/// # Arguments
/// * `chat_id` - The chat the message belongs to
/// * `message_id` - Identifier of the message. Use messageProperties.can_edit_scheduling_state to check whether the message is suitable
/// * `scheduling_state` - The new message scheduling state; pass null to send the message immediately. Must be null for messages in the state messageSchedulingStateSendWhenVideoProcessed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_message_scheduling_state(
    chat_id: i64,
    message_id: i64,
    scheduling_state: Option<crate::enums::MessageSchedulingState>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "editMessageSchedulingState",
    "chat_id": chat_id,
    "message_id": message_id,
    "scheduling_state": scheduling_state,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
