use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a Telegram Passport authorization form for sharing data with a service
/// # Arguments
/// * `bot_user_id` - User identifier of the service's bot
/// * `scope` - Telegram Passport element types requested by the service
/// * `public_key` - Service's public key
/// * `nonce` - Unique request identifier provided by the service
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_passport_authorization_form(
    bot_user_id: i64,
    scope: String,
    public_key: String,
    nonce: String,
    client_id: i32,
) -> Result<crate::enums::PassportAuthorizationForm, crate::types::Error> {
    let request = json!({
    "@type": "getPassportAuthorizationForm",
    "bot_user_id": bot_user_id,
    "scope": scope,
    "public_key": public_key,
    "nonce": nonce,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
