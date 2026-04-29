#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Resets the login email address. May return an error with a message "TASK_ALREADY_EXISTS" if reset is still pending.
/// Works only when the current authorization state is authorizationStateWaitEmailCode and authorization_state.can_reset_email_address == true
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reset_authentication_email_address(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "resetAuthenticationEmailAddress",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
