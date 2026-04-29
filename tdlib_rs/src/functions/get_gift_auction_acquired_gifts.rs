#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the gifts that were acquired by the current user on a gift auction
/// # Arguments
/// * `gift_id` - Identifier of the auctioned gift
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_gift_auction_acquired_gifts(gift_id: i64, client_id: i32) -> Result<crate::enums::GiftAuctionAcquiredGifts, crate::types::Error> {
    let request = json!({
        "@type": "getGiftAuctionAcquiredGifts",
        "gift_id": gift_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
