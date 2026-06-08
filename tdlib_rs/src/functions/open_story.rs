use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Informs TDLib that a story is opened and is being viewed by the user
/// # Arguments
/// * `story_poster_chat_id` - The identifier of the chat that posted the opened story
/// * `story_id` - The identifier of the story
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn open_story(
    story_poster_chat_id: i64,
    story_id: i32,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "openStory",
    "story_poster_chat_id": story_poster_chat_id,
    "story_id": story_id,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
