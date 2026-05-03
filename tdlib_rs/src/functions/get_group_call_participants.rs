use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about participants of a non-joined group call that is not bound to a chat
/// # Arguments
/// * `input_group_call` - The group call which participants will be returned
/// * `limit` - The maximum number of participants to return; must be positive
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_group_call_participants(
    input_group_call: crate::enums::InputGroupCall,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::GroupCallParticipants, crate::types::Error> {
    let request = json!({
    "@type": "getGroupCallParticipants",
    "input_group_call": input_group_call,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
