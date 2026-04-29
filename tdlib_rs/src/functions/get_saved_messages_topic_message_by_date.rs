#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the last message sent in a Saved Messages topic no later than the specified date
/// # Arguments
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic which message will be returned
/// * `date` - Point in time (Unix timestamp) relative to which to search for messages
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_saved_messages_topic_message_by_date(saved_messages_topic_id: i64, date: i32, client_id: i32) -> Result<crate::enums::Message, crate::types::Error> {
    let request = json!({
        "@type": "getSavedMessagesTopicMessageByDate",
        "saved_messages_topic_id": saved_messages_topic_id,
        "date": date,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
