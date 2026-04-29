#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Creates a new temporary password for processing payments
/// # Arguments
/// * `password` - The 2-step verification password of the current user
/// * `valid_for` - Time during which the temporary password will be valid, in seconds; must be between 60 and 86400
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_temporary_password(password: String, valid_for: i32, client_id: i32) -> Result<crate::enums::TemporaryPasswordState, crate::types::Error> {
    let request = json!({
        "@type": "createTemporaryPassword",
        "password": password,
        "valid_for": valid_for,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
