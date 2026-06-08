use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Searches for public stories containing the given hashtag or cashtag. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// # Arguments
/// * `story_poster_chat_id` - Identifier of the chat that posted the stories to search for; pass 0 to search stories in all chats
/// * `tag` - Hashtag or cashtag to search for
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of stories to be returned; up to 100. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn search_public_stories_by_tag(
    story_poster_chat_id: i64,
    tag: String,
    offset: String,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::FoundStories, crate::types::Error> {
    let request = json!({
    "@type": "searchPublicStoriesByTag",
    "story_poster_chat_id": story_poster_chat_id,
    "tag": tag,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
