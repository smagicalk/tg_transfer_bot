#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Recovers the 2-step verification password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
/// # Arguments
/// * `recovery_code` - Recovery code to check
/// * `new_password` - New 2-step verification password of the user; may be empty to remove the password
/// * `new_hint` - New password hint; may be empty
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn recover_authentication_password(recovery_code: String, new_password: String, new_hint: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "recoverAuthenticationPassword",
        "recovery_code": recovery_code,
        "new_password": new_password,
        "new_hint": new_hint,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
