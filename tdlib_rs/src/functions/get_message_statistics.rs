#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns detailed statistics about a message. Can be used only if messageProperties.can_get_statistics == true
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `message_id` - Message identifier
/// * `is_dark` - Pass true if a dark theme is used by the application
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_message_statistics(chat_id: i64, message_id: i64, is_dark: bool, client_id: i32) -> Result<crate::enums::MessageStatistics, crate::types::Error> {
    let request = json!({
        "@type": "getMessageStatistics",
        "chat_id": chat_id,
        "message_id": message_id,
        "is_dark": is_dark,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
