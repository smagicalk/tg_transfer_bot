#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reports a chat to the Telegram moderators. A chat can be reported only from the chat action bar, or if chat.can_be_reported
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `option_id` - Option identifier chosen by the user; leave empty for the initial request
/// * `message_ids` - Identifiers of reported messages. Use messageProperties.can_report_chat to check whether the message can be reported
/// * `text` - Additional report details if asked by the server; 0-1024 characters; leave empty for the initial request
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_chat(chat_id: i64, option_id: String, message_ids: Vec<i64>, text: String, client_id: i32) -> Result<crate::enums::ReportChatResult, crate::types::Error> {
    let request = json!({
        "@type": "reportChat",
        "chat_id": chat_id,
        "option_id": option_id,
        "message_ids": message_ids,
        "text": text,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
