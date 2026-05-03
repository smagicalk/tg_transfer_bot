use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes a proxy server. Can be called before authorization
/// # Arguments
/// * `proxy_id` - Proxy identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_proxy(proxy_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "removeProxy",
    "proxy_id": proxy_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
