use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes all messages between the specified dates in the topic in a channel direct messages chat administered by the current user. Messages sent in the last 30 seconds will not be deleted
/// # Arguments
/// * `chat_id` - Chat identifier of the channel direct messages chat
/// * `topic_id` - Identifier of the topic which messages will be deleted
/// * `min_date` - The minimum date of the messages to delete
/// * `max_date` - The maximum date of the messages to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_direct_messages_chat_topic_messages_by_date(
    chat_id: i64,
    topic_id: i64,
    min_date: i32,
    max_date: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteDirectMessagesChatTopicMessagesByDate",
    "chat_id": chat_id,
    "topic_id": topic_id,
    "min_date": min_date,
    "max_date": max_date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
