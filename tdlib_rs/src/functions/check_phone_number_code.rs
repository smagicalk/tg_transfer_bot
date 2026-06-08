use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Checks the authentication code and completes the request for which the code was sent if appropriate
/// # Arguments
/// * `code` - Authentication code to check
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_phone_number_code(
    code: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "checkPhoneNumberCode",
    "code": code,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
