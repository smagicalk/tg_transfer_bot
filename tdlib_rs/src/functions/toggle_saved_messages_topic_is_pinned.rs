#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the pinned state of a Saved Messages topic. There can be up to getOption("pinned_saved_messages_topic_count_max") pinned topics. The limit can be increased with Telegram Premium
/// # Arguments
/// * `saved_messages_topic_id` - Identifier of Saved Messages topic to pin or unpin
/// * `is_pinned` - Pass true to pin the topic; pass false to unpin it
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_saved_messages_topic_is_pinned(saved_messages_topic_id: i64, is_pinned: bool, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "toggleSavedMessagesTopicIsPinned",
        "saved_messages_topic_id": saved_messages_topic_id,
        "is_pinned": is_pinned,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
