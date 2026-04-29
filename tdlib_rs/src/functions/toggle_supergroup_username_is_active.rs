#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes active state for a username of a supergroup or channel, requires owner privileges in the supergroup or channel. The editable username can't be disabled.
/// May return an error with a message "USERNAMES_ACTIVE_TOO_MUCH" if the maximum number of active usernames has been reached
/// # Arguments
/// * `supergroup_id` - Identifier of the supergroup or channel
/// * `username` - The username to change
/// * `is_active` - Pass true to activate the username; pass false to disable it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_supergroup_username_is_active(supergroup_id: i64, username: String, is_active: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSupergroupUsernameIsActive",
        "supergroup_id": supergroup_id,
        "username": username,
        "is_active": is_active,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
