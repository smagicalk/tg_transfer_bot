use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of stories that posted by the given chat to its chat page. If from_story_id == 0, then pinned stories are returned first.
/// Then, stories are returned in reverse chronological order (i.e., in order of decreasing story_id). For optimal performance, the number of returned stories is chosen by TDLib
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `from_story_id` - Identifier of the story starting from which stories must be returned; use 0 to get results from pinned and the newest story
/// * `limit` - The maximum number of stories to be returned.
/// For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_chat_posted_to_chat_page_stories(
    chat_id: i64,
    from_story_id: i32,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Stories, crate::types::Error> {
    let request = json!({
    "@type": "getChatPostedToChatPageStories",
    "chat_id": chat_id,
    "from_story_id": from_story_id,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
