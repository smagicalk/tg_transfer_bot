#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends call signaling data
/// # Arguments
/// * `call_id` - Call identifier
/// * `data` - The data
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_call_signaling_data(call_id: i32, data: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendCallSignalingData",
        "call_id": call_id,
        "data": data,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
