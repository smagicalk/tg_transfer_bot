#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Cancels or re-enables Telegram Star subscription
/// # Arguments
/// * `subscription_id` - Identifier of the subscription to change
/// * `is_canceled` - New value of is_canceled
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_star_subscription(subscription_id: String, is_canceled: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editStarSubscription",
        "subscription_id": subscription_id,
        "is_canceled": is_canceled,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
