use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes order of story albums. If the albums are owned by a supergroup or a channel chat, then requires can_edit_stories administrator right in the chat
/// # Arguments
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_ids` - New order of story albums
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn reorder_story_albums(
    chat_id: i64,
    story_album_ids: Vec<i32>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "reorderStoryAlbums",
    "chat_id": chat_id,
    "story_album_ids": story_album_ids,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
