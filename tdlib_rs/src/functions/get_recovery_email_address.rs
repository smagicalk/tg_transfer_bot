#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user
/// # Arguments
/// * `password` - The 2-step verification password for the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_recovery_email_address(password: String, client_id: i32) -> Result<crate::enums::RecoveryEmailAddress, crate::types::Error> {
    let request = json!({
        "@type": "getRecoveryEmailAddress",
        "password": password,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
