use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes a story posted by the bot on behalf of a business account; for bots only
/// # Arguments
/// * `story_poster_chat_id` - Identifier of the chat that posted the story
/// * `story_id` - Identifier of the story to edit
/// * `content` - New content of the story
/// * `areas` - New clickable rectangle areas to be shown on the story media
/// * `caption` - New story caption
/// * `privacy_settings` - The new privacy settings for the story
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_business_story(
    story_poster_chat_id: i64,
    story_id: i32,
    content: crate::enums::InputStoryContent,
    areas: crate::types::InputStoryAreas,
    caption: crate::types::FormattedText,
    privacy_settings: crate::enums::StoryPrivacySettings,
    client_id: i32,
) -> Result<crate::enums::Story, crate::types::Error> {
    let request = json!({
    "@type": "editBusinessStory",
    "story_poster_chat_id": story_poster_chat_id,
    "story_id": story_id,
    "content": content,
    "areas": areas,
    "caption": caption,
    "privacy_settings": privacy_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
