#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Sends an offer to purchase an upgraded gift
/// # Arguments
/// * `owner_id` - Identifier of the user or the channel chat that currently owns the gift and will receive the offer
/// * `gift_name` - Name of the upgraded gift
/// * `price` - The price that the user agreed to pay for the gift
/// * `duration` - Duration of the offer, in seconds; must be one of 21600, 43200, 86400, 129600, 172800, or 259200. Can also be 120 if Telegram test environment is used
/// * `paid_message_star_count` - The number of Telegram Stars the user agreed to pay additionally for sending of the offer message to the current gift owner; pass userFullInfo.outgoing_paid_message_star_count for users and 0 otherwise
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn send_gift_purchase_offer(owner_id: crate::enums::MessageSender, gift_name: String, price: crate::enums::GiftResalePrice, duration: i32, paid_message_star_count: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "sendGiftPurchaseOffer",
        "owner_id": owner_id,
        "gift_name": gift_name,
        "price": price,
        "duration": duration,
        "paid_message_star_count": paid_message_star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
