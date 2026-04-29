#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Checks the email address verification code for Telegram Passport
/// # Arguments
/// * `code` - Verification code to check
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn check_email_address_verification_code(code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "checkEmailAddressVerificationCode",
        "code": code,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
