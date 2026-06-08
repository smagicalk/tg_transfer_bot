use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Cancels verification of the 2-step verification recovery email address
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn cancel_recovery_email_address_verification(
    client_id: i32,
) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
    "@type": "cancelRecoveryEmailAddressVerification",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
