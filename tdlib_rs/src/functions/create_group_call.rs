#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Creates a new group call that isn't bound to a chat
/// # Arguments
/// * `join_parameters` - Parameters to join the call; pass null to only create call link without joining the call
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_group_call(join_parameters: Option<crate::types::GroupCallJoinParameters>, client_id: i32) -> Result<crate::enums::GroupCallInfo, crate::types::Error> {
    let request = json!({
        "@type": "createGroupCall",
        "join_parameters": join_parameters,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
