use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns information about an upgraded gift by its name
/// # Arguments
/// * `name` - Unique name of the upgraded gift
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_upgraded_gift(
    name: String,
    client_id: i32,
) -> Result<crate::enums::UpgradedGift, crate::types::Error> {
    let request = json!({
    "@type": "getUpgradedGift",
    "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
