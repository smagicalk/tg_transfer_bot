#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Changes the notification settings of a forum topic in a forum supergroup chat or a chat with a bot with topics
/// # Arguments
/// * `chat_id` - Chat identifier
/// * `forum_topic_id` - Forum topic identifier
/// * `notification_settings` - New notification settings for the forum topic. If the topic is muted for more than 366 days, it is considered to be muted forever
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_forum_topic_notification_settings(chat_id: i64, forum_topic_id: i32, notification_settings: crate::types::ChatNotificationSettings, client_id: i32) -> Result<(), crate::types::Error> {
    let request = json!({
        "@type": "setForumTopicNotificationSettings",
        "chat_id": chat_id,
        "forum_topic_id": forum_topic_id,
        "notification_settings": notification_settings,
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(())
}
