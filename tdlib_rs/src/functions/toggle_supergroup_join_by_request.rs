use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether all users directly joining the supergroup need to be approved by supergroup administrators; requires can_restrict_members administrator right
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup that isn't a broadcast group and isn't a channel direct message group
/// * `join_by_request` - New value of join_by_request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_join_by_request(
    supergroup_id: i64,
    join_by_request: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleSupergroupJoinByRequest",
    "supergroup_id": supergroup_id,
    "join_by_request": join_by_request,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
