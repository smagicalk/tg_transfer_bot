#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends a simple network request to the Telegram servers via proxy; for testing only. Can be called before authorization
/// # Arguments
/// * `proxy` - The proxy to test
/// * `dc_id` - Identifier of a datacenter with which to test connection
/// * `timeout` - The maximum overall timeout for the request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn test_proxy(proxy: crate::types::Proxy, dc_id: i32, timeout: f64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "testProxy",
        "proxy": proxy,
        "dc_id": dc_id,
        "timeout": timeout,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
