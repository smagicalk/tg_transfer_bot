use crate::send_request;
#[allow(clippy::all)]
use serde_json::json;
/// Changes notification settings for reactions
/// # Arguments
/// * `notification_settings` - The new notification settings for reactions
/// * `client_id` - The client id to send the request to
#[allow(clippy::too_many_arguments)]
pub async fn set_reaction_notification_settings(
    notification_settings: crate::types::ReactionNotificationSettings,
    client_id: i32,
) -> Result<(), crate::types::Error> {
    let request = json!({
    "@type": "setReactionNotificationSettings",
    "notification_settings": notification_settings,
    });
    let response = send_request(client_id, request).await;
    if response["@type"] == "error" {
        return Err(serde_json::from_value(response).unwrap());
    }
    Ok(())
}
