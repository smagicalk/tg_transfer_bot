#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Revokes invite link for a group call. Requires groupCall.can_be_managed right for video chats or groupCall.is_owned otherwise
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn revoke_group_call_invite_link(group_call_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "revokeGroupCallInviteLink",
        "group_call_id": group_call_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
