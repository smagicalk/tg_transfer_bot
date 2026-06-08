use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Returns a story
/// # Arguments
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Story identifier
/// * `only_local` - Pass true to get only locally available information without sending network requests
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_story(
    story_poster_chat_id: i64,
    story_id: i32,
    only_local: bool,
    client_id: i32,
) -> Result<crate::enums::Story, crate::types::Error> {
    let request = json!({
    "@type": "getStory",
    "story_poster_chat_id": story_poster_chat_id,
    "story_id": story_id,
    "only_local": only_local,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
