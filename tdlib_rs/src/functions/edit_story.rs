use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes content and caption of a story. Can be called only if story.can_be_edited == true
/// # Arguments
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story to edit
/// * `content` - New content of the story; pass null to keep the current content
/// * `areas` - New clickable rectangle areas to be shown on the story media; pass null to keep the current areas. Areas can't be edited if story content isn't changed
/// * `caption` - New story caption; pass null to keep the current caption
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_story(
    story_poster_chat_id: i64,
    story_id: i32,
    content: Option<crate::enums::InputStoryContent>,
    areas: Option<crate::types::InputStoryAreas>,
    caption: Option<crate::types::FormattedText>,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "editStory",
    "story_poster_chat_id": story_poster_chat_id,
    "story_id": story_id,
    "content": content,
    "areas": areas,
    "caption": caption,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
