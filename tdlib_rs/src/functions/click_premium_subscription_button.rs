#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that the user clicked Premium subscription button on the Premium features screen
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn click_premium_subscription_button(client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "clickPremiumSubscriptionButton",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
