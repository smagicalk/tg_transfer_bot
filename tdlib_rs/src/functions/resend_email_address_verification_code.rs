use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Resends the code to verify an email address to be added to a user's Telegram Passport
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn resend_email_address_verification_code(
    client_id: i32,
) -> Result<crate::enums::EmailAddressAuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
    "@type": "resendEmailAddressVerificationCode",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
