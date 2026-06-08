use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the sticker to be used as representation of the Telegram Premium subscription
/// # Arguments
/// * `month_count` - Number of months the Telegram Premium subscription will be active
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_info_sticker(
    month_count: i32,
    client_id: i32,
) -> Result<crate::enums::Sticker, crate::types::Error> {
    let request = json!({
    "@type": "getPremiumInfoSticker",
    "month_count": month_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
