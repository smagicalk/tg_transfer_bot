#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Loads more participants of a group call; not supported in live stories. The loaded participants will be received through updates.
/// Use the field groupCall.loaded_all_participants to check whether all participants have already been loaded
/// # Arguments
/// * `group_call_id` - Group call identifier. The group call must be previously received through getGroupCall and must be joined or being joined
/// * `limit` - The maximum number of participants to load; up to 100
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn load_group_call_participants(group_call_id: i32, limit: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "loadGroupCallParticipants",
        "group_call_id": group_call_id,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
