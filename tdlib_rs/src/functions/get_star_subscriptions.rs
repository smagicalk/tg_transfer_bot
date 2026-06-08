use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of Telegram Star subscriptions for the current user
/// # Arguments
/// * `only_expiring` - Pass true to receive only expiring subscriptions for which there are no enough Telegram Stars to extend
/// * `offset` - Offset of the first subscription to return as received from the previous request; use empty string to get the first chunk of results
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_star_subscriptions(
    only_expiring: bool,
    offset: String,
    client_id: i32,
) -> Result<crate::enums::StarSubscriptions, crate::types::Error> {
    let request = json!({
    "@type": "getStarSubscriptions",
    "only_expiring": only_expiring,
    "offset": offset,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
