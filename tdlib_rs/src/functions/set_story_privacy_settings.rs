use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes privacy settings of a story. The method can be called only for stories posted on behalf of the current user and if story.can_set_privacy_settings == true
/// # Arguments
/// * `story_id` - Identifier of the story
/// * `privacy_settings` - The new privacy settings for the story
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_story_privacy_settings(
    story_id: i32,
    privacy_settings: crate::enums::StoryPrivacySettings,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setStoryPrivacySettings",
    "story_id": story_id,
    "privacy_settings": privacy_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
