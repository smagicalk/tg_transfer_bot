#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends a code to verify an email address to be added to a user's Telegram Passport
/// # Arguments
/// * `email_address` - Email address
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_email_address_verification_code(email_address: String, client_id: i32) -> Result<crate::enums::EmailAddressAuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "sendEmailAddressVerificationCode",
        "email_address": email_address,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
