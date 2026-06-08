use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Joins an active video chat. Returns join response payload for tgcalls
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `participant_id` - Identifier of a group call participant, which will be used to join the call; pass null to join as self
/// * `join_parameters` - Parameters to join the call
/// * `invite_hash` - Invite hash as received from internalLinkTypeVideoChat
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn join_video_chat(
    group_call_id: i32,
    participant_id: Option<crate::enums::MessageSender>,
    join_parameters: crate::types::GroupCallJoinParameters,
    invite_hash: String,
    client_id: i32,
) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
    "@type": "joinVideoChat",
    "group_call_id": group_call_id,
    "participant_id": participant_id,
    "join_parameters": join_parameters,
    "invite_hash": invite_hash,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
