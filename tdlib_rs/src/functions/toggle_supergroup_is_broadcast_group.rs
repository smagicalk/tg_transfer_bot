#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Upgrades supergroup to a broadcast group; requires owner privileges in the supergroup
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_is_broadcast_group(supergroup_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupIsBroadcastGroup",
        "supergroup_id": supergroup_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
