#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Invites a user to an active group call; for group calls not bound to a chat only. Sends a service message of the type messageGroupCall.
/// The group call can have at most getOption("group_call_participant_count_max") participants
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `user_id` - User identifier
/// * `is_video` - Pass true if the group call is a video call
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn invite_group_call_participant(group_call_id: i32, user_id: i64, is_video: bool, client_id: i32) -> Result<crate::enums::InviteGroupCallParticipantResult, crate::types::Error> {
    let request = json!({
        "@type": "inviteGroupCallParticipant",
        "group_call_id": group_call_id,
        "user_id": user_id,
        "is_video": is_video,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
