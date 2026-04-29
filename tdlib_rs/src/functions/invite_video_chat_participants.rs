#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Invites users to an active video chat. Sends a service message of the type messageInviteVideoChatParticipants to the chat bound to the group call
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `user_ids` - User identifiers. At most 10 users can be invited simultaneously
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn invite_video_chat_participants(group_call_id: i32, user_ids: Vec<i64>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "inviteVideoChatParticipants",
        "group_call_id": group_call_id,
        "user_ids": user_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
