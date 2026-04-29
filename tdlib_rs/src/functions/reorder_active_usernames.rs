#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes order of active usernames of the current user
/// # Arguments
/// * `usernames` - The new order of active usernames. All currently active usernames must be specified
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reorder_active_usernames(usernames: Vec<String>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reorderActiveUsernames",
        "usernames": usernames,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
