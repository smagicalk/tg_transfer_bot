#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Resends the authentication code sent to a phone number. Works only if the previously received authenticationCodeInfo next_code_type was not null and the server-specified timeout has passed
/// # Arguments
/// * `reason` - Reason of code resending; pass null if unknown
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn resend_phone_number_code(reason: Option<crate::enums::ResendCodeReason>, client_id: i32) -> Result<crate::enums::AuthenticationCodeInfo, crate::types::Error> {
    let request = json!({
        "@type": "resendPhoneNumberCode",
        "reason": reason,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
