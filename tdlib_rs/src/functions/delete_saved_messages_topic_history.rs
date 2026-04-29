#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes all messages in a Saved Messages topic
/// # Arguments
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic which messages will be deleted
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_saved_messages_topic_history(saved_messages_topic_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteSavedMessagesTopicHistory",
        "saved_messages_topic_id": saved_messages_topic_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
