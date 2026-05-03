use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Removes stories from an album. If the album is owned by a supergroup or a channel chat, then
/// requires can_edit_stories administrator right in the chat. Returns the changed album
/// # Arguments
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_id` - Identifier of the story album
/// * `story_ids` - Identifier of the stories to remove from the album
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn remove_story_album_stories(
    chat_id: i64,
    story_album_id: i32,
    story_ids: Vec<i32>,
    client_id: i32,
) -> Result<crate::enums::StoryAlbum, crate::types::Error> {
    let request = json!({
    "@type": "removeStoryAlbumStories",
    "chat_id": chat_id,
    "story_album_id": story_album_id,
    "story_ids": story_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
