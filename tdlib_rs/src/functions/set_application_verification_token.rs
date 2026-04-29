#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that application or reCAPTCHA verification has been completed. Can be called before authorization
/// # Arguments
/// * `verification_id` - Unique identifier for the verification process as received from updateApplicationVerificationRequired or updateApplicationRecaptchaVerificationRequired
/// * `token` - Play Integrity API token for the Android application, or secret from push notification for the iOS application for application verification, or reCAPTCHA token for reCAPTCHA verifications;
    /// pass an empty string to abort verification and receive the error "VERIFICATION_FAILED" for the request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_application_verification_token(verification_id: i64, token: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setApplicationVerificationToken",
        "verification_id": verification_id,
        "token": token,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
