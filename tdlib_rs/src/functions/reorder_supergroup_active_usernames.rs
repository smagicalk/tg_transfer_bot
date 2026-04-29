#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes order of active usernames of a supergroup or channel, requires owner privileges in the supergroup or channel
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `usernames` - The new order of active usernames. All currently active usernames must be specified
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reorder_supergroup_active_usernames(supergroup_id: i64, usernames: Vec<String>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderSupergroupActiveUsernames",
        "supergroup_id": supergroup_id,
        "usernames": usernames,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
