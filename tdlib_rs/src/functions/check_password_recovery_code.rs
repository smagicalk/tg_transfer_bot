use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks whether a 2-step verification password recovery code sent to an email address is valid
/// # Arguments
/// * `recovery_code` - Recovery code to check
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_password_recovery_code(
    recovery_code: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "checkPasswordRecoveryCode",
    "recovery_code": recovery_code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
