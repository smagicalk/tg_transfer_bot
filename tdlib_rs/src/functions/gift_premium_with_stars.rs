use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Allows to buy a Telegram Premium subscription for another user with payment in Telegram Stars; for bots only
/// # Arguments
/// * `user_id` - Identifier of the user which will receive Telegram Premium
/// * `star_count` - The number of Telegram Stars to pay for subscription
/// * `month_count` - Number of months the Telegram Premium subscription will be active for the user
/// * `text` - Text to show to the user receiving Telegram Premium; 0-getOption("gift_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Spoiler, and CustomEmoji entities are allowed
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn gift_premium_with_stars(
    user_id: i64,
    star_count: i64,
    month_count: i32,
    text: crate::types::FormattedText,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "giftPremiumWithStars",
    "user_id": user_id,
    "star_count": star_count,
    "month_count": month_count,
    "text": text,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
