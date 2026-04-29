#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends log file for a call to Telegram servers
/// # Arguments
/// * `call_id` - Call identifier
/// * `log_file` - Call log file. Only inputFileLocal and inputFileGenerated are supported
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_call_log(call_id: crate::enums::InputCall, log_file: crate::enums::InputFile, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendCallLog",
        "call_id": call_id,
        "log_file": log_file,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
