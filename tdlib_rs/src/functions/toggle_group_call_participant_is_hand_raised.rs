#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Toggles whether a group call participant hand is rased; for video chats only
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `participant_id` - Participant identifier
/// * `is_hand_raised` - Pass true if the user's hand needs to be raised. Only self hand can be raised. Requires groupCall.can_be_managed right to lower other's hand
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_group_call_participant_is_hand_raised(group_call_id: i32, participant_id: crate::enums::MessageSender, is_hand_raised: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleGroupCallParticipantIsHandRaised",
        "group_call_id": group_call_id,
        "participant_id": participant_id,
        "is_hand_raised": is_hand_raised,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
