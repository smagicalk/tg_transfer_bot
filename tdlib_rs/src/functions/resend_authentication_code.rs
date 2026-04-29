#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Resends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode, the next_code_type of the result is not null
/// and the server-specified timeout has passed, or when the current authorization state is authorizationStateWaitEmailCode
/// # Arguments
/// * `reason` - Reason of code resending; pass null if unknown
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn resend_authentication_code(reason: Option<crate::enums::ResendCodeReason>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "resendAuthenticationCode",
        "reason": reason,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
