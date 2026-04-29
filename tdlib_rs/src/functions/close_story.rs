#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Informs TDLib that a story is closed by the user
/// # Arguments
/// * `story_poster_chat_id` - The identifier of the poster of the story to close
/// * `story_id` - The identifier of the story
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn close_story(story_poster_chat_id: i64, story_id: i32, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "closeStory",
        "story_poster_chat_id": story_poster_chat_id,
        "story_id": story_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
