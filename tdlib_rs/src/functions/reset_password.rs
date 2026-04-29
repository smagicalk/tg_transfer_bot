#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Removes 2-step verification password without previous password and access to recovery email address. The password can't be reset immediately and the request needs to be repeated after the specified time
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reset_password(client_id: i32) -> Result<crate::enums::ResetPasswordResult, crate::types::Error> {
    let request = json!({
        "@type": "resetPassword",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
