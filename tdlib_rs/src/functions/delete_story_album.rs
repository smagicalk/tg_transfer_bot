#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes a story album. If the album is owned by a supergroup or a channel chat, then requires can_edit_stories administrator right in the chat
/// # Arguments
/// * `chat_id` - Identifier of the chat that owns the stories
/// * `story_album_id` - Identifier of the story album
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_story_album(chat_id: i64, story_album_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteStoryAlbum",
        "chat_id": chat_id,
        "story_album_id": story_album_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
