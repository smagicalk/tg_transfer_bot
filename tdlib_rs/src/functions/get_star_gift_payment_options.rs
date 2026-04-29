#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns available options for Telegram Stars gifting
/// # Arguments
/// * `user_id` - Identifier of the user who will receive Telegram Stars; pass 0 to get options for an unspecified user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_star_gift_payment_options(user_id: i64, client_id: i32) -> Result<crate::enums::StarPaymentOptions, crate::types::Error> {
    let request = json!({
        "@type": "getStarGiftPaymentOptions",
        "user_id": user_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
