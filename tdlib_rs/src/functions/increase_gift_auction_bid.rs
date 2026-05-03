use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Increases a bid for an auction gift without changing gift text and receiver
/// # Arguments
/// * `gift_id` - Identifier of the gift to put the bid on
/// * `star_count` - The number of Telegram Stars to put in the bid
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn increase_gift_auction_bid(
    gift_id: i64,
    star_count: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "increaseGiftAuctionBid",
    "gift_id": gift_id,
    "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
