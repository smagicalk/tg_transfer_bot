use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether a participant of an active group call is muted, unmuted, or allowed to unmute themselves; not supported for live stories
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `participant_id` - Participant identifier
/// * `is_muted` - Pass true to mute the user; pass false to unmute them
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_participant_is_muted(
    group_call_id: i32,
    participant_id: crate::enums::MessageSender,
    is_muted: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleGroupCallParticipantIsMuted",
    "group_call_id": group_call_id,
    "participant_id": participant_id,
    "is_muted": is_muted,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
