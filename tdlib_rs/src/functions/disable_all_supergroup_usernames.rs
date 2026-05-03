use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Disables all active non-editable usernames of a supergroup or channel, requires owner privileges in the supergroup or channel
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn disable_all_supergroup_usernames(
    supergroup_id: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "disableAllSupergroupUsernames",
    "supergroup_id": supergroup_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
