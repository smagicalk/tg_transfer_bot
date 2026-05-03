use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Requests to send a 2-step verification password recovery code to an email address that was previously set up
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn request_password_recovery(
    client_id: i32,
) -> Result<crate::enums::EmailAddressAuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
    "@type": "requestPasswordRecovery",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
