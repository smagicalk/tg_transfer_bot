use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the editable username of the current user
/// # Arguments
/// * `username` - The new value of the username. Use an empty string to remove the username. The username can't be completely removed if there is another active or disabled username
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_username(username: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setUsername",
    "username": username,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
