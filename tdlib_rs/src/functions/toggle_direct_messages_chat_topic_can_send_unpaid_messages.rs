use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Allows to send unpaid messages to the given topic of the channel direct messages chat administered by the current user
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `topic_id` - Identifier of the topic
/// * `can_send_unpaid_messages` - Pass true to allow unpaid messages; pass false to disallow unpaid messages
/// * `refund_payments` - Pass true to refund the user previously paid messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_direct_messages_chat_topic_can_send_unpaid_messages(
    chat_id: i64,
    topic_id: i64,
    can_send_unpaid_messages: bool,
    refund_payments: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleDirectMessagesChatTopicCanSendUnpaidMessages",
    "chat_id": chat_id,
    "topic_id": topic_id,
    "can_send_unpaid_messages": can_send_unpaid_messages,
    "refund_payments": refund_payments,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
