use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization
/// # Arguments
/// * `proxy` - The proxy to test; pass null to ping a Telegram server without a proxy
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn ping_proxy(
    proxy: Option<crate::types::Proxy>,
    client_id: i32,
) -> Result<crate::enums::Seconds, crate::types::Error> {
    let request = json!({
    "@type": "pingProxy",
    "proxy": proxy,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
