use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Handles a pending gift purchase offer
/// # Arguments
/// * `message_id` - Identifier of the message with the gift purchase offer
/// * `accept` - Pass true to accept the request; pass false to reject it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn process_gift_purchase_offer(
    message_id: i64,
    accept: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "processGiftPurchaseOffer",
    "message_id": message_id,
    "accept": accept,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
