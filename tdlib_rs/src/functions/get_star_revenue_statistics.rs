#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns detailed Telegram Star revenue statistics
/// # Arguments
/// * `owner_id` - Identifier of the owner of the Telegram Stars; can be identifier of the current user, an owned bot, or a supergroup or a channel chat with supergroupFullInfo.can_get_star_revenue_statistics == true
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_star_revenue_statistics(owner_id: crate::enums::MessageSender, is_dark: bool, client_id: i32) -> Result<crate::enums::StarRevenueStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getStarRevenueStatistics",
        "owner_id": owner_id,
        "is_dark": is_dark,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
