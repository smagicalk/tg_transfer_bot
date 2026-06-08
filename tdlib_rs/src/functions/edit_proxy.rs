use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Edits an existing proxy server for network requests. Can be called before authorization
/// # Arguments
/// * `proxy_id` - Proxy identifier
/// * `proxy` - The new information about the proxy
/// * `enable` - Pass true to immediately enable the proxy
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_proxy(
    proxy_id: i32,
    proxy: crate::types::Proxy,
    enable: bool,
    client_id: i32,
) -> Result<crate::enums::AddedProxy, crate::types::Error> {
    let request = json!({
    "@type": "editProxy",
    "proxy_id": proxy_id,
    "proxy": proxy,
    "enable": enable,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
