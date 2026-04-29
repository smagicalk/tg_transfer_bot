#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns invite link to a video chat in a public chat
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `can_self_unmute` - Pass true if the invite link needs to contain an invite hash, passing which to joinVideoChat would allow the invited user to unmute themselves. Requires groupCall.can_be_managed right
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_video_chat_invite_link(group_call_id: i32, can_self_unmute: bool, client_id: i32) -> Result<crate::enums::HttpUrl, crate::types::Error> {
    let request = json!({
        "@type": "getVideoChatInviteLink",
        "group_call_id": group_call_id,
        "can_self_unmute": can_self_unmute,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
