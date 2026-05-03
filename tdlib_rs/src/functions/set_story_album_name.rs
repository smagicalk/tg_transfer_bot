use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes name of an album of stories. If the album is owned by a supergroup or a channel chat, then requires can_edit_stories administrator right in the chat. Returns the changed album
/// # Arguments
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_id` - Identifier of the story album
/// * `name` - New name of the album; 1-12 characters
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_story_album_name(
    chat_id: i64,
    story_album_id: i32,
    name: String,
    client_id: i32,
) -> Result<crate::enums::StoryAlbum, crate::types::Error> {
    let request = json!({
    "@type": "setStoryAlbumName",
    "chat_id": chat_id,
    "story_album_id": story_album_id,
    "name": name,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
