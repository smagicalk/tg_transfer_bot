use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Bans users from a group call not bound to a chat; requires groupCall.is_owned. Only the owner of the group call can invite the banned users back
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `user_ids` - Identifiers of group call participants to ban; identifiers of unknown users from the update updateGroupCallParticipants can be also passed to the method
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn ban_group_call_participants(
    group_call_id: i32,
    user_ids: Vec<i64>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "banGroupCallParticipants",
    "group_call_id": group_call_id,
    "user_ids": user_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
