#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Accepts an incoming call
/// # Arguments
/// * `call_id` - Call identifier
/// * `protocol` - The call protocols supported by the application
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn accept_call(call_id: i32, protocol: crate::types::CallProtocol, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "acceptCall",
        "call_id": call_id,
        "protocol": protocol,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
