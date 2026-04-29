#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Starts a new live story on behalf of a chat; requires can_post_stories administrator right for channel chats
/// # Arguments
/// * `chat_id` - Identifier of the chat that will start the live story. Pass Saved Messages chat identifier when starting a live story on behalf of the current user, or a channel chat identifier
/// * `privacy_settings` - The privacy settings for the story; ignored for stories posted on behalf of channel chats
/// * `protect_content` - Pass true if the content of the story must be protected from screenshotting
/// * `is_rtmp_stream` - Pass true to create an RTMP stream instead of an ordinary group call
/// * `enable_messages` - Pass true to allow viewers of the story to send messages
/// * `paid_message_star_count` - The minimum number of Telegram Stars that must be paid by viewers for each sent message to the call; 0-getOption("paid_group_call_message_star_count_max")
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn start_live_story(chat_id: i64, privacy_settings: crate::enums::StoryPrivacySettings, protect_content: bool, is_rtmp_stream: bool, enable_messages: bool, paid_message_star_count: i64, client_id: i32) -> Result<crate::enums::StartLiveStoryResult, crate::types::Error> {
    let request = json!({
        "@type": "startLiveStory",
        "chat_id": chat_id,
        "privacy_settings": privacy_settings,
        "protect_content": protect_content,
        "is_rtmp_stream": is_rtmp_stream,
        "enable_messages": enable_messages,
        "paid_message_star_count": paid_message_star_count,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
