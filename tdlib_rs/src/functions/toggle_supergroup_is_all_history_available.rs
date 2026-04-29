#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Toggles whether the message history of a supergroup is available to new members; requires can_change_info member right
/// # Arguments
/// * `supergroup_id` - The identifier of the supergroup
/// * `is_all_history_available` - The new value of is_all_history_available
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_is_all_history_available(supergroup_id: i64, is_all_history_available: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupIsAllHistoryAvailable",
        "supergroup_id": supergroup_id,
        "is_all_history_available": is_all_history_available,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
