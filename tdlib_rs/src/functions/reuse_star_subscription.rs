#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reuses an active Telegram Star subscription to a channel chat and joins the chat again
/// # Arguments
/// * `subscription_id` - Identifier of the subscription
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reuse_star_subscription(subscription_id: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reuseStarSubscription",
        "subscription_id": subscription_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
