use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a code to the specified phone number. Aborts previous phone number verification if there was one. On success, returns information about the sent code
/// # Arguments
/// * `phone_number` - The phone number, in international format
/// * `settings` - Settings for the authentication of the user's phone number; pass null to use default settings
/// * `r#type` - Type of the request for which the code is sent
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_phone_number_code(
    phone_number: String,
    settings: Option<crate::types::PhoneNumberAuthenticationSettings>,
    r#type: crate::enums::PhoneNumberCodeType,
    client_id: i32,
) -> Result<crate::enums::AuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
    "@type": "sendPhoneNumberCode",
    "phone_number": phone_number,
    "settings": settings,
    "type": r#type,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
