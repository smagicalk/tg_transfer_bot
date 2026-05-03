use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Toggles whether a story is accessible after expiration. Can be called only if story.can_toggle_is_posted_to_chat_page == true
/// # Arguments
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story
/// * `is_posted_to_chat_page` - Pass true to make the story accessible after expiration; pass false to make it private
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn toggle_story_is_posted_to_chat_page(
    story_poster_chat_id: i64,
    story_id: i32,
    is_posted_to_chat_page: bool,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "toggleStoryIsPostedToChatPage",
    "story_poster_chat_id": story_poster_chat_id,
    "story_id": story_id,
    "is_posted_to_chat_page": is_posted_to_chat_page,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
