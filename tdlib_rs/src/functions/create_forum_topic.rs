use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Creates a topic in a forum supergroup chat or a chat with a bot with topics; requires can_manage_topics administrator or can_create_topics member right in the supergroup
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `name` - Name of the topic; 1-128 characters
/// * `is_name_implicit` - Pass true if the name of the topic wasn't entered explicitly; for chats with bots only
/// * `icon` - Icon of the topic. Icon color must be one of 0x6FB9F0, 0xFFD67E, 0xCB86DB, 0x8EEE98, 0xFF93B2, or 0xFB6F5F. Telegram Premium users can use any custom emoji as topic icon, other users can use only a custom emoji returned by getForumTopicDefaultIcons
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn create_forum_topic(
    chat_id: i64,
    name: String,
    is_name_implicit: bool,
    icon: crate::types::ForumTopicIcon,
    client_id: i32,
) -> Result<crate::enums::ForumTopicInfo, crate::types::Error> {
    let request = json!({
    "@type": "createForumTopic",
    "chat_id": chat_id,
    "name": name,
    "is_name_implicit": is_name_implicit,
    "icon": icon,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(serde_json::from_value(response).unwrap())
}
