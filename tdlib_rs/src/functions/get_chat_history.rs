#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns messages in a chat. The messages are returned in reverse chronological order (i.e., in order of decreasing message_id).
/// For optimal performance, the number of returned messages is chosen by TDLib. This is an offline method if only_local is true
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `from_message_id` - Identifier of the message starting from which history must be fetched; use 0 to get results from the last message
/// * `offset` - Specify 0 to get results from exactly the message from_message_id or a negative number from -99 to -1 to get additionally -offset newer messages
/// * `limit` - The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, then the limit must be greater than or equal to -offset.
    /// For optimal performance, the number of returned messages is chosen by TDLib and can be smaller than the specified limit
/// * `only_local` - Pass true to get only messages that are available without sending network requests
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_history(chat_id: i64, from_message_id: i64, offset: i32, limit: i32, only_local: bool, client_id: i32) -> Result<crate::enums::Messages, crate::types::Error> {
    let request = json!({
        "@type": "getChatHistory",
        "chat_id": chat_id,
        "from_message_id": from_message_id,
        "offset": offset,
        "limit": limit,
        "only_local": only_local,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
