use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns network data usage statistics. Can be called before authorization
/// # Arguments
/// * `only_current` - Pass true to get statistics only for the current library launch
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_network_statistics(
    only_current: bool,
    client_id: i32,
) -> Result<crate::enums::NetworkStatistics, crate::types::Error> {
    let request = json!({
    "@type": "getNetworkStatistics",
    "only_current": only_current,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
