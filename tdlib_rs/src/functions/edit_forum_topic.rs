#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Edits title and icon of a topic in a forum supergroup chat or a chat with a bot with topics; for supergroup chats requires can_manage_topics administrator right
/// unless the user is creator of the topic
/// # Arguments
/// * `chat_id` - Identifier of the chat
/// * `forum_topic_id` - Forum topic identifier
/// * `name` - New name of the topic; 0-128 characters. If empty, the previous topic name is kept
/// * `edit_icon_custom_emoji` - Pass true to edit the icon of the topic. Icon of the General topic can't be edited
/// * `icon_custom_emoji_id` - Identifier of the new custom emoji for topic icon; pass 0 to remove the custom emoji. Ignored if edit_icon_custom_emoji is false. Telegram Premium users can use any custom emoji, other users can use only a custom emoji returned by getForumTopicDefaultIcons
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn edit_forum_topic(chat_id: i64, forum_topic_id: i32, name: String, edit_icon_custom_emoji: bool, icon_custom_emoji_id: i64, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "editForumTopic",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "name": name,
        "edit_icon_custom_emoji": edit_icon_custom_emoji,
        "icon_custom_emoji_id": icon_custom_emoji_id,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
