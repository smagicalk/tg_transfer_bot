use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether participants of a group call can send messages there. Requires groupCall.can_toggle_are_messages_allowed right
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `are_messages_allowed` - New value of the are_messages_allowed setting
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_are_messages_allowed(
    group_call_id: i32,
    are_messages_allowed: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleGroupCallAreMessagesAllowed",
    "group_call_id": group_call_id,
    "are_messages_allowed": are_messages_allowed,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
