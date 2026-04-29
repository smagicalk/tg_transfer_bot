#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Toggles whether new participants of a video chat can be unmuted only by administrators of the video chat. Requires groupCall.can_toggle_mute_new_participants right
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `mute_new_participants` - New value of the mute_new_participants setting
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_video_chat_mute_new_participants(group_call_id: i32, mute_new_participants: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleVideoChatMuteNewParticipants",
        "group_call_id": group_call_id,
        "mute_new_participants": mute_new_participants,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
