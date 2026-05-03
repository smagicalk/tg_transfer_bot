use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates an album of stories; requires can_edit_stories administrator right for supergroup and channel chats
/// # Arguments
/// * `story_poster_chat_id` - Identifier of the chat that posted the stories
/// * `name` - Name of the album; 1-12 characters
/// * `story_ids` - Identifiers of stories to add to the album; 0-getOption("story_album_size_max") identifiers
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_story_album(
    story_poster_chat_id: i64,
    name: String,
    story_ids: Vec<i32>,
    client_id: i32,
) -> Result<crate::enums::StoryAlbum, crate::types::Error> {
    let request = json!({
    "@type": "createStoryAlbum",
    "story_poster_chat_id": story_poster_chat_id,
    "name": name,
    "story_ids": story_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
