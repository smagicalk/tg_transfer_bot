use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Refunds a previously done payment in Telegram Stars; for bots only
/// # Arguments
/// * `user_id` - Identifier of the user who did the payment
/// * `telegram_payment_charge_id` - Telegram payment identifier
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn refund_star_payment(
    user_id: i64,
    telegram_payment_charge_id: String,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "refundStarPayment",
    "user_id": user_id,
    "telegram_payment_charge_id": telegram_payment_charge_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
