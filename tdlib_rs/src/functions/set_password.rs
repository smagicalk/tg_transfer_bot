#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the 2-step verification password for the current user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed
/// # Arguments
/// * `old_password` - Previous 2-step verification password of the user
/// * `new_password` - New 2-step verification password of the user; may be empty to remove the password
/// * `new_hint` - New password hint; may be empty
/// * `set_recovery_email_address` - Pass true to change also the recovery email address
/// * `new_recovery_email_address` - New recovery email address; may be empty
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_password(old_password: String, new_password: String, new_hint: String, set_recovery_email_address: bool, new_recovery_email_address: String, client_id: i32) -> Result<crate::enums::PasswordState, crate::types::Error> {
    let request = json!({
        "@type": "setPassword",
        "old_password": old_password,
        "new_password": new_password,
        "new_hint": new_hint,
        "set_recovery_email_address": set_recovery_email_address,
        "new_recovery_email_address": new_recovery_email_address,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
