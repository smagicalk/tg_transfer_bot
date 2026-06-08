use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the total number of Telegram Stars received by the current user for paid messages from the given user
/// # Arguments
/// * `user_id` - Identifier of the user
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_paid_message_revenue(
    user_id: i64,
    client_id: i32,
) -> Result<crate::enums::StarCount, crate::types::Error> {
    let request = json!({
    "@type": "getPaidMessageRevenue",
    "user_id": user_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
