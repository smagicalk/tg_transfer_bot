#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns forwards of a story as a message to public chats and reposts by public channels. Can be used only if the story is posted on behalf of the current user or story.can_get_statistics == true.
/// For optimal performance, the number of returned messages and stories is chosen by TDLib
/// # Arguments
/// * `story_poster_chat_id` - The identifier of the poster of the story
/// * `story_id` - The identifier of the story
/// * `offset` - Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
/// * `limit` - The maximum number of messages and stories to be returned; must be positive and can't be greater than 100. For optimal performance, the number of returned objects is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_story_public_forwards(story_poster_chat_id: i64, story_id: i32, offset: String, limit: i32, client_id: i32) -> Result<crate::enums::PublicForwards, crate::types::Error> {
    let request = json!({
        "@type": "getStoryPublicForwards",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        "offset": offset,
        "limit": limit,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
