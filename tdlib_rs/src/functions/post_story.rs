use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Posts a new story on behalf of a chat; requires can_post_stories administrator right for supergroup and channel chats. Returns a temporary story
/// # Arguments
/// * `chat_id` - Identifier of the chat that will post the story. Pass Saved Messages chat identifier when posting a story on behalf of the current user
/// * `content` - Content of the story
/// * `areas` - Clickable rectangle areas to be shown on the story media; pass null if none
/// * `caption` - Story caption; pass null to use an empty caption; 0-getOption("story_caption_length_max") characters; can have entities only if getOption("can_use_text_entities_in_story_caption")
/// * `privacy_settings` - The privacy settings for the story; ignored for stories posted on behalf of supergroup and channel chats
/// * `album_ids` - Identifiers of story albums to which the story will be added upon posting. An album can have up to getOption("story_album_size_max") stories
/// * `active_period` - Period after which the story is moved to archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400 for Telegram Premium users, and 86400 otherwise
/// * `from_story_full_id` - Full identifier of the original story, which content was used to create the story; pass null if the story isn't repost of another story
/// * `is_posted_to_chat_page` - Pass true to keep the story accessible after expiration
/// * `protect_content` - Pass true if the content of the story must be protected from forwarding and screenshotting
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn post_story(
    chat_id: i64,
    content: crate::enums::InputStoryContent,
    areas: Option<crate::types::InputStoryAreas>,
    caption: Option<crate::types::FormattedText>,
    privacy_settings: crate::enums::StoryPrivacySettings,
    album_ids: Vec<i32>,
    active_period: i32,
    from_story_full_id: Option<crate::types::StoryFullId>,
    is_posted_to_chat_page: bool,
    protect_content: bool,
    client_id: i32,
) -> Result<crate::enums::Story, crate::types::Error> {
    let request = json!({
    "@type": "postStory",
    "chat_id": chat_id,
    "content": content,
    "areas": areas,
    "caption": caption,
    "privacy_settings": privacy_settings,
    "album_ids": album_ids,
    "active_period": active_period,
    "from_story_full_id": from_story_full_id,
    "is_posted_to_chat_page": is_posted_to_chat_page,
    "protect_content": protect_content,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
