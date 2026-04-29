#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reports a sponsored message to Telegram moderators
/// # Arguments
/// * `chat_id` - Chat identifier of the sponsored message
/// * `message_id` - Identifier of the sponsored message
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_chat_sponsored_message(chat_id: i64, message_id: i64, option_id: String, client_id: i32) -> Result<crate::enums::ReportSponsoredResult, crate::types::Error> {
    let request = json!({
        "@type": "reportChatSponsoredMessage",
        "chat_id": chat_id,
        "message_id": message_id,
        "option_id": option_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
