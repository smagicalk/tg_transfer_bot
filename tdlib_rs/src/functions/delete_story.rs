#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Deletes a previously posted story. Can be called only if story.can_be_deleted == true
/// # Arguments
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story to delete
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn delete_story(story_poster_chat_id: i64, story_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "deleteStory",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
