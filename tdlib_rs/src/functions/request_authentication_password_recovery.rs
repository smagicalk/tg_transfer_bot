#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Requests to send a 2-step verification password recovery code to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn request_authentication_password_recovery(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "requestAuthenticationPasswordRecovery",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
