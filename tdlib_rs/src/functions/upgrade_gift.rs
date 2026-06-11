use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Upgrades a regular gift
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request; for bots only
/// * `received_gift_id` - Identifier of the gift
/// * `keep_original_details` - Pass true to keep the original gift text, sender and receiver in the upgraded gift
/// * `star_count` - The Telegram Star amount required to pay for the upgrade. It the gift has prepaid_upgrade_star_count > 0, then pass 0, otherwise, pass gift.upgrade_star_count
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn upgrade_gift(
    business_connection_id: String,
    received_gift_id: String,
    keep_original_details: bool,
    star_count: i64,
    client_id: i32,
) -> Result<crate::enums::UpgradeGiftResult, crate::types::Error> {
    let request = json!({
    "@type": "upgradeGift",
    "business_connection_id": business_connection_id,
    "received_gift_id": received_gift_id,
    "keep_original_details": keep_original_details,
    "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
