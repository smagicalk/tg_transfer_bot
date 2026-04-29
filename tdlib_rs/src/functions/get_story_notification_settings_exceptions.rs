#[allow(clippy::all)]
    use serde_json::json;
    use crate::send_request;
/// Returns the list of chats with non-default notification settings for stories
/// # Arguments
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn get_story_notification_settings_exceptions(client_id: i32) -> Result<crate::enums::Chats, crate::types::Error> {
    let request = json!({
        "@type": "getStoryNotificationSettingsExceptions",
        });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap())
    }
    Ok(serde_json::from_value(response).unwrap())
}
