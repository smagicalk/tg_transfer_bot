#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Reports a chat photo to the Telegram moderators. A chat photo can be reported only if chat.can_be_reported
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `file_id` - Identifier of the photo to report. Only full photos from chatPhoto can be reported
/// * `reason` - The reason for reporting the chat photo
/// * `text` - Additional report details; 0-1024 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn report_chat_photo(chat_id: i64, file_id: i32, reason: crate::enums::ReportReason, text: String, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "reportChatPhoto",
        "chat_id": chat_id,
        "file_id": file_id,
        "reason": reason,
        "text": text,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
