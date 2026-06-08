use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the last message sent in the topic in a channel direct messages chat administered by the current user no later than the specified date
/// # Arguments
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Identifier of the topic which messages will be fetched
/// * `date` - Point in time (Unix timestamp) relative to which to search for messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_direct_messages_chat_topic_message_by_date(
    chat_id: i64,
    topic_id: i64,
    date: i32,
    client_id: i32,
) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
    "@type": "getDirectMessagesChatTopicMessageByDate",
    "chat_id": chat_id,
    "topic_id": topic_id,
    "date": date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
