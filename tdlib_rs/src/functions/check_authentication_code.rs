use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode
/// # Arguments
/// * `code` - Authentication code to check
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_authentication_code(
    code: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "checkAuthenticationCode",
    "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
