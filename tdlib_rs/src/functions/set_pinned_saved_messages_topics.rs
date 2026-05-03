use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes the order of pinned Saved Messages topics
/// # Arguments
/// * `saved_messages_topic_ids` - Identifiers of the new pinned Saved Messages topics
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_pinned_saved_messages_topics(
    saved_messages_topic_ids: Vec<i64>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setPinnedSavedMessagesTopics",
    "saved_messages_topic_ids": saved_messages_topic_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
