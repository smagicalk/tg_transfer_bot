#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes resale price of a unique gift owned by the current user
/// # Arguments
/// * `received_gift_id` - Identifier of the unique gift
/// * `price` - The new price for the unique gift; pass null to disallow gift resale. The current user will receive
    /// getOption("gift_resale_star_earnings_per_mille") Telegram Stars for each 1000 Telegram Stars paid for the gift if the gift price is in Telegram Stars or
    /// getOption("gift_resale_ton_earnings_per_mille") Toncoins for each 1000 Toncoins paid for the gift if the gift price is in Toncoins
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_gift_resale_price(received_gift_id: String, price: Option<crate::enums::GiftResalePrice>, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setGiftResalePrice",
        "received_gift_id": received_gift_id,
        "price": price,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
