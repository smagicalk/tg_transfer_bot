use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns the list of stories added to the given story album. For optimal performance, the number of returned stories is chosen by TDLib
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `story_album_id` - Story album identifier
/// * `offset` - Offset of the first entry to return; use 0 to get results from the first album story
/// * `limit` - The maximum number of stories to be returned. For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_story_album_stories(
    chat_id: i64,
    story_album_id: i32,
    offset: i32,
    limit: i32,
    client_id: i32,
) -> Result<crate::enums::Stories, crate::types::Error> {
    let request = json!({
    "@type": "getStoryAlbumStories",
    "chat_id": chat_id,
    "story_album_id": story_album_id,
    "offset": offset,
    "limit": limit,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
