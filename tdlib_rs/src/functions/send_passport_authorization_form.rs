use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after getPassportAuthorizationFormAvailableElements if some previously available elements are going to be reused
/// # Arguments
/// * `authorization_form_id` - Authorization form identifier
/// * `types` - Types of Telegram Passport elements chosen by user to complete the authorization form
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_passport_authorization_form(
    authorization_form_id: i32,
    types: Vec<crate::enums::PassportElementType>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "sendPassportAuthorizationForm",
    "authorization_form_id": authorization_form_id,
    "types": types,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
