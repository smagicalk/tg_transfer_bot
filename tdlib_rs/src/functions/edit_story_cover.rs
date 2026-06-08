use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes cover of a video story. Can be called only if story.can_be_edited == true and the story isn't being edited now
/// # Arguments
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story to edit
/// * `cover_frame_timestamp` - New timestamp of the frame, which will be used as video thumbnail
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_story_cover(
    story_poster_chat_id: i64,
    story_id: i32,
    cover_frame_timestamp: f64,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "editStoryCover",
    "story_poster_chat_id": story_poster_chat_id,
    "story_id": story_id,
    "cover_frame_timestamp": cover_frame_timestamp,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
