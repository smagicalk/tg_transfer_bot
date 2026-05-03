use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks the 2-step verification password for correctness. Works only when the current authorization state is authorizationStateWaitPassword
/// # Arguments
/// * `password` - The 2-step verification password to check
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_password(
    password: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "checkAuthenticationPassword",
    "password": password,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
