use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Deletes all messages between the specified dates in a Saved Messages topic. Messages sent in the last 30 seconds will not be deleted
/// # Arguments
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic which messages will be deleted
/// * `min_date` - The minimum date of the messages to delete
/// * `max_date` - The maximum date of the messages to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_saved_messages_topic_messages_by_date(
    saved_messages_topic_id: i64,
    min_date: i32,
    max_date: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "deleteSavedMessagesTopicMessagesByDate",
    "saved_messages_topic_id": saved_messages_topic_id,
    "min_date": min_date,
    "max_date": max_date,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
