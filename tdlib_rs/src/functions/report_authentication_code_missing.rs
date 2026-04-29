#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reports that authentication code wasn't delivered via SMS; for official mobile applications only. Works only when the current authorization state is authorizationStateWaitCode
/// # Arguments
/// * `mobile_network_code` - Current mobile network code
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_authentication_code_missing(mobile_network_code: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportAuthenticationCodeMissing",
        "mobile_network_code": mobile_network_code,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
