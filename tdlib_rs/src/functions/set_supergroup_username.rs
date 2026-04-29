#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the editable username of a supergroup or channel, requires owner privileges in the supergroup or channel
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `username` - New value of the username. Use an empty string to remove the username. The username can't be completely removed if there is another active or disabled username
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_supergroup_username(supergroup_id: i64, username: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setSupergroupUsername",
        "supergroup_id": supergroup_id,
        "username": username,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
