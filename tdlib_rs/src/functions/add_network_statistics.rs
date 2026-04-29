#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds the specified data to data usage statistics. Can be called before authorization
/// # Arguments
/// * `entry` - The network statistics entry with the data to be added to statistics
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_network_statistics(entry: crate::enums::NetworkStatisticsEntry, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "addNetworkStatistics",
        "entry": entry,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
