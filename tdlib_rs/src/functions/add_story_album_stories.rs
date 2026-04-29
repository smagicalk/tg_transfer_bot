#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Adds stories to the beginning of a previously created story album. If the album is owned by a supergroup or a channel chat, then
/// requires can_edit_stories administrator right in the chat. Returns the changed album
/// # Arguments
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_id` - Identifier of the story album
/// * `story_ids` - Identifier of the stories to add to the album; 1-getOption("story_album_size_max") identifiers.
    /// If after addition the album has more than getOption("story_album_size_max") stories, then the last one are removed from the album
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn add_story_album_stories(chat_id: i64, story_album_id: i32, story_ids: Vec<i32>, client_id: i32) -> Result<crate::enums::StoryAlbum, crate::types::Error> {
    let request = json!({
        "@type": "addStoryAlbumStories",
        "chat_id": chat_id,
        "story_album_id": story_album_id,
        "story_ids": story_ids,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
