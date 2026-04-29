#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that speaking state of a participant of an active group call has changed. Returns identifier of the participant if it is found
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `audio_source` - Group call participant's synchronization audio source identifier, or 0 for the current user
/// * `is_speaking` - Pass true if the user is speaking
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_group_call_participant_is_speaking(group_call_id: i32, audio_source: i32, is_speaking: bool, client_id: i32) -> Result<crate::enums::MessageSender, crate::types::Error> {
    let request = json!({
        "@type": "setGroupCallParticipantIsSpeaking",
        "group_call_id": group_call_id,
        "audio_source": audio_source,
        "is_speaking": is_speaking,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
