use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed.
/// If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation
/// # Arguments
/// * `password` - The 2-step verification password of the current user
/// * `new_recovery_email_address` - New recovery email address
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_recovery_email_address(
    password: String,
    new_recovery_email_address: String,
    client_id: i32,
) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
    "@type": "setRecoveryEmailAddress",
    "password": password,
    "new_recovery_email_address": new_recovery_email_address,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
