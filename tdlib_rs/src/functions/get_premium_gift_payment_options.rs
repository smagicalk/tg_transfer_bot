#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns available options for gifting Telegram Premium to a user
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_premium_gift_payment_options(client_id: i32) -> Result<crate::enums::PremiumGiftPaymentOptions, crate::types::Error> {
    let request = json!({
        "@type": "getPremiumGiftPaymentOptions",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
