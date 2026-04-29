#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first
/// # Arguments
/// * `element` - Input Telegram Passport element
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_passport_element(element: crate::enums::InputPassportElement, password: String, client_id: i32) -> Result<crate::enums::PassportElement, crate::types::Error> {
    let request = json!({
        "@type": "setPassportElement",
        "element": element,
        "password": password,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
