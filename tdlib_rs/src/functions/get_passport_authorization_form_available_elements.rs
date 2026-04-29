#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form
/// # Arguments
/// * `authorization_form_id` - Authorization form identifier
/// * `password` - The 2-step verification password of the current user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_passport_authorization_form_available_elements(authorization_form_id: i32, password: String, client_id: i32) -> Result<crate::enums::PassportElementsWithErrors, crate::types::Error> {
    let request = json!({
        "@type": "getPassportAuthorizationFormAvailableElements",
        "authorization_form_id": authorization_form_id,
        "password": password,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
