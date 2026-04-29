#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of message sender identifiers, on whose behalf messages can be sent to a live story
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_live_story_available_message_senders(group_call_id: i32, client_id: i32) -> Result<crate::enums::ChatMessageSenders, crate::types::Error> {
    let request = json!({
        "@type": "getLiveStoryAvailableMessageSenders",
        "group_call_id": group_call_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
