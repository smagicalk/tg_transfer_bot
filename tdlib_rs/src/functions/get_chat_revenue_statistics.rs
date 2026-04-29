#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns detailed revenue statistics about a chat. Currently, this method can be used only
/// for channels if supergroupFullInfo.can_get_revenue_statistics == true or bots if userFullInfo.bot_info.can_get_revenue_statistics == true
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_revenue_statistics(chat_id: i64, is_dark: bool, client_id: i32) -> Result<crate::enums::ChatRevenueStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getChatRevenueStatistics",
        "chat_id": chat_id,
        "is_dark": is_dark,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
