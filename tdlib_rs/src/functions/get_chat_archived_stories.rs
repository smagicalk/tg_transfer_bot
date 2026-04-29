#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of all stories posted by the given chat; requires can_edit_stories administrator right in the chat.
/// The stories are returned in reverse chronological order (i.e., in order of decreasing story_id). For optimal performance, the number of returned stories is chosen by TDLib
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `from_story_id` - Identifier of the story starting from which stories must be returned; use 0 to get results from the last story
/// * `limit` - The maximum number of stories to be returned.
    /// For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_archived_stories(chat_id: i64, from_story_id: i32, limit: i32, client_id: i32) -> Result<crate::enums::Stories, crate::types::Error> {
    let request = json!({
        "@type": "getChatArchivedStories",
        "chat_id": chat_id,
        "from_story_id": from_story_id,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
