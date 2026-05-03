use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Joins a group call of an active live story. Returns join response payload for tgcalls
/// # Arguments
/// * `group_call_id` - Group call identifier
/// * `join_parameters` - Parameters to join the call
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn join_live_story(
    group_call_id: i32,
    join_parameters: crate::types::GroupCallJoinParameters,
    client_id: i32,
) -> Result<crate::enums::Text, crate::types::Error> {
    let request = json!({
    "@type": "joinLiveStory",
    "group_call_id": group_call_id,
    "join_parameters": join_parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
