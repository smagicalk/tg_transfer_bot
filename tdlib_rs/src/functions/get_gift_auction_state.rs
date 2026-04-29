#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns auction state for a gift
/// # Arguments
/// * `auction_id` - Unique identifier of the auction
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_auction_state(auction_id: String, client_id: i32) -> Result<crate::enums::GiftAuctionState, crate::types::Error> {
    let request = json!({
        "@type": "getGiftAuctionState",
        "auction_id": auction_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
