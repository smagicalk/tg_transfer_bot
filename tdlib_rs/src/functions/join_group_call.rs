use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Joins a regular group call that is not bound to a chat
/// # Arguments
/// * `input_group_call` - The group call to join
/// * `join_parameters` - Parameters to join the call
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn join_group_call(
    input_group_call: crate::enums::InputGroupCall,
    join_parameters: crate::types::GroupCallJoinParameters,
    client_id: i32,
) -> Result<crate::enums::GroupCallInfo, crate::types::Error> {
    let request = json!({
    "@type": "joinGroupCall",
    "input_group_call": input_group_call,
    "join_parameters": join_parameters,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
