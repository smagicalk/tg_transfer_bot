use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns available options for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members
/// # Arguments
/// * `boosted_chat_id` - Identifier of the supergroup or channel chat, which will be automatically boosted by receivers of the gift codes and which is administered by the user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_giveaway_payment_options(
    boosted_chat_id: i64,
    client_id: i32,
) -> Result<crate::enums::PremiumGiveawayPaymentOptions, crate::types::Error> {
    let request = json!({
    "@type": "getPremiumGiveawayPaymentOptions",
    "boosted_chat_id": boosted_chat_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
