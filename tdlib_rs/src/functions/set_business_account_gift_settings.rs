use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes settings for gift receiving of a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection
/// * `settings` - The new settings
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_business_account_gift_settings(
    business_connection_id: String,
    settings: crate::types::GiftSettings,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setBusinessAccountGiftSettings",
    "business_connection_id": business_connection_id,
    "settings": settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
