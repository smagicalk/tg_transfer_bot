#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends debug information for a call to Telegram servers
/// # Arguments
/// * `call_id` - Call identifier
/// * `debug_information` - Debug information in application-specific format
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_call_debug_information(call_id: crate::enums::InputCall, debug_information: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendCallDebugInformation",
        "call_id": call_id,
        "debug_information": debug_information,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
