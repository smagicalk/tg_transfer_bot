#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes volume level of a participant of an active group call; not supported for live stories. If the current user can manage the group call or is the owner of the group call,
/// then the participant's volume level will be changed for all users with the default volume level
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `participant_id` - Participant identifier
/// * `volume_level` - New participant's volume level; 1-20000 in hundreds of percents
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_group_call_participant_volume_level(group_call_id: i32, participant_id: crate::enums::MessageSender, volume_level: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setGroupCallParticipantVolumeLevel",
        "group_call_id": group_call_id,
        "participant_id": participant_id,
        "volume_level": volume_level,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
