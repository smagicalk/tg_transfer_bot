use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns available options for Telegram Star giveaway creation
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_star_giveaway_payment_options(
    client_id: i32,
) -> Result<crate::enums::StarGiveawayPaymentOptions, crate::types::Error> {
    let request = json!({
    "@type": "getStarGiveawayPaymentOptions",
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
