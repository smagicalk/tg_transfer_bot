use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Places a bid on an auction gift
/// # Arguments
/// * `gift_id` - Identifier of the gift to place the bid on
/// * `star_count` - The number of Telegram Stars to place in the bid
/// * `user_id` - Identifier of the user who will receive the gift
/// * `text` - Text to show along with the gift; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed.
/// Must be empty if the receiver enabled paid messages
/// * `is_private` - Pass true to show gift text and sender only to the gift receiver; otherwise, everyone will be able to see them
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn place_gift_auction_bid(
    gift_id: i64,
    star_count: i64,
    user_id: i64,
    text: crate::types::FormattedText,
    is_private: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "placeGiftAuctionBid",
    "gift_id": gift_id,
    "star_count": star_count,
    "user_id": user_id,
    "text": text,
    "is_private": is_private,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
