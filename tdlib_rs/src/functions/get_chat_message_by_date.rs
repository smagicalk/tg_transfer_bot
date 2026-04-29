#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the last message sent in a chat no later than the specified date. Returns a 404 error if such message doesn't exist
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `date` - Point in time (Unix timestamp) relative to which to search for messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_message_by_date(chat_id: i64, date: i32, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getChatMessageByDate",
        "chat_id": chat_id,
        "date": date,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
