use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the Telegram Star amount owned by a business account; for bots only
/// # Arguments
/// * `business_connection_id` - Unique identifier of business connection
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_business_account_star_amount(
    business_connection_id: String,
    client_id: i32,
) -> Result<crate::enums::StarAmount, crate::types::Error> {
    let request = json!({
    "@type": "getBusinessAccountStarAmount",
    "business_connection_id": business_connection_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
