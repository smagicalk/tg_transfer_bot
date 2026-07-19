use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Sells a gift for Telegram Stars; requires owner privileges for gifts owned by a chat
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection on behalf of which to send the request; for bots only
/// * `received_gift_id` - Identifier of the gift
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn sell_gift(
    business_connection_id: String,
    received_gift_id: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "sellGift",
    "business_connection_id": business_connection_id,
    "received_gift_id": received_gift_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
