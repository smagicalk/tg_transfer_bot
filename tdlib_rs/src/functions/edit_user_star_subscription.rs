use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Cancels or re-enables Telegram Star subscription for a user; for bots only
/// # Arguments
/// * `user_id` - User identifier
/// * `telegram_payment_charge_id` - Telegram payment identifier of the subscription
/// * `is_canceled` - Pass true to cancel the subscription; pass false to allow the user to enable it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_user_star_subscription(
    user_id: i64,
    telegram_payment_charge_id: String,
    is_canceled: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "editUserStarSubscription",
    "user_id": user_id,
    "telegram_payment_charge_id": telegram_payment_charge_id,
    "is_canceled": is_canceled,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
