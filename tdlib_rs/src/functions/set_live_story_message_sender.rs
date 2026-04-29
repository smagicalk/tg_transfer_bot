#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Selects a message sender to send messages in a live story call
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `message_sender_id` - New message sender for the group call
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_live_story_message_sender(group_call_id: i32, message_sender_id: crate::enums::MessageSender, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setLiveStoryMessageSender",
        "group_call_id": group_call_id,
        "message_sender_id": message_sender_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
