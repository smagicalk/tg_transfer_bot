use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Transfers Telegram Stars from the business account to the business bot; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection
/// * `star_count` - Number of Telegram Stars to transfer
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn transfer_business_account_stars(
    business_connection_id: String,
    star_count: i64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "transferBusinessAccountStars",
    "business_connection_id": business_connection_id,
    "star_count": star_count,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
