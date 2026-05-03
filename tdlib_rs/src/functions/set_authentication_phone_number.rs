use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitPhoneNumber,
/// or if there is no pending authentication query and the current authorization state is authorizationStateWaitPremiumPurchase, authorizationStateWaitEmailAddress,
/// authorizationStateWaitEmailCode, authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
/// # Arguments
/// * `phone_number` - The phone number of the user, in international format
/// * `settings` - Settings for the authentication of the user's phone number; pass null to use default settings
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_authentication_phone_number(
    phone_number: String,
    settings: Option<crate::types::PhoneNumberAuthenticationSettings>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setAuthenticationPhoneNumber",
    "phone_number": phone_number,
    "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
