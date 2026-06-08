use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Resends the 2-step verification recovery email address verification code
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn resend_recovery_email_address_code(
    client_id: i32,
) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
    "@type": "resendRecoveryEmailAddressCode",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
