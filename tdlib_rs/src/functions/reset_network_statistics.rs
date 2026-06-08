use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Resets all network data usage statistics to zero. Can be called before authorization
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reset_network_statistics(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "resetNetworkStatistics",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
